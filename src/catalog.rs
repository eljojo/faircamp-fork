use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::{
    artist::Artist,
    asset_cache::{
        CacheManifest,
        CachedImageAssets,
        CachedTrackAssets,
        SourceFileSignature
    },
    audio_meta::AudioMeta,
    build_settings::BuildSettings,
    download_option::DownloadOption,
    image::Image,
    manifest::Overrides,
    release::Release,
    track::Track,
    ffmpeg,
    manifest,
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
        catalog.read_dir(catalog_dir, &Overrides::default()).unwrap();
        catalog
    }
    
    fn read_dir(&mut self, dir: &Path, parent_overrides: &Overrides) -> Result<(), String> {
        let mut local_overrides = None;
        
        let mut images: Vec<Image> = Vec::new();
        let mut release_artists: Vec<Rc<Artist>> = Vec::new();
        let mut release_tracks: Vec<Track> = Vec::new();
        
        let mut dir_paths: Vec<PathBuf> = Vec::new();
        let mut image_paths: Vec<PathBuf> = Vec::new();
        let mut meta_paths: Vec<PathBuf> = Vec::new();
        let mut track_paths: Vec<(PathBuf, String)> = Vec::new();
        
        match dir.read_dir() {
            Ok(dir_entries) => {
                for dir_entry_result in dir_entries {
                    if let Ok(dir_entry) = dir_entry_result {
                        if let Some(filename) = dir_entry.file_name().to_str() {
                            if filename.starts_with(".") {
                                info!("Ignoring hidden file '{}'", filename);
                                continue
                            }
                        }
                        
                        if let Ok(file_type) = dir_entry.file_type() {
                            let path = dir_entry.path();
                            
                            if file_type.is_dir() {
                                dir_paths.push(path);
                            } else if file_type.is_file() {
                                if let Some(extension) = path.extension()
                                    .map(|osstr|
                                        osstr.to_str().map(|str|
                                            str.to_lowercase().as_str().to_string()
                                        )
                                    )
                                    .flatten() {
                                    if extension == "txt" {
                                        meta_paths.push(path);
                                    } else if SUPPORTED_AUDIO_EXTENSIONS.contains(&&extension[..]) {
                                        track_paths.push((path, extension));
                                    } else if SUPPORTED_IMAGE_EXTENSIONS.contains(&&extension[..]) {
                                        image_paths.push(path);
                                    } else {
                                        warn!("Ignoring unsupported file '{}'", path.display());
                                    }
                                } else {
                                    warn!("Ignoring unsupported file '{}'", path.display());
                                }
                            } else if file_type.is_symlink() {
                                warn!("Ignoring symlink '{}'", path.display());
                            } else {
                                warn!("Ignoring unsupported file '{}'", path.display());
                            }
                        }
                    }
                }
            }
            Err(err) => error!("Cannot read directory '{}' ({})", dir.display(), err)
        }
        
        for meta_path in &meta_paths {
            info!("Reading meta {}", meta_path.display());
            
            manifest::apply_overrides(
                meta_path,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }
        
        for (track_path, extension) in &track_paths {
            info!("Reading track {}", track_path.display());
            let audio_meta = AudioMeta::extract(track_path, extension);
            let track = self.read_track(track_path, audio_meta, local_overrides.as_ref().unwrap_or(parent_overrides));
            
            if local_overrides.as_ref().unwrap_or(parent_overrides).release_artists.is_none() {
                for track_artist in &track.artists {
                    if release_artists
                    .iter()
                    .find(|release_artist| Rc::ptr_eq(release_artist, track_artist))
                    .is_none() {
                        release_artists.push(track_artist.clone());
                    }
                }
            }
            
            release_tracks.push(track);
        }
        
        for image_path in &image_paths {
            info!("Reading image {}", image_path.display());
            images.push(Image::init(image_path, util::uuid()));
        }
        
        for dir_path in &dir_paths {
            info!("Reading directory {}", dir_path.display());
            self.read_dir(dir_path, local_overrides.as_ref().unwrap_or(&parent_overrides)).unwrap();
        }
        
        if !release_tracks.is_empty() {
            if let Some(artist_names) = &local_overrides.as_ref().unwrap_or(parent_overrides).release_artists {
                for artist_name in artist_names {
                    let artist = self.track_artist(Some(artist_name.clone()));
                    release_artists.push(artist);
                }
            }
            
            let title = dir.file_name().unwrap().to_str().unwrap().to_string();
            
            let release = Release::init(
                release_artists,
                match local_overrides {
                    Some(overrides) => overrides.download_option.clone(),
                    None => parent_overrides.download_option.clone()
                },
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

    pub fn read_track(&mut self, path: &Path, audio_meta: AudioMeta, overrides: &Overrides) -> Track {
        let artists = if let Some(artist_names) = &overrides.track_artists {
            artist_names
                .iter()
                .map(|name| self.track_artist(Some(name.to_string())))
                .collect()
        } else {
            vec![self.track_artist(audio_meta.artist)]
        };
        
        let title = audio_meta.title.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
        
        Track::init(artists, path.to_path_buf(), title, util::uuid())
    }
    
    // TODO: track_artist is confusing because does it mean "track the artist" or "the track artist"
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
        if build_settings.transcode_flac {
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
        
        if build_settings.transcode_mp3_320cbr {
            if cached_track_assets.mp3_cbr_320.is_none() {
                info!("Transcoding {:?} to MP3 320 (no cached assets available)", track.source_file);
                let cache_relative_path = format!("{}.cbr_320.mp3", track.uuid);
                ffmpeg::transcode(&track.source_file, &build_settings.cache_dir.join(&cache_relative_path)).unwrap();
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