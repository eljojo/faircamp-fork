use std::fs;
use std::path::Path;

use crate::{
    download_option::DownloadOption,
    download_formats::DownloadFormats,
    message
};

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
            if trimmed_line == "disable-aac" {
                overrides.download_formats.aac = false;
            } else if trimmed_line == "disable-flac" {
                overrides.download_formats.flac = false;
            } else if trimmed_line == "disable-mp3-320" {
                overrides.download_formats.mp3_320 = false;
            } else if trimmed_line == "disable-mp3-v0" {
                overrides.download_formats.mp3_v0 = false;
            } else if trimmed_line == "disable-ogg-vorbis" {
                overrides.download_formats.ogg_vorbis = false;
            } else if trimmed_line.starts_with("download:") {
                match trimmed_line[9..].trim() {
                    "disabled" => {
                        overrides.download_option = DownloadOption::Disabled;
                    },
                    "free" => {
                        overrides.download_option = DownloadOption::init_free();
                    },
                    "anyprice" => {
                        overrides.download_option = DownloadOption::NameYourPrice;
                    },
                    "minprice" => {
                        overrides.download_option = DownloadOption::PayMinimum("10 Republican Credits".to_string());
                    },
                    "exactprice" => {
                        overrides.download_option = DownloadOption::PayExactly("10 Republican Credits".to_string());
                    },
                    _ => message::error(&format!("Ignoring invalid download setting value '{}' in {:?}", &trimmed_line[9..], path))
                }
            } else if trimmed_line == "enable-aac" {
                overrides.download_formats.aac = true;
            } else if trimmed_line == "enable-flac" {
                overrides.download_formats.flac = true;
            } else if trimmed_line == "enable-mp3-320" {
                overrides.download_formats.mp3_320 = true;
            } else if trimmed_line == "enable-mp3-v0" {
                overrides.download_formats.mp3_v0 = true;
            } else if trimmed_line == "enable-ogg-vorbis" {
                overrides.download_formats.ogg_vorbis = true;
            } else if trimmed_line.starts_with("release-artist:") {
                overrides.release_artists = Some(vec![trimmed_line[15..].trim().to_string()]);
            } else if trimmed_line.starts_with("release-text:") {
                overrides.release_text = Some(trimmed_line[13..].trim().to_string());
            } else if trimmed_line.starts_with("track-artist:") {
                overrides.track_artists = Some(vec![trimmed_line[13..].trim().to_string()]);
            } else if !trimmed_line.is_empty() && !trimmed_line.starts_with(">") {
                message::error(&format!("Ignoring unrecognized manifest line {}", trimmed_line));
            }
        }
        Err(err) => message::error(&format!("Could not read meta file {:?} ({})", path, err))
    } 
}