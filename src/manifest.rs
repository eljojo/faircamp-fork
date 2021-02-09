use std::fs;
use std::path::Path;

use crate::download_option::DownloadOption;

#[derive(Clone)]
pub struct Overrides {
    pub download_option: DownloadOption,
    pub release_artists: Option<Vec<String>>,
    pub track_artists: Option<Vec<String>>
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            download_option: DownloadOption::Disabled,
            release_artists: None,
            track_artists: None
        }
    }
}

pub fn apply_overrides(path: &Path, overrides: &mut Overrides) {
    match fs::read_to_string(path) {
        Ok(content) => for line in content.lines() {
            if line.starts_with("download:") {
                match line[9..].trim() {
                    "disabled" => {
                        debug!("Applying download option override (disabled)");
                        overrides.download_option = DownloadOption::Disabled;
                    },
                    "free" => {
                        debug!("Applying download option override (free)");
                        overrides.download_option = DownloadOption::init_free();
                    },
                    "anyprice" => {
                        debug!("Applying download option override (anyprice)");
                        overrides.download_option = DownloadOption::NameYourPrice;
                    },
                    "minprice" => {
                        debug!("Applying download option override (minprice)");
                        overrides.download_option = DownloadOption::PayMinimum("10 Republican Credits".to_string());
                    },
                    "exactprice" => {
                        debug!("Applying download option override (exactprice)");
                        overrides.download_option = DownloadOption::PayExactly("10 Republican Credits".to_string());
                    },
                    _ => error!("Ignoring invalid download setting value '{}' in {:?}", &line[10..], path)
                }
            } else if line.starts_with("release-artist:") {
                debug!("Applying release-artist override {}", line[15..].trim());
                overrides.release_artists = Some(vec![line[15..].trim().to_string()]);
            } else if line.starts_with("track-artist:") {
                debug!("Applying track-artist override {}", line[13..].trim());
                overrides.track_artists = Some(vec![line[13..].trim().to_string()]);
            } else if !line.trim().is_empty() && !line.starts_with(">") {
                error!("Ignoring unrecognized manifest line {}", line);
            }
        }
        Err(err) => error!("Could not read meta file {:?} ({})", path, err)
    } 
}