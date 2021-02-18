use slug;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
    rc::Rc
};
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    artist::Artist,
    asset_cache::{Asset, CacheManifest, SourceFileSignature},
    audio_format::AudioFormat,
    build_settings::BuildSettings,
    catalog::Catalog,
    download_option::DownloadOption,
    image::Image,
    image_format::ImageFormat,
    manifest::Overrides,
    message,
    payment_option::PaymentOption,
    render,
    track::Track,
    util
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedReleaseAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub flac: Option<Asset>,
    pub mp3_128: Option<Asset>,
    pub mp3_320: Option<Asset>,
    pub mp3_v0: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub source_file_signatures: Vec<SourceFileSignature>,
    pub uid: String,
    pub wav: Option<Asset>
}

#[derive(Debug)]
pub struct Release {
    pub artists: Vec<Rc<Artist>>,
    pub cached_assets: CachedReleaseAssets,
    pub cover: Option<Image>,
    pub download_formats: Vec<AudioFormat>,
    pub download_option: DownloadOption,
    pub payment_options: Vec<PaymentOption>,
    pub slug: String,
    pub streaming_format: AudioFormat,
    pub text: Option<String>,
    pub title: String,
    pub tracks: Vec<Track>
}

impl CachedReleaseAssets {
    pub fn deserialize(path: &Path) -> Option<CachedReleaseAssets> {
        if let Ok(bytes) = fs::read(path) {
            if let Ok(mut cached_assets) = bincode::deserialize::<CachedReleaseAssets>(&bytes) {
                cached_assets.aac.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.aiff.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.flac.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.mp3_128.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.mp3_320.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.mp3_v0.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.ogg_vorbis.iter_mut().for_each(|asset| asset.mark_stale());
                cached_assets.wav.iter_mut().for_each(|asset| asset.mark_stale());
                
                return Some(cached_assets);
            }
        }
        
        None
    }
    
    pub fn get(&self, format: &AudioFormat) -> &Option<Asset> {
        match format {
            AudioFormat::Aac => &self.aac,
            AudioFormat::Aiff => &self.aiff,
            AudioFormat::Flac => &self.flac,
            AudioFormat::Mp3Cbr128 => &self.mp3_128,
            AudioFormat::Mp3Cbr320 => &self.mp3_320,
            AudioFormat::Mp3VbrV0 => &self.mp3_v0,
            AudioFormat::OggVorbis => &self.ogg_vorbis,
            AudioFormat::Wav => &self.wav
        }
    }
    
    pub fn get_mut(&mut self, format: &AudioFormat) -> &mut Option<Asset> {
        match format {
            AudioFormat::Aac => &mut self.aac,
            AudioFormat::Aiff => &mut self.aiff,
            AudioFormat::Flac => &mut self.flac,
            AudioFormat::Mp3Cbr128 => &mut self.mp3_128,
            AudioFormat::Mp3Cbr320 => &mut self.mp3_320,
            AudioFormat::Mp3VbrV0 => &mut self.mp3_v0,
            AudioFormat::OggVorbis => &mut self.ogg_vorbis,
            AudioFormat::Wav => &mut self.wav
        }
    }
    
    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_RELEASES_DIR).join(filename)
    }

    pub fn new(source_file_signatures: Vec<SourceFileSignature>) -> CachedReleaseAssets {
        CachedReleaseAssets {
            aac: None,
            aiff: None,
            flac: None,
            mp3_128: None,
            mp3_320: None,
            mp3_v0: None,
            ogg_vorbis: None,
            source_file_signatures,
            uid: util::uid(),
            wav: None
        }
    }
    
    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), &serialized).unwrap();
    }
}

impl Release {
    pub fn init(
        artists: Vec<Rc<Artist>>,
        cached_assets: CachedReleaseAssets,
        mut images: Vec<Image>,
        manifest_overrides: &Overrides,
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
            download_formats: manifest_overrides.download_formats.clone(),
            download_option: manifest_overrides.download_option.clone(),
            payment_options: manifest_overrides.payment_options.clone(),
            slug,
            streaming_format: manifest_overrides.streaming_format.clone(),
            text: manifest_overrides.release_text.clone(),
            title,
            tracks
        }
    }
    
    pub fn write_download_archives(&mut self, build_settings: &mut BuildSettings) {
        if self.download_option != DownloadOption::Disabled {
            for format in &self.download_formats {
                let cached_format = self.cached_assets.get_mut(format);
                
                if cached_format.is_none() {
                    let target_filename = format!("{}.zip", util::uid());
                    
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
                        
                    match zip_writer.finish() {
                        Ok(_) => cached_format.replace(Asset::init(&build_settings.cache_dir, target_filename)),
                        Err(err) => panic!(err)
                    };
                }
                
                let download_archive_asset = cached_format.as_mut().unwrap();
                
                download_archive_asset.marked_stale = None; // TODO: Something more linguistically meaningful (custom enum or such) ?
                
                fs::copy(
                    build_settings.cache_dir.join(&download_archive_asset.filename),
                    build_settings.build_dir.join(&download_archive_asset.filename)
                ).unwrap();
                
                build_settings.stats.add_archive(download_archive_asset.filesize_bytes);
                
                self.cached_assets.persist(&build_settings.cache_dir);
            }
        }
    }
    
    pub fn write_files(&self, build_settings: &BuildSettings, catalog: &Catalog) {
        match &self.download_option {
            DownloadOption::Disabled => (),
            DownloadOption::Free { download_page_uid }  => {
                let download_page_dir = build_settings.build_dir.join("download").join(download_page_uid);
                let download_html = render::render_download(build_settings, catalog, self);
                util::ensure_dir_and_write_index(&download_page_dir, &download_html);
            }
            DownloadOption::Paid { checkout_page_uid, download_page_uid, .. } => {
                let checkout_page_dir = build_settings.build_dir.join("checkout").join(checkout_page_uid);
                let checkout_html = render::render_checkout(build_settings, catalog, self, download_page_uid);
                util::ensure_dir_and_write_index(&checkout_page_dir, &checkout_html);
                
                let download_page_dir = build_settings.build_dir.join("download").join(download_page_uid);
                let download_html = render::render_download(build_settings, catalog, self);
                util::ensure_dir_and_write_index(&download_page_dir, &download_html);
            }
        }
        
        let release_dir = build_settings.build_dir.join(&self.slug);
        let release_html = render::render_release(build_settings, catalog, self);
        util::ensure_dir_and_write_index(&release_dir, &release_html);
    }
}