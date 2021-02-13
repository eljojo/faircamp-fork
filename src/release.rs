use slug;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    artist::Artist,
    asset_cache::{Asset, CachedImageAssets, CachedTrackAssets},
    build_settings::BuildSettings,
    download_formats::DownloadFormats,
    download_option::DownloadOption,
    ffmpeg::{self, TranscodeFormat},
    image::Image,
    track::Track,
    message,
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
    pub streaming_format: TranscodeFormat,
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
        streaming_format: TranscodeFormat,
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
            streaming_format,
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
        build_settings: &mut BuildSettings,
        cached_image_assets: &mut CachedImageAssets,
        image: &Image,
        target_format: &TranscodeFormat
    ) {
        let filename = format!("{}{}", image.uuid, target_format.suffix_and_extension());
        let jpg = cached_image_assets.jpg.get_or_insert_with(|| {
            message::transcoding(&format!("{:?} to {}", image.source_file, target_format));
            ffmpeg::transcode(
                &image.source_file,
                &build_settings.cache_dir.join(&filename),
                target_format
            ).unwrap();
            
            Asset::init(&build_settings.cache_dir, filename.clone())
        });
        
        jpg.used = true;

        fs::copy(
            build_settings.cache_dir.join(&jpg.filename),
            build_settings.build_dir.join(&filename)
        ).unwrap();
        
        build_settings.stats.add_image(jpg.filesize_bytes);
    }
    
    pub fn write_track_assets(
        &self,
        build_settings: &mut BuildSettings,
        cached_track_assets: &mut CachedTrackAssets,
        track: &Track,
        target_format: &TranscodeFormat
    ) {
        let target_filename = format!("{}{}", track.uuid, target_format.suffix_and_extension());
        
        let cached_format = match target_format {
            TranscodeFormat::Aac => &mut cached_track_assets.aac,
            TranscodeFormat::Aiff => &mut cached_track_assets.aiff,
            TranscodeFormat::Flac => &mut cached_track_assets.flac,
            TranscodeFormat::Mp3Cbr128 => &mut cached_track_assets.mp3_128,
            TranscodeFormat::Mp3Cbr320 => &mut cached_track_assets.mp3_320,
            TranscodeFormat::Mp3VbrV0 => &mut cached_track_assets.mp3_v0,
            TranscodeFormat::OggVorbis => &mut cached_track_assets.ogg_vorbis,
            TranscodeFormat::Wav => &mut cached_track_assets.wav,
            TranscodeFormat::Jpeg => unreachable!() // TODO: Maybe rather have a separate AudioFormat and ImageFormat so we don't mix static code paths
        };
        
        let cached_format_unpacked = cached_format.get_or_insert_with(|| {
            message::transcoding(&format!("{:?} to {}", track.source_file, target_format));
            ffmpeg::transcode(
                &track.source_file,
                &build_settings.cache_dir.join(&target_filename),
                target_format
            ).unwrap();
            
            Asset::init(&build_settings.cache_dir, target_filename.clone())
        });
        
        cached_format_unpacked.used = true;
        
        fs::copy(
            build_settings.cache_dir.join(&cached_format_unpacked.filename),
            build_settings.build_dir.join(&target_filename)
        ).unwrap();
        
        build_settings.stats.add_track(cached_format_unpacked.filesize_bytes);
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