use std::fs;
use std::path::Path;

use crate::download_option::DownloadOption;
use crate::download_formats::DownloadFormats;

#[derive(Clone)]
pub struct Overrides {
    pub download_option: DownloadOption,
    pub download_formats: DownloadFormats,
    pub release_artists: Option<Vec<String>>,
    pub release_text: Option<String>,
    pub track_artists: Option<Vec<String>>
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            download_option: DownloadOption::Disabled,
            download_formats: DownloadFormats::none(),
            release_artists: None,
            release_text: None,
            track_artists: None
        }
    }
}

pub fn apply_overrides(path: &Path, overrides: &mut Overrides) {
    match fs::read_to_string(path) {
        Ok(content) => for trimmed_line in content.lines().map(|line| line.trim()) {
            if trimmed_line == "disable-flac" {
                debug!("Applying disable-flac override");
                overrides.download_formats.flac = false;
            } else if trimmed_line == "disable-mp3-320" {
                debug!("Applying disable-mp3-320 override");
                overrides.download_formats.mp3_320 = false;
            } else if trimmed_line == "disable-mp3-v0" {
                debug!("Applying disable-mp3-v0 override");
                overrides.download_formats.mp3_v0 = false;
            } else if trimmed_line.starts_with("download:") {
                match trimmed_line[9..].trim() {
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
                    _ => error!("Ignoring invalid download setting value '{}' in {:?}", &trimmed_line[10..], path)
                }
            } else if trimmed_line == "enable-flac" {
                debug!("Applying enable-flac override");
                overrides.download_formats.flac = true;
            } else if trimmed_line == "enable-mp3-320" {
                debug!("Applying enable-mp3-320 override");
                overrides.download_formats.mp3_320 = true;
            } else if trimmed_line == "enable-mp3-v0" {
                debug!("Applying enable-mp3-v0 override");
                overrides.download_formats.mp3_v0 = true;
            } else if trimmed_line.starts_with("release-artist:") {
                debug!("Applying release-artist override {}", trimmed_line[15..].trim());
                overrides.release_artists = Some(vec![trimmed_line[15..].trim().to_string()]);
            } else if trimmed_line.starts_with("release-text:") {
                debug!("Applying release-text override {}", trimmed_line[13..].trim());
                overrides.release_text = Some(trimmed_line[13..].trim().to_string());
            } else if trimmed_line.starts_with("track-artist:") {
                debug!("Applying track-artist override {}", trimmed_line[13..].trim());
                overrides.track_artists = Some(vec![trimmed_line[13..].trim().to_string()]);
            } else if !trimmed_line.is_empty() && !trimmed_line.starts_with(">") {
                error!("Ignoring unrecognized manifest line {}", trimmed_line);
            }
        }
        Err(err) => error!("Could not read meta file {:?} ({})", path, err)
    } 
}