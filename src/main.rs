use std::{env, fs, path::Path};

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

use catalog::Catalog;

const BUILD_DIR: &str = "/tmp/faircamp_public/";

fn main() {
    let build_dir = Path::new(BUILD_DIR);

    fs::remove_dir_all(BUILD_DIR).unwrap();
    fs::create_dir_all(BUILD_DIR).unwrap();

    let mut catalog = Catalog::init();
    
    if let Ok(current_dir) = env::current_dir() {
        source::source_catalog(build_dir, current_dir, &mut catalog).unwrap();
    }
    
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
        release.write_files(build_dir);
    }

    fs::write(build_dir.join("styles.css"), css::DEFAULT).unwrap();
}
