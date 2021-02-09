use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::{
    artist::Artist,
    asset_cache::{
        CacheManifest,
        CachedImageAssets,
        CachedTrackAssets,
        SourceFileSignature
    },
    build_settings::BuildSettings,
    download_option::DownloadOption,
    image::Image,
    meta::Meta,
    release::Release,
    track::Track,
    ffmpeg,
    util
};

const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "png"];

#[derive(Debug)]
pub struct Catalog {
    pub artists: Vec<Rc<Artist>>,
    pub images: Vec<Image>, // TODO: Do we need these + what to do with them (also consider "label cover" aspect)
    pub releases: Vec<Release>
}

impl Catalog {
    fn init_empty() -> Catalog {
        Catalog {
            artists: Vec::new(),
            images: Vec::new(),
            releases: Vec::new()
        }
    }
    
    pub fn read(catalog_dir: &Path) -> Catalog {
        let mut catalog = Catalog::init_empty();
        catalog.read_dir(catalog_dir, DownloadOption::Disabled);
        catalog
    }
    
    fn read_dir(&mut self, dir: &Path, mut download_option: DownloadOption) -> Result<(), String> {
        let mut images: Vec<Image> = Vec::new();
        let mut release_artists: Vec<Rc<Artist>> = Vec::new();
        let mut release_tracks: Vec<Track> = Vec::new();
        
        // TODO: We need to ensure proper read-order:
        //       - First we read all meta
        //       - Then we read audio/images
        //       - Then we recurse into subdirectories
        
        match dir.read_dir() {
            Ok(dir_entries) => {
                for dir_entry_result in dir_entries {
                    if let Ok(dir_entry) = dir_entry_result {
                        // Skip hidden files
                        if let Some(str) = dir_entry.file_name().to_str() {
                            if str.starts_with(".") {
                                info!("Ignoring hidden file {:?} in catalog", dir_entry.path());
                                continue
                            }
                        }
                        
                        if let Ok(file_type) = dir_entry.file_type() {
                            if file_type.is_dir() {
                                self.read_dir(&dir_entry.path(), download_option.clone()).unwrap();
                            } else if file_type.is_file() {
                                if let Some(track) = self.read_track(&dir_entry.path()) {
                                    if release_artists
                                        .iter()
                                        .find(|release_artist| Rc::ptr_eq(release_artist, &track.artist))
                                        .is_none() {
                                        release_artists.push(track.artist.clone());
                                    }
                                    
                                    release_tracks.push(track);
                                } else if let Some(image) = self.read_image(&dir_entry.path()) {
                                    images.push(image);
                                } else if let Some(download_option_override) = self.read_meta(&dir_entry.path()) {
                                    info!("Reading meta {:?}", dir_entry.path());
                                    download_option = download_option_override;
                                } else {
                                    warn!("Ignoring unsupported filetype {:?} in catalog", dir_entry.path());
                                }
                            } else if file_type.is_symlink() {
                                warn!("Ignoring symlink {:?} in catalog", dir_entry.path());
                            } else {
                                warn!("Ignoring unknown filetype {:?} in catalog", dir_entry.path());
                            }
                        }
                    }
                }
                
                if !release_tracks.is_empty() {
                    let title = dir.file_name().unwrap().to_str().unwrap().to_string();
                    let release = Release::init(
                        release_artists,
                        download_option.clone(),
                        images,
                        title,
                        release_tracks
                    );
                    
                    self.releases.push(release);
                } else if !images.is_empty() {
                    // TODO: Some future logic/configuration lookup for  associating images with an artist
                    self.images.append(&mut images);
                }
                
                Ok(())
            }
            Err(_) => Err(String::from("Cannot read directory."))
        }
    }
    
    pub fn read_image(&self, path: &Path) -> Option<Image> {
        if let Some(extension_osstr) = path.extension() {
            if let Some(extension_str) =  extension_osstr.to_str() {
                if SUPPORTED_IMAGE_EXTENSIONS.contains(&extension_str.to_lowercase().as_str()) {
                    return Some(Image::init(path.to_path_buf(), util::uuid()));
                }
            }
        }
        
        None
    }
    
    pub fn read_meta(&self, path: &Path) -> Option<DownloadOption> {
        if let Some(extension_osstr) = path.extension() {
            if let Some(extension_str) =  extension_osstr.to_str() {
                if extension_str.to_lowercase().as_str() == "eno" {
                    match fs::read_to_string(path) {
                        Ok(content) => {
                            let mut download_option = None;
                            
                            for line in content.lines() {
                                if line.starts_with("download:") {
                                    match &line[10..] {
                                        "disabled" => {
                                            download_option = Some(DownloadOption::Disabled);
                                        },
                                        "free" => {
                                            download_option = Some(DownloadOption::init_free());
                                        },
                                        "anyprice" => {
                                            download_option = Some(DownloadOption::NameYourPrice);
                                        },
                                        "minprice" => {
                                            download_option = Some(DownloadOption::PayMinimum("10 Republican Credits".to_string()));
                                        },
                                        "exactprice" => {
                                            download_option = Some(DownloadOption::PayExactly("10 Republican Credits".to_string()));
                                        },
                                        _ => error!("Ignoring invalid download setting value '{}' in {:?}", &line[10..], path)
                                    }
                                }
                            }
                            
                            return download_option;
                        }
                        Err(err) => error!("Could not read meta file {:?} ({})", path, err)
                    } 
                }
            }
        }
        
        None
    }

