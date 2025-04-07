// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-FileCopyrightText: 2025 Sandro Santilli
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::process::ExitCode;

use clap::Parser;
use indoc::formatdoc;

#[macro_use]
mod message;

mod archives;
mod args;
mod artist;
mod asset;
mod audio_format;
mod audio_meta;
mod build;
mod cache;
mod catalog;
mod cover_generator;
mod debug;
mod decode;
mod deploy;
mod download_format;
mod downloads;
mod fair_dir;
mod favicon;
mod feeds;
mod ffmpeg;
mod heuristic_audio_meta;
mod icons;
mod image;
mod link;
mod locale;
mod m3u;
mod manifest;
mod markdown;
mod opengraph;
mod permalink;
mod release;
mod render;
mod rsync;
mod server;
mod site_metadata;
mod site_url;
mod source_file_signature;
mod streaming_quality;
mod scripts;
mod styles;
mod tags;
mod theme;
mod track;
mod track_numbering;
mod transcodes;
mod util;

use archives::{Archive, Archives, ArchivesRc};
use args::Args;
use artist::{Artist, ArtistRc};
use asset::{Asset, AssetIntent};
use audio_format::{AudioFormat, AudioFormatFamily};
use audio_meta::AudioMeta;
use build::{AssetHashes, Build, GENERATOR_INFO, PostBuildAction};
use cache::{Cache, CacheOptimization, View};
use catalog::Catalog;
use cover_generator::{CoverGenerator, ProceduralCover, ProceduralCoverAsset, ProceduralCoverRc};
use download_format::DownloadFormat;
use downloads::{DownloadAccess, DownloadAccessOption, ExtraDownloads, Price};
use fair_dir::FairDir;
use favicon::Favicon;
use feeds::Feeds;
use heuristic_audio_meta::HeuristicAudioMeta;
use crate::image::{DescribedImage, FeedImageAsset, Image, ImageProcessor, ImageRc, ImageRcView, ImgAttributes};
use link::Link;
use locale::Locale;
use m3u::M3U_PLAYLIST_FILENAME;
use manifest::{LocalOptions, Overrides};
use markdown::HtmlAndStripped;
use opengraph::{OpenGraphImage, OpenGraphMeta};
use permalink::{Permalink, PermalinkUsage};
use release::{Extra, Release, ReleaseRc, TRACK_NUMBERS};
use site_metadata::{SiteAsset, SiteMetadata};
use site_url::SiteUrl;
use source_file_signature::{FileMeta, SourceHash};
use streaming_quality::StreamingQuality;
use tags::{ImageEmbed, TagAgenda, TagMapping};
use theme::{Theme, ThemeBase, ThemeFont, ThemeVarsHsl, ThemeVarsOklch};
use track::Track;
use track_numbering::TrackNumbering;
use transcodes::{Transcode, Transcodes, TranscodesRc, TranscodesRcView};

const MANUAL_URL: &str = "https://simonrepp.com/faircamp/manual/";

