use std::{fs, path::{Path, PathBuf}, rc::Rc};
use uuid::Uuid;

use crate::artist::Artist;
use crate::catalog::Catalog;
use crate::image::Image;
use crate::meta::Meta;
use crate::release::Release;
use crate::track::Track;

const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "png"];

pub fn source_artist() -> Artist {
    Artist {
        image: None,
        links: Vec::new(),
        location: None,
        name: String::from("Dummy Artist")
    }
}

pub fn source_catalog(build_dir: &Path, dir: PathBuf, catalog: &mut Catalog) -> Result<(), String> {
    let mut images: Vec<Image> = Vec::new();
    let mut release_artists: Vec<Rc<Artist>> = Vec::new();
    let mut release_tracks: Vec<Track> = Vec::new();
    
    match dir.read_dir() {
        Ok(dir_entries) => {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Ok(file_type) = dir_entry.file_type() {
                        if file_type.is_dir() {
                            source_catalog(build_dir, dir_entry.path(), catalog).unwrap();
                        } else if file_type.is_file() {
                            if let Some(track) = source_track(build_dir, dir_entry.path(), catalog) {
                                if let None = release_artists.iter().find(|release_artist| Rc::ptr_eq(release_artist, &track.artist)) {
                                    release_artists.push(track.artist.clone());
                                }
                                
                                release_tracks.push(track);
                            } else if let Some(image) = source_image(build_dir, dir_entry.path()) {
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
                
                catalog.releases.push(release);
            } else if !images.is_empty() {
                // TODO: Some future logic/configuration lookup for  associating images with an artist
                catalog.images.append(&mut images);
            }
            
            Ok(())
        }
        Err(_) => Err(String::from("Cannot read directory."))
    }
}

pub fn source_image(build_dir: &Path, path: PathBuf) -> Option<Image> {
    let path_clone = path.clone();
    
    if let Some(extension_osstr) = path.extension() {
        if let Some(extension_str) =  extension_osstr.to_str() {
            if SUPPORTED_IMAGE_EXTENSIONS.contains(&extension_str.to_lowercase().as_str()) {
                let uuid = Uuid::new_v4().to_string();
                let source_file = path.to_str().unwrap().to_string();
                let transcoded_file = format!("{}.{}", uuid, extension_str);
                
                fs::copy(path_clone, build_dir.join(&transcoded_file)).unwrap();
                
                return Some(Image::init(source_file, transcoded_file));
            }
        }
    }
    
    None
}

pub fn source_track(build_dir: &Path, path: PathBuf, catalog: &mut Catalog) -> Option<Track> {
    let path_clone = path.clone();
    let filename = path.file_name().unwrap().to_str().unwrap();
    
    if let Some(extension_osstr) = path.extension() {
        if let Some(extension_str) =  extension_osstr.to_str() {
            if SUPPORTED_AUDIO_EXTENSIONS.contains(&extension_str.to_lowercase().as_str()) {
                let uuid = Uuid::new_v4().to_string();
                let source_file = path.to_str().unwrap().to_string();
                let meta = Meta::extract(extension_str, &path);
                let transcoded_file = format!("{}.{}", uuid, extension_str);
                
                fs::copy(path_clone, build_dir.join(&transcoded_file)).unwrap();
                
                let artist = catalog.track_artist(meta.artist);
                let title = meta.title.unwrap_or(filename.to_string());
                return Some(Track::init(artist, source_file, title, transcoded_file));
            }
        }
    }
    
    None
}