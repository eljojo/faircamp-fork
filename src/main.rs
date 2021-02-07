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

    let artist = source::source_artist();
    let mut catalog = Catalog::init();
    
    if let Ok(current_dir) = env::current_dir() {
        source::source_catalog(build_dir, current_dir, &mut catalog).unwrap();
    }
    
    // Render index for all releases 
    let html = render::render_releases(&artist, &catalog);
    fs::write(build_dir.join("index.html"), html).unwrap();
    
    // Render index for each release
    for release in &catalog.releases {
        release.zip(build_dir).unwrap();
        
        let html = render::render_release(&artist, &release);
        fs::create_dir(build_dir.join(&release.slug)).ok();
        fs::write(build_dir.join(&release.slug).join("index.html"), html).unwrap();
    }

    fs::write(build_dir.join("styles.css"), css::DEFAULT).unwrap();
}
