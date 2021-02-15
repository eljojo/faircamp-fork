use slug;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
    rc::Rc
};
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    artist::Artist,
    audio_format::AudioFormat,
    build_settings::BuildSettings,
    catalog::Catalog,
    download_formats::DownloadFormats,
    download_option::DownloadOption,
    image::Image,
    track::Track,
    render
};

#[derive(Debug)]
pub struct Release {
    pub artists: Vec<Rc<Artist>>,
    pub cover: Option<Image>,
    pub download_formats: DownloadFormats,
    pub download_option: DownloadOption,
    pub slug: String,
    pub streaming_format: AudioFormat,
    pub text: Option<String>,
    pub title: String,
    pub tracks: Vec<Track>
}

impl Release {
    pub fn init(
        artists: Vec<Rc<Artist>>,
        download_formats: DownloadFormats,
        download_option: DownloadOption,
        mut images: Vec<Image>,
        streaming_format: AudioFormat,
        text: Option<String>,
        title: String,
        tracks: Vec<Track>
    ) -> Release {
        // TODO: Use/store multiple images (beyond just one cover)
        // TOOD: Basic logic to determine which of multiple images most likely is the cover
        let slug = slug::slugify(&title);
        
        Release {
            artists,
            cover: images.pop(),
            download_formats,
            download_option,
            slug,
            streaming_format,
            text,
            title,
            tracks
        }
    }
    
    pub fn write_files(&self, build_settings: &BuildSettings, catalog: &Catalog) {
        if let DownloadOption::Free(download_hash) = &self.download_option {
            fs::create_dir_all(build_settings.build_dir.join("download").join(download_hash)).ok();
            
            self.zip(&build_settings.build_dir).unwrap();
            
            let download_release_html = render::render_download(build_settings, &catalog, self);
            fs::write(build_settings.build_dir.join("download").join(download_hash).join("index.html"), download_release_html).unwrap();
        }
        
        let release_html = render::render_release(build_settings, catalog, self);
        fs::create_dir(build_settings.build_dir.join(&self.slug)).ok();
        fs::write(build_settings.build_dir.join(&self.slug).join("index.html"), release_html).unwrap();
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