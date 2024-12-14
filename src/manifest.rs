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
    DownloadsConfig,
    DownloadOption,
    ExtraDownloads,
    Favicon,
    HtmlAndStripped,
    Link,
    Locale,
    markdown,
    Permalink,
    Price,
    StreamingQuality,
    TagAgenda,
    Theme,
    ThemeBase,
    ThemeFont,
    TrackNumbering
};
use crate::util::{html_escape_outside_attribute, uid};

const MAX_SYNOPSIS_CHARS: usize = 256;

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
    pub download_codes: Vec<String>,
    pub downloads: DownloadOption,
    pub downloads_config: DownloadsConfig,
    pub embedding: bool,
    pub m3u_enabled: bool,
    pub more_label: Option<String>,
    pub payment_info: Option<String>,
    pub price: Price,
    pub release_artists: Option<Vec<String>>,
    pub release_cover: Option<DescribedImage>,
    pub release_synopsis: Option<String>,
    pub release_text: Option<HtmlAndStripped>,
    pub streaming_quality: StreamingQuality,
    pub tag_agenda: TagAgenda,
    pub theme: Theme,
    pub track_artists: Option<Vec<String>>,
    pub track_numbering: TrackNumbering,
    pub unlock_info: Option<String>
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
            download_codes: Vec::new(),
            downloads: DownloadOption::Free,
            downloads_config: DownloadsConfig::default(),
            embedding: false,
            m3u_enabled: true,
            more_label: None,
            payment_info: None,
            price: Price::default(),
            release_artists: None,
            release_cover: None,
            release_synopsis: None,
            release_text: None,
            streaming_quality: StreamingQuality::Standard,
            tag_agenda: TagAgenda::normalize(),
            theme: Theme::new(),
            track_artists: None,
            track_numbering: TrackNumbering::ArabicDotted,
            unlock_info: None
        }
    }
}