fn main() -> ExitCode {
    let args: Args = Args::parse();

    if args.manual {
        if webbrowser::open(MANUAL_URL).is_err() {
            error!("Could not open browser for displaying the manual");
            return ExitCode::FAILURE;
        } else {
            return ExitCode::SUCCESS;
        }
    }

    let mut build = Build::new(&args);

    if !build.catalog_dir.is_dir() {
        error!("Configured catalog directory does not exist - aborting build");
        return ExitCode::FAILURE;
    }

    info!("You can safely terminate faircamp at any point (using Ctrl+C) - all progress is continuously saved and new builds always continue where the previous build left off.");

    let mut cache = Cache::retrieve(&build);

    if args.analyze_cache {
        cache.report_stale();
        return ExitCode::SUCCESS;
    }

    if args.optimize_cache {
        cache.optimization = CacheOptimization::Immediate;
        cache.maintain(&build);
        return ExitCode::SUCCESS;
    }

    if args.wipe_all || args.wipe_build || args.wipe_cache {
        if args.wipe_build || args.wipe_all {
            info!("The build directory was wiped, as requested");
            let _ = fs::remove_dir_all(&build.build_dir);
        }
        if args.wipe_cache || args.wipe_all {
            info_cache!("The cache directory was wiped, as requested");
            let _ = fs::remove_dir_all(&build.cache_dir);
        }
        info!("No further actions are performed due to requested wipe operation(s)");
        return ExitCode::SUCCESS;
    }

    cache.mark_all_stale(&build.build_begin);

    let mut catalog = match Catalog::read(&mut build, &mut cache) {
        Ok(catalog) => catalog,
        Err(()) => return ExitCode::FAILURE
    };

    if args.debug {
        debug::debug_catalog(&catalog);
        return ExitCode::SUCCESS;
    }

    util::ensure_empty_dir(&build.build_dir);

    // Generation of scripts depends on final image assets and paths being
    // available, hence the assets (audio and image files) are the first
    // thing we compute.
    catalog.write_assets(&mut build, &mut cache);

    // Rendering of the actual pages (html) depends on assets hashes
    // (for css/favicon/js assets) being available, hence these are the
    // second thing we compute.
    scripts::generate(&mut build, &catalog);
    styles::generate(&mut build, &catalog);
    catalog.favicon.write(&mut build);

    if build.base_url.is_some() {
        // Render M3U playlist
        if catalog.m3u {
            let r_m3u = m3u::generate_for_catalog(&build, &catalog);
            fs::write(build.build_dir.join(M3U_PLAYLIST_FILENAME), r_m3u).unwrap();
            build.reserve_filename(M3U_PLAYLIST_FILENAME);
        }

        if catalog.feeds.any_requested() {
            // Render feed (xml) files (Atom, Generic RSS, Media RSS, Podcast RSS, as enabled)
            catalog.feeds.generate(&mut build, &catalog);

            // Render subscription choices page
            let subscribe_permalink = catalog.subscribe_permalink.as_ref().unwrap();
            let subscribe_dir = build.build_dir.join(subscribe_permalink);
            util::ensure_dir_all(&subscribe_dir);
            let subscribe_html = render::subscribe::subscribe_html(&build, &catalog);
            fs::write(subscribe_dir.join("index.html"), subscribe_html).unwrap();
            build.reserve_filename(subscribe_permalink);
        }
    }

    // Render homepage (page for all releases)
    let index_html = render::index::index_html(&build, &catalog);
    fs::write(build.build_dir.join("index.html"), index_html).unwrap();
    build.reserve_filename("index.html");

    // Render pages for each release (including playlists, track pages, embeds, etc.)
    for release in &catalog.releases {
        let release_mut = release.borrow_mut();
        release_mut.write_pages_and_playlist_files(&mut build, &catalog);
        build.reserve_filename(release_mut.permalink.slug.clone());
    }

    // Render pages for featured artists (these are populated only in label mode)
    for artist in &catalog.featured_artists {
        let artist_ref = artist.borrow();
        let artist_dir = build.build_dir.join(&artist_ref.permalink.slug);

        util::ensure_dir_all(&artist_dir);

        // Render m3u playlist
        if let Some(base_url) = &build.base_url {
            if artist_ref.m3u {
                let r_m3u = m3u::generate_for_artist(&artist_ref, base_url, &build);
                fs::write(artist_dir.join(M3U_PLAYLIST_FILENAME), r_m3u).unwrap();
            }
        }

        let artist_html = render::artist::artist_html(&artist_ref, &build, &catalog);
        fs::write(artist_dir.join("index.html"), artist_html).unwrap();
        build.reserve_filename(artist_ref.permalink.slug.clone());
    }

    // Render image descriptions page (when needed)
    if build.missing_image_descriptions {
        let t_image_descriptions_permalink = *build.locale.translations.image_descriptions_permalink;
        let image_descriptions_dir = build.build_dir.join(t_image_descriptions_permalink);
        let image_descriptions_html = render::image_descriptions::image_descriptions_html(&build, &catalog);
        fs::create_dir(&image_descriptions_dir).unwrap();
        fs::write(image_descriptions_dir.join("index.html"), image_descriptions_html).unwrap();
        build.reserve_filename(t_image_descriptions_permalink);
    }

    // Must be the last step because we need to check for collisions against
    // everything we wrote to the build directory ourselves beforehand.
    if let Err(collisions) = catalog.write_user_assets(&mut build) {
        let collisions_joined = collisions
            .iter()
            .map(|filename| format!("'{filename}'"))
            .collect::<Vec<String>>()
            .join(", ");

        let message = formatdoc!(r#"
            One or more filenames of your custom site assets collide with filenames already used by faircamp itself: {collisions_joined}
            Please rename the respective site assets, making sure to update all references pointing to it (both in site_metadata and in your own files, if applies).
        "#);

        error!("{}", message);
        return ExitCode::FAILURE;
    }

    if build.base_url.is_none() {
        let mut not_generated = Vec::new();

        if build.embeds_requested { not_generated.push("Embeds"); }
        if catalog.opengraph { not_generated.push("Open Graph meta tags"); }
        if catalog.feeds.any_requested() { not_generated.push("Feeds"); }
        if catalog.m3u ||
            catalog.artists.iter().any(|artist| artist.borrow().m3u) ||
            catalog.releases.iter().any(|release| release.borrow().m3u) {
            not_generated.push("M3U playlists");
        }

        if !not_generated.is_empty() {
            let r_not_generated = not_generated.join(", ");
            warn!("No catalog.base_url specified, therefore the following could not be generated: {}", r_not_generated);
        }
    }

    cache.maintain(&build);

    build.print_stats();

    match build.post_build_action {
        PostBuildAction::None => (),
        PostBuildAction::Deploy => {
            if build.theming_widget {
                // TODO: But maybe someone *wants* to deploy it to a "live" page, e.g. to ask their bandmates for their color preferences? Follow up again :)
                error!("Aborting deploy because --theming-widget is enabled, we probably don't want that on the live page.");
                return ExitCode::FAILURE;
            } else {
                deploy::deploy(&build);
            }
        }
        PostBuildAction::Preview { ip, port } => {
            if build.clean_urls || build.theming_widget {
                // Here we serve the preview through an actual http server. In
                // the case of clean urls, so that /foo/ gets resolved
                // to /foo/index.html. In the case of the theming widget,
                // because it can only retain its localStorage state across
                // pages if the origin (in this case http://localhost:xxxx/) is
                // stable (and not file://...).
                server::serve_preview(&build.build_dir, ip, port);
            } else {
                // We don't need an actively running server to preview a build
                // without clean urls, we can just open everything directly in
                // a browser.
                let local_file_url = build.build_dir.join("index.html");
                if webbrowser::open(&local_file_url.to_string_lossy()).is_err() {
                    error!("Could not open browser for previewing the site");
                    return ExitCode::FAILURE
                }
            }
        }
    }

    ExitCode::SUCCESS
}
