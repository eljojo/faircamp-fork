// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use enolib::{prelude::*, Document, Field, Item, Section};
use url::Url;

use crate::{
    Build,
    Cache,
    CacheOptimization,
    Catalog,
    CoverGenerator,
    DescribedImage,
    DownloadFormat,
    DownloadGranularity,
    DownloadOption,
    Favicon,
    HtmlAndStripped,
    Link,
    Locale,
    markdown,
    PaymentOption,
    Permalink,
    StreamingQuality,
    TagAgenda,
    Theme,
    ThemeBase,
    ThemeFont,
    TrackNumbering,
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

/// Options specified in a manifest that only apply to everything found in the
/// same folder as the manifest. For instance the permalink for a release can
/// only uniquely apply to one release, thus it is a local option only.
#[derive(Clone)]
pub struct LocalOptions {
    pub links: Vec<Link>,
    pub release_date: Option<NaiveDate>,
    pub release_permalink: Option<Permalink>,
    pub release_title: Option<String>,
    pub unlisted_release: bool
}

/// Options specified in a manifest that apply to everything in the same
/// folder, but which are also passed down and applied to child folders
/// (unless overriden there once again). For instance one might enable
/// downloads in a manifest in the root folder of the catalog, this would
/// apply to everything in the catalog then, however one can also disable it
/// in a manifest further down the hierarchy, hence it is an override.
#[derive(Clone)]
pub struct Overrides {
    pub copy_link: bool,
    pub download_formats: Vec<DownloadFormat>,
    pub download_granularity: DownloadGranularity,
    pub download_option: DownloadOption,
    pub embedding: bool,
    pub include_extras: bool,
    pub payment_options: Vec<PaymentOption>,
    pub release_artists: Option<Vec<String>>,
    pub release_cover: Option<DescribedImage>,
    pub release_text: Option<HtmlAndStripped>,
    pub release_track_numbering: TrackNumbering,
    pub streaming_quality: StreamingQuality,
    pub tag_agenda: TagAgenda,
    pub theme: Theme,
    pub track_artists: Option<Vec<String>>,
    pub unlock_text: Option<String>
}

impl LocalOptions {
    pub fn new() -> LocalOptions {
        LocalOptions {
            links: Vec::new(),
            release_date: None,
            release_permalink: None,
            release_title: None,
            unlisted_release: false
        }
    }
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            copy_link: true,
            download_formats: vec![DownloadFormat::DEFAULT],
            download_granularity: DownloadGranularity::EntireRelease,
            download_option: DownloadOption::Disabled,
            embedding: false,
            include_extras: true,
            payment_options: Vec::new(),
            release_artists: None,
            release_cover: None,
            release_text: None,
            release_track_numbering: TrackNumbering::Arabic,
            streaming_quality: StreamingQuality::Standard,
            tag_agenda: TagAgenda::normalize(),
            theme: Theme::new(),
            track_artists: None,
            unlock_text: None
        }
    }
}

