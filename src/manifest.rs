use enolib::{Document, Item, Section};
use iso_currency::Currency;
use std::{
    fs,
    path::Path,
};
use url::Url;

use crate::{
    asset_cache::CacheOptimization,
    audio_format::AudioFormat,
    build::Build,
    catalog::Catalog,
    download_option::DownloadOption,
    localization::WritingDirection,
    payment_option::PaymentOption,
    permalink::Permalink,
    release::TrackNumbering,
    styles::{Theme, ThemeBase, ThemeFont},
    util
};

macro_rules! err_line {
    ($path:expr, $error:expr) => {
        format!(
            "{}:{}",
            $path.display(),
            $error.line
        )
    };
}

#[derive(Clone)]
pub struct LocalOptions {
    pub release_permalink: Option<Permalink>
}

#[derive(Clone)]
pub struct Overrides {
    pub download_option: DownloadOption,
    pub download_formats: Vec<AudioFormat>,
    pub embedding: bool,
    pub payment_options: Vec<PaymentOption>,
    pub release_artists: Option<Vec<String>>,
    pub release_image_description: Option<String>,
    pub release_text: Option<String>,
    pub release_title: Option<String>,
    pub release_track_numbering: TrackNumbering,
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
            embedding: true,
            payment_options: Vec::with_capacity(5),   // assuming e.g. Liberapay + Patreon + PayPal + SEPA + Custom option as a reasonable complex assumption
            release_artists: None,
            release_image_description: None,
            release_text: None,
            release_title: None,
            release_track_numbering: TrackNumbering::Arabic,
            streaming_format: AudioFormat::Mp3Cbr128,
            track_artists: None
        }
    }
}

