#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use chrono::Utc;
use clap::Clap;
use std::fs;

mod args;
mod artist;
mod asset_cache;
mod audio_format;
mod audio_meta;
mod build_settings;
mod catalog;
mod deploy;
mod download_option;
mod eno;
mod feed;
mod ffmpeg;
mod image;
mod image_format;
mod manifest;
mod message;
mod payment_option;
mod release;
mod render;
mod rsync;
mod styles;
mod track;
mod util;

use args::Args;
use asset_cache::{CacheManifest, CacheOptimization};
use ffmpeg::MediaFormat;
use image_format::ImageFormat;
use build_settings::{BuildSettings, PostBuildAction};
use catalog::Catalog;

fn main() {
    env_logger::init();
    
    let args: Args = Args::parse();
    let mut build_settings = BuildSettings::init(&args);
    
    CacheManifest::ensure_dirs(&build_settings.cache_dir);
    let mut cache_manifest = CacheManifest::retrieve(&build_settings.cache_dir);
    
    if args.analyze_cache {
        asset_cache::analyze_cache(&cache_manifest, &Catalog::init_empty());
        return;
    }
    
    if args.optimize_cache {
        asset_cache::optimize_cache(
            &build_settings.cache_dir,
            &mut cache_manifest,
            &CacheOptimization::Immediate,
            &mut Catalog::init_empty()
        );
        return;
    }
    
    if args.wipe_all || args.wipe_build || args.wipe_cache {
        if args.wipe_build || args.wipe_all {
            util::remove_dir(&build_settings.build_dir);
        }
        if args.wipe_cache || args.wipe_all {
            util::remove_dir(&build_settings.cache_dir);
        }
        return;
    }
    
    cache_manifest.mark_all_stale(&Utc::now());
    
    let mut catalog = Catalog::read(&mut build_settings, &mut cache_manifest);
    
    util::ensure_empty_dir(&build_settings.build_dir);
    
    if args.wipe_cache {
        util::ensure_empty_dir(&build_settings.cache_dir);
    } else {
        util::ensure_dir(&build_settings.cache_dir);
    }
    
    util::ensure_dir(&build_settings.build_dir.join("download"));
    catalog.write_assets(&mut build_settings);

    // Render about page
    let about_html = render::render_about(&build_settings, &catalog);
    fs::create_dir(build_settings.build_dir.join("about")).unwrap();
    fs::write(build_settings.build_dir.join("about/index.html"), about_html).unwrap();
    
    // Render page for all artists
    let artists_html = render::render_artists(&build_settings, &catalog);
    fs::create_dir(build_settings.build_dir.join("artists")).unwrap();
    fs::write(build_settings.build_dir.join("artists/index.html"), artists_html).unwrap();
    
    // Render page for all releases
    let releases_html = render::render_releases(&build_settings, &catalog);
    fs::write(build_settings.build_dir.join("index.html"), releases_html).unwrap();
    
    // Render page for each artist
    for artist in &catalog.artists {
        let artist_html = render::render_artist(&build_settings, &artist, &catalog);
        fs::create_dir(build_settings.build_dir.join(&artist.slug)).unwrap();
        fs::write(build_settings.build_dir.join(&artist.slug).join("index.html"), artist_html).unwrap();
    }
    
    // Render page for each release
    for release in &catalog.releases {
        release.write_files(&build_settings, &catalog);
    }
    
    if let Some(background_image) = &build_settings.background_image {
        ffmpeg::transcode(
            &build_settings.catalog_dir.join(background_image),
            &build_settings.build_dir.join("background.jpg"),
            MediaFormat::Image(&ImageFormat::Jpeg)
        ).unwrap();
    }

    fs::write(build_settings.build_dir.join("barlow-v5-latin-regular.woff2"), include_bytes!("assets/barlow-v5-latin-regular.woff2")).unwrap();
    fs::write(build_settings.build_dir.join("scripts.js"), include_bytes!("assets/scripts.js")).unwrap();
    
    let css = styles::generate(&build_settings.theme, build_settings.background_image.is_some());
    fs::write(build_settings.build_dir.join("styles.css"), css).unwrap();
    
    if let Some(base_url) = &build_settings.base_url {
        let feed_xml = feed::generate(base_url, &catalog);
        fs::write(build_settings.build_dir.join("feed.rss"), feed_xml).unwrap();
    } else {
        message::warning(&format!("No base_url specified, skipping RSS feed generation"));
    }
    
    match build_settings.cache_optimization {
        CacheOptimization::Delayed | CacheOptimization::Immediate => {
            asset_cache::optimize_cache(
                &build_settings.cache_dir,
                &mut cache_manifest,
                &build_settings.cache_optimization,
                &mut catalog
            );
        }
        CacheOptimization::Manual => asset_cache::analyze_cache(&cache_manifest, &catalog),
        CacheOptimization::Wipe => {
            util::remove_dir(&build_settings.cache_dir);
            message::cache(&format!("Wiped cache"));
        }
    }
    
    build_settings.print_stats();
    
    match build_settings.post_build_action {
        PostBuildAction::None => (),
        PostBuildAction::Deploy => deploy::deploy(&build_settings),
        PostBuildAction::Preview => unimplemented!("Preview functionality is yet to be tackled")
    }
}
