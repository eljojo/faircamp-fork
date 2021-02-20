use iso_currency::Currency;
use std::{fs, path::Path};
use url::Url;

use crate::{
    asset_cache::CacheOptimization,
    audio_format::AudioFormat,
    build::Build,
    catalog::Catalog,
    download_option::DownloadOption,
    eno::{self, Element, FieldContent},
    localization::WritingDirection,
    payment_option::PaymentOption,
    styles::{Theme, ThemeBase},
    util
};

#[derive(Clone)]
pub struct Overrides {
    pub download_option: DownloadOption,
    pub download_formats: Vec<AudioFormat>,
    pub payment_options: Vec<PaymentOption>,
    pub release_artists: Option<Vec<String>>,
    pub release_text: Option<String>,
    pub release_title: Option<String>,
    pub streaming_format: AudioFormat,
    pub track_artists: Option<Vec<String>>
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            download_option: DownloadOption::Disabled,
            download_formats: Vec::with_capacity(5),  // assuming e.g. MP3 320 + MP3 V0 + Ogg Vorbis + AAC + FLAC as a reasonably frequent choice
            payment_options: Vec::with_capacity(5),   // assuming e.g. Liberapay + Patreon + PayPal + SEPA + Custom option as a reasonable complex assumption
            release_artists: None,
            release_text: None,
            release_title: None,
            streaming_format: AudioFormat::Mp3Cbr128,
            track_artists: None
        }
    }
}

