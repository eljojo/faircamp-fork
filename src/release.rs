use slug;
use std::{
    fs::{self, File},
    io::prelude::*,
    rc::Rc
};
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    artist::Artist,
    asset_cache::{Asset, CachedReleaseAssets},
    audio_format::AudioFormat,
    build_settings::BuildSettings,
    catalog::Catalog,
    download_option::DownloadOption,
    image::Image,
    image_format::ImageFormat,
    message,
    render,
    track::Track,
    util
};

#[derive(Debug)]
pub struct Release {
    pub artists: Vec<Rc<Artist>>,
    pub cached_assets: CachedReleaseAssets,
    pub cover: Option<Image>,
    pub download_formats: Vec<AudioFormat>,
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
        cached_assets: CachedReleaseAssets,
        download_formats: Vec<AudioFormat>,
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
            cached_assets,
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
    
    pub fn write_download_archives(&mut self, build_settings: &mut BuildSettings) {
        if self.download_option != DownloadOption::Disabled {
            for format in &self.download_formats {
                let cached_format = self.cached_assets.get_mut(format);
                
                if cached_format.is_none() {
                    let target_filename = format!("{}.zip", util::uuid());
                    
                    let zip_file = File::create(
                        build_settings.cache_dir.join(&target_filename)
                    ).unwrap();
                    let mut zip_writer = ZipWriter::new(zip_file);
                    let options = FileOptions::default()
                        .compression_method(CompressionMethod::Deflated)
                        .unix_permissions(0o755);
                        
                    let mut buffer = Vec::new();
                    
                    for (index, track) in self.tracks.iter_mut().enumerate() {
                        if !track.cached_assets.source_meta.lossless {
                            match format {
                                AudioFormat::Aiff |
                                AudioFormat::Flac |
                                AudioFormat::Wav => {
                                    message::discouraged(&format!("Track {} comes from a lossy format, offering it in a lossless format is wasteful and misleading to those who will download it.", &track.source_file.display()));
                                }
                                AudioFormat::Aac |
                                AudioFormat::Mp3Cbr128 |
                                AudioFormat::Mp3Cbr320 |
                                AudioFormat::Mp3VbrV0 |
                                AudioFormat::OggVorbis => () // we spell out all formats so the compiler catches future modifications/additions that need to be added here
                            }
                        }
                        
                        let filename = format!(
                            "{track_number:02} {artists}{separator}{title}{extension}",
                            artists=track.artists
                                .iter()
                                .map(|artist| artist.name.clone())
                                .collect::<Vec<String>>()
                                .join(", "),
                            extension=format.extension(),
                            separator=if track.artists.is_empty() { "" } else { " - " },
                            track_number=index + 1,
                            title=track.title
                        );
                        
                        // TODO: Should probably be track.get_cached (...) or such to indicate we're just pre-building an asset in the cache
                        let download_track_asset = track.get_or_transcode_as(format, &build_settings.cache_dir);
                        
                        zip_writer.start_file(&filename, options).unwrap();
                    
                        let mut zip_inner_file = File::open(
                            &build_settings.cache_dir.join(&download_track_asset.filename)
                        ).unwrap();
                            
                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&*buffer).unwrap();
                        buffer.clear();
                    }
                    
                    if let Some(cover) = &mut self.cover {
                        let cover_asset = cover.get_or_transcode_as(&ImageFormat::Jpeg, &build_settings.cache_dir);
                        
                        zip_writer.start_file("cover.jpg", options).unwrap();
                        
                        let mut zip_inner_file = File::open(
                            &build_settings.cache_dir.join(&cover_asset.filename)
                        ).unwrap();
                        
                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&*buffer).unwrap();
                        buffer.clear();
                    }
                        
                    return match zip_writer.finish() {
                        Ok(_) => {
                            let mut download_archive_asset = Asset::init(&build_settings.cache_dir, target_filename);
                            
                            download_archive_asset.used = true;
                            
                            fs::copy(
                                build_settings.cache_dir.join(&download_archive_asset.filename),
                                build_settings.build_dir.join(&download_archive_asset.filename)
                            ).unwrap();
                            
                            build_settings.stats.add_archive(download_archive_asset.filesize_bytes);
                            
                            cached_format.replace(download_archive_asset);
                        },
                        Err(err) => panic!(err)
                    };
                }
            }
        }
    }
    
    pub fn write_files(&self, build_settings: &BuildSettings, catalog: &Catalog) {
        if let DownloadOption::Free(download_hash) = &self.download_option {
            fs::create_dir_all(build_settings.build_dir.join("download").join(download_hash)).ok();
            
            let download_release_html = render::render_download(build_settings, &catalog, self);
            fs::write(build_settings.build_dir.join("download").join(download_hash).join("index.html"), download_release_html).unwrap();
        }
        
        let release_html = render::render_release(build_settings, catalog, self);
        fs::create_dir(build_settings.build_dir.join(&self.slug)).ok();
        fs::write(build_settings.build_dir.join(&self.slug).join("index.html"), release_html).unwrap();
    }
}