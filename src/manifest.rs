use std::fs;
use std::path::Path;

use crate::download_option::DownloadOption;

pub struct Manifest {
    pub download_option: Option<DownloadOption>,
    pub release_artist: Option<String>,
    pub track_artist: Option<String>
}

impl Manifest {
    pub fn empty() -> Manifest {
        Manifest {
            download_option: None,
            release_artist: None,
            track_artist: None
        }
    }
}

pub fn read(path: &Path) -> Manifest {
    let mut manifest = Manifest::empty();
    
    match fs::read_to_string(path) {
        Ok(content) => for line in content.lines() {
            if line.starts_with("download:") {
                match line[9..].trim() {
                    "disabled" => {
                        manifest.download_option = Some(DownloadOption::Disabled);
                    },
                    "free" => {
                        manifest.download_option = Some(DownloadOption::init_free());
                    },
                    "anyprice" => {
                        manifest.download_option = Some(DownloadOption::NameYourPrice);
                    },
                    "minprice" => {
                        manifest.download_option = Some(DownloadOption::PayMinimum("10 Republican Credits".to_string()));
                    },
                    "exactprice" => {
                        manifest.download_option = Some(DownloadOption::PayExactly("10 Republican Credits".to_string()));
                    },
                    _ => error!("Ignoring invalid download setting value '{}' in {:?}", &line[10..], path)
                }
            } else if line.starts_with("release-artist:") {
                manifest.release_artist = Some(line[15..].trim().to_string());
            } else if line.starts_with("track-artist:") {
                manifest.track_artist = Some(line[13..].trim().to_string());
            } else if !line.trim().is_empty() && !line.starts_with(">") {
                warn!("Ignoring unrecognized manifest line {}", line);
            }
        }
        Err(err) => error!("Could not read meta file {:?} ({})", path, err)
    } 
    
    manifest
}