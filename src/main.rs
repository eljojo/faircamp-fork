use clap::Parser;
use std::fs;

#[macro_use]
mod message;

mod args;
mod artist;
mod asset_cache;
mod audio_format;
mod audio_meta;
mod build;
mod catalog;
mod deploy;
mod download_option;
mod feed;
mod ffmpeg;
mod image;
mod image_format;
mod localization;
mod manifest;
mod payment_option;
mod release;
mod render;
mod rsync;
mod styles;
mod track;
mod util;

use args::Args;
use asset_cache::{CacheManifest, CacheOptimization};
use build::{Build, PostBuildAction};
use catalog::Catalog;

fn main() {
    let args: Args = Args::parse();
    let mut build = Build::init(&args);
    
    if !build.catalog_dir.is_dir() {
        error!("Configured catalog directory does not exist - aborting build");
        return;
    }
    
    CacheManifest::ensure_dirs(&build.cache_dir);
    let mut cache_manifest = CacheManifest::retrieve(&build.cache_dir);
    
    if args.analyze_cache {
        build.cache_optimization = CacheOptimization::Immediate;
        asset_cache::report_stale(&cache_manifest, &Catalog::new());
        return;
    }
    
    if args.optimize_cache {
        build.cache_optimization = CacheOptimization::Immediate;
        asset_cache::optimize_cache(&build, &mut cache_manifest, &mut Catalog::new());
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
    
    let mut catalog = match Catalog::read(&mut build, &mut cache_manifest) {
        Ok(catalog) => catalog,
        Err(()) => return
    };
    
    util::ensure_empty_dir(&build.build_dir);
    util::ensure_dir(&build.build_dir.join("download"));
    
    catalog.write_assets(&mut build);

    // Render about page
    let about_html = render::about::about_html(&build, &catalog);
    fs::create_dir(build.build_dir.join("about")).unwrap();
    fs::write(build.build_dir.join("about/index.html"), about_html).unwrap();
    
    // Render page for all releases (= root index)
    let releases_html = render::releases::releases_html(&build, &catalog);
    fs::write(build.build_dir.join("index.html"), releases_html).unwrap();
    
    // Render page for each artist
    for artist in &catalog.artists {
        let artist_html = render::artist::artist_html(&build, &artist, &catalog);
        fs::create_dir(build.build_dir.join(&artist.permalink.get())).unwrap();
        fs::write(build.build_dir.join(&artist.permalink.get()).join("index.html"), artist_html).unwrap();
    }
    
    // Render page for each release
    for release in &catalog.releases {
        release.write_files(&build, &catalog);
    }

    fs::write(build.build_dir.join("scripts.js"), include_bytes!("assets/scripts.js")).unwrap();
    
    styles::generate(&mut build);
    feed::generate(&build, &catalog);
    
    match build.cache_optimization {
        CacheOptimization::Default |
        CacheOptimization::Delayed |
        CacheOptimization::Immediate =>
            asset_cache::optimize_cache(&build, &mut cache_manifest, &mut catalog),
        CacheOptimization::Manual =>
            asset_cache::report_stale(&cache_manifest, &catalog),
        CacheOptimization::Wipe => {
            util::remove_dir(&build.cache_dir);
            info_cache!("Wiped cache");
        }
    }
    
    build.print_stats();
    
    match build.post_build_action {
        PostBuildAction::None => (),
        PostBuildAction::Deploy => {
            if args.theming_widget {
                // TODO: But maybe someone *wants* to deploy it to a "live" page, e.g. to ask their bandmates for their color preferences? Follow up again :)
                error!("Aborting deploy because --theming-widget is enabled, we probably don't want that on the live page.")
            } else {
                deploy::deploy(&build);
            }
        },
        PostBuildAction::Preview => unimplemented!("Preview functionality is yet to be tackled")
    }
}
