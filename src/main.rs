use clap::Parser;
use std::rc::Rc;
use std::fs;
use webbrowser;

#[macro_use]
mod message;

mod args;
mod artist;
mod asset_cache;
mod audio_format;
mod audio_meta;
mod build;
mod catalog;
mod decode;
mod deploy;
mod download_option;
mod feed;
mod ffmpeg;
mod image;
mod image_format;
mod localization;
mod manifest;
mod payment_option;
mod permalink;
mod release;
mod render;
mod rsync;
mod styles;
mod theme;
mod track;
mod util;

use args::Args;
use asset_cache::{CacheManifest, CacheOptimization};
use build::{Build, PostBuildAction};
use catalog::Catalog;

fn main() {
    let args: Args = Args::parse();
    let mut build = Build::new(&args);
    
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

    // Artists without images are assigned a cover image from one of their releases here
    for artist in &catalog.artists {
        if artist.borrow().image.is_none() {
            for release in &catalog.releases {
                if let Some(cover) = &release.cover {
                    if release.artists
                        .iter()
                        .find(|release_artist| Rc::ptr_eq(release_artist, artist))
                        .is_some() {
                        let mut artist_mut = artist.borrow_mut();
                        artist_mut.image = Some(cover.clone());
                    }
                }
            }
        }
    }
    
    // Render about page
    let about_html = render::about::about_html(&build, &catalog);
    fs::create_dir(build.build_dir.join("about")).unwrap();
    fs::write(build.build_dir.join("about/index.html"), about_html).unwrap();
    
    // Render page for all releases (= root index)
    let releases_html = render::releases::releases_html(&build, &catalog);
    fs::write(build.build_dir.join("index.html"), releases_html).unwrap();
    
    // Render page for each artist
    for artist in &catalog.artists {
        let artist_ref = artist.borrow();
        if let Some(image) = &artist_ref.image {
            if image.borrow().description.is_none() {
                warn_discouraged!("The image for artist '{}' is missing an image description.", artist_ref.name);
                build.missing_image_descriptions = true;
            }
        }

        let artist_html = render::artist::artist_html(&build, artist, &catalog);
        let artist_ref = artist.borrow();

        fs::create_dir(build.build_dir.join(&artist_ref.permalink.slug)).unwrap();
        fs::write(build.build_dir.join(&artist_ref.permalink.slug).join("index.html"), artist_html).unwrap();
    }
    
    // Render page for each release
    for release in &catalog.releases {
        release.write_files(&mut build, &catalog);
    }

    fs::write(build.build_dir.join("scripts.js"), include_bytes!("assets/scripts.js")).unwrap();
    
    styles::generate(&build);
    feed::generate(&build, &catalog);

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
        PostBuildAction::Preview => {
            let local_file_url = build.build_dir.join("index.html");
            if webbrowser::open(&local_file_url.to_string_lossy()).is_err() {
                error!("Could not open browser for previewing the site")
            }
        }
    }
}
