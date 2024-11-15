// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::Parser;
use std::fs;

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
mod download_option;
mod favicon;
mod feed;
mod ffmpeg;
mod heuristic_audio_meta;
mod icons;
mod image;
mod image_processor;
mod link;
mod locale;
mod m3u;
mod manifest;
mod markdown;
mod permalink;
mod release;
mod render;
mod rsync;
mod server;
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
use build::{Build, PostBuildAction};
use cache::{Cache, CacheOptimization, View};
use catalog::Catalog;
use cover_generator::CoverGenerator;
use download_format::DownloadFormat;
use download_option::DownloadOption;
use favicon::Favicon;
use heuristic_audio_meta::HeuristicAudioMeta;
use crate::image::{DescribedImage, Image, ImageRc, ImageRcView};
use image_processor::{ImageInMemory, ImageProcessor, ResizeMode};
use link::Link;
use locale::Locale;
use manifest::Overrides;
use markdown::HtmlAndStripped;
use permalink::{Permalink, PermalinkUsage};
use release::{DownloadGranularity, Extra, Release, ReleaseRc};
use render::CrawlerMeta;
use scripts::Scripts;
use source_file_signature::{FileMeta, SourceHash};
use streaming_quality::StreamingQuality;
use tags::{ImageEmbed, TagAgenda, TagMapping};
use theme::{Theme, ThemeBase, ThemeFont, ThemeVarsHsl, ThemeVarsOklch};
use track::Track;
use track_numbering::TrackNumbering;
use transcodes::{Transcode, Transcodes, TranscodesRc, TranscodesRcView};

const MANUAL_URL: &str = "https://simonrepp.com/faircamp/manual/";

fn main() {
    let args: Args = Args::parse();

    if args.manual {
        if webbrowser::open(MANUAL_URL).is_err() {
            error!("Could not open browser for displaying the manual")
        }
        return;
    }

    let mut build = Build::new(&args);
    
    if !build.catalog_dir.is_dir() {
        error!("Configured catalog directory does not exist - aborting build");
        return;
    }

    let mut cache = Cache::retrieve(&build);
    
    if args.analyze_cache {
        cache.report_stale();
        return;
    }
    
    if args.optimize_cache {
        cache.optimization = CacheOptimization::Immediate;
        cache.maintain(&build);
        return;
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
        return;
    }
    
    cache.mark_all_stale(&build.build_begin);
    
    let mut catalog = match Catalog::read(&mut build, &mut cache) {
        Ok(catalog) => catalog,
        Err(()) => return
    };

    if args.debug {
        debug::debug_catalog(&catalog);
        return;
    }

    util::ensure_empty_dir(&build.build_dir);

    scripts::generate(&mut build, &catalog);
    styles::generate(&mut build, &catalog);
    catalog.favicon.write(&mut build);
    catalog.write_assets(&mut build);

    // Render homepage (page for all releases)
    let index_html = render::index::index_html(&build, &catalog);
    fs::write(build.build_dir.join("index.html"), index_html).unwrap();
    
    // Render page for each release
    for release in &catalog.releases {
        release.borrow_mut().write_files(&mut build, &catalog);
    }

    // Render pages for featured artists (these are populated only in label mode)
    for artist in &catalog.featured_artists {
        let artist_ref = artist.borrow();
        let artist_html = render::artist::artist_html(&build, &artist_ref, &catalog);
        fs::create_dir(build.build_dir.join(&artist_ref.permalink.slug)).unwrap();
        fs::write(build.build_dir.join(&artist_ref.permalink.slug).join("index.html"), artist_html).unwrap();
    }

    // Render image descriptions page (when needed)
    if build.missing_image_descriptions {
        let t_image_descriptions_permalink = *build.locale.translations.image_descriptions_permalink;
        let image_descriptions_dir = build.build_dir.join(t_image_descriptions_permalink);
        let image_descriptions_html = render::image_descriptions::image_descriptions_html(&build, &catalog);
        fs::create_dir(&image_descriptions_dir).unwrap();
        fs::write(image_descriptions_dir.join("index.html"), image_descriptions_html).unwrap();
    }

    if catalog.feed_enabled {
        feed::generate(&build, &catalog);
    }

    if build.base_url.is_none() {
        if build.embeds_requested {
            warn!("No catalog.base_url specified, embeds for releases and the RSS feed were not generated");
        } else {
            warn!("No catalog.base_url specified, RSS feed was not generated");
        }
    }
    
    cache.maintain(&build);

    build.print_stats();

    match build.post_build_action {
        PostBuildAction::None => (),
        PostBuildAction::Deploy => {
            if build.theming_widget {
                // TODO: But maybe someone *wants* to deploy it to a "live" page, e.g. to ask their bandmates for their color preferences? Follow up again :)
                error!("Aborting deploy because --theming-widget is enabled, we probably don't want that on the live page.")
            } else {
                deploy::deploy(&build);
            }
        },
        PostBuildAction::Preview { port } => {
            if build.clean_urls || build.theming_widget {
                // Here we serve the preview through an actual http server. In
                // the case of clean urls, so that /foo/ gets resolved
                // to /foo/index.html. In the case of the theming widget,
                // because it can only retain its localStorage state across
                // pages if the origin (in this case http://localhost:xxxx/) is
                // stable (and not file://...).
                server::serve_preview(&build.build_dir, port);
            } else {
                // We don't need an actively running server to preview a build
                // without clean urls, we can just open everything directly in
                // a browser.
                let local_file_url = build.build_dir.join("index.html");
                if webbrowser::open(&local_file_url.to_string_lossy()).is_err() {
                    error!("Could not open browser for previewing the site")
                }
            }
        }
    }
}
