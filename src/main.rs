#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use clap::Clap;
use std::fs;

mod args;
mod artist;
mod asset_cache;
mod audio_format;
mod audio_meta;
mod build;
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
use build::{Build, PostBuildAction};
use catalog::Catalog;

fn main() {
    env_logger::init();
    
    let args: Args = Args::parse();
    let mut build = Build::init(&args);
    
    CacheManifest::ensure_dirs(&build.cache_dir);
    let mut cache_manifest = CacheManifest::retrieve(&build.cache_dir);
    
    if args.analyze_cache {
        build.cache_optimization = CacheOptimization::Immediate;
        asset_cache::report_stale(&cache_manifest, &Catalog::init_empty());
        return;
    }
    
    if args.optimize_cache {
        build.cache_optimization = CacheOptimization::Immediate;
        asset_cache::optimize_cache(&build, &mut cache_manifest, &mut Catalog::init_empty());
        return;
    }
    
    if args.wipe_all || args.wipe_build || args.wipe_cache {
        if args.wipe_build || args.wipe_all {
            util::remove_dir(&build.build_dir);
        }
        if args.wipe_cache || args.wipe_all {
            util::remove_dir(&build.cache_dir);
        }
        return;
    }
    
    cache_manifest.mark_all_stale(&build.build_begin);
    
    let mut catalog = Catalog::read(&mut build, &mut cache_manifest);
    
    util::ensure_empty_dir(&build.build_dir);
    util::ensure_dir(&build.build_dir.join("download"));
    
    catalog.write_assets(&mut build);

    // Render about page
    let about_html = render::render_about(&build, &catalog);
    fs::create_dir(build.build_dir.join("about")).unwrap();
    fs::write(build.build_dir.join("about/index.html"), about_html).unwrap();
    
    // Render page for all artists
    let artists_html = render::render_artists(&build, &catalog);
    fs::create_dir(build.build_dir.join("artists")).unwrap();
    fs::write(build.build_dir.join("artists/index.html"), artists_html).unwrap();
    
    // Render page for all releases
    let releases_html = render::render_releases(&build, &catalog);
    fs::write(build.build_dir.join("index.html"), releases_html).unwrap();
    
    // Render page for each artist
    for artist in &catalog.artists {
        let artist_html = render::render_artist(&build, &artist, &catalog);
        fs::create_dir(build.build_dir.join(&artist.slug)).unwrap();
        fs::write(build.build_dir.join(&artist.slug).join("index.html"), artist_html).unwrap();
    }
    
    // Render page for each release
    for release in &catalog.releases {
        release.write_files(&build, &catalog);
    }
    
    // TODO: Go through asset cache with this
    if let Some(background_image) = &build.theme.background_image {
        ffmpeg::transcode(
            &build.catalog_dir.join(background_image),
            &build.build_dir.join("background.jpg"),
            MediaFormat::Image(&ImageFormat::Jpeg)
        ).unwrap();
    }

    fs::write(build.build_dir.join("barlow-v5-latin-regular.woff2"), include_bytes!("assets/barlow-v5-latin-regular.woff2")).unwrap();
    fs::write(build.build_dir.join("scripts.js"), include_bytes!("assets/scripts.js")).unwrap();
    
    styles::generate(&build);
    feed::generate(&build, &catalog);
    
    match build.cache_optimization {
        CacheOptimization::Delayed |
        CacheOptimization::Immediate =>
            asset_cache::optimize_cache(&build, &mut cache_manifest, &mut catalog),
        CacheOptimization::Manual =>
            asset_cache::report_stale(&cache_manifest, &catalog),
        CacheOptimization::Wipe => {
            util::remove_dir(&build.cache_dir);
            message::cache(&format!("Wiped cache"));
        }
    }
    
    build.print_stats();
    
    match build.post_build_action {
        PostBuildAction::None => (),
        PostBuildAction::Deploy => deploy::deploy(&build),
        PostBuildAction::Preview => unimplemented!("Preview functionality is yet to be tackled")
    }
}
