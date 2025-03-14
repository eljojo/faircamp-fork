// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use url::Url;

use crate::{
    Build,
    Cache,
    CacheOptimization,
    Catalog,
    DescribedImage,
    Favicon,
    LocalOptions,
    Locale,
    Overrides
};
use crate::util::uid;

use super::{
    ARTIST_CATALOG_RELEASE_OPTIONS,
    ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
    CATALOG_RELEASE_OPTIONS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    not_supported_error,
    platform_printer,
    read_artist_catalog_release_option,
    read_artist_catalog_release_track_option,
    read_catalog_release_option,
    read_obsolete_option
};

/// Options that exist only for the catalog manifest, or are handled in a
/// catalog-specific way in spite of existing on artist or release manifests
/// as well, are tracked here. This has to correspond 1:1 with the keys that
/// are matched against in `read_catalog_manifest`.
const CATALOG_OPTIONS: &[&str] = &[
    "base_url",
    "cdn_url",
    "cache_optimization",
    "disable_feed",
    "faircamp_signature",
    "favicon",
    "feature_support_artists",
    "freeze_download_urls",
    "home_image",
    "label_mode",
    "language",
    "m3u",
    "opengraph",
    "rotate_download_urls",
    "show_support_artists",
    "title"
];

pub fn read_catalog_manifest(
    build: &mut Build,
    cache: &mut Cache,
    catalog: &mut Catalog,
    dir: &Path,
    local_options: &mut LocalOptions,
    manifest_path: &Path,
    overrides: &mut Overrides
) {
    let content = match fs::read_to_string(manifest_path) {
        Ok(content) => content,
        Err(err) => {
            let error = format!("Could not read catalog manifest {} ({err})", manifest_path.display());
            build.error(&error);
            return
        }
    };

    let document = match enolib::parse_with_printer(&content, platform_printer()) {
        Ok(document) => document,
        Err(err) => {
            let error = format!("Syntax error in {}:{} ({err})", manifest_path.display(), err.line);
            build.error(&error);
            return
        }
    };

    for element in document.elements() {
        match element.key() {
            _ if read_obsolete_option(build, element, manifest_path) => (),
            "base_url" => 'base_url: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            // Ensure the value has a trailing slash. Without one, Url::parse below
                            // would interpret the final path segment as a file, which would lead to
                            // incorrect url construction at a later point.
                            let normalized_url = match value.ends_with('/') {
                                true => value.to_string(),
                                false => format!("{value}/")
                            };

                            match Url::parse(&normalized_url) {
                                Ok(url) => build.base_url = Some(url),
                                Err(err) => {
                                    let message = format!("The base_url setting value '{value}' is not a valid URL: {err}");
                                    let error = element_error_with_snippet(element, manifest_path, &message);
                                    build.error(&error);
                                }
                            }
                        } else {
                            build.base_url = None;
                        }

                        break 'base_url;
                    }
                }

                let message = "base_url needs to be provided as a field with a value, e.g.: 'base_url: https://example.com'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "cdn_url" => 'cdn_url: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            // Ensure a trailing slash for proper URL joining.
                            let normalized_url = if value.ends_with('/') {
                                value.to_string()
                            } else {
                                format!("{value}/")
                            };

                            match Url::parse(&normalized_url) {
                                Ok(url) => build.cdn_url = Some(url),
                                Err(err) => {
                                    let message = format!("The cdn_url setting value '{value}' is not a valid URL: {err}");
                                    let error = element_error_with_snippet(element, manifest_path, &message);
                                    build.error(&error);
                                }
                            }
                        } else {
                            build.cdn_url = None;
                        }
                        break 'cdn_url;
                    }
                }
                let message = "cdn_url needs to be provided as a field with a value, e.g.: 'cdn_url: https://example.com'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "cache_optimization" => 'cache_optimization: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match CacheOptimization::from_manifest_key(value) {
                                Some(strategy) => cache.optimization = strategy,
                                None => {
                                    let message = "This cache_optimization setting was not recognized (supported values are 'delayed', 'immediate', 'manual' and 'wipe')";
                                    let error = element_error_with_snippet(element, manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'cache_optimization;
                    }
                }

                let message = "cache_optimization needs to be provided as a field with the value 'delayed', 'immediate', 'manual' or 'wipe', e.g.: 'cache_optimization: delayed'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "disable_feed" => {
                if element.is_flag() {
                    catalog.feed_enabled = false;
                } else {
                    let message = "disable_feed needs to be provided as a flag, that is, exactly as 'disable_feed' (without colon and without value)";
                    let error = element_error_with_snippet(element, manifest_path, message);
                    build.error(&error);
                }
            }
            "faircamp_signature" => 'faircamp_signature: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match value {
                                "disabled" => {
                                    catalog.faircamp_signature = false;
                                }
                                "enabled" => {
                                    catalog.faircamp_signature = true;
                                }
                                _ => {
                                    let message = "This faircamp_signature setting was not recognized (supported values are 'disabled' and 'enabled)";
                                    let error = element_error_with_snippet(element, manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'faircamp_signature;
                    }
                }

                let message = "faircamp_signature needs to be provided as a field with the value 'disabled' or 'enabled', e.g.: 'faircamp_signature: disabled'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "favicon" => 'favicon: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            if value == "none" {
                                catalog.favicon = Favicon::None;
                            } else {
                                let absolute_path = dir.join(value);
                                if absolute_path.exists() {
                                    match Favicon::custom(absolute_path) {
                                        Ok(favicon) => catalog.favicon = favicon,
                                        Err(err) => {
                                            let error = element_error_with_snippet(element, manifest_path, &err);
                                            build.error(&error);
                                        }
                                    }
                                } else {
                                    let message = format!("The referenced file {} was not found", absolute_path.display());
                                    let error = element_error_with_snippet(element, manifest_path, &message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'favicon;
                    }
                }

                let message = "favicon needs to be provided as a field with a value (relative path to an .ico/.png file), e.g.: 'favicon: favicon.png'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "feature_support_artists" => {
                if element.is_flag() {
                    catalog.feature_support_artists = true;
                } else {
                    let message = "feature_support_artists needs to be provided as a flag, that is, exactly as 'feature_support_artists' (without colon and without value)";
                    let error = element_error_with_snippet(element, manifest_path, message);
                    build.error(&error);
                }
            }
            "freeze_download_urls" => 'freeze_download_urls: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            build.url_salt = value.to_string();
                        }

                        break 'freeze_download_urls;
                    }
                }

                let message = "freeze_download_urls needs to be provided as a field with a value, e.g.: 'freeze_download_urls: April 2024'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "home_image" => 'home_image: {
                if let Ok(field) = element.as_field() {
                    if let Ok(attributes) = field.attributes() {
                        let mut path_relative_to_catalog = None;
                        let mut description = None;

                        for attribute in attributes {
                            match attribute.key() {
                                "description" => {
                                    if let Some(value) = attribute.value() {
                                        description = Some(value.to_string());
                                    }
                                }
                                "file" => {
                                    // file is a path relative to the manifest
                                    if let Some(value) = attribute.value() {
                                        let absolute_path = dir.join(value);
                                        if absolute_path.exists() {
                                            path_relative_to_catalog = Some(absolute_path.strip_prefix(&build.catalog_dir).unwrap().to_path_buf());
                                        } else {
                                            let message = format!("The referenced file was not found ({})", absolute_path.display());
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }

                                    }
                                }
                                _ => {
                                    let message = "The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside an home_image field";
                                    let error = element_error_with_snippet(element, manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        if let Some(path) = path_relative_to_catalog {
                            let image = cache.get_or_create_image(build, &path);
                            catalog.home_image = Some(DescribedImage::new(description, image));
                        }

                        break 'home_image;
                    }
                }

                let message = "home_image needs to be provided as a field with attributes, e.g.:\n\nhome_image:\ndescription = Alice, looking amused\nfile = alice.jpg";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "label_mode" => {
                if element.is_flag() {
                    catalog.label_mode = true;
                } else {
                    let message = "label_mode needs to be provided as a flag, that is, exactly as 'label_mode' (without colon and without value)";
                    let error = element_error_with_snippet(element, manifest_path, message);
                    build.error(&error);
                }
            }
            "language" => 'language: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            build.locale = Locale::from_code(value);
                        }

                        break 'language;
                    }
                }

                let message = "language needs to be provided as a field with a value, e.g.: 'language: fr'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "m3u" => 'm3u: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match value {
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
                                _ => {
                                    let message = "This m3u setting was not recognized (supported values are 'catalog', 'disabled', 'enabled' and 'releases')";
                                    let error = element_error_with_snippet(element, manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'm3u;
                    }
                }

                let message = "m3u needs to be provided as a field with the value 'catalog', 'disabled', 'enabled' or 'releases', e.g.: 'm3u: disable'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "opengraph" => 'opengraph: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match value {
                                "disabled" => {
                                    catalog.opengraph = false;
                                }
                                "enabled" => {
                                    catalog.opengraph = true;
                                }
                                _ => {
                                    let message = "This opengraph setting was not recognized (supported values are 'disabled' and 'enabled)";
                                    let error = element_error_with_snippet(element, manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'opengraph;
                    }
                }

                let message = "The opengraph option needs to be provided as a field with the value 'disabled' or 'enabled', e.g.: 'opengraph: disabled'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "rotate_download_urls" => {
                // TODO: Would make sense to report if both rotate_download_urls and
                // freeze_download_urls are set (or the latter twice e.g.), as this
                // could lead to unexpected, frustrating behavior for users (and it
                // can happen by accident).
                if element.is_flag() {
                    build.url_salt = uid();
                } else {
                    let message = "rotate_download_urls needs to be provided as a flag, that is, exactly as 'rotate_download_urls' (without colon and without value)";
                    let error = element_error_with_snippet(element, manifest_path, message);
                    build.error(&error);
                }
            }
            "show_support_artists" => {
                if element.is_flag() {
                    catalog.show_support_artists = true;
                } else {
                    let message = "show_support_artists needs to be provided as a flag, that is, exactly as 'show_support_artists' (without colon and without value)";
                    let error = element_error_with_snippet(element, manifest_path, message);
                    build.error(&error);
                }
            }
            "title" => 'title: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            catalog.set_title(value.to_string());
                        }

                        break 'title;
                    }
                }

                let message = "title needs to be provided as a field with a value, e.g.: 'title: My music'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            _ if read_artist_catalog_release_option(build, element, manifest_path, overrides) => (),
            _ if read_artist_catalog_release_track_option(build, cache, element, local_options, manifest_path, overrides) => (),
            _ if read_catalog_release_option(build, catalog, element, manifest_path) => (),
            other => {
                let message = not_supported_error(
                    "catalog.eno",
                    other,
                    &[
                        CATALOG_OPTIONS,
                        ARTIST_CATALOG_RELEASE_OPTIONS,
                        ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
                        CATALOG_RELEASE_OPTIONS
                    ]
                );
                let error = element_error_with_snippet(element, manifest_path, &message);
                build.error(&error);
            }
        }
    }
}
