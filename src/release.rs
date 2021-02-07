use slug;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use zip::{
    result::ZipError,
    write::FileOptions,
    CompressionMethod,
    ZipWriter
};

use crate::track::Track;
use crate::types::DownloadOption;

#[derive(Debug)]
pub struct Release {
    pub cover: Option<String>,
    pub download_option: DownloadOption,
    pub release_date: Option<String>,
    pub slug: String,
    pub text: Option<String>,
    pub title: String,
    pub tracks: Vec<Track>
}

impl Release {
    pub fn init(title: String, tracks: Vec<Track>) -> Release {
        Release {
            cover: None,
            download_option: DownloadOption::Disabled,
            release_date: None,
            slug: slug::slugify(&title),
            text: Some(String::from("This is a dummy description for a release.")), // TODO: Remove and replace with true sourcing from content
            title,
            tracks
        }
    }
    
    pub fn zip(&self, build_dir: &Path) -> Result<(), String> {
        // TODO: For now we skip this time-consuming computation
        return Ok(());
        
        let zip_file = File::create(build_dir.join("release.zip")).unwrap();
        let mut zip_writer = ZipWriter::new(zip_file);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);
            
        let mut buffer = Vec::new();
        
        for track in &self.tracks {
            let filename = Path::new(&track.transcoded_file);
            
            zip_writer.start_file_from_path(filename, options).unwrap();
            
            // TODO: Read file into buffer in one go (helper method in fs:: available?)
            let mut zip_inner_file = File::open(build_dir.join(&track.transcoded_file)).unwrap();
            zip_inner_file.read_to_end(&mut buffer).unwrap();
            
            zip_writer.write_all(&*buffer).unwrap();
            buffer.clear();
        }
            
        match zip_writer.finish() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }
}