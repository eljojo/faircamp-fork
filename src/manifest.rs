// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use chrono::NaiveDate;
use enolib::prelude::*;
use enolib::{Attribute, Item};
use url::Url;

use crate::{
    Build,
    Cache,
    CoverGenerator,
    DescribedImage,
    DownloadFormat,
    DownloadOption,
    DownloadsConfig,
    ExtraDownloads,
    HtmlAndStripped,
    Link,
    Permalink,
    Price,
    StreamingQuality,
    TagAgenda,
    Theme,
    ThemeBase,
    ThemeFont,
    TrackNumbering
};
use crate::markdown;

const MAX_SYNOPSIS_CHARS: usize = 256;

mod artist;
mod catalog;
mod release;

pub use artist::read_artist_manifest;
pub use catalog::read_catalog_manifest;
pub use release::read_release_manifest;

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
            m3u_enabled: false,
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

fn attribute_error_with_snippet(
    attribute: &Attribute,
    manifest_path: &Path,
    error: &str
) {
    let snippet = attribute.snippet();
    error!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), attribute.line_number, snippet, error);
}

fn element_error_with_snippet(
    element: &Box<dyn SectionElement>,
    manifest_path: &Path,
    error: &str
) {
    let snippet = element.snippet();
    error!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), element.line_number(), snippet, error);
}

fn item_error_with_snippet(
    item: &Item,
    manifest_path: &Path,
    error: &str
) {
    let snippet = item.snippet();
    error!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), item.line_number, snippet, error);
}

