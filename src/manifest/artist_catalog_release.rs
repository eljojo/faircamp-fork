// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;
use url::Url;

use crate::{
    Build,
    Cache,
    CoverGenerator,
    DownloadFormat,
    DownloadOption,
    ExtraDownloads,
    Link,
    LocalOptions,
    Overrides,
    Permalink,
    Price,
    StreamingQuality,
    ThemeBase,
    ThemeFont,
    TrackNumbering
};
use crate::markdown;

use super::{
    attribute_error_with_snippet,
    element_error_with_snippet,
    item_error_with_snippet,
    read_obsolete_theme_attribute
};

pub const ARTIST_CATALOG_RELEASE_OPTIONS: &[&str] = &[
    "copy_link",
    "download_code",
    "download_codes",
    "downloads",
    "embedding",
    "extra_downloads",
    "link",
    "more_label",
    "payment_info",
    "price",
    "release_downloads",
    "streaming_quality",
    "theme",
    "track_artist",
    "track_artists",
    "track_downloads",
    "track_numbering",
    "unlock_info"
];

/// Try to read a single option from the passed element. Processes
/// options that are present in artist, catalog and release manifests.
pub fn read_artist_catalog_release_option(
    build: &mut Build,
    cache: &mut Cache,
    element: &Box<dyn SectionElement>,
    local_options: &mut LocalOptions,
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
                                let message = "This copy_link setting was not recognized (supported values are 'enabled' and 'disabled')";
                                let error = element_error_with_snippet(element, manifest_path, message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'copy_link;
                }
            }

            let message = "copy_link needs to be provided as a field with a value, e.g.: 'copy_link: disabled'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "download_code" => 'download_code: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Permalink::new(value) {
                            Ok(_) => overrides.download_codes = vec![value.to_string()],
                            Err(err) => {
                                let message = format!("The download code '{value}' contains non-permitted characters ({err})");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'download_code;
                }
            }

            let message = "download_code needs to be provided as a field with a value, e.g.: 'download_code: enter3!'\n\nFor multiple download_codes specify the download_codes field:\n\ndownload_codes:\n- enter3!\n- enter2x";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "download_codes" => 'download_codes: {
            if let Ok(field) = element.as_field() {
                if let Ok(items) = field.items() {
                    overrides.download_codes = items
                        .iter()
                        .filter_map(|item| {
                            match item.value() {
                                Some(value) => {
                                    match Permalink::new(value) {
                                        Ok(_) => Some(value.to_string()),
                                        Err(err) => {
                                            let message = format!("The download code '{value}' contains non-permitted characters ({err})");
                                            let error = item_error_with_snippet(item, manifest_path, &message);
                                            build.error(&error);
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

            let message = "download_codes needs to be provided as a field with items, e.g.:\n\ndownload_codes:\n- enter3!\n- enter2x";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
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
                                        let message = format!("This external downloads url is somehow not valid ({err})");
                                        let error = element_error_with_snippet(element, manifest_path, &message);
                                        build.error(&error);
                                    }
                                }
                            }
                            _ => {
                                let message = "This downloads setting was not recognized (supported values are 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com')";
                                let error = element_error_with_snippet(element, manifest_path, message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'downloads;
                }
            }

            let message = "downloads needs to be provided as a field with the value 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com', e.g.: 'downloads: code'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "embedding" => 'embedding: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "disabled" => overrides.embedding = false,
                            "enabled" => overrides.embedding = true,
                            _ => {
                                let message = format!("The value '{value}' is not recognized for the embedding option, allowed values are 'enabled' and 'disabled'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'embedding;
                }
            }

            let message = "embedding needs to be provided as a field with the value 'enabled' or 'disabled', e.g.: 'embedding: enabled'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
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
                                let message = format!("The value '{value}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
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
                                let message = format!("The value '{other}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                            None => ()
                        }
                    }

                    break 'extra_downloads;
                }
            }

            let message = "extra_downloads needs to be provided either as a field with a value (e.g. 'extra_downloads: disabled') or as a field with items, e.g.:\n\nextra_downloads:\n- bundled\n- separate\n\n(The available options are 'bundled', 'disabled' and 'separate')";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "link" => 'link: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Url::parse(value) {
                            Ok(url) => {
                                let link = Link::new(false, None, false, url);
                                local_options.links.push(link);
                            }
                            Err(err) => {
                                let message = format!("The url supplied for the link seems to be malformed ({err})");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'link;
                } else if let Ok(attributes) = field.attributes() {
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
                                        Ok(parsed_url) => url = Some(parsed_url),
                                        Err(err) => {
                                            let message = format!("The url supplied for the link seems to be malformed ({err})");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
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
                                            let message = format!("The verification attribute value '{value}' is not recognized, allowed are 'rel-me' and 'rel-me-hidden'");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            other => {
                                let message = format!("The attribute '{other}' is not recognized here (supported attributes are 'label', 'url' and 'verification'");
                                let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    if let Some(url) = url {
                        let link = Link::new(hidden, label, rel_me, url);
                        local_options.links.push(link);
                    } else {
                        let message = "The link option must supply an url attribute at least, e.g.:\n\nlink:\nurl = https://example.com";
                        let error = element_error_with_snippet(element, manifest_path, message);
                        build.error(&error);
                    }

                    break 'link;
                }
            }

            let message = "link must be provided as a basic field with a value (e.g. 'link: https://example.com') or in its extended form as a field with attributes, e.g.:\n\nlink:\nurl = https://example.com\nlabel = Example";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
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

            let message = "more_label needs to be provided as a field with a value, e.g.: 'more_label: About'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "payment_info" => {
            if let Ok(embed) = element.as_embed() {
                if let Some(value) = embed.value() {
                    overrides.payment_info = Some(markdown::to_html(&build.base_url, value));
                }
            } else {
                let message = "payment_info needs to be provided as an embed, e.g.:\n-- payment_info\nThe payment info text\n--payment_info";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
        }
        "price" => 'price: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Price::new_from_price_string(value) {
                            Ok(price) => overrides.price = price,
                            Err(err) => {
                                let message = format!("Invalid price value ({err})");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'price;
                }
            }

            let message = "price needs to be provided as a field with a currency and price (range) value, e.g.: 'price: USD 0+', 'price: 3.50 GBP', 'price: INR 230+' or 'price: JPY 400-800'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "release_downloads" => 'release_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        // TODO: Implement via FromStr
                        match DownloadFormat::from_manifest_key(value) {
                            Some(format) => overrides.downloads_config.release_formats = vec![format],
                            None => {
                                let message = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'release_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.downloads_config.release_formats = items
                        .iter()
                        .filter_map(|item| {
                            match item.value() {
                                Some(value) => {
                                    match DownloadFormat::from_manifest_key(value) {
                                        Some(format) => Some(format),
                                        None => {
                                            let message = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                            let error = item_error_with_snippet(item, manifest_path, &message);
                                            build.error(&error);
                                            None
                                        }
                                    }
                                }
                                None => None
                            }
                        })
                        .collect();

                    break 'release_downloads;
                }
            }

            let message = "release_downloads needs to be provided either as a field with a value (e.g. 'release_downloads: mp3') or as a field with items, e.g.:\n\nrelease_downloads:\n- mp3\n- flac\n- opus\n\n(All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "streaming_quality" => 'streaming_quality: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match StreamingQuality::from_key(value) {
                            Ok(streaming_quality) => overrides.streaming_quality = streaming_quality,
                            Err(err) => {
                                let error = element_error_with_snippet(element, manifest_path, &err);
                                build.error(&error);
                            }
                        }
                    }

                    break 'streaming_quality;
                }
            }

            let message = "streaming_quality needs to be provided as a field with a value, e.g.: 'streaming_quality: frugal'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "theme" => 'theme: {
            if let Ok(field) = element.as_field() {
                if let Ok(attributes) = field.attributes() {
                    for attribute in attributes {
                        match attribute.key() {
                            _ if read_obsolete_theme_attribute(build, attribute, manifest_path) => (),
                            "accent_brightening" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.accent_brightening = percentage,
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'accent_brightening' (accepts a percentage in the range 0-100 - without the % sign)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            "accent_chroma" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.accent_chroma = Some(percentage),
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'accent_chroma' (accepts a percentage in the range 0-100 - without the % sign)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            "accent_hue" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                        Some(degrees) => overrides.theme.accent_hue = Some(degrees),
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'accent_hue' (accepts an amount of degrees in the range 0-360)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            "background_alpha" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.background_alpha = percentage,
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'background_alpha' (accepts a percentage in the range 0-100 - without the % sign)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
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
                                        let message = format!("Invalid background_image setting value '{path_relative_to_manifest}' (The referenced file was not found)");
                                        let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                        build.error(&error);
                                    }
                                }
                            }
                            "base" => {
                                if let Some(value) = attribute.value() {
                                    match ThemeBase::from_manifest_key(value) {
                                        Some(variant) => overrides.theme.base = variant,
                                        None => {
                                            let message = format!("Invalid base setting value '{value}' (supported values are 'dark' and 'light')");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            "base_chroma" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.base_chroma = percentage,
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'base_chroma' (accepts a percentage in the range 0-100 - without the % sign)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            "base_hue" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u16>().ok().filter(|degrees| *degrees <= 360) {
                                        Some(degrees) => overrides.theme.base_hue = degrees,
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'base_hue' (accepts an amount of degrees in the range 0-360)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
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
                                            let message = format!("Invalid cover_generator setting value '{value}' (supported values are {supported})");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
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
                                                let message = format!("Invalid custom_font setting value '{relative_path}' ({err})");
                                                let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                                build.error(&error);
                                            }
                                        }
                                    } else {
                                        let message = format!("Invalid custom_font setting value '{relative_path}' (The referenced file was not found)");
                                        let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                        build.error(&error);
                                    }
                                }
                            }
                            "dynamic_range" => {
                                if let Some(value) = attribute.value() {
                                    match value.parse::<u8>().ok().filter(|percentage| *percentage <= 100) {
                                        Some(percentage) => overrides.theme.dynamic_range = percentage,
                                        None => {
                                            let message = format!("Unsupported value '{value}' for 'dynamic_range' (accepts a percentage in the range 0-100 - without the % sign)");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
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
                                            let message = format!("Ignoring unsupported round_corners setting value '{value}' (supported values are 'disabled' and 'enabled')");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
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
                                            let message = format!("Ignoring unsupported waveforms setting value '{value}' (supported values are 'absolute', 'relative' and 'disabled')");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            other => {
                                let message = format!("The attribute '{other}' is not recognized here (supported attributes are 'accent_brightening', 'accent_chroma', 'accent_hue', 'background_alpha', 'background_image', 'base', 'base_chroma', 'base_hue', 'cover_generator', 'custom_font', 'dynamic_range', 'round_corners', 'system_font' and 'waveforms')");
                                let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'theme;
                }
            }

            let message = "theme needs to be provided as a field with attributes, e.g.:\n\ntheme:\nbase = light\nwaveforms = absolute";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "track_artist" => 'track_artist: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        overrides.track_artists = vec![value.to_string()];
                    }

                    break 'track_artist;
                }
            }

            let message = "track_artist needs to be provided as a field with a value, e.g.: 'track_artist: Alice'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "track_artists" => 'track_artists: {
            if let Ok(field) = element.as_field() {
                if let Ok(items) = field.items() {
                    overrides.track_artists = items
                            .iter()
                            .filter_map(|item| item.optional_value().ok().flatten())
                            .collect();

                    break 'track_artists;
                }
            }

            let message = "track_artists needs to be provided as a field with items, e.g.:\n\ntrack_artists:\n- Alice\n- Bob'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "track_downloads" => 'track_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        // TODO: Implement via FromStr
                        match DownloadFormat::from_manifest_key(value) {
                            Some(format) => overrides.downloads_config.track_formats = vec![format],
                            None => {
                                let message = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
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
                                            let message = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                            let error = item_error_with_snippet(item, manifest_path, &message);
                                            build.error(&error);
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

            let message = "track_downloads needs to be provided either as a field with a value (e.g. 'track_downloads: mp3') or as a field with items, e.g.:\n\ntrack_downloads:\n- mp3\n- flac\n- opus\n\n(All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "track_numbering" => 'track_numbering: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match TrackNumbering::from_manifest_key(value) {
                            Some(variant) => overrides.track_numbering = variant,
                            None => {
                                let message = format!("track_numbering value '{value}' was not recognized (supported values are 'arabic', 'arabic-dotted', 'arabic-padded', 'disabled', 'hexadecimal', 'hexadecimal-padded', 'roman' and 'roman-dotted')");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'track_numbering;
                }
            }

            let message = "track_numbering needs to be provided as a field with a value, e.g.: 'track_numbering: arabic-dotted'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "unlock_info" => {
            if let Ok(embed) = element.as_embed() {
                if let Some(value) = embed.value() {
                    overrides.unlock_info = Some(markdown::to_html(&build.base_url, value));
                }
            } else {
                let message = "unlock_info needs to be provided as an embed, e.g.:\n-- unlock_info\nThe text instructing on how to get a download code\n--unlock_info";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
        }
        _ => return false
    }

    true
}
