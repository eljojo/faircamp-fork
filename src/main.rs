use std::{env, fs};

mod artist;
mod catalog;
mod css;
mod download_option;
mod image;
mod meta;
mod release;
mod render;
mod source;
mod track;
mod util;

use catalog::Catalog;

fn main() {
    let catalog_dir = env::current_dir()
        .expect("Current working directory can not be determined or is unaccessible");
    
    let build_dir = catalog_dir.join(".faircamp_build");
    let cache_dir = catalog_dir.join(".faircamp_cache");
    
    let mut catalog = Catalog::read(&catalog_dir);
    
    util::ensure_empty(&build_dir);
    
    catalog.write_assets(&build_dir, &cache_dir);
    
    
    // Render page for all artists
    let artists_html = render::render_artists(&catalog);
    fs::create_dir(build_dir.join("artists")).ok();
    fs::write(build_dir.join("artists").join("index.html"), artists_html).unwrap();
    
    // Render page for all releases
    let releases_html = render::render_releases(&catalog);
    fs::write(build_dir.join("index.html"), releases_html).unwrap();
    
    // Render page for each artist
    for artist in &catalog.artists {
        let artist_html = render::render_artist(&artist, &catalog);
        fs::create_dir(build_dir.join(&artist.slug)).ok();
        fs::write(build_dir.join(&artist.slug).join("index.html"), artist_html).unwrap();
    }
    
    // Render page for each release
    for release in &catalog.releases {
        release.write_files(&build_dir);
    }

    fs::write(build_dir.join("styles.css"), css::DEFAULT).unwrap();
}