/// Try to read a single option from the passed element. Processes
/// options that are present in artist, catalog and release manifests.
pub fn read_artist_catalog_release_option(
    build: &Build,
    cache: &mut Cache,
    element: &Box<dyn SectionElement>,
    local_options: &mut LocalOptions,
    manifest_path: &Path,
    overrides: &mut Overrides
) -> bool {
    match element.key() {
        "embedding" => 'embedding: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "disabled" => overrides.embedding = false,
                            "enabled" => overrides.embedding = true,
                            _ => {
                                let error = format!("The value '{value}' is not recognized for the embedding option, allowed values are 'enabled' and 'disabled'");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'embedding;
                }
            }

            let error = "embedding needs to be provided as a field with the value 'enabled' or 'disabled', e.g.: 'embedding: enabled'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "link" => 'link: {
            if let Ok(field) = element.as_field() {
                if let Ok(attributes) = field.attributes() {
                    let mut hidden = false;
                    let mut label = None;
                    let mut rel_me = false;
                    let mut url = None;

                    for attribute in attributes {
                        match attribute.key() {
                            "label" => {
                                if let Some(value) = attribute.value() {
                                    label = Some(value.to_string());
                                }
                            }
                            "url" => {
                                if let Some(value) = attribute.value() {
                                    match Url::parse(value) {
                                        Ok(parsed) => url = Some(parsed),
                                        Err(err) => {
                                            let error = format!("The url supplied for the link seems to be malformed ({err})");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "verification" => {
                                if let Some(value) = attribute.value() {
                                    match value {
                                        "rel-me" => {
                                            hidden = false;
                                            rel_me = true;
                                        }
                                        "rel-me-hidden" => {
                                            hidden = true;
                                            rel_me = true;
                                        }
                                        _ => {
                                            let error = format!("The verification attribute value '{value}' is not recognized, allowed are 'rel-me' and 'rel-me-hidden'");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            other => {
                                let error = format!("The attribute '{other}' is not recognized here (supported attributes are 'label', 'url' and 'verification'");
                                attribute_error_with_snippet(attribute, &manifest_path, &error);
                            }
                        }
                    }

                    if let Some(url) = url {
                        let link = Link::new(hidden, label, rel_me, url);
                        local_options.links.push(link);
                    } else {
                        let error = "The link option must supply an url attribute at least, e.g.:\n\nlink:\nurl = https://example.com";
                        element_error_with_snippet(element, &manifest_path, error);
                    }

                    break 'link;
                }
            }

            let error = "link must be provided as a field with attributes, e.g.:\n\nlink:\nurl = https://example.com\nlabel = Example";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "more_label" => 'more_label: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        overrides.more_label = Some(value.to_string());
                    }

                    break 'more_label;
                }
            }

            let error = "more_label needs to be provided as a field with a value, e.g.: 'more_label: About'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "theme" => 'theme: {
            if let Ok(field) = element.as_field() {
                if let Ok(attributes) = field.attributes() {
                    for attribute in attributes {
                        match attribute.key() {
                            "accent_brightening" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.accent_brightening = percentage,
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'accent_brightening' (accepts a percentage in the range 0-100 - without the % sign)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "accent_chroma" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.accent_chroma = Some(percentage),
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'accent_chroma' (accepts a percentage in the range 0-100 - without the % sign)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "accent_hue" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                        Some(degrees) => overrides.theme.accent_hue = Some(degrees),
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'accent_hue' (accepts an amount of degrees in the range 0-360)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "background_alpha" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.background_alpha = percentage,
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'background_alpha' (accepts a percentage in the range 0-100 - without the % sign)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "background_image" => {
                                if let Some(Ok(path_relative_to_manifest)) = attribute.optional_value::<String>() {
                                    let absolute_path = manifest_path.parent().unwrap().join(&path_relative_to_manifest);
                                    if absolute_path.exists() {
                                        let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                                        let image = cache.get_or_create_image(build, path_relative_to_catalog);
                                        overrides.theme.background_image = Some(image);
                                    } else {
                                        let error = format!("Invalid background_image setting value '{path_relative_to_manifest}' (The referenced file was not found)");
                                        attribute_error_with_snippet(attribute, &manifest_path, &error);
                                    }
                                }
                            }
                            "base" => {
                                if let Some(value) = attribute.value() {
                                    match ThemeBase::from_manifest_key(value) {
                                        Some(variant) => overrides.theme.base = variant,
                                        None => {
                                            let error = format!("Invalid base setting value '{value}' (supported values are dark and light)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "base_chroma" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.base_chroma = percentage,
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'base_chroma' (accepts a percentage in the range 0-100 - without the % sign)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "base_hue" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                        Some(degrees) => overrides.theme.base_hue = degrees,
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'base_hue' (accepts an amount of degrees in the range 0-360)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "cover_generator" => {
                                if let Some(value) = attribute.value() {
                                    match CoverGenerator::from_manifest_key(value) {
                                        Some(cover_generator) => overrides.theme.cover_generator = cover_generator,
                                        None => {
                                            let supported = CoverGenerator::ALL_GENERATORS.map(|key| format!("'{key}'")).join(", ");
                                            let error = format!("Invalid cover_generator setting value '{value}' (supported values are {supported})");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "custom_font" => {
                                if let Some(Ok(relative_path)) = attribute.optional_value::<String>() {
                                    let absolute_path = manifest_path.parent().unwrap().join(&relative_path);
                                    if absolute_path.exists() {
                                        match ThemeFont::custom(absolute_path) {
                                            Ok(theme_font) => overrides.theme.font = theme_font,
                                            Err(err) => {
                                                let error = format!("Invalid custom_font setting value '{relative_path}' ({err})");
                                                attribute_error_with_snippet(attribute, &manifest_path, &error);
                                            }
                                        }
                                    } else {
                                        let error = format!("Invalid custom_font setting value '{relative_path}' (The referenced file was not found)");
                                        attribute_error_with_snippet(attribute, &manifest_path, &error);
                                    }
                                }
                            }
                            "dynamic_range" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.dynamic_range = percentage,
                                        None => {
                                            let error = format!("Unsupported value '{value}' for 'dynamic_range' (accepts a percentage in the range 0-100 - without the % sign)");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "round_corners" => {
                                if let Some(value) = attribute.value() {
                                    match value {
                                        "disabled" => overrides.theme.round_corners = false,
                                        "enabled" => overrides.theme.round_corners = true,
                                        _ => {
                                            let error = format!("Ignoring unsupported round_corners setting value '{value}' (supported values are 'disabled' and 'enabled')");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            "system_font" => {
                                if let Some(value) = attribute.value() {
                                    overrides.theme.font = match value {
                                        "sans" => ThemeFont::SystemSans,
                                        "mono" => ThemeFont::SystemMono,
                                        _ => ThemeFont::System(value.to_string())
                                    };
                                }
                            }
                            "waveforms" => {
                                if let Some(value) = attribute.value() {
                                    match value {
                                        "absolute" => {
                                            // TODO: Turn this into an Enum (absolute/relative/disabled)?
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
                                        _ => {
                                            let error = format!("Ignoring unsupported waveforms setting value '{value}' (supported values are 'absolute', 'relative' and 'disabled')");
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            other => {
                                let error = format!("The attribute '{other}' is not recognized here (supported attributes are 'accent_brightening', 'accent_chroma', 'accent_hue', 'background_alpha', 'background_image', 'base', 'base_chroma', 'base_hue', 'cover_generator', 'custom_font', 'dynamic_range', 'round_corners', 'system_font' and 'waveforms'");
                                attribute_error_with_snippet(attribute, &manifest_path, &error);
                            }
                        }
                    }

                    break 'theme;
                }
            }

            let error = "theme needs to be provided as a field with attributes, e.g.:\n\ntheme:\nbase = light\nwaveforms = absolute";
            element_error_with_snippet(element, &manifest_path, error);
        }
        // TODO: Is this option documented? (Respectively: Should it be public yet?)
        "track_artist" => 'track_artist: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        overrides.track_artists = Some(vec![value.to_string()]);
                    }

                    break 'track_artist;
                }
            }

            let error = "track_artist needs to be provided as a field with a value, e.g.: 'track_artist: Alice'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        // TODO: Is this option documented? (Respectively: Should it be public yet?)
        "track_artists" => 'track_artists: {
            if let Ok(field) = element.as_field() {
                if let Ok(items) = field.items() {
                    overrides.track_artists = Some(
                        items
                            .iter()
                            .filter_map(|item| item.optional_value().ok().flatten())
                            .collect()
                    );

                    break 'track_artists;
                }
            }

            let error = "track_artists needs to be provided as a field with items, e.g.:\n\ntrack_artists:\n- Alice\n- Bob'";
            element_error_with_snippet(element, &manifest_path, error);
        }

        _ => return false
    }

    true
}

/// Try to read a single option from the passed element. Processes
/// options that are present in artist and release manifests.
pub fn read_artist_release_option(
    element: &Box<dyn SectionElement>,
    manifest_path: &Path,
    overrides: &mut Overrides
) -> bool {
    match element.key() {
        "copy_link" => 'copy_link: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "enabled" => overrides.copy_link = true,
                            "disabled" => overrides.copy_link = false,
                            _ => {
                                let error = "This copy_link setting was not recognized (supported values are 'enabled' and 'disabled')";
                                element_error_with_snippet(element, &manifest_path, error);
                            }
                        }
                    }

                    break 'copy_link;
                }
            }

            let error = "copy_link needs to be provided as a field with a value, e.g.: 'copy_link: disable'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        _ => return false
    }

    true
}

/// Try to read a single option from the passed element. Processes
/// options that are present in catalog and release manifests.
fn read_catalog_release_option(
    build: &Build,
    element: &Box<dyn SectionElement>,
    manifest_path: &Path,
    overrides: &mut Overrides
) -> bool {
    match element.key() {
        "archive_downloads" => 'archive_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        // TODO: Implement via FromStr
                        match DownloadFormat::from_manifest_key(value) {
                            Some(format) => overrides.downloads_config.archive_formats = vec![format],
                            None => {
                                let error = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'archive_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.downloads_config.archive_formats = items
                        .iter()
                        .filter_map(|item| {
                            match item.value() {
                                Some(value) => {
                                    match DownloadFormat::from_manifest_key(value) {
                                        Some(format) => Some(format),
                                        None => {
                                            let error = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                            item_error_with_snippet(item, &manifest_path, &error);
                                            None
                                        }
                                    }
                                }
                                None => None
                            }
                        })
                        .collect();

                    break 'archive_downloads;
                }
            }

            let error = "archive_downloads needs to be provided either as a field with a value (e.g. 'archive_downloads: mp3') or as a field with items, e.g.:\n\narchive_downloads:\n- mp3\n- flac\n- opus\n\n(All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "download_code" => 'download_code: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Permalink::new(&value) {
                            Ok(_) => overrides.download_codes = vec![value.to_string()],
                            Err(err) => {
                                let error = format!("The download code '{value}' contains non-permitted characters ({err})");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'download_code;
                }
            }

            let error = "download_code needs to be provided as a field with a value, e.g.: 'download_code: enter3!'\n\nFor multiple download_codes specify the download_codes field:\n\ndownload_codes:\n- enter3!\n- enter2x";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "download_codes" => 'download_codes: {
            if let Ok(field) = element.as_field() {
                if let Ok(items) = field.items() {
                    overrides.download_codes = items
                        .iter()
                        .filter_map(|item| {
                            match item.value() {
                                Some(value) => {
                                    match Permalink::new(&value) {
                                        Ok(_) => Some(value.to_string()),
                                        Err(err) => {
                                            let error = format!("The download code '{value}' contains non-permitted characters ({err})");
                                            item_error_with_snippet(item, &manifest_path, &error);
                                            None
                                        }
                                    }
                                }
                                None => None
                            }
                        })
                        .collect();

                    break 'download_codes;
                }
            }

            let error = "download_codes needs to be provided as a field with items, e.g.:\n\ndownload_codes:\n- enter3!\n- enter2x";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "downloads" => 'downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
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
                                    Err(err) => {
                                        let error = format!("This external downloads url is somehow not valid ({err})");
                                        element_error_with_snippet(element, &manifest_path, &error);
                                    }
                                }
                            }
                            _ => {
                                let error = "This downloads setting was not recognized (supported values are 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com')";
                                element_error_with_snippet(element, &manifest_path, error);
                            }
                        }
                    }

                    break 'downloads;
                }
            }

            let error = "downloads needs to be provided as a field with the value 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com', e.g.: 'downloads: code'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "extra_downloads" => 'extra_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "bundled" => overrides.downloads_config.extra_downloads = ExtraDownloads::BUNDLED,
                            "disabled" => overrides.downloads_config.extra_downloads = ExtraDownloads::DISABLED,
                            "separate" => overrides.downloads_config.extra_downloads = ExtraDownloads::SEPARATE,
                            _ => {
                                let error = format!("The value '{value}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'extra_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.downloads_config.extra_downloads = ExtraDownloads::DISABLED;

                    for item in items {
                        match item.value() {
                            Some("bundled") => overrides.downloads_config.extra_downloads.bundled = true,
                            Some("disabled") => overrides.downloads_config.extra_downloads = ExtraDownloads::DISABLED,
                            Some("separate") => overrides.downloads_config.extra_downloads.separate = true,
                            Some(other) => {
                                let error = format!("The value '{other}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                            None => ()
                        }
                    }

                    break 'extra_downloads;
                }
            }

            let error = "extra_downloads needs to be provided either as a field with a value (e.g. 'extra_downloads: disabled') or as a field with items, e.g.:\n\nextra_downloads:\n- bundled\n- separate\n\n(The available options are 'bundled', 'disabled' and 'separate')";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "payment_info" => {
            if let Ok(embed) = element.as_embed() {
                if let Some(value) = embed.value() {
                    overrides.payment_info = Some(markdown::to_html(&build.base_url, value));
                }
            } else {
                let error = "payment_info needs to be provided as an embed, e.g.:\n-- payment_info\nThe payment info text\n--payment_info";
                element_error_with_snippet(element, &manifest_path, error);
            }
        }
        "price" => 'price: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Price::new_from_price_string(value) {
                            Ok(price) => overrides.price = price,
                            Err(err) => {
                                let error = format!("Invalid price value ({err})");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'price;
                }
            }

            let error = "price needs to be provided as a field with a currency and price (range) value, e.g.: 'price: USD 0+', 'price: 3.50 GBP', 'price: INR 230+' or 'price: JPY 400-800'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "streaming_quality" => 'streaming_quality: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match StreamingQuality::from_key(value) {
                            Ok(streaming_quality) => overrides.streaming_quality = streaming_quality,
                            Err(error) => element_error_with_snippet(element, &manifest_path, &error)
                        }
                    }

                    break 'streaming_quality;
                }
            }

            let error = "streaming_quality needs to be provided as a field with a value, e.g.: 'streaming_quality: frugal'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "track_downloads" => 'track_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        // TODO: Implement via FromStr
                        match DownloadFormat::from_manifest_key(value) {
                            Some(format) => overrides.downloads_config.track_formats = vec![format],
                            None => {
                                let error = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'track_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.downloads_config.track_formats = items
                        .iter()
                        .filter_map(|item| {
                            match item.value() {
                                Some(value) => {
                                    match DownloadFormat::from_manifest_key(value) {
                                        Some(format) => Some(format),
                                        None => {
                                            let error = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                            item_error_with_snippet(item, &manifest_path, &error);
                                            None
                                        }
                                    }
                                }
                                None => None
                            }
                        })
                        .collect();

                    break 'track_downloads;
                }
            }

            let error = "track_downloads needs to be provided either as a field with a value (e.g. 'track_downloads: mp3') or as a field with items, e.g.:\n\ntrack_downloads:\n- mp3\n- flac\n- opus\n\n(All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "track_numbering" => 'track_numbering: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match TrackNumbering::from_manifest_key(value) {
                            Some(variant) => overrides.track_numbering = variant,
                            None => {
                                let error = format!("track_numbering value '{value}' was not recognized (supported values are 'arabic', 'arabic-dotted', 'arabic-padded', 'disabled', 'hexadecimal', 'hexadecimal-padded', 'roman' and 'roman-dotted')");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'track_numbering;
                }
            }

            let error = "track_numbering needs to be provided as a field with a value, e.g.: 'track_numbering: arabic-dotted'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "unlock_info" => {
            if let Ok(embed) = element.as_embed() {
                if let Some(value) = embed.value() {
                    overrides.unlock_info = Some(markdown::to_html(&build.base_url, value));
                }
            } else {
                let error = "unlock_info needs to be provided as an embed, e.g.:\n-- unlock_info\nThe text instructing on how to get a download code\n--unlock_info";
                element_error_with_snippet(element, &manifest_path, error);
            }
        }
        _ => return false
    }

    true
}

// TODO: Occasionally remove some of these, ideally at some point remove the
// whole thing when everything becomes completely stable.
// (rough timeline: 0.16.0 was released in summer 2024, 1.0 in december 2024)
pub fn read_obsolete_option(
    build: &Build,
    element: &Box<dyn SectionElement>,
    manifest_path: &Path
) -> bool {

    // IMPORTANT:
    // Make sure that this does not "over-consume"!
    // Only elements that are clearly obsolete (e.g. because it's a section,
    // or because the key is not in use anymore _at all_ can be matched here,
    // everything else needs to be ignored, else it shadows real fields that
    // should be regularly processed)

    match element.key() {
        "artist" if element.is_section() => {
            if manifest_path.ends_with("artist.eno") {
                let error = "Since faircamp 1.0, '# artist' sections are not required anymore - just remove the line '# artist'";
                element_error_with_snippet(element, &manifest_path, error);
            } else {
                let error = "Since faircamp 1.0, '# artist' sections are not used anymore. Remove the line '# artist', and move all options below to a file called 'artist.eno', inside a separate directory dedicated to the artist only";
                element_error_with_snippet(element, &manifest_path, &error);
            }
        }
        "cache" if element.is_section() => {
            let error = r##"Since faircamp 0.16.0, the "# cache ... " section was merged into the catalog manifest as the "cache_optimization: delayed|immediate|wipe|manual" option, please move and adapt the current definition accordingly."##;
            element_error_with_snippet(element, &manifest_path, error);
        }
        "catalog" if element.is_section() => {
            if manifest_path.ends_with("catalog.eno") && manifest_path.parent().unwrap() == build.catalog_dir {
                let error = "Since faircamp 1.0, '# catalog' sections are not required anymore - just remove the line '# catalog'";
                element_error_with_snippet(element, &manifest_path, error);
            } else {
                let error = "Since faircamp 1.0, '# catalog' sections are not used anymore. Remove the line '# catalog', and move all options below to a file called 'catalog.eno' in the catalog root folder";
                element_error_with_snippet(element, &manifest_path, error);
            }
        }
        "download" if element.is_section() => {
            let error = "Since faircamp 1.0, the '# download' section is obsolete and its options can/must now be put directly into the 'catalog.eno' and 'release.eno' files, please move and adapt the current options accordingly.";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "embedding" if element.is_section() => {
            let error = "Since faircamp 1.0 the embedding option must be specified as 'embedding: enabled|disabled' inside an 'artist.eno', 'catalog.eno' or 'release.eno' manifest, please move and adapt the current definiton accordingly.";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "link_brightness" => {
            let error = "Since faircamp 0.16.0, theming works differently and the link_brightness setting needs to be replaced (the dynamic_range attribute in the theme field is somewhat related in function now)";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "link_hue" => {
            let error = "Since faircamp 0.16.0, theming works differently and the link_hue setting needs to be replaced (the base_hue attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "link_saturation" => {
            let error = "Since faircamp 0.16.0, theming works differently and the link_saturation setting needs to be replaced (the base_chroma attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "localization" if element.is_section() => {
            let error = "Since faircamp 0.16.0, specify the language directly in the 'catalog.eno' manifest using e.g. 'language: fr' (the writing direction is determined from language automatically now). The localization section must be removed, it's not supported anymore.";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "payment" if element.is_section() => {
            let error = "Since faircamp 1.0, specify payment options directly in an artist.eno, catalog.eno or release.eno manifest using the single 'payment_info' field. The payment section is not supported anymore.";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "release" if element.is_section() => {
            if manifest_path.ends_with("release.eno") {
                let error = "Since faircamp 1.0, '# release' sections are not required anymore - just remove the line '# release'";
                element_error_with_snippet(element, &manifest_path, error);
            } else {
                let error = "Since faircamp 1.0, '# release' sections are not used anymore. Remove the line '# release', and move all options below to a file called 'release.eno'";
                element_error_with_snippet(element, &manifest_path, error);
            }
        }
        "rewrite_tags" => {
            let error = "Since faircamp 1.0, 'rewrite_tags: no' must be specified as 'tags: copy', 'rewrite_tags: yes' as 'tags: normalize'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "streaming" if element.is_section() => {
            let error = r##"Since faircamp 0.16.0, the "# streaming" section has been merged directly into the catalog/release manifests as the 'streaming_quality: frugal|standard' option, please adapt and move the setting accordingly."##;
            element_error_with_snippet(element, &manifest_path, error);
        }
        "text_hue" => {
            let error = "Since faircamp 0.16.0, theming works differently and the text_hue setting needs to be replaced (the base_hue attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "tint_back" => {
            let error = "Since faircamp 0.16.0, theming works differently and the tint_back setting needs to be replaced (the base_chroma attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, &manifest_path, error);
        }
        "tint_front" => {
            let error = "Since faircamp 0.16.0, theming works differently and the tint_front setting needs to be replaced (the base_chroma and dynamic_range attributes in the theme field in combination serve a similar purpose)";
            element_error_with_snippet(element, &manifest_path, error);
        }
        _ => return false
    }

    true
}