pub fn apply_globals_and_overrides(path: &Path, build: &mut Build, catalog: &mut Catalog, overrides: &mut Overrides) {
    match fs::read_to_string(path) {
        Ok(content) => {
            match eno::parse(&content) {
                Ok(elements) => for element in elements {
                    match element {
                        Element::Empty { key } => match key.as_str() {
                            "disable_download" => overrides.download_option = DownloadOption::Disabled,
                            "free_download" => overrides.download_option = DownloadOption::init_free(),
                            key => error!("Ignoring unsupported Empty with key '{}' in manifest '{:?}'", key, path)
                        }
                        Element::Field { content: FieldContent::Entries(entries), key } => match key.as_str() {
                            "localization" => {
                                for entry in &entries {
                                    match entry.key.as_str() {
                                        "language" => build.localization.language = entry.value.clone(),
                                        "writing_direction" => match entry.value.as_str() {
                                            "ltr" => build.localization.writing_direction = WritingDirection::Ltr,
                                            "rtl" => build.localization.writing_direction = WritingDirection::Rtl,
                                            value => error!("Ignoring unsupported value '{}' for global 'localization.writing_direction' (supported values are 'ltr' and 'rtl') in manifest '{:?}'", value, path)
                                        }
                                        key => error!("Ignoring unsupported global 'localization.{}' in manifest '{:?}'", key, path)
                                    }
                                }
                            }
                            "payment_options" => {
                                overrides.payment_options = entries
                                    .iter()
                                    .filter_map(|entry|
                                        match entry.key.as_str() {
                                            "custom" => Some(PaymentOption::init_custom(&entry.value)),
                                            "liberapay" => Some(PaymentOption::init_liberapay(&entry.value)),
                                            key => {
                                                error!("Ignoring unsupported payment_options entry '{}' in manifest '{:?}'", key, path);
                                                None
                                            }
                                        }
                                    )
                                    .collect();
                            }
                            "theme" => {
                                if build.theme.is_some() {
                                    warn_global_set_repeatedly!("theme");
                                }
                                
                                let mut theme = Theme::defaults();
                                
                                for entry in &entries {
                                    match entry.key.as_str() {
                                        "background_image" => theme.background_image = Some(entry.value.clone()),  // TODO: Verify file exists at provided location
                                        "base" => match ThemeBase::from_manifest_key(entry.value.as_str()) {
                                            Some(variant) => theme.base = variant,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.base' (supported values are 'dark' and 'light') in manifest '{:?}'", entry.value, path)
                                        }
                                        "hue" => match entry.value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                            Some(degrees) => theme.hue = degrees,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.hue' (accepts an amount of degrees in the range 0-360) in manifest '{:?}'", entry.value, path)
                                        }
                                        "hue_spread" => match entry.value.parse::<i16>().ok() {
                                            Some(degree_offset) => theme.hue_spread = degree_offset,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.hue_spread' (accepts an amount of degrees as a signed integer) in manifest '{:?}'", entry.value, path)
                                        }
                                        "tint_back" => match entry.value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                                            Some(percentage) => theme.tint_back = percentage,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.tint_back' (accepts a percentage in the range 0-100) in manifest '{:?}'", entry.value, path)
                                        }
                                        "tint_front" => match entry.value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                                            Some(percentage) => theme.tint_front = percentage,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.tint_front' (accepts a percentage in the range 0-100) in manifest '{:?}'", entry.value, path)
                                        }
                                        key => error!("Ignoring unsupported global 'theme.{}' in manifest '{:?}'", key, path)
                                    }
                                }
                                
                                build.theme = Some(theme);
                            }
                            key => error!("Ignoring unsupported Field with key '{}' in manifest '{:?}'", key, path)
                        }
                        Element::Field { content: FieldContent::Items(items), key } => match key.as_str() {
                            "download_formats" => {
                                overrides.download_formats = items
                                    .iter()
                                    .filter_map(|key|
                                        match AudioFormat::from_manifest_key(key.as_str()) {
                                            None => {
                                                error!("Ignoring invalid download_formats setting value '{}' in {:?}", key, path);
                                                None
                                            }
                                            some_format => some_format
                                        }
                                    )
                                    .collect();
                            }
                            "release_artists" => overrides.release_artists = Some(items),
                            "track_artists" => overrides.release_artists = Some(items),
                            key => error!("Ignoring unsupported Field with key '{}' in manifest '{:?}'", key, path)
                        }
                        Element::Field { content: FieldContent::Value(value), key } => match key.as_str() {
                            "base_url" => match Url::parse(&value) {
                                Ok(url) => {
                                    if let Some(previous_url) = &build.base_url {
                                        warn_global_set_repeatedly!("base_url", previous_url, url);
                                    }
                                    
                                    build.base_url = Some(url);
                                }
                                Err(err) => error!("Ignoring invalid base_url setting value '{}' in manifest '{:?}' ({})", value, path, err)
                            }
                            "cache_optimization" => match CacheOptimization::from_manifest_key(value.as_str()) {
                                Some(strategy) => {
                                    if build.cache_optimization != CacheOptimization::Default {
                                        warn_global_set_repeatedly!("cache_optimization", build.cache_optimization, strategy);
                                    }
                                    
                                    build.cache_optimization = strategy;
                                }
                                None => error!("Ignoring invalid cache_optimization setting '{}' (available: delayed, immediate, manual, wipe) in {:?}", value, path)
                            }
                            "catalog_text" => {
                                if let Some(previous) = &catalog.text {
                                    warn_global_set_repeatedly!("catalog_text", previous, value);
                                }
                                
                                catalog.text = Some(value);      
                            }
                            "catalog_title" => {
                                if let Some(previous) = &catalog.title {
                                    warn_global_set_repeatedly!("catalog_title", previous, value);
                                }
                                
                                catalog.title = Some(value);      
                            }
                            "download_format" => match AudioFormat::from_manifest_key(value.as_str()) {
                                Some(format) => overrides.download_formats = vec![format],
                                None => error!("Ignoring invalid download_format setting value '{}' in {:?}", value, path)
                            }
                            "feed_image" => {
                                if let Some(previous) = &catalog.feed_image {
                                    warn_global_set_repeatedly!("feed_image", previous, value);
                                }
                                
                                catalog.feed_image = Some(value); // TODO: Verify file exists at provided location
                            },
                            "paid_download" =>  {
                                let mut split_by_whitespace = value.split_ascii_whitespace();
                                
                                if let Some(first_token) = split_by_whitespace.next() {
                                    if let Some(currency) = Currency::from_code(first_token) {
                                        let recombined = &value[4..];
                                        
                                        if recombined.ends_with("+") {
                                            if let Ok(amount_parsed) = recombined[..(recombined.len() - 1)].parse::<f32>() {
                                                overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..f32::INFINITY);
                                            } else {
                                                error!("Ignoring paid_download option '{}' with malformed minimum price in {:?}", value, path);
                                            }
                                        } else {
                                            let mut split_by_dash = recombined.split("-");
                                            
                                            if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                                                if let Some(max_amount) = split_by_dash.next() {
                                                    if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                                                        overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..max_amount_parsed);
                                                    } else {
                                                        error!("Ignoring paid_download option '{}' with malformed minimum price in {:?}", value, path);
                                                    }
                                                } else {
                                                    overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..amount_parsed);
                                                }
                                            } else {
                                                error!("Ignoring paid_download option '{}' with malformed price in {:?}", value, path);
                                            }
                                        }
                                    } else if let Some(last_token) = split_by_whitespace.last() {
                                        if let Some(currency) = Currency::from_code(last_token) {
                                            let recombined = &value[..(value.len() - 4)];
                                            
                                            // TODO: DRY - exact copy from above
                                            if recombined.ends_with("+") {
                                                if let Ok(amount_parsed) = recombined[..(recombined.len() - 1)].parse::<f32>() {
                                                    overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..f32::INFINITY);
                                                } else {
                                                    error!("Ignoring paid_download option '{}' with malformed minimum price in {:?}", value, path);
                                                }
                                            } else {
                                                let mut split_by_dash = recombined.split("-");
                                                
                                                if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                                                    if let Some(max_amount) = split_by_dash.next() {
                                                        if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                                                            overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..max_amount_parsed);
                                                        } else {
                                                            error!("Ignoring paid_download option '{}' with malformed minimum price in {:?}", value, path);
                                                        }
                                                    } else {
                                                        overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..amount_parsed);
                                                    }
                                                } else {
                                                    error!("Ignoring paid_download option '{}' with malformed price in {:?}", value, path);
                                                }
                                            }
                                        } else {
                                            error!("Ignoring paid_download option '{}' without recognizable currency code in {:?}", value, path);
                                        }
                                    } else {
                                        error!("Ignoring unrecognized paid_download option '{}' in {:?}", value, path);
                                    }
                                } else {
                                    error!("Ignoring unrecognized paid_download option '{}' in {:?}", value, path);
                                }
                            }
                            "release_artist" => overrides.release_artists = Some(vec![value]),
                            "release_text" => overrides.release_text = Some(util::markdown_to_html(&value)),
                            "release_title" => overrides.release_title = Some(value),
                            "streaming_quality" => match value.as_str() {
                                "standard" => overrides.streaming_format = AudioFormat::Mp3Cbr128,
                                "transparent" => overrides.streaming_format = AudioFormat::Mp3VbrV0,
                                value => error!("Ignoring invalid streaming_quality setting value '{}' (available: standard, transparent) in {:?}", value, path)
                            }
                            "track_artist" => overrides.track_artists = Some(vec![value]),
                            key => error!("Ignoring unsupported Field with key '{}' in manifest '{:?}'", key, path)
                        }
                        element => error!("Ignoring unsupported element '{:?}' in manifest '{:?}'", element, path)
                    }
                }
                Err(err) => error!("Syntax error in manifest {:?} ({})", path, err)
            }
        }
        Err(err) => error!("Could not read manifest {:?} ({})", path, err)
    } 
}