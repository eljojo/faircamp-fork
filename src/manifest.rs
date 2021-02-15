use std::fs;
use std::path::Path;
use url::Url;

use crate::{
    audio_format::AudioFormat,
    download_option::DownloadOption,
    download_formats::DownloadFormats,
    message,
    styles::{Theme, DARK_THEME, LIGHT_THEME}
};

pub struct Globals {
    pub background_image: Option<String>,
    pub base_url: Option<Url>,
    pub catalog_text: Option<String>,
    pub catalog_title: Option<String>,
    pub theme: Option<Theme>
}

#[derive(Clone)]
pub struct Overrides {
    pub download_option: DownloadOption,
    pub download_formats: DownloadFormats,
    pub release_artists: Option<Vec<String>>,
    pub release_text: Option<String>,
    pub streaming_format: AudioFormat,
    pub track_artists: Option<Vec<String>>
}

impl Globals {
    pub fn empty() -> Globals {
        Globals {
            background_image: None,
            base_url: None,
            catalog_text: None,
            catalog_title: None,
            theme: None
        }
    }
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            download_option: DownloadOption::Disabled,
            download_formats: DownloadFormats::none(),
            release_artists: None,
            release_text: None,
            streaming_format: AudioFormat::Mp3Cbr128,
            track_artists: None
        }
    }
}

pub fn apply_globals_and_overrides(path: &Path, globals: &mut Globals, overrides: &mut Overrides) {
    match fs::read_to_string(path) {
        Ok(content) => for trimmed_line in content.lines().map(|line| line.trim()) {
            if trimmed_line.starts_with("background_image:") {    
                if let Some(previous_image) = &globals.background_image {
                    message::warning(&format!(
                        "Global 'background_image' is set more than once ('{previous_image}', '{new_image}')",
                        previous_image=previous_image,
                        new_image=trimmed_line[17..].trim()
                    ));
                }
                
                globals.background_image = Some(trimmed_line[17..].trim().to_string());    
            } else if trimmed_line.starts_with("base_url:") {        
                match Url::parse(trimmed_line[9..].trim()) {
                    Ok(url) => {
                        if let Some(previous_url) = &globals.base_url {
                            message::warning(&format!(
                                "Global 'base_url' is set more than once ('{previous_url}', '{new_url}')",
                                previous_url=previous_url,
                                new_url=url
                            ));
                        }
                        
                        globals.base_url = Some(url);
                    }
                    Err(err) => {
                        message::error(&format!(
                            "Global 'base_url' supplied with invalid value ({err})",
                            err=err
                        ));
                    }
                };
                 Some(trimmed_line[9..].trim().to_string());
            } else if trimmed_line.starts_with("catalog_text:") {
                if let Some(previous_title) = &globals.catalog_text {
                    message::warning(&format!(
                        "Global 'catalog_text' is set more than once ('{previous_title}', '{new_title}')",
                        previous_title=previous_title,
                        new_title=trimmed_line[14..].trim()
                    ));
                }
                
                globals.catalog_text = Some(trimmed_line[14..].trim().to_string());                
            } else if trimmed_line.starts_with("catalog_title:") {
                if let Some(previous_title) = &globals.catalog_title {
                    message::warning(&format!(
                        "Global 'catalog_title' is set more than once ('{previous_title}', '{new_title}')",
                        previous_title=previous_title,
                        new_title=trimmed_line[14..].trim()
                    ));
                }
                
                globals.catalog_title = Some(trimmed_line[14..].trim().to_string());
            } else if trimmed_line == "disable-aac" {
                overrides.download_formats.aac = false;
            } else if trimmed_line == "disable-aiff" {
                overrides.download_formats.aiff = false;
            } else if trimmed_line == "disable-flac" {
                overrides.download_formats.flac = false;
            } else if trimmed_line == "disable-mp3-320" {
                overrides.download_formats.mp3_320 = false;
            } else if trimmed_line == "disable-mp3-v0" {
                overrides.download_formats.mp3_v0 = false;
            } else if trimmed_line == "disable-ogg-vorbis" {
                overrides.download_formats.ogg_vorbis = false;
            } else if trimmed_line == "disable-wav" {
                overrides.download_formats.wav = false;
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
            } else if trimmed_line == "enable-aiff" {
                overrides.download_formats.aiff = true;
            } else if trimmed_line == "enable-flac" {
                overrides.download_formats.flac = true;
            } else if trimmed_line == "enable-mp3-320" {
                overrides.download_formats.mp3_320 = true;
            } else if trimmed_line == "enable-mp3-v0" {
                overrides.download_formats.mp3_v0 = true;
            } else if trimmed_line == "enable-ogg-vorbis" {
                overrides.download_formats.ogg_vorbis = true;
            } else if trimmed_line == "enable-wav" {
                overrides.download_formats.wav = true;
            } else if trimmed_line.starts_with("release-artist:") {
                overrides.release_artists = Some(vec![trimmed_line[15..].trim().to_string()]);
            } else if trimmed_line.starts_with("release-text:") {
                overrides.release_text = Some(trimmed_line[13..].trim().to_string());
            } else if trimmed_line == "stream-mp3-128" {
                overrides.streaming_format = AudioFormat::Mp3Cbr128;
            } else if trimmed_line == "stream-mp3-320" {
                overrides.streaming_format = AudioFormat::Mp3Cbr320;
            } else if trimmed_line == "stream-mp3-v0" {
                overrides.streaming_format = AudioFormat::Mp3VbrV0;
            } else if trimmed_line.starts_with("theme:") {
                if globals.theme.is_some() {
                    message::warning(&format!("Global 'theme' is set more than once"));
                }
                
                match trimmed_line[6..].trim() {
                    "dark" => globals.theme = Some(DARK_THEME),
                    "light" => globals.theme = Some(LIGHT_THEME),
                    unsupported => message::error(&format!("Ignoring unsupported value '{}' for global 'theme' (supported values are 'dark' and 'light')", unsupported))
                }
            } else if trimmed_line.starts_with("track-artist:") {
                overrides.track_artists = Some(vec![trimmed_line[13..].trim().to_string()]);
            } else if !trimmed_line.is_empty() && !trimmed_line.starts_with(">") {
                message::error(&format!("Ignoring unrecognized manifest line '{}'", trimmed_line));
            }
        }
        Err(err) => message::error(&format!("Could not read meta file {:?} ({})", path, err))
    } 
}