    pub fn read_track(&mut self, path: &Path) -> Option<Track> {
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if let Some(extension_osstr) = path.extension() {
            if let Some(extension_str) =  extension_osstr.to_str() {
                if SUPPORTED_AUDIO_EXTENSIONS.contains(&extension_str.to_lowercase().as_str()) {
                    let meta = Meta::extract(extension_str, &path);
                    
                    let artist = self.track_artist(meta.artist); // TODO: track_artist is confusing because does it mean "track the artist" or "the track artist"
                    let title = meta.title.unwrap_or(filename.to_string());
                    return Some(Track::init(artist, path.to_path_buf(), title, util::uuid()));
                }
            }
        }
        
        None
    }
    
    pub fn track_artist(&mut self, new_artist_name: Option<String>) -> Rc<Artist> {
        if let Some(new_artist_name) = new_artist_name {
            self.artists
                .iter()
                .find(|artist| artist.name == new_artist_name)
                .map(|existing_artist| existing_artist.clone())
                .unwrap_or_else(|| {
                    let new_artist = Rc::new(Artist::init(new_artist_name));
                    self.artists.push(new_artist.clone());
                    new_artist
                })
        } else {
            self.artists
                .iter()
                .find(|artist| artist.name == "UNKNOWN_SPECIAL_STRING")
                .map(|existing_artist| existing_artist.clone())
                .unwrap_or_else(|| {
                    let new_artist = Rc::new(Artist::init(String::from("UNKNOWN_SPECIAL_STRING")));
                    self.artists.push(new_artist.clone());
                    new_artist
                })
        }
    }
    
    pub fn write_assets(&self, build_settings: &BuildSettings) {
        let mut cache_manifest = CacheManifest::retrieve(&build_settings.cache_dir);
        
        for release in &self.releases {
            if let Some(image) = &release.cover {
                let source_file_signature = SourceFileSignature::init(&image.source_file);
                
                if let Some(mut cached_image_assets) = cache_manifest.images
                    .iter_mut()
                    .find(|cached_image| cached_image.source_file_signature == source_file_signature) {
                    self.write_image_assets(build_settings, &mut cached_image_assets, image);
                } else {
                    let mut cached_image_assets = CachedImageAssets::new(source_file_signature);
                    self.write_image_assets(build_settings, &mut cached_image_assets, image);
                    cache_manifest.images.push(cached_image_assets);
                }
            }
            
            for track in &release.tracks {
                let source_file_signature = SourceFileSignature::init(&track.source_file);
                
                if let Some(mut cached_track_assets) = cache_manifest.tracks
                    .iter_mut()
                    .find(|cached_track| cached_track.source_file_signature == source_file_signature) {
                    self.write_track_assets(build_settings, &mut cached_track_assets, track);
                } else {
                    let mut cached_track_assets = CachedTrackAssets::new(source_file_signature);
                    self.write_track_assets(build_settings, &mut cached_track_assets, track);
                    cache_manifest.tracks.push(cached_track_assets);
                }
            }
        }
        
        cache_manifest.persist(&build_settings.cache_dir);
    }
    
    pub fn write_image_assets(
        &self,
        build_settings: &BuildSettings,
        cached_image_assets: &mut CachedImageAssets,
        image: &Image) {
        if cached_image_assets.image.is_none() {
            info!("Transcoding {:?} (no cached assets available)", image.source_file);
            let cache_relative_path = format!("{}.jpg", image.uuid);
            ffmpeg::transcode(&image.source_file, &build_settings.cache_dir.join(&cache_relative_path));
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
        if build_settings.transcode_flac {
            if cached_track_assets.flac.is_none() {
                info!("Transcoding {:?} to FLAC (no cached assets available)", track.source_file);
                let cache_relative_path = format!("{}.flac", track.uuid);
                ffmpeg::transcode(&track.source_file, &build_settings.cache_dir.join(&cache_relative_path));
                cached_track_assets.flac = Some(cache_relative_path);
            }
            
            fs::copy(
                build_settings.cache_dir.join(cached_track_assets.flac.as_ref().unwrap()),
                build_settings.build_dir.join(format!("{}.flac", &track.uuid))
            ).unwrap();
        }
        
        if build_settings.transcode_mp3_320cbr {
            if cached_track_assets.mp3_cbr_320.is_none() {
                info!("Transcoding {:?} to MP3 320 (no cached assets available)", track.source_file);
                let cache_relative_path = format!("{}.cbr_320.mp3", track.uuid);
                ffmpeg::transcode(&track.source_file, &build_settings.cache_dir.join(&cache_relative_path));
                cached_track_assets.mp3_cbr_320 = Some(cache_relative_path);
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
}