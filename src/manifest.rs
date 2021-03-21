use enolib::{FieldContent, Kind};
use iso_currency::Currency;
use std::{
    fs,
    path::Path,
    rc::Rc
};
use url::Url;

use crate::{
    artist::Artist,
    asset_cache::CacheOptimization,
    audio_format::AudioFormat,
    build::Build,
    catalog::Catalog,
    download_option::DownloadOption,
    localization::WritingDirection,
    payment_option::PaymentOption,
    styles::{Theme, ThemeBase, ThemeFont},
    util
};

macro_rules! file_line {
    ($path:expr, $element:expr) => {
        format!(
            "{}:{}",
            $path.display(),
            $element.line_number
        );
    };
}

#[derive(Clone)]
pub struct LocalOptions {
    pub release_permalink: Option<String>
}

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

impl LocalOptions {
    pub fn new() -> LocalOptions {
        LocalOptions {
            release_permalink: None
        }
    }
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

pub fn apply_options(path: &Path, build: &mut Build, catalog: &mut Catalog, local_options: &mut LocalOptions, overrides: &mut Overrides) {
    match fs::read_to_string(path) {
        Ok(content) => {
            match enolib::parse(&content) {
                Ok(document) => for element in document.elements {
                    match element.key.as_ref() {
                        "artist" => match &element.kind {
                            Kind::Section(section_elements) => {
                                let mut name = None;
                                let mut permalink = None;
                                let mut text = None;
                                
                                for section_element in section_elements {
                                    match section_element.key.as_ref() {
                                        "name" => match &section_element.kind {
                                            Kind::Field(FieldContent::Value(value)) => name = Some(value.clone()),
                                            _ => error!("Ignoring invalid artist.name option (can only be a field containing a value) in {}", file_line!(path, section_element))
                                        }
                                        "permalink" => match &section_element.kind {
                                            Kind::Field(FieldContent::Value(value)) => permalink = Some(value.clone()),
                                            _ => error!("Ignoring invalid artist.permalink option (can only be a field containing a value) in {}", file_line!(path, section_element))
                                        }
                                        "text" => match &section_element.kind {
                                            Kind::Embed(Some(value)) |
                                            Kind::Field(FieldContent::Value(value)) => text = Some(util::markdown_to_html(&value)),
                                            _ => error!("Ignoring invalid artist.text option (can only be an embed or field containing a value) in {}", file_line!(path, section_element))
                                        }
                                        key => error!("Ignoring unsupported artist.{} option in manifest '{:?}'", key, path)
                                    }
                                }
                                
                                if let Some(name) = name {                                    
                                    // TODO: At this point the artist might already exist - need to scan
                                    //       over all existing artists and modify the existing one if found.
                                    //       Next challenge is implementing the "aliases" functionality, i.e.
                                    //       allow user to specify multiple names under which the artist appears
                                    //       (in metadata, that is), and then any previously created or
                                    //       successively found artists with that alias need to be combined into
                                    //       one, including changing the reference on the releases/tracks themselves,
                                    //       this shall be fun, fun, fun! :)
                                    let new_artist = Artist::new_from_manifest(name, permalink, text);
                                    
                                    catalog.artists.push(Rc::new(new_artist));
                                } else {
                                    error!("An artist was specified without a name, and therefore discarded, in {}", file_line!(path, element))
                                }
                            }
                            _ => error!("Ignoring invalid artist option (can only be a section containing specific elements) in {}", file_line!(path, element))
                        }
                        "base_url" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                match Url::parse(&value) {
                                    Ok(url) => {
                                        if let Some(previous_url) = &build.base_url {
                                            warn_global_set_repeatedly!("base_url", previous_url, url);
                                        }
                                        
                                        build.base_url = Some(url);
                                    }
                                    Err(err) => error!("Ignoring invalid base_url setting value '{}' in {} ({})", value, file_line!(path, element), err)
                                }
                            }
                            _ => error!("Ignoring invalid base_url option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "cache_optimization" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                match CacheOptimization::from_manifest_key(value.as_str()) {
                                    Some(strategy) => {
                                        if build.cache_optimization != CacheOptimization::Default {
                                            warn_global_set_repeatedly!("cache_optimization", build.cache_optimization, strategy);
                                        }
                                        
                                        build.cache_optimization = strategy;
                                    }
                                    None => error!("Ignoring invalid cache_optimization setting '{}' (available: delayed, immediate, manual, wipe) in {}", value, file_line!(path, element))
                                }
                            }
                            _ => error!("Ignoring invalid cache_optimization option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "catalog_text" => match element.kind {
                            Kind::Embed(Some(value)) |
                            Kind::Field(FieldContent::Value(value)) => {
                                if let Some(previous) = &catalog.text {
                                    warn_global_set_repeatedly!("catalog_text", previous, value);
                                }
                                
                                catalog.text = Some(value);      
                            }
                            _ => error!("Ignoring invalid catalog_text option (can only be an embed or field containing a value) in {}", file_line!(path, element))
                        }
                        "catalog_title" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                if let Some(previous) = &catalog.title {
                                    warn_global_set_repeatedly!("catalog_title", previous, value);
                                }
                                
                                catalog.title = Some(value);      
                            }
                            _ => error!("Ignoring invalid catalog_title option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "disable_download" => match element.kind {
                            Kind::Empty  => overrides.download_option = DownloadOption::Disabled,
                            _ => error!("Ignoring invalid disable_download option (can only be an empty) in {}", file_line!(path, element))
                        }
                        "download_format" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                match AudioFormat::from_manifest_key(value.as_str()) {
                                    Some(format) => overrides.download_formats = vec![format],
                                    None => error!("Ignoring invalid download_format setting value '{}' in {}", value, file_line!(path, element))
                                }
                            }
                            _ => error!("Ignoring invalid download_format option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "download_formats" => match &element.kind {
                            Kind::Field(FieldContent::Items(items))  => {
                                overrides.download_formats = items
                                    .iter()
                                    .filter_map(|item|
                                        match AudioFormat::from_manifest_key(item.value.as_str()) {
                                            None => {
                                                error!("Ignoring invalid download_formats format specifier '{}' in {}", item.value, file_line!(path, item));
                                                None
                                            }
                                            some_format => some_format
                                        }
                                    )
                                    .collect();
                            }
                            Kind::Field(FieldContent::None) => (),
                            _ => error!("Ignoring invalid download_formats option (can only be a field containing a list) in {}", file_line!(path, element))
                        }
                        "feed_image" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                if let Some(previous) = &catalog.feed_image {
                                    warn_global_set_repeatedly!("feed_image", previous, value);
                                }
                                
                                catalog.feed_image = Some(value); // TODO: Verify file exists at provided location
                            }
                            _ => error!("Ignoring invalid feed_image option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "free_download" => match element.kind {
                            Kind::Empty  => overrides.download_option = DownloadOption::init_free(),
                            _ => error!("Ignoring invalid free_download option (can only be an empty) in {}", file_line!(path, element))
                        }
                        "localization" => match element.kind {
                            Kind::Field(FieldContent::Attributes(attributes))  => {
                                for attribute in &attributes {
                                    match attribute.key.as_str() {
                                        "language" => build.localization.language = attribute.value.clone(),
                                        "writing_direction" => match attribute.value.as_str() {
                                            "ltr" => build.localization.writing_direction = WritingDirection::Ltr,
                                            "rtl" => build.localization.writing_direction = WritingDirection::Rtl,
                                            value => error!("Ignoring unsupported value '{}' for global 'localization.writing_direction' (supported values are 'ltr' and 'rtl') in {}", value, file_line!(path, element))
                                        }
                                        key => error!("Ignoring unsupported global 'localization.{}' in {}", key, file_line!(path, element))
                                    }
                                }
                            }
                            Kind::Field(FieldContent::None) => (),
                            _ => error!("Ignoring invalid localization option (can only be a field containing a map of attributes) in {}", file_line!(path, element))
                        }
                        "paid_download" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                let mut split_by_whitespace = value.split_ascii_whitespace();
                                
                                if let Some(first_token) = split_by_whitespace.next() {
                                    if let Some(currency) = Currency::from_code(first_token) {
                                        let recombined = &value[4..];
                                        
                                        if recombined.ends_with("+") {
                                            if let Ok(amount_parsed) = recombined[..(recombined.len() - 1)].parse::<f32>() {
                                                overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..f32::INFINITY);
                                            } else {
                                                error!("Ignoring paid_download option '{}' with malformed minimum price in {}", value, file_line!(path, element));
                                            }
                                        } else {
                                            let mut split_by_dash = recombined.split("-");
                                            
                                            if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                                                if let Some(max_amount) = split_by_dash.next() {
                                                    if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                                                        overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..max_amount_parsed);
                                                    } else {
                                                        error!("Ignoring paid_download option '{}' with malformed minimum price in {}", value, file_line!(path, element));
                                                    }
                                                } else {
                                                    overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..amount_parsed);
                                                }
                                            } else {
                                                error!("Ignoring paid_download option '{}' with malformed price in {}", value, file_line!(path, element));
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
                                                    error!("Ignoring paid_download option '{}' with malformed minimum price in {}", value, file_line!(path, element));
                                                }
                                            } else {
                                                let mut split_by_dash = recombined.split("-");
                                                
                                                if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                                                    if let Some(max_amount) = split_by_dash.next() {
                                                        if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                                                            overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..max_amount_parsed);
                                                        } else {
                                                            error!("Ignoring paid_download option '{}' with malformed minimum price in {}", value, file_line!(path, element));
                                                        }
                                                    } else {
                                                        overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..amount_parsed);
                                                    }
                                                } else {
                                                    error!("Ignoring paid_download option '{}' with malformed price in {}", value, file_line!(path, element));
                                                }
                                            }
                                        } else {
                                            error!("Ignoring paid_download option '{}' without recognizable currency code in {}", value, file_line!(path, element));
                                        }
                                    } else {
                                        error!("Ignoring unrecognized paid_download option '{}' in {}", value, file_line!(path, element));
                                    }
                                } else {
                                    error!("Ignoring unrecognized paid_download option '{}' in {}", value, file_line!(path, element));
                                }
                            }
                            _ => error!("Ignoring invalid paid_download option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "payment_options" => match &element.kind {
                            Kind::Field(FieldContent::Attributes(attributes))  => {
                                overrides.payment_options = attributes
                                    .iter()
                                    .filter_map(|attribute|
                                        match attribute.key.as_str() {
                                            "custom" => Some(PaymentOption::init_custom(&attribute.value)),
                                            "liberapay" => Some(PaymentOption::init_liberapay(&attribute.value)),
                                            key => {
                                                error!("Ignoring unsupported payment_options attribute '{}' in {}", key, file_line!(path, element));
                                                None
                                            }
                                        }
                                    )
                                    .collect();
                            }
                            Kind::Field(FieldContent::None) => (),
                            _ => error!("Ignoring invalid payment_options option (can only be a field containing a map of attributes) in {}", file_line!(path, element))
                        }
                        "release_artist" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => overrides.release_artists = Some(vec![value]),
                            Kind::Field(FieldContent::Items(_)) => error!("Ignoring release_artist option with multiple values (use the key release_artists instead) in {}", file_line!(path, element)),
                            _ => error!("Ignoring invalid release_artist option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "release_artists" => match element.kind {
                            Kind::Field(FieldContent::Items(items)) => overrides.release_artists = Some(items.iter().map(|item| item.value.clone()).collect()),
                            Kind::Field(FieldContent::None) => (),
                            _ => error!("Ignoring invalid release_artists option (can only be a field containing a list) in {}", file_line!(path, element))
                        }
                        "release_permalink" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                if let Some(previous) = &local_options.release_permalink {
                                    warn!("Option release_permalink is set more than once - overriding previous value '{}' with '{}'", previous, value);
                                }
                                
                                local_options.release_permalink = Some(value);
                            }
                            _ => error!("Ignoring invalid release_permalink option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "release_text" => match element.kind {
                            Kind::Embed(Some(value)) |
                            Kind::Field(FieldContent::Value(value)) => overrides.release_text = Some(util::markdown_to_html(&value)),
                            _ => error!("Ignoring invalid release_text option (can only be an embed or field containing a value) in {}", file_line!(path, element))
                        }
                        "release_title" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => overrides.release_title = Some(value),
                            _ => error!("Ignoring invalid release_title option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "streaming_quality" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => {
                                match value.as_str() {
                                    "standard" => overrides.streaming_format = AudioFormat::Mp3Cbr128,
                                    "transparent" => overrides.streaming_format = AudioFormat::Mp3VbrV0,
                                    value => error!("Ignoring invalid streaming_quality setting value '{}' (available: standard, transparent) in {}", value, file_line!(path, element))
                                }
                            },
                            _ => error!("Ignoring invalid streaming_quality option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "theme" => match element.kind {
                            Kind::Field(FieldContent::Attributes(attributes))  => {
                                if build.theme.is_some() {
                                    warn_global_set_repeatedly!("theme");
                                }
                                
                                let mut theme = Theme::defaults();
                                
                                for attribute in &attributes {
                                    match attribute.key.as_str() {
                                        "background_image" => theme.background_image = Some(attribute.value.clone()),  // TODO: Verify file exists at provided location
                                        "base" => match ThemeBase::from_manifest_key(attribute.value.as_str()) {
                                            Some(variant) => theme.base = variant,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.base' (supported values are 'dark' and 'light') in {}", attribute.value, file_line!(path, attribute))
                                        }
                                        "custom_font" => {
                                            if attribute.value.is_empty() {
                                                error!("Ignoring unsupported empty value for global 'theme.custom_font' (an existing path to a .woff2 file needs to be given) in {}", file_line!(path, attribute));
                                            } else {
                                                theme.font = ThemeFont::Custom(attribute.value.clone());
                                            }
                                        }
                                        "hue" => match attribute.value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                            Some(degrees) => theme.hue = degrees,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.hue' (accepts an amount of degrees in the range 0-360) in {}", attribute.value, file_line!(path, attribute))
                                        }
                                        "hue_spread" => match attribute.value.parse::<i16>().ok() {
                                            Some(degree_offset) => theme.hue_spread = degree_offset,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.hue_spread' (accepts an amount of degrees as a signed integer) in {}", attribute.value, file_line!(path, attribute))
                                        }
                                        "system_font" => {
                                            theme.font = if attribute.value.is_empty() || attribute.value == "sans" {
                                                ThemeFont::SystemSans
                                            } else if attribute.value == "mono" {
                                                ThemeFont::SystemMono
                                            } else {
                                                ThemeFont::System(attribute.value.clone())
                                            };
                                        }
                                        "tint_back" => match attribute.value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                                            Some(percentage) => theme.tint_back = percentage,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.tint_back' (accepts a percentage in the range 0-100) in {}", attribute.value, file_line!(path, attribute))
                                        }
                                        "tint_front" => match attribute.value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                                            Some(percentage) => theme.tint_front = percentage,
                                            None => error!("Ignoring unsupported value '{}' for global 'theme.tint_front' (accepts a percentage in the range 0-100) in {}", attribute.value, file_line!(path, attribute))
                                        }
                                        key => error!("Ignoring unsupported global 'theme.{}' in manifest '{:?}'", key, path)
                                    }
                                }
                                
                                build.theme = Some(theme);
                            }
                            Kind::Field(FieldContent::None) => (),
                            _ => error!("Ignoring invalid theme option (can only be a field containing a map of attributes) in {}", file_line!(path, element))
                        }
                        "track_artist" => match element.kind {
                            Kind::Field(FieldContent::Value(value)) => overrides.track_artists = Some(vec![value]),
                            Kind::Field(FieldContent::Items(_)) => error!("Ignoring track_artist option with multiple values (use the key track_artists instead) in {}", file_line!(path, element)),
                            _ => error!("Ignoring invalid track_artist option (can only be a field containing a value) in {}", file_line!(path, element))
                        }
                        "track_artists" => match element.kind {
                            Kind::Field(FieldContent::Items(items)) => overrides.track_artists = Some(items.iter().map(|item| item.value.clone()).collect()),
                            Kind::Field(FieldContent::None) => (),
                            _ => error!("Ignoring invalid track_artists option (can only be a field containing a list) in {}", file_line!(path, element))
                        }
                        unsupported_key => error!("Ignoring unsupported option '{}' in {}", unsupported_key, file_line!(path, element))
                    }
                }
                Err(err) => error!("Syntax error in {}:{} ({})", path.display(), err.line, err)
            }
        }
        Err(err) => error!("Could not read manifest {} ({})", path.display(), err)
    } 
}