fn apply_downloads(
    build: &Build,
    section: &Section,
    overrides: &mut Overrides,
    path: &Path
) {
    if let Ok(Some(field)) = section.optional_field("archive_downloads") {
        field.touch();
        if let Ok(key) = field.value() {
            // TODO: Implement via FromStr
            match DownloadFormat::from_manifest_key(&key) {
                Some(format) => overrides.downloads_config.archive_formats = vec![format],
                None => error!("Ignoring invalid archive_downloads format specifier '{}' in {}:{}", key, path.display(), field.line_number)
            }
        } else if let Ok(items) = field.items() {
            overrides.downloads_config.archive_formats = items
                .iter()
                .filter_map(|item| {
                    item.touch();
                    match item.value() {
                        Ok(key) => {
                            match DownloadFormat::from_manifest_key(&key) {
                                Some(format) => Some(format),
                                None => {
                                    error!("Ignoring invalid archive_downloads format specifier '{}' in {}:{}", key, path.display(), item.line_number);
                                    None
                                }
                            }
                        }
                        Err(()) => None
                    }
                })
                .collect();
        }
    }

    if let Some(value) = optional_field_value(section, "download_code", path) {
        match Permalink::new(&value) {
            Ok(_) => {
                overrides.download_codes = vec![value];
            }
            Err(err) => {
                error!("Ignoring invalid download_code value '{}' ({}) in {}", value, err, path.display())
            }
        };
    }

    optional_field_with_items(section, "download_codes", path, &mut |items: &[Item]| {
        overrides.download_codes = items
                .iter()
                .filter_map(|item| {
                    item.touch();
                    if let Ok(value) = item.required_value::<String>() {
                        match Permalink::new(&value) {
                            Ok(_) => Some(value),
                            Err(err) => {
                                error!("Ignoring invalid download_codes value '{}' ({}) in {}", value, err, path.display());
                                None
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect();
    });

    if let Ok(Some(field)) = section.optional_field("downloads") {
        field.touch();
        if let Ok(value) = field.value() {
            match value {
                "code" => overrides.downloads = DownloadOption::Code,
                "disabled" => overrides.downloads = DownloadOption::Disabled,
                "free" => overrides.downloads = DownloadOption::Free,
                "paycurtain" => overrides.downloads = DownloadOption::Paycurtain,
                other if other.starts_with("http://") || other.starts_with("https://") => {
                    match Url::parse(value) {
                        Ok(_) => {
                            overrides.downloads = DownloadOption::External { link: value.to_string() };
                        }
                        Err(err) => error!("Ignoring invalid external downloads url '{}' in {}:{} ({})", value, path.display(), field.line_number, err)
                    }
                }
                _ => error!("Ignoring invalid downloads setting value '{}' in {}:{} (allowed options are 'code', 'curtain', 'disabled', 'free', or an external link like 'https://example.com')", value, path.display(), field.line_number)
            }
        } else {
            error!("Ignoring invalid downloads field (only a single value is supported and required) in {}:{}", path.display(), field.line_number);
        }
    }

    if let Ok(Some(field)) = section.optional_field("extra_downloads") {
        field.touch();
        if let Ok(value) = field.value() {
            match value {
                "bundled" => overrides.downloads_config.extra_downloads = ExtraDownloads::BUNDLED,
                "disabled" => overrides.downloads_config.extra_downloads = ExtraDownloads::DISABLED,
                "separate" => overrides.downloads_config.extra_downloads = ExtraDownloads::SEPARATE,
                other => error!("Ignoring invalid extra_downloads value '{}' (allowed are either 'bundled', 'disabled' or 'separate') in {}:{}", other, path.display(), field.line_number)
            }
        } else if let Ok(items) = field.items() {
            overrides.downloads_config.extra_downloads = ExtraDownloads::DISABLED;

            for item in items {
                item.touch();
                match item.value() {
                    Ok("bundled") => overrides.downloads_config.extra_downloads.bundled = true,
                    Ok("disabled") => overrides.downloads_config.extra_downloads = ExtraDownloads::DISABLED,
                    Ok("separate") => overrides.downloads_config.extra_downloads.separate = true,
                    Ok(other) => error!("Ignoring invalid extra_downloads value '{}' (allowed are either 'bundled', 'disabled' or 'separate') in {}:{}", other, path.display(), field.line_number),
                    Err(()) => ()
                }
            }
        } else {
            error!("Ignoring invalid extra_downloads field (supports only a single value or items) in {}:{}", path.display(), field.line_number);
        }
    }

    if let Some(text_markdown) = optional_embed_value(section, "payment_info", path) {
        overrides.payment_info = Some(markdown::to_html(&build.base_url, &text_markdown));
    }

    if let Some((value, line)) = optional_field_value_with_line(section, "price", path) {
         match Price::new_from_price_string(&value) {
            Ok(price) => overrides.price = price,
            Err(err) => error!(
                "Ignoring price option '{}' ({}) in {}:{}",
                value,
                err,
                path.display(),
                line
            )
        };
    }

    if let Ok(Some(field)) = section.optional_field("track_downloads") {
        field.touch();
        if let Ok(key) = field.value() {
            // TODO: Implement via FromStr
            match DownloadFormat::from_manifest_key(&key) {
                Some(format) => overrides.downloads_config.track_formats = vec![format],
                None => error!("Ignoring invalid track_downloads format specifier '{}' in {}:{}", key, path.display(), field.line_number)
            }
        } else if let Ok(items) = field.items() {
            overrides.downloads_config.track_formats = items
                .iter()
                .filter_map(|item| {
                    item.touch();
                    match item.value() {
                        Ok(key) => {
                            match DownloadFormat::from_manifest_key(&key) {
                                Some(format) => Some(format),
                                None => {
                                    error!("Ignoring invalid track_downloads format specifier '{}' in {}:{}", key, path.display(), item.line_number);
                                    None
                                }
                            }
                        }
                        Err(()) => None
                    }
                })
                .collect();
        }
    }

    if let Some(text_markdown) = optional_embed_value(section, "unlock_info", path) {
        overrides.unlock_info = Some(markdown::to_html(&build.base_url, &text_markdown));
    }
}

fn apply_link(
    element: &Box<dyn SectionElement>,
    local_options: &mut LocalOptions,
    path: &Path
) {
    if element.key() == "link" {
        element.touch();
        if let Some(field) = element.as_field() {
            match field.required_attribute("url") {
                Ok(attribute) => {
                    attribute.touch();
                    match attribute.required_value::<String>() {
                        Ok(value) => match Url::parse(&value) {
                            Ok(url) => {
                                // TODO: Errors, optional_attribute does not exist in enolib?
                                let label = match field.required_attribute("label") {
                                    Ok(attribute) => {
                                        attribute.touch();
                                        match attribute.required_value() {
                                            Ok(label) => Some(label),
                                            Err(_) => None
                                        }
                                    }
                                    Err(_) => None
                                };

                                let (hidden, rel_me) = match field.required_attribute("verification") {
                                    Ok(attribute) => {
                                        attribute.touch();
                                        match attribute.required_value::<String>() {
                                            Ok(value) => match value.as_str() {
                                                "rel-me" => (false, true),
                                                "rel-me-hidden" => (true, true),
                                                _ => (false, false)
                                            }
                                            Err(_) => (false, false)
                                        }
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

// TODO: Add this to _artist.eno processing (and move to document level in the course of that)
fn apply_theme(
    build: &Build,
    cache: &mut Cache,
    section: &Section,
    overrides: &mut Overrides,
    path: &Path
) {
    // TODO: Errors, clean up
    if let Ok(Some(field)) = section.optional_field("theme") {
        field.touch();
        if let Ok(attributes) = field.attributes() {
            for attribute in attributes {
                attribute.touch();
                match attribute.key() {
                    "accent_brightening" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                Some(percentage) => overrides.theme.accent_brightening = percentage,
                                None => error!("Ignoring unsupported value '{}' for 'theme.accent_brightening' (accepts a percentage in the range 0-100 - without the % sign) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "accent_chroma" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                Some(percentage) => overrides.theme.accent_chroma = Some(percentage),
                                None => error!("Ignoring unsupported value '{}' for 'theme.accent_chroma' (accepts a percentage in the range 0-100 - without the % sign) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "accent_hue" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                Some(degrees) => overrides.theme.accent_hue = Some(degrees),
                                None => error!("Ignoring unsupported value '{}' for 'theme.accent_hue' (accepts an amount of degrees in the range 0-360)) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "background_alpha" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                Some(percentage) => overrides.theme.background_alpha = percentage,
                                None => error!("Ignoring unsupported value '{}' for 'theme.background_alpha' (accepts a percentage in the range 0-100 - without the % sign) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "background_image" => {
                        if let Some(Ok(path_relative_to_manifest)) = attribute.optional_value::<String>() {
                            let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                            if absolute_path.exists() {
                                let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                                let image = cache.get_or_create_image(build, path_relative_to_catalog);
                                overrides.theme.background_image = Some(image);
                            } else {
                                error!("Ignoring invalid theme.background_image setting value '{}' in {}:{} (The referenced file was not found)", path_relative_to_manifest, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "base" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match ThemeBase::from_manifest_key(value.as_str()) {
                                Some(variant) => overrides.theme.base = variant,
                                None => {
                                    error!("Ignoring unsupported value '{}' for 'theme.base' (supported values are dark and light) in {}:{}", value, path.display(), attribute.line_number);
                                }
                            }
                        }
                    }
                    "base_chroma" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                Some(percentage) => overrides.theme.base_chroma = percentage,
                                None => error!("Ignoring unsupported value '{}' for 'theme.base_chroma' (accepts a percentage in the range 0-100 - without the % sign) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "base_hue" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                Some(degrees) => overrides.theme.base_hue = degrees,
                                None => error!("Ignoring unsupported value '{}' for 'theme.base_hue' (accepts an amount of degrees in the range 0-360)) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "cover_generator" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match CoverGenerator::from_manifest_key(value.as_str()) {
                                Some(cover_generator) => overrides.theme.cover_generator = cover_generator,
                                None => {
                                    let supported = CoverGenerator::ALL_GENERATORS.map(|key| format!("'{key}'")).join(", ");
                                    error!("Ignoring unsupported value '{}' for 'theme.cover_generator' (supported values are {}) in {}:{}", value, supported, path.display(), attribute.line_number);
                                }
                            }
                        }
                    }
                    "custom_font" => {
                        if let Some(Ok(relative_path)) = attribute.optional_value::<String>() {
                            let absolute_path = path.parent().unwrap().join(&relative_path);
                            if absolute_path.exists() {
                                match ThemeFont::custom(absolute_path) {
                                    Ok(theme_font) => overrides.theme.font = theme_font,
                                    Err(message) => error!("Ignoring invalid theme.font setting value '{}' in {}:{} ({})", relative_path, path.display(), attribute.line_number, message)
                                }
                            } else {
                                error!("Ignoring invalid theme.font setting value '{}' in {}:{} (The referenced file was not found)", relative_path, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "dynamic_range" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                Some(percentage) => overrides.theme.dynamic_range = percentage,
                                None => error!("Ignoring unsupported value '{}' for 'theme.dynamic_range' (accepts a percentage in the range 0-100 - without the % sign) in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "round_corners" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.as_str() {
                                "disabled" => overrides.theme.round_corners = false,
                                "enabled" => overrides.theme.round_corners = true,
                                value => error!("Ignoring unsupported theme.waveforms setting value '{}' (supported values are 'absolute', 'relative' and 'disabled') in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "waveforms" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            match value.as_str() {
                                "absolute" => {
                                    // TODO: Turn this into an Enum (absolute/relative/disabled)
                                    overrides.theme.waveforms = true;
                                    overrides.theme.relative_waveforms = false;
                                }
                                "disabled" => {
                                    overrides.theme.waveforms = false;
                                }
                                "relative" => {
                                    overrides.theme.waveforms = true;
                                    overrides.theme.relative_waveforms = true;
                                }
                                value => error!("Ignoring unsupported theme.waveforms setting value '{}' (supported values are 'absolute', 'relative' and 'disabled') in {}:{}", value, path.display(), attribute.line_number)
                            }
                        }
                    }
                    "system_font" => {
                        if let Some(Ok(value)) = attribute.optional_value::<String>() {
                            overrides.theme.font = if value == "sans" {
                                ThemeFont::SystemSans
                            } else if value == "mono" {
                                ThemeFont::SystemMono
                            } else {
                                ThemeFont::System(value)
                            };
                        }
                    }
                    "link_brightness" => {
                        error!(r##"From faircamp 0.16.0 onwards, theming works a little differently, and the link_brightness setting in {}:{} needs to be replaced, see https://simonrepp.com/faircamp/manual/ for updated instructions."##, path.display(), attribute.line_number);
                    }
                    "link_hue" => {
                        error!(r##"From faircamp 0.16.0 onwards, theming works a little differently, and the link_hue setting in {}:{} needs to be replaced, see https://simonrepp.com/faircamp/manual/ for updated instructions."##, path.display(), attribute.line_number);
                    }
                    "link_saturation" => {
                        error!(r##"From faircamp 0.16.0 onwards, theming works a little differently, and the link_saturation setting in {}:{} needs to be replaced, see https://simonrepp.com/faircamp/manual/ for updated instructions."##, path.display(), attribute.line_number);
                    }
                    "tint_back" => {
                        error!(r##"From faircamp 0.16.0 onwards, theming works a little differently, and the tint_back setting in {}:{} needs to be replaced, see https://simonrepp.com/faircamp/manual/ for updated instructions."##, path.display(), attribute.line_number);
                    }
                    "tint_front" => {
                        error!(r##"From faircamp 0.16.0 onwards, theming works a little differently, and the tint_front setting in {}:{} needs to be replaced, see https://simonrepp.com/faircamp/manual/ for updated instructions."##, path.display(), attribute.line_number);
                    }
                    "text_hue" => {
                        error!(r##"From faircamp 0.16.0 onwards, theming works a little differently, and the text_hue setting in {}:{} needs to be replaced, see https://simonrepp.com/faircamp/manual/ for updated instructions."##, path.display(), attribute.line_number);
                    }
                    other => {
                        error!("Ignoring unrecognized attribute '{}' in {}:{}", other, path.display(), attribute.line_number)
                    }
                }
            }
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

    let optional_field_value_in_document = |document: &Document, key: &str| -> Option<String> {
        match document.optional_field(key) {
            Ok(Some(field)) => {
                field.touch();
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

    let optional_field_with_items_in_document = |document: &Document, key: &str, callback: &mut dyn FnMut(&[Item])| {
        match document.optional_field(key) {
            Ok(Some(field)) => {
                field.touch();
                match field.items() {
                    Ok(items) => callback(items),
                    Err(err) => error!("{} {}", err.message, err_line!(path, err))
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
    };

    let optional_flag_present = |section: &Section, key: &str| -> bool {
        match section.optional_flag(key) {
            Ok(Some(flag)) => {
                flag.touch();
                return true;
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }
        false
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
                attribute.touch();
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
        section.touch();

        apply_theme(build, cache, section, overrides, path);

        match section.field("name").and_then(|field| {
            field.touch();
            field.required_value::<String>()
        }) {
            Ok(name) => {
                let artist = catalog.create_artist(overrides.copy_link, &name, overrides.theme.clone());
                let mut artist_mut = artist.borrow_mut();

                optional_field_with_items(section, "aliases", path, &mut |items: &[Item]| {
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
                    field.touch();
                    match required_attribute_value_with_line(field, "file") {
                        Some((path_relative_to_manifest, line)) => {
                            let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                            if absolute_path.exists() {
                                // TODO: Print errors, refactor
                                let description = match field.required_attribute("description") {
                                    Ok(attribute) => {
                                        attribute.touch();
                                        match attribute.required_value() {
                                            Ok(description) => Some(description),
                                            _ => None
                                        }
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

                if let Some((slug, line)) = optional_field_value_with_line(section, "permalink", path) {
                    match Permalink::new(&slug) {
                        Ok(permalink) => artist_mut.permalink = permalink,
                        Err(err) => error!("Ignoring invalid artist.permalink value '{}' in {}:{} ({})", slug, path.display(), line, err)
                    }
                }

                if let Some(text_markdown) = optional_embed_value(section, "text", path) {
                    artist_mut.text = Some(markdown::to_html_and_stripped(&build.base_url, &text_markdown));
                }
            }
            Err(err) => error!("An artist was specified without a name, and therefore discarded, in {}", err_line!(path, err))
        }
    }

    if let Some(section) = optional_section(&document, "cache", path) {
        section.touch();
        error!(r##"From faircamp 0.16.0 onwards, the "# cache ... " section was merged into "# catalog ..." as the "cache_optimization: delayed|immediate|wipe|manual" option, please move and adapt the current definiton in {}:{} accordingly."##, path.display(), section.line_number);
    }

    if let Some(section) = optional_section(&document, "catalog", path) {
        section.touch();

        if path.parent().unwrap() != build.catalog_dir {
            error!("From faircamp 0.16.0 onwards, \"# catalog ...\" may only be specified from a manifest placed in the catalog root directory, please move the catalog section specified in {}:{} to a manifest in the catalog root directory.", path.display(), section.line_number);
        } else {
            apply_downloads(build, section, overrides, path);
            apply_theme(build, cache, section, overrides, path);

            if let Some((mut value, line)) = optional_field_value_with_line(section, "base_url", path) {
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

            match section.optional_field("cache_optimization") {
                Ok(Some(field)) => {
                    field.touch();
                    match field.optional_value() {
                        Ok(Some(value)) => {
                            match CacheOptimization::from_manifest_key(value.as_str()) {
                                Some(strategy) => {
                                    if cache.optimization != CacheOptimization::Default {
                                        warn_global_set_repeatedly!("cache.optimization", cache.optimization, strategy);
                                    }

                                    cache.optimization = strategy;
                                }
                                None => error!("Ignoring invalid catalog.cache_optimization setting '{}' (available: delayed, immediate, manual, wipe) in {}:{}", value, path.display(), field.line_number)
                            }
                        }
                        Ok(None) => (),
                        Err(err) => error!("{} {}:{}", err.message, path.display(), err.line)
                    }
                }
                Ok(None) => (),
                Err(err) => error!("{} {}:{}", err.message, path.display(), err.line)
            }

            if optional_flag_present(section, "disable_feed") {
                catalog.feed_enabled = false;
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "embedding", path) {
                match value.as_str() {
                    "disabled" => overrides.embedding = false,
                    "enabled" => overrides.embedding = true,
                    value => error!("Ignoring unsupported catalog.embedding setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
                }
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "favicon", path){
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

            if let Some(value) = optional_field_value(section, "freeze_download_urls", path) {
                build.url_salt = value;
            }

            // TODO: Remove this deprecation notice with/around the 1.0 release (introduced feb 2024)
            if let Some((path_relative_to_manifest, line)) = optional_field_value_with_line(section, "feed_image", path) {
                info!("From faircamp 0.13.0 onwards, feed images are auto-generated - catalog.feed_image '{}' specified in {}:{} can be removed, it won't be used anymore.", path_relative_to_manifest, path.display(), line);
            }

            if let Some(field) = optional_field(section, "home_image", path) {
                field.touch();
                match required_attribute_value_with_line(field, "file") {
                    Some((path_relative_to_manifest, line)) => {
                        let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                        if absolute_path.exists() {
                            // TODO: Print errors, refactor
                            let description = match field.required_attribute("description") {
                                Ok(attribute) => {
                                    attribute.touch();
                                    match attribute.required_value() {
                                        Ok(description) => Some(description),
                                        _ => None
                                    }
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

            if let Some(value) = optional_field_value(section, "language", path) {
                build.locale = Locale::from_code(&value);
            }

            for element in section.elements() {
                apply_link(element, local_options, path);
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "m3u", path) {
                match value.as_str() {
                    "catalog" => {
                        catalog.m3u = true;
                        overrides.m3u_enabled = false;
                    }
                    "disabled" => {
                        catalog.m3u = false;
                        overrides.m3u_enabled = false;
                    }
                    "enabled" => {
                        catalog.m3u = true;
                        overrides.m3u_enabled = true;
                    }
                    "releases" => {
                        catalog.m3u = false;
                        overrides.m3u_enabled = true;
                    }
                    value => error!("Ignoring unsupported catalog.m3u setting value '{}' (supported values are 'catalog', 'enabled', 'disabled' and 'releases') in {}:{}", value, path.display(), line)
                }
            }

            if let Some(value) = optional_field_value(section, "more_label", path) {
                catalog.more_label = Some(value);
            }

            // TODO: Would make sense to report if both rotate_download_urls and
            // freeze_download_urls are set (or the latter twice e.g.), as this
            // could lead to unexpected, frustrating behavior for users (and it
            // can happen by accident).
            if optional_flag_present(section, "rotate_download_urls") {
                build.url_salt = uid();
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "copy_link", path) {
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

            match section.optional_field("streaming_quality") {
                Ok(Some(field)) => {
                    field.touch();
                    match field.optional_value() {
                        Ok(Some(key)) => {
                            match StreamingQuality::from_key(&key) {
                                Ok(streaming_quality) => overrides.streaming_quality = streaming_quality,
                                Err(err) => {
                                    error!("Ignoring invalid catalog.streaming_quality value '{}' in {}:{} ({})", key, path.display(), field.line_number, err);
                                    println!("{}", field.snippet());
                                }
                            }
                        }
                        Ok(None) => (),
                        Err(err) => error!("{} {}:{}", err.message, path.display(), err.line)
                    }
                }
                Ok(None) => (),
                Err(err) => error!("{} {}:{}", err.message, path.display(), err.line)
            }

            if let Some(synopsis) = optional_embed_value(section, "synopsis", path) {
                let synopsis_chars = synopsis.chars().count();

                if synopsis_chars <= MAX_SYNOPSIS_CHARS {
                    if let Some(previous_synopsis) = &catalog.synopsis {
                        warn_global_set_repeatedly!("catalog.synopsis", previous_synopsis, synopsis);
                    }

                    let synopsis_escaped = html_escape_outside_attribute(&synopsis);

                    catalog.synopsis = Some(synopsis_escaped);
                } else {
                    error!("Ignoring catalog.synopsis value in {} because it is too long ({} / {} characters)", path.display(), synopsis_chars, MAX_SYNOPSIS_CHARS);
                }
            }

            if let Some(value) = optional_field_value(section, "title", path) {
                if let Some(previous) = catalog.set_title(value.clone()) {
                    warn_global_set_repeatedly!("catalog.title", previous, value);
                }
            }

            if let Some(text_markdown) = optional_embed_value(section, "text", path) {
                let new_text = markdown::to_html_and_stripped(&build.base_url, &text_markdown);

                if let Some(previous_text) = &catalog.text {
                    warn_global_set_repeatedly!("catalog.text", previous_text.stripped, new_text.stripped);
                }

                catalog.text = Some(new_text);
            }

            if let Some((value, line)) = optional_field_value_with_line(section, "track_numbering", path) {
                match TrackNumbering::from_manifest_key(value.as_str()) {
                    Some(variant) => overrides.track_numbering = variant,
                    None => error!("Ignoring unsupported value '{}' for 'catalog.track_numbering' option (supported values are 'arabic', 'arabic-dotted', 'arabic-padded', 'disabled', 'hexadecimal', 'hexadecimal-padded', 'roman' and 'roman-dotted') in {}:{}", value, path.display(), line)
                }
            }
        }
    }

    if let Ok(Some(section)) = document.optional_section("download") {
        section.touch();
        error!(r##"From faircamp 1.0 onwards, the '# download' section is obsolete and its options can/must now be put directly into the "catalog.eno" and "release.eno" files, please move and adapt the current options starting at {}:{} accordingly."##, path.display(), section.line_number);
    }

    if let Ok(Some(section)) = document.optional_section("embedding") {
        section.touch();
        error!(r##"The embedding option must be specified as "embedding: enabled|disabled" (since faircamp 0.16.0) either inside the "catalog.eno" manifest (since faircamp 1.0) or a "# release ..." section, please move and adapt the current definiton in {}:{} accordingly."##, path.display(), section.line_number);
    }

    for element in document.elements() {
        apply_link(element, local_options, path);
    }

    if let Some(section) = optional_section(&document, "localization", path) {
        section.touch();
        error!(r##"From faircamp 0.16.0 onwards, specify the language directly in "# catalog ..." using e.g. "language: fr" (the writing direction is determined from language automatically now). The localization section specified in {}:{} should be removed, it's not supported anymore."##, path.display(), section.line_number);
    }

    if let Some(section) = optional_section(&document, "payment", path) {
        section.touch();
        error!(r##"From faircamp 1.0 onwards, specify payment options directly in catalog.eno or release.eno using the single "payment_info" field. The payment section specified in {}:{} is not supporte anymore."##, path.display(), section.line_number());
    }

    if let Some(section) = optional_section(&document, "release", path) {
        section.touch();

        apply_downloads(build, section, overrides, path);
        apply_theme(build, cache, section, overrides, path);

        if let Some(value) = optional_field_value(section, "artist", path) {
            overrides.release_artists = Some(vec![value]);
        }

        optional_field_with_items(section, "artists", path, &mut |items: &[Item]| {
            overrides.release_artists = Some(
                items
                    .iter()
                    .filter_map(|item| {
                        item.touch();
                        item.optional_value().ok().flatten()
                    })
                    .collect()
            );
        });

        if let Some((value, line)) = optional_field_value_with_line(section, "copy_link", path) {
            match value.as_str() {
                "enabled" => overrides.copy_link = true,
                "disabled" => overrides.copy_link = false,
                value => error!("Ignoring unsupported release.copy_link setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
            }
        }

        if let Some(field) = optional_field(section, "cover", path) {
            field.touch();
            match required_attribute_value_with_line(field, "file") {
                Some((path_relative_to_manifest, line)) => {
                    let absolute_path = path.parent().unwrap().join(&path_relative_to_manifest);
                    if absolute_path.exists() {
                        // TODO: Print errors, refactor
                        let description = match field.required_attribute("description") {
                            Ok(attribute) => {
                                attribute.touch();
                                match attribute.required_value() {
                                    Ok(description) => Some(description),
                                    _ => None
                                }
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

        if let Some((date_str, line)) = optional_field_value_with_line(section, "date", path) {
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

        if let Some((value, line)) = optional_field_value_with_line(section, "embedding", path) {
            match value.as_str() {
                "disabled" => overrides.embedding = false,
                "enabled" => overrides.embedding = true,
                value => error!("Ignoring unsupported release.embedding setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
            }
        }

        for element in section.elements() {
            apply_link(element, local_options, path);
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "m3u", path) {
            match value.as_str() {
                "disabled" => overrides.m3u_enabled = false,
                "enabled" => overrides.m3u_enabled = true,
                value => error!("Ignoring unsupported release.m3u setting value '{}' (supported values are 'enabled' and 'disabled') in {}:{}", value, path.display(), line)
            }
        }

        if let Some(value) = optional_field_value(section, "more_label", path) {
            overrides.more_label = Some(value);
        }

        if let Some((slug, line)) = optional_field_value_with_line(section, "permalink", path) {
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

        if let Some((value, line)) = optional_field_value_with_line(section, "rewrite_tags", path) {
            match value.as_str() {
                "no" => {
                    info!("From faircamp 0.15.0 onwards, 'rewrite_tags: no' should be specified as 'tags: copy' - this will eventually become mandatory (seen in {}:{}).", path.display(), line);
                    overrides.tag_agenda = TagAgenda::Copy;
                }
                "yes" => {
                    info!("From faircamp 0.15.0 onwards, 'rewrite_tags: yes' should be specified as 'tags: normalize' - this will eventually become mandatory (seen in {}:{}).", path.display(), line);
                    overrides.tag_agenda = TagAgenda::normalize();
                }
                other => error!("Ignoring invalid release.rewrite_tags value '{}' (allowed are either 'yes or 'no') in {}:{}", other, path.display(), line)
            }
        }

        match section.optional_field("streaming_quality") {
            Ok(Some(field)) => {
                field.touch();
                match field.optional_value() {
                    Ok(Some(key)) => {
                        match StreamingQuality::from_key(&key) {
                            Ok(streaming_quality) => overrides.streaming_quality = streaming_quality,
                            Err(err) => {
                                error!("Ignoring invalid release.streaming_quality value '{}' in {}:{} ({})", key, path.display(), field.line_number, err);
                                println!("{}", field.snippet());
                            }
                        }
                    }
                    Ok(None) => (),
                    Err(err) => error!("{} {}:{}", err.message, path.display(), err.line)
                }
            }
            Ok(None) => (),
            Err(err) => error!("{} {}:{}", err.message, path.display(), err.line)
        }

        if let Some(synopsis) = optional_embed_value(section, "synopsis", path) {
            let synopsis_chars = synopsis.chars().count();

            if synopsis_chars <= MAX_SYNOPSIS_CHARS {
                let synopsis_escaped = html_escape_outside_attribute(&synopsis);
                overrides.release_synopsis = Some(synopsis_escaped);
            } else {
                error!("Ignoring release.synopsis value in {} because it is too long ({} / {} characters)", path.display(), synopsis_chars, MAX_SYNOPSIS_CHARS);
            }
        }

        match section.optional_field("tags") {
            Ok(Some(field)) => {
                field.touch();
                if let Ok(attributes) = field.attributes() {
                    overrides.tag_agenda = TagAgenda::Remove;

                    for attribute in attributes {
                        attribute.touch();
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
                        other => error!("Ignoring invalid release.tags value '{}' (allowed are either 'copy', 'normalize' or 'remove') in {}:{}", other, path.display(), field.line_number)
                    }
                } else {
                    error!("Ignoring invalid release.tags setting (allowed are either 'copy', 'normalize', 'remove' as value, or a customization with attributes such as 'title = copy', 'artist = rewrite', 'album_artist = remove' etc.) in {}:{}", path.display(), field.line_number)
                }
            }
            Err(err) => error!("{} {}", err.message, err_line!(path, err)),
            _ => ()
        }

        if let Some(text_markdown) = optional_embed_value(section, "text", path) {
            overrides.release_text = Some(markdown::to_html_and_stripped(&build.base_url, &text_markdown));
        }

        if let Some(value) = optional_field_value(section, "title", path) {
            local_options.release_title = Some(value);
        }

        if let Some((value, line)) = optional_field_value_with_line(section, "track_numbering", path) {
            match TrackNumbering::from_manifest_key(value.as_str()) {
                Some(variant) => overrides.track_numbering = variant,
                None => error!("Ignoring unsupported value '{}' for 'release.track_numbering' option (supported values are 'arabic', 'arabic-dotted', 'arabic-padded', 'disabled', 'hexadecimal', 'hexadecimal-padded', 'roman' and 'roman-dotted') in {}:{}", value, path.display(), line)
            }
        }

        if optional_flag_present(section, "unlisted") {
            local_options.unlisted_release = true;
        }
    }

    if let Some(section) = optional_section(&document, "streaming", path) {
        section.touch();
        error!(r##"From faircamp 0.16.0 onwards, "# streaming ..." has been merged into "# catalog ..." and "# release ..." as the 'streaming_quality: frugal|standard' option, please adapt and move the setting currently located in {}:{} accordingly."##, path.display(), section.line_number);
    }

    // TODO: We probably should have these props on a section too - not in the root scope (where it's likely to cause problems/confusion for users)
    if let Some(value) = optional_field_value_in_document(&document, "track_artist") {
        overrides.track_artists = Some(vec![value])
    }

    optional_field_with_items_in_document(&document, "track_artists", &mut |items: &[Item]| {
        overrides.track_artists = Some(
            items
                .iter()
                .filter_map(|item| {
                    item.touch();
                    item.optional_value().ok().flatten()
                })
                .collect()
        );
    });

    let untouched_elements = document.untouched_elements();

    for element in &untouched_elements {
        if let Some(attribute) = element.as_attribute() {
            // TODO: If we e.g. load a release cover ...
            //       cover:
            //       file = foo.jpg
            //       description = The description
            //       ... but the 'file' attribute triggers an error already, the 'description' attribute will be reported
            //       as unsupported because we never process it afterwards! (might apply in other places similarly)
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

fn optional_embed_value(section: &Section, key: &str, path: &Path) -> Option<String> {
    match section.optional_embed(key) {
        Ok(Some(embed)) => {
            embed.touch();
            match embed.required_value() {
                Ok(value) => return Some(value),
                Err(err) => error!("{} {}", err.message, err_line!(path, err))
            }
        }
        Err(err) => error!("{} {}", err.message, err_line!(path, err)),
        _ => ()
    }
    None
}

fn optional_field<'a>(section: &'a Section, key: &str, path: &Path) -> Option<&'a Field> {
    match section.optional_field(key) {
        Ok(field_option) => field_option,
        Err(err) => {
            error!("{} {}", err.message, err_line!(path, err));
            None
        }
    }
}

fn optional_field_value(section: &Section, key: &str, path: &Path) -> Option<String> {
    match section.optional_field(key) {
        Ok(Some(field)) => {
            field.touch();
            match field.required_value() {
                Ok(value) => return Some(value),
                Err(err) => error!("{} {}", err.message, err_line!(path, err))
            }
        }
        Err(err) => error!("{} {}", err.message, err_line!(path, err)),
        _ => ()
    }
    None
}

fn optional_field_with_items(section: &Section, key: &str, path: &Path, callback: &mut dyn FnMut(&[Item])) {
    match section.optional_field(key) {
        Ok(Some(field)) => {
            field.touch();
            match field.items() {
                Ok(items) => callback(items),
                Err(err) => error!("{} {}", err.message, err_line!(path, err))
            }
        }
        Err(err) => error!("{} {}", err.message, err_line!(path, err)),
        _ => ()
    }
}

fn optional_field_value_with_line(section: &Section, key: &str, path: &Path) -> Option<(String, u32)> {
    match section.optional_field(key) {
        Ok(Some(field)) => {
            field.touch();
            match field.required_value() {
                Ok(value) => return Some((value, field.line_number)),
                Err(err) => error!("{} {}", err.message, err_line!(path, err))
            }
        }
        Err(err) => error!("{} {}", err.message, err_line!(path, err)),
        _ => ()
    }
    None
}
