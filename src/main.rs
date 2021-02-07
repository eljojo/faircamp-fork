use std::{env, fs, path::Path};

mod css;
mod image;
mod meta;
mod release;
mod render;
mod source;
mod track;
mod types;

use release::Release;

const BUILD_DIR: &str = "/tmp/faircamp_public/";

fn main() {
    let build_dir = Path::new(BUILD_DIR);

    fs::remove_dir_all(BUILD_DIR).unwrap();
    fs::create_dir_all(BUILD_DIR).unwrap();

    let artist = source::source_artist();
    let mut releases: Vec<Release> = Vec::new();
    
    if let Ok(current_dir) = env::current_dir() {
        source::source_releases(build_dir, current_dir, &mut releases);
    }
    
    // Render index for all releases 
    let html = render::render_releases(&artist, &releases);
    fs::write(build_dir.join("index.html"), html).unwrap();
    
    // Render index for each release
    for release in releases {
        release.zip(build_dir).unwrap();
        
        let html = render::render_release(&artist, &release);
        fs::create_dir(build_dir.join(&release.slug)).ok();
        fs::write(build_dir.join(&release.slug).join("index.html"), html).unwrap();
    }

    fs::write(build_dir.join("styles.css"), css::DEFAULT).unwrap();
}
