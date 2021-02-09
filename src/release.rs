use slug;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    artist::Artist,
    asset_cache::{CachedImageAssets, CachedTrackAssets},
    build_settings::BuildSettings,
    download_formats::DownloadFormats,
    download_option::DownloadOption,
    image::Image,
    track::Track,
    ffmpeg,
    render
};

#[derive(Debug)]
pub struct Release {
    pub artists: Vec<Rc<Artist>>,
    pub cover: Option<Image>,
    pub download_formats: DownloadFormats,
    pub download_option: DownloadOption,
    pub release_date: Option<String>,
    pub slug: String,
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
            release_date: None,
            slug,
            text,
            title,
            tracks
        }
    }
    
    pub fn write_files(&self, build_dir: &Path) {
        if let DownloadOption::Free(download_hash) = &self.download_option {
            fs::create_dir_all(build_dir.join("download").join(download_hash)).ok();
            
            self.zip(build_dir).unwrap();
            
            let download_release_html = render::render_download(self);
            fs::write(build_dir.join("download").join(download_hash).join("index.html"), download_release_html).unwrap();
        }
        
        let release_html = render::render_release(self);
        fs::create_dir(build_dir.join(&self.slug)).ok();
        fs::write(build_dir.join(&self.slug).join("index.html"), release_html).unwrap();
    }
    
    pub fn write_image_assets(
        &self,
        build_settings: &BuildSettings,
        cached_image_assets: &mut CachedImageAssets,
        image: &Image) {
        if cached_image_assets.image.is_none() {
            info!("Transcoding {:?} (no cached assets available)", image.source_file);
            let cache_relative_path = format!("{}.jpg", image.uuid);
            ffmpeg::transcode(&image.source_file, &build_settings.cache_dir.join(&cache_relative_path)).unwrap();
            cached_image_assets.image = Some(cache_relative_path);
        }

        fs::copy(
            build_settings.cache_dir.join(cached_image_assets.image.as_ref().unwrap()),
            build_settings.build_dir.join(format!("{}.jpg", &image.uuid))
        ).unwrap();
        
        // TODO: Resized variants etc.
    }
    
    pub fn write_track_assets(
        &self,
        build_settings: &BuildSettings,
        cached_track_assets: &mut CachedTrackAssets,
        track: &Track) {
        if self.download_formats.flac {
            if cached_track_assets.flac.is_none() {
                info!("Transcoding {:?} to FLAC (no cached assets available)", track.source_file);
                let cache_relative_path = format!("{}.flac", track.uuid);
                ffmpeg::transcode(&track.source_file, &build_settings.cache_dir.join(&cache_relative_path)).unwrap();
                cached_track_assets.flac = Some(cache_relative_path);
            }
            
            fs::copy(
                build_settings.cache_dir.join(cached_track_assets.flac.as_ref().unwrap()),
                build_settings.build_dir.join(format!("{}.flac", &track.uuid))
            ).unwrap();
        }
        
        if self.download_formats.mp3_320 {
            if cached_track_assets.mp3_320.is_none() {
                info!("Transcoding {:?} to MP3 320 (no cached assets available)", track.source_file);
                let cache_relative_path = format!("{}.cbr_320.mp3", track.uuid);
                ffmpeg::transcode(&track.source_file, &build_settings.cache_dir.join(&cache_relative_path)).unwrap();
                cached_track_assets.mp3_320 = Some(cache_relative_path);
            }
            
            // TODO: Only one type of format should be copied to staging as separate tracks,
            //       namely the one that is used for (streaming) playback on the page. All
            //       other formats go into the zip downloads only (if downloads are enabled even - needs checking here as well!)
            fs::copy(
                build_settings.cache_dir.join(cached_track_assets.flac.as_ref().unwrap()),
                build_settings.build_dir.join(format!("{}.mp3", &track.uuid))
            ).unwrap();
        }
        
        // TODO: Other formats
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