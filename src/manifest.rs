use std::fs;
use std::path::Path;
use url::Url;

use crate::{
    audio_format::AudioFormat,
    download_option::DownloadOption,
    download_formats::DownloadFormats,
    eno::{self, Element, FieldContent},
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
        Ok(content) => {
            match eno::parse(&content) {
                Ok(elements) => for element in elements {
                    match element {
                        Element::Empty { key } => match key.as_str() {
                            "disable_aac" => overrides.download_formats.aac = false,
                            "disable_aiff" => overrides.download_formats.aiff = false,
                            "disable_flac" => overrides.download_formats.flac = false,
                            "disable_mp3_320" => overrides.download_formats.mp3_320 = false,
                            "disable_mp3_v0" => overrides.download_formats.mp3_v0 = false,
                            "disable_ogg_vorbis" => overrides.download_formats.ogg_vorbis = false,
                            "disable_wav" => overrides.download_formats.wav = false,
                            "enable_aac" => overrides.download_formats.aac = true,
                            "enable_aiff" => overrides.download_formats.aiff = true,
                            "enable_flac" => overrides.download_formats.flac = true,
                            "enable_mp3_320" => overrides.download_formats.mp3_320 = true,
                            "enable_mp3_v0" => overrides.download_formats.mp3_v0 = true,
                            "enable_ogg_vorbis" => overrides.download_formats.ogg_vorbis = true,
                            "enable_wav" => overrides.download_formats.wav = true,
                            key => message::error(&format!("Ignoring unsupported Empty with key '{key}' in manifest '{path:?}'", key=key, path=path))
                        }
                        Element::Field { content: FieldContent::Items(items), key } => match key.as_str() {
                            "release_artists" => overrides.release_artists = Some(items),
                            "track_artists" => overrides.release_artists = Some(items),
                            key => message::error(&format!("Ignoring unsupported Field with key '{key}' in manifest '{path:?}'", key=key, path=path))
                        }
                        Element::Field { content: FieldContent::Value(value), key } => match key.as_str() {
                            "background_image" => {
                                if let Some(previous_image) = &globals.background_image {
                                    message::warning(&format!(
                                        "Global 'background_image' is set more than once ('{previous_image}', '{new_image}')",
                                        previous_image=previous_image,
                                        new_image=value
                                    ));
                                }
                                
                                globals.background_image = Some(value); 
                            }
                            "base_url" => match Url::parse(&value) {
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
                            }
                            "catalog_text" => {
                                if let Some(previous_text) = &globals.catalog_text {
                                    message::warning(&format!(
                                        "Global 'catalog_text' is set more than once ('{previous_text}', '{new_text}')",
                                        previous_text=previous_text,
                                        new_text=value
                                    ));
                                }
                                
                                globals.catalog_text = Some(value);      
                            }
                            "catalog_title" => {
                                if let Some(previous_title) = &globals.catalog_title {
                                    message::warning(&format!(
                                        "Global 'catalog_title' is set more than once ('{previous_title}', '{new_title}')",
                                        previous_title=previous_title,
                                        new_title=value
                                    ));
                                }
                                
                                globals.catalog_title = Some(value);      
                            }
                            "download" => {
                                match value.as_str() {
                                    "disabled" => overrides.download_option = DownloadOption::Disabled,
                                    "free" => overrides.download_option = DownloadOption::init_free(),
                                    "anyprice" => overrides.download_option = DownloadOption::NameYourPrice,
                                    "minprice" => overrides.download_option = DownloadOption::PayMinimum("10 Republican Credits".to_string()),
                                    "exactprice" => overrides.download_option = DownloadOption::PayExactly("10 Republican Credits".to_string()),
                                    value => message::error(&format!("Ignoring invalid download setting value '{value}' in {path:?}", path=path, value=value))
                                }
                            }
                            "release_artist" => overrides.release_artists = Some(vec![value]),
                            "release_text" => overrides.release_text = Some(value),
                            "streaming_quality" => match value.as_str() {
                                "standard" => overrides.streaming_format = AudioFormat::Mp3Cbr128,
                                "transparent" => overrides.streaming_format = AudioFormat::Mp3VbrV0,
                                value => message::error(&format!("Ignoring invalid streaming_quality setting value '{value}' (available: standard, transparent) in {path:?}", path=path, value=value))
                            }
                            "theme" => {
                                if globals.theme.is_some() {
                                    message::warning(&format!("Global 'theme' is set more than once"));
                                }
                                
                                match value.as_str() {
                                    "dark" => globals.theme = Some(DARK_THEME),
                                    "light" => globals.theme = Some(LIGHT_THEME),
                                    unsupported => message::error(&format!("Ignoring unsupported value '{}' for global 'theme' (supported values are 'dark' and 'light')", unsupported))
                                }
                            }
                            "track_artist" => overrides.track_artists = Some(vec![value]),
                            key => message::error(&format!("Ignoring unsupported Field with key '{key}' in manifest '{path:?}'", key=key, path=path))
                        }
                        element => message::error(&format!("Ignoring unsupported element '{element:?}' in manifest '{path:?}'", element=element, path=path))
                    }
                }
                Err(err) => message::error(&format!("Syntax error in manifest {path:?} ({err})", path=path, err=err))
            }
        }
        Err(err) => message::error(&format!("Could not read manifest {path:?} ({err})", path=path, err=err))
    } 
}