/// Receives the path to a manifest (.eno) file, alongside references to
/// various mutable structures used by faircamp. The options found in the
/// manifest are applied to these various structures (e.g. the catalog title
/// is set on the passed catalog instance when it is encountered in the
/// manifest).
pub fn apply_options(
    path: &Path,
    build: &mut Build,
    cache: &mut Cache,
    catalog: &mut Catalog,
    local_options: &mut LocalOptions,
    overrides: &mut Overrides
) {
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

    fn optional_field<'a>(section: &'a Section, key: &str, path: &Path) -> Option<&'a Field> {
        match section.optional_field(key) {
            Ok(field_option) => field_option,
            Err(err) => {
                error!("{} {}", err.message, err_line!(path, err));
                None
            }
        }
    }

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

    let required_attribute_value_with_line = |field: &Field, key: &str| -> Option<(String, u32)> {
        match field.required_attribute(key) {
            Ok(attribute) => {
                match attribute.required_value() {
                    Ok(value) => return Some((value, attribute.line_number)),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
        }
        None
    };

    if let Some(section) = optional_section(&document, "artist", path) {
        match section.field("name").and_then(|field| field.required_value::<String>()) {
            Ok(name) => {
                let artist = catalog.create_artist(overrides.copy_link, &name, overrides.theme.clone());
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

                if let Some(field) = optional_field(section, "image", path) {
                    match required_attribute_value_with_line(field, "file") {
                        Some((path_relative_to_manifest, line)) => {
                            let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                            if absolute_path.exists() {
                                // TODO: Print errors, refactor
                                let description = match field.required_attribute("description") {
                                    Ok(attribute) => match attribute.required_value() {
                                        Ok(description) => Some(description),
                                        _ => None
                                    }
                                    _ => None
                                };

                                let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                                let image = cache.get_or_create_image(build, path_relative_to_catalog);

                                artist_mut.image = Some(DescribedImage::new(description, image));
                            } else {
                                error!("Ignoring invalid artist.image.file setting value '{}' in {}:{} (The referenced file was not found)", path_relative_to_manifest, path.display(), line)
                            }
                        }
                        None => ()
                    }
                }

                if let Some((slug, line)) = optional_field_value_with_line(section, "permalink") {
                    match Permalink::new(&slug) {
                        Ok(permalink) => artist_mut.permalink = permalink,
                        Err(err) => error!("Ignoring invalid artist.permalink value '{}' in {}:{} ({})", slug, path.display(), line, err)
                    }
                }
                
                if let Some(text_markdown) = optional_embed_value(section, "text") {
                    artist_mut.text = Some(markdown::to_html_and_stripped(&text_markdown));
                }
            }
            Err(err) => error!("An artist was specified without a name, and therefore discarded, in {}", err_line!(path, err))
        }
    }
    
    if let Some(section) = optional_section(&document, "cache", path) {
        if let Some((value, line)) = optional_field_value_with_line(section, "optimization") {
            match CacheOptimization::from_manifest_key(value.as_str()) {
                Some(strategy) => {
                    if cache.optimization != CacheOptimization::Default {
                        warn_global_set_repeatedly!("cache.optimization", cache.optimization, strategy);
                    }

                    cache.optimization = strategy;
                }
                None => error!("Ignoring invalid cache.optimization setting '{}' (available: delayed, immediate, manual, wipe) in {}:{}", value, path.display(), line)
            }
        }
    }
    
    if let Some(section) = optional_section(&document, "catalog", path) {
        if path.parent().unwrap() != build.catalog_dir {
            error!("From faircamp 0.16.0 onwards, \"# catalog ...\" may only be specified from a manifest placed in the catalog root directory, please move the catalog section specified in {}:{} to a manifest in the catalog root directory.", path.display(), section.line_number);
        } else {
            if let Some((mut value, line)) = optional_field_value_with_line(section, "base_url") {
                // Ensure the value has a trailing slash. Without one, Url::parse below
                // would interpret the final path segment as a file, which would lead to
                // incorrect url construction at a later point.
                if !value.ends_with('/') { value.push('/'); }

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

            if optional_flag_present(section, "disable_feed") {
                catalog.feed_enabled = false;
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "embedding") {
                match value.as_str() {
                    "disabled" => overrides.embedding = false,
                    "enabled" => overrides.embedding = true,
                    value => error!("Ignoring unsupported catalog.embedding setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
                }
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "favicon"){
                if let Favicon::Custom { absolute_path, .. } = &catalog.favicon {
                    warn_global_set_repeatedly!("catalog.favicon", absolute_path.display(), value);
                } else if let Favicon::None = &catalog.favicon {
                    warn_global_set_repeatedly!("catalog.favicon", "none", value);
                }

                if value == "none" {
                    catalog.favicon = Favicon::None;
                } else {
                    let absolute_path = path.parent().unwrap().join(&value);
                    if absolute_path.exists() {
                        match Favicon::custom(absolute_path) {
                            Ok(favicon) => catalog.favicon = favicon,
                            Err(message) => error!("Ignoring invalid catalog.favicon setting value '{}' in {}:{} ({})", value, path.display(), line, message) 
                        }
                    } else {
                        error!("Ignoring invalid catalog.favicon setting value '{}' in {}:{} (The referenced file was not found)", value, path.display(), line)
                    }
                }
            }

            if optional_flag_present(section, "feature_support_artists") {
                catalog.feature_support_artists = true;
            }

            if let Some(value) = optional_field_value(section, "freeze_download_urls") {
                build.url_salt = value;
            }

            // TODO: Remove this deprecation notice with/around the 1.0 release (introduced feb 2024)
            if let Some((path_relative_to_manifest, line)) = optional_field_value_with_line(section, "feed_image") {
                info!("From faircamp 0.13.0 onwards, feed images are auto-generated - catalog.feed_image '{}' specified in {}:{} can be removed, it won't be used anymore.", path_relative_to_manifest, path.display(), line);
            }

            if let Some(field) = optional_field(section, "home_image", path) {
                match required_attribute_value_with_line(field, "file") {
                    Some((path_relative_to_manifest, line)) => {
                        let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                        if absolute_path.exists() {
                            // TODO: Print errors, refactor
                            let description = match field.required_attribute("description") {
                                Ok(attribute) => match attribute.required_value() {
                                    Ok(description) => Some(description),
                                    _ => None
                                }
                                _ => None
                            };

                            let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                            let image = cache.get_or_create_image(build, path_relative_to_catalog);

                            catalog.home_image = Some(DescribedImage::new(description, image));
                        } else {
                            error!("Ignoring invalid catalog.home_image.file setting value '{}' in {}:{} (The referenced file was not found)", path_relative_to_manifest, path.display(), line)
                        }
                    }
                    None => ()
                }
            }

            if optional_flag_present(section, "label_mode") {
                catalog.label_mode = true;
            }

            // TODO: Would make sense to report if both rotate_download_urls and
            // freeze_download_urls are set (or the latter twice e.g.), as this
            // could lead to unexpected, frustrating behavior for users (and it
            // can happen by accident).
            if optional_flag_present(section, "rotate_download_urls") {
                build.url_salt = util::uid();
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "copy_link") {
                match value.as_str() {
                    "enabled" => {
                        catalog.copy_link = true;
                        overrides.copy_link = true;
                    }
                    "disabled" => {
                        catalog.copy_link = false;
                        overrides.copy_link = false;
                    }
                    value => error!("Ignoring unsupported catalog.copy_link setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
                }
            }

            if optional_flag_present(section, "show_support_artists") {
                catalog.show_support_artists = true;
            }

            if let Some(value) = optional_field_value(section, "title") {
                if let Some(previous) = catalog.set_title(value.clone()) {
                    warn_global_set_repeatedly!("catalog.title", previous, value);
                }
            }

            if let Some(text_markdown) = optional_embed_value(section, "text") {
                let new_text = markdown::to_html_and_stripped(&text_markdown);

                if let Some(previous_text) = &catalog.text {
                    warn_global_set_repeatedly!("catalog.text", previous_text.stripped, new_text.stripped);
                }

                catalog.text = Some(new_text);
            }
        }
    }

    if let Some(section) = optional_section(&document, "download", path) {
        if let Some(value) = optional_field_value(section, "code") {
            match Permalink::new(&value) {
                Ok(_) => {
                    overrides.download_option = DownloadOption::Codes {
                        codes: vec![value],
                        unlock_text: None
                    };
                }
                Err(err) => {
                    error!("Ignoring invalid download.code value '{}' ({}) in {}", value, err, path.display())
                }
            };
        }

        optional_field_with_items(section, "codes", &mut |items: &[Item]| {
            let codes: Vec<String> = items
                    .iter()
                    .filter_map(|item| {
                        if let Ok(value) = item.required_value::<String>() {
                            match Permalink::new(&value) {
                                Ok(_) => Some(value),
                                Err(err) => {
                                    error!("Ignoring invalid download.codes value '{}' ({}) in {}", value, err, path.display());
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

            if !codes.is_empty() {
                overrides.download_option = DownloadOption::Codes {
                    codes,
                    unlock_text: None
                };
            }
        });

        if optional_flag_present(section, "disabled") {
            overrides.download_option = DownloadOption::Disabled;
        }

        if let Some(value) = optional_field_value(section, "format") {
            // TODO: Implement via FromStr
            match DownloadFormat::from_manifest_key(value.as_str()) {
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
                        match DownloadFormat::from_manifest_key(&key) {
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
            overrides.download_option = DownloadOption::Free;
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "price") {
             match DownloadOption::new_from_price_string(&value) {
                Ok(download_option) => overrides.download_option = download_option,
                Err(err) => error!(
                    "Ignoring download.price option '{}' ({}) in {}:{}",
                    value,
                    err,
                    path.display(),
                    line
                )
            };
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "single_files") {
            match value.as_str() {
                "enabled" => overrides.download_granularity = DownloadGranularity::AllOptions,
                "disabled" => overrides.download_granularity = DownloadGranularity::EntireRelease,
                "only" => overrides.download_granularity = DownloadGranularity::SingleFiles,
                value => error!("Ignoring unsupported download.single_files setting value '{}' (supported values are 'enabled', 'disabled' and 'only') in {}:{}", value, path.display(), line)
            }
        }

        if let Some(text_markdown) = optional_embed_value(section, "unlock_text") {
            overrides.unlock_text = Some(markdown::to_html(&text_markdown));
        }
    }

    if let Some(section) = optional_section(&document, "embedding", path) {
        error!(r##"From faircamp 0.16.0 onwards, the embedding option must be specified as "embedding: enabled|disabled" either in a "# catalog ..." or "# release ..." section, please move and adapt the current definiton in {}:{} accordingly."##, path.display(), section.line_number);
    }

    for element in document.elements() {
        if element.key() == "link" {
            if let Some(field) = element.as_field() {
                match field.required_attribute("url") {
                    Ok(attribute) => {
                        match attribute.required_value::<String>() {
                            Ok(value) => match Url::parse(&value) {
                                Ok(url) => {
                                    // TODO: Errors, optional_attribute does not exist in enolib?
                                    let label = match field.required_attribute("label") {
                                        Ok(attribute) => match attribute.required_value() {
                                            Ok(label) => Some(label),
                                            Err(_) => None
                                        }
                                        Err(_) => None
                                    };

                                    let (hidden, rel_me) = match field.required_attribute("verification") {
                                        Ok(attribute) => match attribute.required_value::<String>() {
                                            Ok(value) => match value.as_str() {
                                                "rel-me" => (false, true),
                                                "rel-me-hidden" => (true, true),
                                                _ => (false, false)
                                            }
                                            Err(_) => (false, false)
                                        }
                                        Err(_) => (false, false)
                                    };

                                    let link = Link::new(hidden, label, rel_me, url);
                                    local_options.links.push(link);
                                }
                                Err(err) => error!("Error in {}:{} ({})", path.display(), attribute.line_number(), err)
                            }
                            Err(err) => error!("Error in {}:{} ({})", path.display(), err.line, err)
                        }
                    }
                    Err(err) => error!("Error in {}:{} ({})", path.display(), err.line, err)
                }
            } else {
                error!("Error in {}:{} (Links must be provided as fields with attributes)", path.display(), element.line_number())
            }
        }
    }

    if let Some(section) = optional_section(&document, "localization", path) {
        if let Some(value) = optional_field_value(section, "language") {
            build.locale = Locale::from_code(&value);
        }
        if let Some((value, line)) = optional_field_value_with_line(section, "writing_direction") {
            info!("From faircamp 0.14.0 onwards, writing direction is determined from the language automatically - localization.writing_direction '{}' specified in {}:{} can be removed, it won't be used anymore.", value, path.display(), line);
        }
    }
    
    if let Some(section) = optional_section(&document, "payment", path) {
        overrides.payment_options = section.elements()
            .iter()
            .filter_map(|element|
                match element.key() {
                    "custom" => if let Some(embed) = element.as_embed() {
                        embed
                            .optional_value::<String>()
                            .and_then(|result| result.ok().map(|value| PaymentOption::init_custom(&value)))
                    } else if let Some(field) = element.as_field() {
                        field
                            .optional_value()
                            .ok()
                            .and_then(|result| result.map(|value| PaymentOption::init_custom(&value)))
                    } else {
                        error!("Ignoring invalid payment.custom option (can only be an embed or field containing a value) in {}:{}", path.display(), element.line_number());
                        None
                    }
                    "liberapay" => if let Some(field) = element.as_field() {
                        field
                            .optional_value()
                            .ok()
                            .and_then(|result| result.map(|value| PaymentOption::init_liberapay(&value)))
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

        if let Some(field) = optional_field(section, "cover", path) {
            match required_attribute_value_with_line(field, "file") {
                Some((path_relative_to_manifest, line)) => {
                    let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                    if absolute_path.exists() {
                        // TODO: Print errors, refactor
                        let description = match field.required_attribute("description") {
                            Ok(attribute) => match attribute.required_value() {
                                Ok(description) => Some(description),
                                _ => None
                            }
                            _ => None
                        };

                        let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                        let image = cache.get_or_create_image(build, path_relative_to_catalog);

                        overrides.release_cover = Some(DescribedImage::new(description, image));
                    } else {
                        error!("Ignoring invalid release.cover.file setting value '{}' in {}:{} (The referenced file was not found)", path_relative_to_manifest, path.display(), line)
                    }
                }
                None => ()
            }
        }    

        if let Some((date_str, line)) = optional_field_value_with_line(section, "date") {
            match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(date) => {
                    if let Some(previous) = &local_options.release_date {
                        warn!("Option release.date is set more than once - overriding previous value '{}' with '{}'", previous, date);
                    }
                    local_options.release_date = Some(date);
                },
                Err(err) => error!("Ignoring invalid release.date value '{}' in {}:{} ({})", date_str, path.display(), line, err)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "embedding") {
            match value.as_str() {
                "disabled" => overrides.embedding = false,
                "enabled" => overrides.embedding = true,
                value => error!("Ignoring unsupported release.embedding setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "include_extras") {
            match value.as_str() {
                "yes" => overrides.include_extras = true,
                "no" => overrides.include_extras = false,
                other => error!("Ignoring invalid release.include_extras value '{}' (allowed are either 'yes or 'no') in {}:{}", other, path.display(), line)
            }
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

        if let Some((value, line)) = optional_field_value_with_line(section, "rewrite_tags") {
            match value.as_str() {
                "no" => {
                    info!("From faircamp 0.15.0 onwards, 'rewrite_tags: no' should be specified as 'tags: copy' - this will eventually become mandatory (seen) in {}:{}).", path.display(), line);
                    overrides.tag_agenda = TagAgenda::Copy;
                }
                "yes" => {
                    info!("From faircamp 0.15.0 onwards, 'rewrite_tags: yes' should be specified as 'tags: normalize' - this will eventually become mandatory (seen) in {}:{}).", path.display(), line);
                    overrides.tag_agenda = TagAgenda::normalize();
                }
                other => error!("Ignoring invalid release.rewrite_tags value '{}' (allowed are either 'yes or 'no') in {}:{}", other, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "copy_link") {
            match value.as_str() {
                "enabled" => overrides.copy_link = true,
                "disabled" => overrides.copy_link = false,
                value => error!("Ignoring unsupported release.copy_link setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
            }
        }

        match section.optional_field("tags") {
            Ok(Some(field)) => {
                if let Ok(attributes) = field.attributes() {
                    overrides.tag_agenda = TagAgenda::Remove;

                    for attribute in attributes {
                        match attribute.required_value::<String>() {
                            Ok(value) => {
                                if let Err(err) = overrides.tag_agenda.set(attribute.key(), &value) {
                                    error!("Error in {}:{} ({})", path.display(), attribute.line_number(), err)
                                }
                            }
                            Err(err) => error!("Error in {}:{} ({})", path.display(), err.line, err)
                        }
                    }
                } else if let Ok(value) = field.required_value::<String>() {
                    match value.as_str() {
                        "copy" => overrides.tag_agenda = TagAgenda::Copy,
                        "normalize" => overrides.tag_agenda = TagAgenda::normalize(),
                        "remove" => overrides.tag_agenda = TagAgenda::Remove,
                        other => error!("Ignoring invalid release.tags value '{}' (allowed are either 'copy', 'remove' or 'rewrite') in {}:{}", other, path.display(), field.line_number)
                    }
                } else {
                    error!("Ignoring invalid release.tags setting (allowed are either 'copy', 'remove', 'rewrite' as value, or a customization with attributes such as 'title = copy', 'artist = rewrite', 'album_artist = remove' etc.) in {}:{}", path.display(), field.line_number)
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }

        if let Some(text_markdown) = optional_embed_value(section, "text") {
            overrides.release_text = Some(markdown::to_html_and_stripped(&text_markdown));
        }

        if let Some(value) = optional_field_value(section, "title") {
            local_options.release_title = Some(value);
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "track_numbering") {
            match TrackNumbering::from_manifest_key(value.as_str()) {
                Some(variant) => overrides.release_track_numbering = variant,
                None => error!("Ignoring unsupported value '{}' for global 'release.track_numbering' (supported values are 'disabled', 'arabic', 'roman' and 'hexadecimal') in {}:{}", value, path.display(), line)
            }
        }

        if optional_flag_present(section, "unlisted") {
            local_options.unlisted_release = true;
        }
    }

    if let Some(section) = optional_section(&document, "streaming", path) {
        if let Some((value, line)) = optional_field_value_with_line(section, "quality") {
            match value.as_str() {
                "standard" => overrides.streaming_quality = StreamingQuality::Standard,
                "frugal" => overrides.streaming_quality = StreamingQuality::Frugal,
                value => error!("Ignoring invalid streaming.quality setting value '{}' (available: standard, frugal) in {}:{}", value, path.display(), line)
            }
        }
    }
    
    if let Some(section) = optional_section(&document, "theme", path) {
        if let Some((value, line)) = optional_field_value_with_line(section, "background_alpha") {
            match value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                Some(percentage) => overrides.theme.background_alpha = percentage,
                None => error!("Ignoring unsupported value '{}' for global 'theme.background_alpha' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((path_relative_to_manifest, line)) = optional_field_value_with_line(section, "background_image") {
            let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
            if absolute_path.exists() {
                let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                let image = cache.get_or_create_image(build, path_relative_to_catalog);
                overrides.theme.background_image = Some(image);
            } else {
                error!("Ignoring invalid theme.background_image setting value '{}' in {}:{} (The referenced file was not found)", path_relative_to_manifest, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "base") {
            match ThemeBase::from_manifest_key(value.as_str()) {
                Some(variant) => overrides.theme.base = variant,
                None => {
                    let supported = ThemeBase::ALL_PRESETS.map(|key| format!("'{key}'")).join(", ");
                    error!("Ignoring unsupported value '{}' for global 'theme.base' (supported values are {}) in {}:{}", value, supported, path.display(), line);
                }
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "cover_generator") {
            match CoverGenerator::from_manifest_key(value.as_str()) {
                Some(cover_generator) => overrides.theme.cover_generator = cover_generator,
                None => {
                    let supported = CoverGenerator::ALL_GENERATORS.map(|key| format!("'{key}'")).join(", ");
                    error!("Ignoring unsupported value '{}' for global 'theme.cover_generator' (supported values are {}) in {}:{}", value, supported, path.display(), line);
                }
            }
        }

        if let Some((relative_path, line)) = optional_field_value_with_line(section, "custom_font") {
            let absolute_path = path.parent().unwrap().join(&relative_path);
            if absolute_path.exists() {
                match ThemeFont::custom(absolute_path) {
                    Ok(theme_font) => overrides.theme.font = theme_font,
                    Err(message) => error!("Ignoring invalid theme.font setting value '{}' in {}:{} ({})", relative_path, path.display(), line, message) 
                }
            } else {
                error!("Ignoring invalid theme.font setting value '{}' in {}:{} (The referenced file was not found)", relative_path, path.display(), line)
            }
        }

        if optional_flag_present(section, "disable_relative_waveforms") {
            overrides.theme.relative_waveforms = false;
        }

        if optional_flag_present(section, "disable_waveforms") {
            overrides.theme.waveforms = false;
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "link_hue") {
            match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                Some(degrees) => overrides.theme.link_h = degrees,
                None => error!("Ignoring unsupported value '{}' for global 'theme.link_hue' (accepts an amount of degrees in the range 0-360) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "link_lightness") {
            match value.parse::<u8>().ok().filter(|degrees| *degrees <= 100) {
                Some(degrees) => overrides.theme.link_l = Some(degrees),
                None => error!("Ignoring unsupported value '{}' for global 'theme.link_lightness' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "link_saturation") {
            match value.parse::<u8>().ok().filter(|degrees| *degrees <= 100) {
                Some(degrees) => overrides.theme.link_s = Some(degrees),
                None => error!("Ignoring unsupported value '{}' for global 'theme.link_saturation' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }

        if optional_flag_present(section, "round_corners") {
            overrides.theme.round_corners = true;
        }

        if let Some(value) = optional_field_value(section, "system_font") {
            overrides.theme.font = if value == "sans" {
                ThemeFont::SystemSans
            } else if value == "mono" {
                ThemeFont::SystemMono
            } else {
                ThemeFont::System(value)
            };
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "text_hue") {
            match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                Some(degrees) => overrides.theme.text_h = degrees,
                None => error!("Ignoring unsupported value '{}' for global 'theme.text_hue' (accepts an amount of degrees in the range 0-360) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "tint_back") {
            match value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                Some(percentage) => overrides.theme.tint_back = percentage,
                None => error!("Ignoring unsupported value '{}' for global 'theme.tint_back' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "tint_front") {
            match value.parse::<u8>().ok().filter(|percent| *percent <= 100) {
                Some(percentage) => overrides.theme.tint_front = percentage,
                None => error!("Ignoring unsupported value '{}' for global 'theme.tint_front' (accepts a percentage in the range 0-100) in {}:{}", value, path.display(), line)
            }
        }
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

    let untouched_elements = document.untouched_elements();

    for element in &untouched_elements {
        if let Some(attribute) = element.as_attribute() {
            error!("Ignoring unsupported attribute '{}' in {}:{}", attribute.key(), path.display(), element.line_number())
        } else if let Some(embed) = element.as_embed() {
            error!("Ignoring unsupported embed '{}' in {}:{}", embed.key(), path.display(), element.line_number())
        } else if let Some(flag) = element.as_flag() {
            error!("Ignoring unsupported flag '{}' in {}:{}", flag.key(), path.display(), element.line_number())
        } else if let Some(field) = element.as_field() {
            error!("Ignoring unsupported field '{}' in {}:{}", field.key(), path.display(), element.line_number())
        } else if let Some(section) = element.as_section() {
            error!("Ignoring unsupported section '{}' in {}:{}", section.key(), path.display(), element.line_number())
        }
    }
}
