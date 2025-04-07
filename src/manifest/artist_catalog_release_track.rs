// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;
use url::Url;

use crate::{
    Build,
    Cache,
    CoverGenerator,
    DownloadAccessOption,
    DownloadFormat,
    Link,
    LocalOptions,
    Overrides,
    Permalink,
    Price,
    StreamingQuality,
    TagAgenda,
    ThemeBase,
    ThemeFont
};
use crate::markdown;
use crate::util::html_escape_outside_attribute;

use super::{
    MAX_SYNOPSIS_CHARS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    item_error_with_snippet,
    read_obsolete_theme_attribute
};

pub const ARTIST_CATALOG_RELEASE_TRACK_OPTIONS: &[&str] = &[
    "copy_link",
    "download_code",
    "download_codes",
    "embedding",
    "link",
    "more",
    "more_label",
    "payment_info",
    "speed_controls",
    "streaming_quality",
    "synopsis",
    "tags",
    "theme",
    "track_artist",
    "track_artists",
    "track_download_access",
    "track_downloads",
    "track_extras",
    "track_price",
    "unlock_info"
];

/// Try to read a single option from the passed element. Processes
/// options that are present in artist, catalog, release and track manifests.
pub fn read_artist_catalog_release_track_option(
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

            let message = "download_code needs to be provided as a field with a value, e.g.: 'download_code: enterfriend'\n\nFor multiple download_codes specify the download_codes field:\n\ndownload_codes:\n- enterfriend\n- enteralternative";
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

            let message = "download_codes needs to be provided as a field with items, e.g.:\n\ndownload_codes:\n- enterfriend\n- enteralternative";
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
        "link" => 'link: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Url::parse(value) {
                            Ok(_) => {
                                let link = Link::new(false, None, false, value);
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
                                        Ok(_) => url = Some(value.to_string()),
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
        "more" => {
            if let Ok(embed) = element.as_embed() {
                if let Some(value) = embed.value() {
                    local_options.more = Some(markdown::to_html_and_stripped(&build.base_url, value));
                } else {
                    local_options.more = None;
                }
            } else {
                let message = "The 'more' option needs to be provided as an embed, e.g.:\n-- more\nA long-form 'more' text\n--more";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
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
        "speed_controls" => 'speed_controls: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "disabled" => overrides.speed_controls = false,
                            "enabled" => overrides.speed_controls = true,
                            _ => {
                                let message = format!("The value '{value}' is not supported (allowed are: 'enabled' or 'disabled'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'speed_controls;
                }
            }

            let message = "speed_controls needs to be provided as a field with the value 'enabled' or 'disabled' (e.g. 'speed_controls: enabled')";
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
        "synopsis" => {
            if let Ok(embed) = element.as_embed() {
                if let Some(value) = embed.value() {
                    let synopsis_chars = value.chars().count();

                    if synopsis_chars <= MAX_SYNOPSIS_CHARS {
                        let synopsis_escaped = html_escape_outside_attribute(value);
                        local_options.synopsis = Some(synopsis_escaped);
                    } else {
                        let message = format!("Synopsis is too long ({synopsis_chars}/{MAX_SYNOPSIS_CHARS} characters)");
                        let error = element_error_with_snippet(element, manifest_path, &message);
                        build.error(&error);
                    }
                } else {
                    local_options.synopsis = None;
                }
            } else {
                let message = "synopsis needs to be provided as an embed, e.g.:\n-- synopsis\nThis is a synopsis\n--synopsis";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
        }
        "tags" => 'tags: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "copy" => overrides.tag_agenda = TagAgenda::Copy,
                            "normalize" => overrides.tag_agenda = TagAgenda::normalize(),
                            "remove" => overrides.tag_agenda = TagAgenda::Remove,
                            _ => {
                                let message = format!("The value '{value}' is not recognized for the tags option, allowed values are 'copy', 'normalize' and 'remove'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'tags;
                } else if let Ok(attributes) = field.attributes() {
                    overrides.tag_agenda = TagAgenda::Remove;
                    for attribute in attributes {
                        if let Some(value) = attribute.value() {
                            if let Err(err) = overrides.tag_agenda.set(attribute.key(), value) {
                                let error = attribute_error_with_snippet(attribute, manifest_path, &err);
                                build.error(&error);
                            }
                        }
                    }

                    break 'tags;
                }
            }

            let message = "tags needs to be provided either as a field with a value (allowed are 'copy', 'normalize' and 'remove') - e.g.: 'tags: copy' - or as a field with attributes, e.g.:\n\ntags:\ntitle = copy\nartist = rewrite\nalbum_artist = remove";
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
                            // Deprecated ~April 2025, eventually remove in the future
                            "custom_font" => {
                                let message = "The 'custom_font' option has been superseded by more flexible and generic functionality provided through the 'site_assets' and 'site_metadata' options. For the time being 'custom_font' will still work, but it won't forever - make sure to update at some point.";
                                let warning = element_error_with_snippet(element, manifest_path, message);
                                build.warning(&warning);

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
                                        // Deprecated ~April 2025, eventually remove in the future
                                        _ => {
                                            let message = "The functionality for using 'system_font' to specify arbitrary system fonts has been superseded by more flexible and generic functionality provided through the 'site_assets' and 'site_metadata' options. For the time being 'system_font' will still work the way you're using it, but it won't forever - make sure to update at some point.";
                                            let warning = element_error_with_snippet(element, manifest_path, message);
                                            build.warning(&warning);

                                            ThemeFont::System(value.to_string())
                                        }
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
                                let message = format!("The attribute '{other}' is not recognized here (supported attributes are 'accent_brightening', 'accent_chroma', 'accent_hue', 'background_alpha', 'background_image', 'base', 'base_chroma', 'base_hue', 'cover_generator', 'dynamic_range', 'round_corners', 'system_font' and 'waveforms')");
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
        "track_download_access" => 'track_download_access: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "code" => overrides.track_download_access = DownloadAccessOption::Code,
                            "disabled" => overrides.track_download_access = DownloadAccessOption::Disabled,
                            "free" => overrides.track_download_access = DownloadAccessOption::Free,
                            "paycurtain" => overrides.track_download_access = DownloadAccessOption::Paycurtain,
                            other if other.starts_with("http://") || other.starts_with("https://") => {
                                match Url::parse(value) {
                                    Ok(_) => {
                                        overrides.track_download_access = DownloadAccessOption::External { link: value.to_string() };
                                    }
                                    Err(err) => {
                                        let message = format!("This external downloads url is somehow not valid ({err})");
                                        let error = element_error_with_snippet(element, manifest_path, &message);
                                        build.error(&error);
                                    }
                                }
                            }
                            _ => {
                                let message = "This track_download_access setting was not recognized (supported values are 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com')";
                                let error = element_error_with_snippet(element, manifest_path, message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'track_download_access;
                }
            }

            let message = "track_download_access needs to be provided as a field with the value 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com', e.g.: 'track_download_access: code'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "track_downloads" => 'track_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        // TODO: Implement via FromStr
                        match DownloadFormat::from_manifest_key(value) {
                            Some(format) => overrides.track_downloads = vec![format],
                            None => {
                                let message = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'track_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.track_downloads = items
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
        "track_extras" => 'track_extras: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "disabled" => overrides.track_extras = false,
                            "enabled" => overrides.track_extras = true,
                            _ => {
                                let message = format!("The value '{value}' is not supported (allowed are: 'disabled' or 'enabled'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'track_extras;
                }
            }

            let message = "track_extras needs to be provided as a field with the value 'disabled' or 'enabled' (e.g. 'track_extras: disabled')";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "track_price" => 'track_price: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Price::new_from_price_string(value) {
                            Ok(price) => overrides.track_price = price,
                            Err(err) => {
                                let message = format!("Invalid price value ({err})");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'track_price;
                }
            }

            let message = "track_price needs to be provided as a field with a currency and price (range) value, e.g.: 'track_price: USD 0+', 'track_price: 3.50 GBP', 'track_price: INR 230+' or 'track_price: JPY 400-800'";
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