pub fn apply_options(path: &Path, build: &mut Build, catalog: &mut Catalog, local_options: &mut LocalOptions, overrides: &mut Overrides) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            error!("Could not read manifest {} ({})", path.display(), err);
            return
        }
    };

    let document = match enolib::parse(&content) {
        Ok(document) => document,
        Err(err) => {
            error!("Syntax error in {}:{} ({})", path.display(), err.line, err);
            return
        }
    };

    let optional_field_value = |section: &Section, key: &str| -> Option<String> {
        match section.optional_field(key) {
            Ok(Some(field)) => {
                match field.required_value() {
                    Ok(value) => return Some(value),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
        None
    };

    let optional_field_value_in_document = |document: &Document, key: &str| -> Option<String> {
        match document.optional_field(key) {
            Ok(Some(field)) => {
                match field.required_value() {
                    Ok(value) => return Some(value),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
        None
    };

    let optional_field_value_with_line = |section: &Section, key: &str| -> Option<(String, u32)> {
        match section.optional_field(key) {
            Ok(Some(field)) => {
                match field.required_value() {
                    Ok(value) => return Some((value, field.line_number)),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
        None
    };

    let optional_embed_value = |section: &Section, key: &str| -> Option<String> {
        match section.optional_embed(key) {
            Ok(Some(embed)) => {
                match embed.required_value() {
                    Ok(value) => return Some(value),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
        None
    };

    let optional_flag_present = |section: &Section, key: &str| -> bool {
        match section.optional_flag(key) {
            Ok(Some(_)) => return true,
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
        false
    };

    let optional_field_with_items = |section: &Section, key: &str, callback: &mut dyn FnMut(&[Item])| {
        match section.optional_field(key) {
            Ok(Some(field)) => {
                match field.items() {
                    Ok(items) => callback(items),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
    };

    let optional_field_with_items_in_document = |document: &Document, key: &str, callback: &mut dyn FnMut(&[Item])| {
        match document.optional_field(key) {
            Ok(Some(field)) => {
                match field.items() {
                    Ok(items) => callback(items),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
    };

    fn optional_section<'a>(document: &'a Document, key: &str, path: &Path) -> Option<&'a Section> {
        match document.optional_section(key) {
            Ok(section_option) => section_option,
            Err(err) => {
                error!("{} {}", err.message, err_line!(path, err));
                None
            }
        }
    }

    if let Some(section) = optional_section(&document, "artist", path) {
        match section.field("name").and_then(|field| field.required_value::<String>()) {
            Ok(name) => {
                let artist = catalog.create_artist(&name);
                let mut artist_mut = artist.borrow_mut();

                optional_field_with_items(section, "aliases", &mut |items: &[Item]| { 
                    artist_mut.aliases = items
                            .iter()
                            .filter_map(|item| {
                                match item.optional_value() {
                                    Ok(value_option) => value_option,
                                    Err(err) => {
                                        error!("{} {}", err.message, err_line!(path, err));
                                        None
                                    }
                                }
                            })
                            .collect();

                });

                if let Some((slug, line)) = optional_field_value_with_line(section, "permalink") {
                    match Permalink::new(&slug) {
                        Ok(permalink) => artist_mut.permalink = permalink,
                        Err(err) => error!("Ignoring invalid artist.permalink value '{}' in {}:{} ({})", slug, path.display(), line, err)
                    }
                }
                
                if let Some(text) = optional_embed_value(section, "text") {
                    artist_mut.text = Some(text); // TODO: Markdown (?)
                }
            }
            Err(err) => error!("An artist was specified without a name, and therefore discarded, in {}", err_line!(path, err))
        }
    }
    
    if let Some(section) = optional_section(&document, "cache", path) {
        if let Some((value, line)) = optional_field_value_with_line(section, "optimization") {
            match CacheOptimization::from_manifest_key(value.as_str()) {
                Some(strategy) => {
                    if build.cache_optimization != CacheOptimization::Default {
                        warn_global_set_repeatedly!("cache.optimization", build.cache_optimization, strategy);
                    }

                    build.cache_optimization = strategy;
                }
                None => error!("Ignoring invalid cache.optimization setting '{}' (available: delayed, immediate, manual, wipe) in {}:{}", value, path.display(), line)
            }
        }
    }
    
    if let Some(section) = optional_section(&document, "catalog", path) {
        if let Some((value, line)) = optional_field_value_with_line(section, "base_url") {
            match Url::parse(&value) {
                Ok(url) => {
                    if let Some(previous_url) = &build.base_url {
                        warn_global_set_repeatedly!("catalog.base_url", previous_url, url);
                    }

                    build.base_url = Some(url);
                }
                Err(err) => error!("Ignoring invalid catalog.base_url setting value '{}' in {}:{} ({})", value, path.display(), line, err)
            }
        }


        if let Some((relative_path, line)) = optional_field_value_with_line(section, "feed_image"){
            if let Some(previous) = &catalog.feed_image {
                warn_global_set_repeatedly!("catalog.feed_image", previous.display(), relative_path);
            }

            let absolute_path = path.parent().unwrap().join(&relative_path);
            if absolute_path.exists() {
                catalog.feed_image = Some(absolute_path);
            } else {
                error!("Ignoring invalid catalog.feed_image setting value '{}' in {}:{} (The referenced file was not found)", relative_path, path.display(), line)
            }
        }

        if let Some(value) = optional_field_value(section, "title") {
            if let Some(previous) = catalog.set_title(value.clone()) {
                warn_global_set_repeatedly!("catalog.title", previous, value);
            }
        }

        if let Some(value) = optional_embed_value(section, "text") {
            if let Some(previous) = &catalog.text {
                warn_global_set_repeatedly!("catalog.text", previous, value);
            }

            catalog.text = Some(value);
        }
    }

    if let Some(section) = optional_section(&document, "download", path) {
        if optional_flag_present(section, "disabled") {
            overrides.download_option = DownloadOption::Disabled;
        }
        if let Some(value) = optional_field_value(section, "format") {
            // TODO: Implement via FromStr
            match AudioFormat::from_manifest_key(value.as_str()) {
                Some(format) => overrides.download_formats = vec![format],
                // TODO: Missing line number (no element access)
                None => error!("Ignoring invalid download.format setting value '{}' in {}", value, path.display())
            }
        }
        optional_field_with_items(section, "formats", &mut |items: &[Item]| { 
            overrides.download_formats = items
                    .iter()
                    .filter_map(|item| {
                        let key = item.required_value().unwrap_or(String::new());
                        // TODO: Implement via FromStr
                        match AudioFormat::from_manifest_key(&key) {
                            None => {
                                error!("Ignoring invalid download.formats format specifier '{}' in {}:{}", key, path.display(), item.line_number);
                                None
                            }
                            some_format => some_format
                        }
                    })
                    .collect();
        });
        if optional_flag_present(section, "free") {
            overrides.download_option = DownloadOption::init_free();
        }
        if let Some((value, line)) = optional_field_value_with_line(section, "price") {
            let mut split_by_whitespace = value.split_ascii_whitespace();

            if let Some(first_token) = split_by_whitespace.next() {
                if let Some(currency) = Currency::from_code(first_token) {
                    let recombined = &value[4..]; // TODO: Why 4?

                    if recombined.ends_with("+") {
                        if let Ok(amount_parsed) = recombined[..(recombined.len() - 1)].parse::<f32>() {
                            overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..f32::INFINITY);
                        } else {
                            error!("Ignoring download.price option '{}' with malformed minimum price in {}:{}", value, path.display(), line);
                        }
                    } else {
                        let mut split_by_dash = recombined.split("-");

                        if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                            if let Some(max_amount) = split_by_dash.next() {
                                if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                                    overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..max_amount_parsed);
                                } else {
                                    error!("Ignoring download.price option '{}' with malformed maximum price in {}:{}", value, path.display(), line);
                                }
                            } else {
                                overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..amount_parsed);
                            }
                        } else {
                            error!("Ignoring download.price option '{}' with malformed price in {}:{}", value, path.display(), line);
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
                                error!("Ignoring download.price option '{}' with malformed minimum price in {}:{}", value, path.display(), line);
                            }
                        } else {
                            let mut split_by_dash = recombined.split("-");

                            if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                                if let Some(max_amount) = split_by_dash.next() {
                                    if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                                        overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..max_amount_parsed);
                                    } else {
                                        error!("Ignoring download.price option '{}' with malformed maximum price in {}:{}", value, path.display(), line);
                                    }
                                } else {
                                    overrides.download_option = DownloadOption::init_paid(currency, amount_parsed..amount_parsed);
                                }
                            } else {
                                error!("Ignoring download.price option '{}' with malformed price in {}:{}", value, path.display(), line);
                            }
                        }
                    } else {
                        error!("Ignoring download.price option '{}' without recognizable currency code in {}:{}", value, path.display(), line);
                    }
                } else {
                    error!("Ignoring unrecognized download.price option '{}' in {}:{}", value, path.display(), line);
                }
            } else {
                error!("Ignoring unrecognized download.price option '{}' in {}:{}", value, path.display(), line);
            }
        }
    }
    
    if let Some(section) = optional_section(&document, "embedding", path) {
        if optional_flag_present(section, "disabled") {
            overrides.embedding = false;
        }
        if optional_flag_present(section, "enabled") {
            overrides.embedding = true;
        }
    }

    if let Some(section) = optional_section(&document, "localization", path) {
        if let Some(value) = optional_field_value(section, "language") {
            build.localization.language = value;
        }
        if let Some((value, line)) = optional_field_value_with_line(section, "writing_direction") {
            match value.as_str() {
                "ltr" => build.localization.writing_direction = WritingDirection::Ltr,
                "rtl" => build.localization.writing_direction = WritingDirection::Rtl,
                value => error!("Ignoring unsupported value '{}' for global 'localization.writing_direction' (supported values are 'ltr' and 'rtl') in {}:{}", value, path.display(), line)
            }
        }
    }
    
    if let Some(section) = optional_section(&document, "payment", path) {
        overrides.payment_options = section.elements()
            .iter()
            .filter_map(|element|
                match element.key() {
                    "custom" => if let Ok(embed) = element.as_embed() {
                        embed.optional_value::<String>().map(|result| result.ok().map(|value| PaymentOption::init_custom(&value))).flatten()
                    } else if let Ok(field) = element.as_field() {
                        field.optional_value().ok().map(|result| result.map(|value| PaymentOption::init_custom(&value))).flatten()
                    } else {
                        error!("Ignoring invalid payment.custom option (can only be an embed or field containing a value) in {}:{}", path.display(), element.line_number());
                        None
                    }
                    "liberapay" => if let Ok(field) = element.as_field() {
                        field.optional_value().ok().map(|result| result.map(|value| PaymentOption::init_liberapay(&value))).flatten()
                    } else {
                        error!("Ignoring invalid payment.liberapay option (can only be a field containing a value) in {}:{}", path.display(), element.line_number());
                        None
                    }
                    key => {
                        error!("Ignoring unsupported payment.options setting '{}' in {}:{}", key, path.display(), element.line_number());
                        None
                    }
                }
            )
            .collect();
    }

    if let Some(section) = optional_section(&document, "release", path) {
        if let Some(value) = optional_field_value(section, "artist") {
            overrides.release_artists = Some(vec![value]);
        }

        optional_field_with_items(section, "artists", &mut |items: &[Item]| { 
            overrides.release_artists = Some(
                items
                    .iter()
                    .filter_map(|item| item.optional_value().ok().flatten())
                    .collect()
            );
        });

        if let Some(value) = optional_field_value(section, "image_description") {
            overrides.release_image_description = Some(value);
        }

        if let Some((slug, line)) = optional_field_value_with_line(section, "permalink") {
            match Permalink::new(&slug) {
                Ok(permalink) => {
                    if let Some(previous) = &local_options.release_permalink {
                        warn!("Option release.permalink is set more than once - overriding previous value '{}' with '{}'", previous.slug, slug);
                    }
                    local_options.release_permalink = Some(permalink);
                },
                Err(err) => error!("Ignoring invalid release.permalink value '{}' in {}:{} ({})", slug, path.display(), line, err)
            }
        }

        if let Some(value) = optional_field_value(section, "text") {
            overrides.release_text = Some(util::markdown_to_html(&value));
        }

        if let Some(value) = optional_field_value(section, "title") {
            overrides.release_title = Some(value);
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "track_numbering") {
            match TrackNumbering::from_manifest_key(value.as_str()) {
                Some(variant) => overrides.release_track_numbering = variant,
                None => error!("Ignoring unsupported value '{}' for global 'release.track_numbering' (supported values are 'disabled', 'arabic', 'roman' and 'hexadecimal') in {}:{}", value, path.display(), line)
            }
        }
    }

    if let Some(section) = optional_section(&document, "streaming", path) {
        if let Some((value, line)) = optional_field_value_with_line(section, "quality") {
            match value.as_str() {
                "standard" => overrides.streaming_format = AudioFormat::Mp3Cbr128,
                "transparent" => overrides.streaming_format = AudioFormat::Mp3VbrV0,
                value => error!("Ignoring invalid streaming.quality setting value '{}' (available: standard, transparent) in {}:{}", value, path.display(), line)
            }
        }
    }
    
    if let Some(section) = optional_section(&document, "theme", path) {
        if build.theme.is_some() {
            warn_global_set_repeatedly!("theme");
        }

        let mut theme = Theme::defaults();

        if let Some((relative_path, line)) = optional_field_value_with_line(section, "background_image") {
            let absolute_path = path.parent().unwrap().join(&relative_path);
            if absolute_path.exists() {
                theme.background_image = Some(absolute_path);
            } else {
                error!("Ignoring invalid theme.background_image setting value '{}' in {}:{} (The referenced file was not found)", relative_path, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "base") {
            match ThemeBase::from_manifest_key(value.as_str()) {
                Some(variant) => theme.base = variant,
                None => error!("Ignoring unsupported value '{}' for global 'theme.base' (supported values are 'dark' and 'light') in {}:{}", value, path.display(), line)
            }
        }

        if let Some((relative_path, line)) = optional_field_value_with_line(section, "custom_font") {
            let absolute_path = path.parent().unwrap().join(&relative_path);
            if absolute_path.exists() {
                match ThemeFont::custom(absolute_path) {
                    Ok(theme_font) => theme.font = theme_font,
                    Err(message) => error!("Ignoring invalid theme.font setting value '{}' in {}:{} ({})", relative_path, path.display(), line, message) 
                }
            } else {
                error!("Ignoring invalid theme.font setting value '{}' in {}:{} (The referenced file was not found)", relative_path, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "hue") {
            match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                Some(degrees) => theme.hue = degrees,
                None => error!("Ignoring unsupported value '{}' for global 'theme.hue' (accepts an amount of degrees in the range 0-360) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "hue_spread") {
            match value.parse::<i16>().ok() {
                Some(degree_offset) => theme.hue_spread = degree_offset,
                None => error!("Ignoring unsupported value '{}' for global 'theme.hue_spread' (accepts an amount of degrees as a signed integer) in {}:{}", value, path.display(), line)
            }
        }

        if let Some(value) = optional_field_value(section, "system_font") {
            theme.font = if value == "sans" {
                ThemeFont::SystemSans
            } else if value == "mono" {
                ThemeFont::SystemMono
            } else {
                ThemeFont::System(value.clone())
            };
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "tint_back") {
            match value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                Some(percentage) => theme.tint_front = percentage,
                None => error!("Ignoring unsupported value '{}' for global 'theme.tint_back' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "tint_front") {
            match value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                Some(percentage) => theme.tint_front = percentage,
                None => error!("Ignoring unsupported value '{}' for global 'theme.tint_front' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }

        build.theme = Some(theme);
    }

    // TODO: We probably should have these props on a section too - not in the root scope (where it's likely to cause problems/confusion for users)
    if let Some(value) = optional_field_value_in_document(&document, "track_artist") {
        overrides.track_artists = Some(vec![value])
    }

    optional_field_with_items_in_document(&document, "track_artists", &mut |items: &[Item]| { 
        overrides.track_artists = Some(
            items
                .iter()
                .filter_map(|item| item.optional_value().ok().flatten())
                .collect()
        );
    });
}
