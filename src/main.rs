// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::Parser;
use std::fs;

#[macro_use]
mod message;

mod args;
mod artist;
mod asset;
mod asset_cache;
mod audio_format;
mod audio_meta;
mod build;
mod catalog;
mod decode;
mod deploy;
mod download_option;
mod favicon;
mod feed;
mod ffmpeg;
mod icons;
mod image;
mod image_processor;
mod locale;
mod manifest;
mod markdown;
mod payment_option;
mod permalink;
mod release;
mod render;
mod rsync;
mod server;
mod styles;
mod theme;
mod track;
mod util;

use args::Args;
use artist::Artist;
use asset::{Asset, AssetIntent};
use asset_cache::{Cache, CacheOptimization, SourceFileSignature};
use audio_format::{AudioFormat, DownloadFormat, StreamingQuality};
use audio_meta::AudioMeta;
use build::{Build, PostBuildAction};
use catalog::Catalog;
use download_option::DownloadOption;
use favicon::Favicon;
use ffmpeg::TagMapping;
use crate::image::{Image, ImageAssets};
use image_processor::{ImageInMemory, ImageProcessor, ResizeMode};
use locale::Locale;
use markdown::HtmlAndStripped;
use payment_option::PaymentOption;
use permalink::{Permalink, PermalinkUsage};
use release::{ArchiveAssets, DownloadGranularity, Extra, Release, TrackNumbering};
use render::CrawlerMeta;
use theme::Theme;
use track::{Track, TrackAssets};

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
    
    let mut cache = Cache::retrieve(&build.cache_dir);
    
    if args.analyze_cache {
        build.cache_optimization = CacheOptimization::Immediate;
        asset_cache::report_stale(&cache, &Catalog::new());
        return;
    }
    
    if args.optimize_cache {
        build.cache_optimization = CacheOptimization::Immediate;
        asset_cache::optimize_cache(&build, &mut cache, &mut Catalog::new());
        return;
    }
    
    if args.wipe_all || args.wipe_build || args.wipe_cache {
        if args.wipe_build || args.wipe_all {
            info!("The build directory was wiped, as requested");
            util::remove_dir(&build.build_dir);
        }
        if args.wipe_cache || args.wipe_all {
            info_cache!("The cache directory was wiped, as requested");
            util::remove_dir(&build.cache_dir);
        }
        info!("No further actions are performed due to requested wipe operation(s)");
        return;
    }
    
    cache.mark_all_stale(&build.build_begin);
    
    let mut catalog = match Catalog::read(&mut build, &mut cache) {
        Ok(catalog) => catalog,
        Err(()) => return
    };

    util::ensure_empty_dir(&build.build_dir);

    catalog.write_assets(&mut build);
    
    // Render homepage (page for all releases)
    let index_html = render::index::index_html(&build, &catalog);
    fs::write(build.build_dir.join("index.html"), index_html).unwrap();
    
    // Render page for each release
    for release in &catalog.releases {
        release.borrow_mut().write_files(&mut build, &catalog);
    }

    if let Some(home_image) = &catalog.home_image {
        if home_image.borrow().description.is_none() {
            warn_discouraged!("The catalog home image is missing an image description.");
            build.missing_image_descriptions = true;
        }
    }

    // Render pages for featured artists (these are populated only in label mode)
    for artist in &catalog.featured_artists {
        let artist_ref = artist.borrow();
        if let Some(image) = &artist_ref.image {
            if image.borrow().description.is_none() {
                warn_discouraged!("The image for artist '{}' is missing an image description.", artist_ref.name);
                build.missing_image_descriptions = true;
            }
        }

        let artist_html = render::artist::artist_html(&build, &artist_ref, &catalog);
        let artist_ref = artist.borrow();

        fs::create_dir(build.build_dir.join(&artist_ref.permalink.slug)).unwrap();
        fs::write(build.build_dir.join(&artist_ref.permalink.slug).join("index.html"), artist_html).unwrap();
    }

    // Render image descriptions page (when needed)
    if build.missing_image_descriptions {
        let t_image_descriptions_permalink = &build.locale.translations.image_descriptions_permalink;
        let image_descriptions_dir = build.build_dir.join(t_image_descriptions_permalink);
        let image_descriptions_html = render::image_descriptions::image_descriptions_html(&build, &catalog);
        fs::create_dir(&image_descriptions_dir).unwrap();
        fs::write(image_descriptions_dir.join("index.html"), image_descriptions_html).unwrap();
    }

    fs::write(build.build_dir.join("scripts.js"), include_bytes!("assets/scripts.js")).unwrap();
    
    styles::generate(&build);
    feed::generate(&build, &catalog);
    icons::generate(&build);

    catalog.favicon.write(&build);

    if build.base_url.is_none() {
        if build.embeds_requested {
            warn!("No catalog.base_url specified, embeds for releases and the RSS feed were not generated");
        } else {
            warn!("No catalog.base_url specified, RSS feed was not generated");
        }
    }
    
    match build.cache_optimization {
        CacheOptimization::Default |
        CacheOptimization::Delayed |
        CacheOptimization::Immediate =>
            asset_cache::optimize_cache(&build, &mut cache, &mut catalog),
        CacheOptimization::Manual =>
            asset_cache::report_stale(&cache, &catalog),
        CacheOptimization::Wipe => {
            util::remove_dir(&build.cache_dir);
            info_cache!("Wiped cache");
        }
    }
    
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
