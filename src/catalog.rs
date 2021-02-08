use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::{
    artist::Artist,
    asset_cache::{CacheManifest, CachedTrackAssets, SourceFileSignature},
    build_settings::BuildSettings,
    image::Image,
    meta::Meta,
    release::Release,
    track::Track,
    transcode,
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
        catalog.read_dir(catalog_dir);
        catalog
    }
    
    fn read_dir(&mut self, dir: &Path) -> Result<(), String> {
        let mut images: Vec<Image> = Vec::new();
        let mut release_artists: Vec<Rc<Artist>> = Vec::new();
        let mut release_tracks: Vec<Track> = Vec::new();
        
        match dir.read_dir() {
            Ok(dir_entries) => {
                for dir_entry_result in dir_entries {
                    if let Ok(dir_entry) = dir_entry_result {
                        // Skip hidden files
                        if let Some(str) = dir_entry.file_name().to_str() {
                            if str.starts_with(".") { continue }
                        }
                        
                        if let Ok(file_type) = dir_entry.file_type() {
                            if file_type.is_dir() {
                                self.read_dir(&dir_entry.path()).unwrap();
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
                                }
                            } else if file_type.is_symlink() {
                                // TODO: Symlinks ignored for now, handle if and when requested
                            } else {
                                
                            }
                        }
                    }
                }
                
                if !release_tracks.is_empty() {
                    let title = dir.file_name().unwrap().to_str().unwrap().to_string();
                    let release = Release::init(release_artists, images, title, release_tracks);
                    
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
                    let source_file = path.to_str().unwrap().to_string();
                    return Some(Image::init(source_file, util::uuid()));
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
    
    pub fn write_assets(&self, build_settings: &BuildSettings, build_dir: &Path, cache_dir: &Path) {
        let mut cache_manifest = CacheManifest::retrieve(cache_dir);
        
        for release in &self.releases {
            // // TODO: Copy image
            // let transcoded_file = format!("{}.{}", util::uuid(), extension_str);
            // fs::copy(path_clone, build_dir.join(&transcoded_file)).unwrap();
            
            for track in &release.tracks {
                dbg!(&track.title);
                
                let source_file_signature = SourceFileSignature::init(&track.source_file);
                
                if let Some(mut cached_track_assets) = cache_manifest.entries
                    .iter_mut()
                    .find(|entry| entry.source_file_signature == source_file_signature) {
                    self.write_track_assets(build_settings, build_dir, cache_dir, &mut cached_track_assets, track);
                } else {
                    let mut cached_track_assets = CachedTrackAssets::new(source_file_signature);
                    self.write_track_assets(build_settings, build_dir, cache_dir, &mut cached_track_assets, track);
                    cache_manifest.entries.push(cached_track_assets);
                };
            }
        }
        
        // dbg!(&cache_manifest);
        
        cache_manifest.persist(cache_dir);
    }
    
    pub fn write_track_assets(
        &self,
        build_settings: &BuildSettings,
        build_dir: &Path,
        cache_dir: &Path,
        cached_track_assets: &mut CachedTrackAssets,
        track: &Track) {
        if build_settings.transcode_flac {
            if cached_track_assets.flac.is_none() {
                dbg!("Transcoding FLAC because we didn't find it in cache.");
                let cache_relative_path = format!("{}.flac", track.transcoded_file);
                transcode::transcode(&track.source_file, &cache_dir.join(&cache_relative_path));
                cached_track_assets.flac = Some(cache_relative_path);
            }
            
            fs::copy(
                cache_dir.join(cached_track_assets.flac.as_ref().unwrap()),
                build_dir.join(format!("{}.flac", &track.transcoded_file))
            ).unwrap();
        }
        
        if build_settings.transcode_mp3_320cbr {
            if cached_track_assets.mp3_cbr_320.is_none() {
                dbg!("Transcoding MP3 CBR 320 because we didn't find it in cache.");
                let cache_relative_path = format!("{}.cbr_320.mp3", track.transcoded_file);
                transcode::transcode(&track.source_file, &cache_dir.join(&cache_relative_path));
                cached_track_assets.mp3_cbr_320 = Some(cache_relative_path);
            }
            
            // TODO: Only one type of format should be copied to staging as separate tracks,
            //       namely the one that is used for (streaming) playback on the page. All
            //       other formats go into the zip downloads only (if downloads are enabled even - needs checking here as well!)
            fs::copy(
                cache_dir.join(cached_track_assets.flac.as_ref().unwrap()),
                build_dir.join(format!("{}.mp3", &track.transcoded_file))
            ).unwrap();
        }
        
        // TODO: Other formats
    }
}