use slug;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    download_option::DownloadOption,
    image::Image,
    render,
    source,
    track::Track
};

#[derive(Debug)]
pub struct Release {
    pub cover: Option<Image>,
    pub download_option: DownloadOption,
    pub release_date: Option<String>,
    pub slug: String,
    pub text: Option<String>,
    pub title: String,
    pub tracks: Vec<Track>
}

impl Release {
    pub fn init(mut images: Vec<Image>, title: String, tracks: Vec<Track>) -> Release {
        // TODO: Use/store multiple images (beyond just one cover)
        // TOOD: Basic logic to determine which of multiple images most likely is the cover
        
        Release {
            cover: images.pop(),
            download_option: DownloadOption::init_free(), // TODO: Revert to DownloadOption::Disabled after testing
            release_date: None,
            slug: slug::slugify(&title),
            text: Some(String::from("This is a dummy description for a release.")), // TODO: Remove and replace with true sourcing from content
            title,
            tracks
        }
    }
    
    pub fn write_files(&self, build_dir: &Path) {
        let artist = source::source_artist();
        
        if let DownloadOption::Free(download_hash) = &self.download_option {
            fs::create_dir_all(build_dir.join("download").join(download_hash)).ok();
            
            self.zip(build_dir).unwrap();
            
            let download_release_html = render::render_download(&artist, self);
            fs::write(build_dir.join("download").join(download_hash).join("index.html"), download_release_html).unwrap();
        }
        
        let release_html = render::render_release(&artist, self);
        fs::create_dir(build_dir.join(&self.slug)).ok();
        fs::write(build_dir.join(&self.slug).join("index.html"), release_html).unwrap();
    }
    
    pub fn zip(&self, build_dir: &Path) -> Result<(), String> {
        let download_uuid = if let DownloadOption::Free(download_uuid) = &self.download_option {
            download_uuid
        } else {
            "todo"
        };
        
        let zip_file = File::create(build_dir.join("download").join(download_uuid).join("original.zip")).unwrap();
        let mut zip_writer = ZipWriter::new(zip_file);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);
            
        // TODO: For now we skip this time-consuming computation
        // let mut buffer = Vec::new();
        // for track in &self.tracks {
        //     let filename = Path::new(&track.transcoded_file);
        // 
        //     zip_writer.start_file_from_path(filename, options).unwrap();
        // 
        //     // TODO: Read file into buffer in one go (helper method in fs:: available?)
        //     let mut zip_inner_file = File::open(build_dir.join(&track.transcoded_file)).unwrap();
        //     zip_inner_file.read_to_end(&mut buffer).unwrap();
        // 
        //     zip_writer.write_all(&*buffer).unwrap();
        //     buffer.clear();
        // }
            
        match zip_writer.finish() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}