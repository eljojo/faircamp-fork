use std::{fs, path::{Path, PathBuf}};
use uuid::Uuid;

use crate::meta;
use crate::release::Release;
use crate::track::Track;
use crate::types::Artist;

const SUPPORTED_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "wav"];

pub fn source_artist() -> Artist {
    Artist {
        image: None,
        links: Vec::new(),
        location: None,
        name: String::from("Dummy Artist")
    }
}

pub fn source_releases(build_dir: &Path, dir: PathBuf, releases: &mut Vec<Release>) -> Result<(), String> {
    match dir.read_dir() {
        Ok(dir_entries) => {
            let mut pending_release: Option<Release> = None;
            
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Ok(file_type) = dir_entry.file_type() {
                        if file_type.is_dir() {
                            source_releases(build_dir, dir_entry.path(), releases);
                        } else if file_type.is_file() {
                            if let Some(track) = source_track(build_dir, dir_entry.path()) {
                                if let Some(release) = &mut pending_release {
                                    release.tracks.push(track);
                                } else {
                                    let filename = dir_entry.path().parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
                                    pending_release = Some(Release::init(filename, vec![track]));
                                }
                            }
                        } else if file_type.is_symlink() {
                            // TODO: Symlinks ignored for now, handle if and when requested
                        } else {
                            
                        }
                    }
                }
            }
            
            if let Some(release) = pending_release {
                releases.push(release);
            }
            
            Ok(())
        }
        Err(_) => Err(String::from("Cannot read directory."))
    }
}

pub fn source_track(build_dir: &Path, path: PathBuf) -> Option<Track> {
    let path_clone = path.clone();
    let filename = path.file_name().unwrap().to_str().unwrap();
    
    if let Some(extension_osstr) = path.extension() {
        if let Some(extension_str) =  extension_osstr.to_str() {
            if SUPPORTED_EXTENSIONS.contains(&extension_str.to_lowercase().as_str()) {
                let uuid = Uuid::new_v4().to_simple().to_string();
                let source_file = path.to_str().unwrap().to_string();
                let title = meta::extract_title(extension_str, &path).unwrap_or(filename.to_string());
                let transcoded_file = format!("{}.{}", uuid, extension_str);
                
                fs::copy(path_clone, build_dir.join(&transcoded_file)).unwrap();
                
                return Some(Track::init(source_file, title, transcoded_file));
            }
        }
    }
    
    None
}