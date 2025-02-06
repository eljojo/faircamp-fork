// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;
use url::Url;

use crate::{
    Build,
    DownloadAccessOption,
    DownloadFormat,
    ExtraDownloads,
    Overrides,
    Price,
    TrackNumbering
};

use super::{
    element_error_with_snippet,
    item_error_with_snippet
};

pub const ARTIST_CATALOG_RELEASE_OPTIONS: &[&str] = &[
    "release_download_access",
    "release_downloads",
    "release_extras",
    "release_price",
    "track_numbering"
];

/// Try to read a single option from the passed element. Processes
/// options that are present in artist, catalog and release manifests.
pub fn read_artist_catalog_release_option(
    build: &mut Build,
    element: &Box<dyn SectionElement>,
    manifest_path: &Path,
    overrides: &mut Overrides
) -> bool {
    match element.key() {
        // TODO: 'downloads' was deprecated in favor of release_download_access in ~february 2025, eventually remove this temporary fallback handling in a future release
        "downloads" => 'downloads: {
            let message = "The 'downloads' option was split into two: 'release_download_access' and 'track_download_access'. Depending on whether you have release and/or track downloads configured you should now use one or both of the two - in terms of the value you can provide for each of them it still works exactly the same as 'downloads' though. For the time being 'downloads' will still work, but it won't forever - make sure to update this at some point.";
            let warning = element_error_with_snippet(element, manifest_path, message);
            build.warning(&warning);

            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "code" => {
                                overrides.release_download_access = DownloadAccessOption::Code;
                                overrides.track_download_access = DownloadAccessOption::Code;
                            }
                            "disabled" => {
                                overrides.release_download_access = DownloadAccessOption::Disabled;
                                overrides.track_download_access = DownloadAccessOption::Disabled;
                            }
                            "free" => {
                                overrides.release_download_access = DownloadAccessOption::Free;
                                overrides.track_download_access = DownloadAccessOption::Free;
                            }
                            "paycurtain" => {
                                overrides.release_download_access = DownloadAccessOption::Paycurtain;
                                overrides.track_download_access = DownloadAccessOption::Paycurtain;
                            }
                            other if other.starts_with("http://") || other.starts_with("https://") => {
                                match Url::parse(value) {
                                    Ok(_) => {
                                        overrides.release_download_access = DownloadAccessOption::External { link: value.to_string() };
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
        // TODO: 'extra_downloads' was deprecated in favor of release_extras in ~february 2025, eventually remove this temporary fallback handling in a future release
        "extra_downloads" => 'extra_downloads: {
            let message = "The 'extra_downloads' option is now called 'release_extras' - it works exactly the same though. For the time being 'extra_downloads' will still work, but it won't forever - make sure to update at some point. Note that in addition to 'release_extras', there is now also a 'track_extras' option!";
            let warning = element_error_with_snippet(element, manifest_path, message);
            build.warning(&warning);

            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "bundled" => overrides.release_extras = ExtraDownloads::BUNDLED,
                            "disabled" => overrides.release_extras = ExtraDownloads::DISABLED,
                            "separate" => overrides.release_extras = ExtraDownloads::SEPARATE,
                            _ => {
                                let message = format!("The value '{value}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'extra_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.release_extras = ExtraDownloads::DISABLED;

                    for item in items {
                        match item.value() {
                            Some("bundled") => overrides.release_extras.bundled = true,
                            Some("disabled") => overrides.release_extras = ExtraDownloads::DISABLED,
                            Some("separate") => overrides.release_extras.separate = true,
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

            let message = "release_extras needs to be provided either as a field with a value (e.g. 'release_extras: disabled') or as a field with items, e.g.:\n\nrelease_extras:\n- bundled\n- separate\n\n(The available options are 'bundled', 'disabled' and 'separate')";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        // TODO: 'price' was deprecated in favor of release_price in ~february 2025, eventually remove this temporary fallback handling in a future release
        "price" => 'price: {
            let message = "The 'price' option is now called 'release_price' - it works exactly the same though. For the time being 'price' will still work, but it won't forever - make sure to update at some point. Note that in addition to 'release_price', there is now also a 'track_price' option!";
            let warning = element_error_with_snippet(element, manifest_path, message);
            build.warning(&warning);

            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Price::new_from_price_string(value) {
                            Ok(price) => overrides.release_price = price,
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

            let message = "release_price needs to be provided as a field with a currency and price (range) value, e.g.: 'release_price: USD 0+', 'release_price: 3.50 GBP', 'release_price: INR 230+' or 'release_price: JPY 400-800'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "release_download_access" => 'release_download_access: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "code" => overrides.release_download_access = DownloadAccessOption::Code,
                            "disabled" => overrides.release_download_access = DownloadAccessOption::Disabled,
                            "free" => overrides.release_download_access = DownloadAccessOption::Free,
                            "paycurtain" => overrides.release_download_access = DownloadAccessOption::Paycurtain,
                            other if other.starts_with("http://") || other.starts_with("https://") => {
                                match Url::parse(value) {
                                    Ok(_) => {
                                        overrides.release_download_access = DownloadAccessOption::External { link: value.to_string() };
                                    }
                                    Err(err) => {
                                        let message = format!("This external downloads url is somehow not valid ({err})");
                                        let error = element_error_with_snippet(element, manifest_path, &message);
                                        build.error(&error);
                                    }
                                }
                            }
                            _ => {
                                let message = "This release_download_access setting was not recognized (supported values are 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com')";
                                let error = element_error_with_snippet(element, manifest_path, message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'release_download_access;
                }
            }

            let message = "release_download_access needs to be provided as a field with the value 'code', 'disabled', 'free', 'paycurtain' or an external url like 'https://example.com', e.g.: 'release_download_access: code'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "release_downloads" => 'release_downloads: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        // TODO: Implement via FromStr
                        match DownloadFormat::from_manifest_key(value) {
                            Some(format) => overrides.release_downloads = vec![format],
                            None => {
                                let message = format!("The download format '{value}' is not supported (All available formats: 'aac', 'aiff', 'alac', 'flac', 'mp3', 'ogg_vorbis', 'opus', 'opus_48', 'opus_96', 'opus_128' and 'wav')");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'release_downloads;
                } else if let Ok(items) = field.items() {
                    overrides.release_downloads = items
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
        "release_extras" => 'release_extras: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "bundled" => overrides.release_extras = ExtraDownloads::BUNDLED,
                            "disabled" => overrides.release_extras = ExtraDownloads::DISABLED,
                            "separate" => overrides.release_extras = ExtraDownloads::SEPARATE,
                            _ => {
                                let message = format!("The value '{value}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'release_extras;
                } else if let Ok(items) = field.items() {
                    overrides.release_extras = ExtraDownloads::DISABLED;

                    for item in items {
                        match item.value() {
                            Some("bundled") => overrides.release_extras.bundled = true,
                            Some("disabled") => overrides.release_extras = ExtraDownloads::DISABLED,
                            Some("separate") => overrides.release_extras.separate = true,
                            Some(other) => {
                                let message = format!("The value '{other}' is not supported (allowed are: 'bundled', 'disabled' or 'separate'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                            None => ()
                        }
                    }

                    break 'release_extras;
                }
            }

            let message = "release_extras needs to be provided either as a field with a value (e.g. 'release_extras: disabled') or as a field with items, e.g.:\n\nrelease_extras:\n- bundled\n- separate\n\n(The available options are 'bundled', 'disabled' and 'separate')";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "release_price" => 'release_price: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Price::new_from_price_string(value) {
                            Ok(price) => overrides.release_price = price,
                            Err(err) => {
                                let message = format!("Invalid price value ({err})");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'release_price;
                }
            }

            let message = "release_price needs to be provided as a field with a currency and price (range) value, e.g.: 'release_price: USD 0+', 'release_price: 3.50 GBP', 'release_price: INR 230+' or 'release_price: JPY 400-800'";
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
        _ => return false
    }

    true
}
