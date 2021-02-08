#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use clap::Clap;
use std::fs;

mod args;
mod artist;
mod asset_cache;
mod build_settings;
mod catalog;
mod css;
mod deploy;
mod download_option;
mod image;
mod meta;
mod release;
mod render;
mod track;
mod transcode;
mod util;

use args::Args;
use build_settings::{BuildSettings, PostBuildAction};
use catalog::Catalog;

fn main() {
    env_logger::init();
    
    let args: Args = Args::parse();
    let build_settings = BuildSettings::init(&args);
    
    let mut catalog = Catalog::read(&build_settings.catalog_dir);
    
    util::ensure_empty_dir(&build_settings.build_dir);
    
    if args.wipe_cache {
        util::ensure_empty_dir(&build_settings.cache_dir);
    } else {
        util::ensure_dir(&build_settings.cache_dir);
    }
    
    catalog.write_assets(&build_settings);

    
    // Render page for all artists
    let artists_html = render::render_artists(&catalog);
    fs::create_dir(build_settings.build_dir.join("artists")).ok();
    fs::write(build_settings.build_dir.join("artists").join("index.html"), artists_html).unwrap();
    
    // Render page for all releases
    let releases_html = render::render_releases(&catalog);
    fs::write(build_settings.build_dir.join("index.html"), releases_html).unwrap();
    
    // Render page for each artist
    for artist in &catalog.artists {
        let artist_html = render::render_artist(&artist, &catalog);
        fs::create_dir(build_settings.build_dir.join(&artist.slug)).ok();
        fs::write(build_settings.build_dir.join(&artist.slug).join("index.html"), artist_html).unwrap();
    }
    
    // Render page for each release
    for release in &catalog.releases {
        release.write_files(&build_settings.build_dir);
    }

    fs::write(build_settings.build_dir.join("styles.css"), css::DEFAULT).unwrap();
    
    match build_settings.post_build_action {
        PostBuildAction::None => (),
        PostBuildAction::Deploy => deploy::deploy(&build_settings),
        PostBuildAction::Preview => unimplemented!("Preview functionality is yet to be tackled")
    }
}
