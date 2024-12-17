// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use enolib::TerminalPrinter;
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
use crate::markdown;
use crate::util::{html_escape_outside_attribute, uid};

use super::{
    MAX_SYNOPSIS_CHARS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    read_artist_catalog_release_option,
    read_catalog_release_option,
    read_obsolete_option
};

pub fn read_catalog_manifest(
    build: &mut Build,
    cache: &mut Cache,
    catalog: &mut Catalog,
    dir: &Path,
    local_options: &mut LocalOptions,
    overrides: &mut Overrides
) {
    let manifest_path = dir.join("catalog.eno");

    let content = match fs::read_to_string(&manifest_path) {
        Ok(content) => content,
        Err(err) => {
            error!("Could not read catalog manifest {} ({})", manifest_path.display(), err);
            return
        }
    };

    let document = match enolib::parse_with_printer(&content, Box::new(TerminalPrinter)) {
        Ok(document) => document,
        Err(err) => {
            error!("Syntax error in {}:{} ({})", manifest_path.display(), err.line, err);
            return
        }
    };

    for element in document.elements() {
        match element.key() {
            _ if read_obsolete_option(build, element, &manifest_path) => (),
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
                                    let error = format!("The base_url setting value '{value}' is not a valid URL: {err}");
                                    element_error_with_snippet(element, &manifest_path, &error);
                                }
                            }
                        } else {
                            build.base_url = None;
                        }

                        break 'base_url;
                    }
                }

                let error = "base_url needs to be provided as a field with a value, e.g.: 'base_url: https://example.com'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "cache_optimization" => 'cache_optimization: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match CacheOptimization::from_manifest_key(value) {
                                Some(strategy) => cache.optimization = strategy,
                                None => {
                                    let error = "This cache_optimization setting was not recognized (supported values are 'delayed', 'immediate', 'manual' and 'wipe')";
                                    element_error_with_snippet(element, &manifest_path, error);
                                }
                            }
                        }

                        break 'cache_optimization;
                    }
                }

                let error = "cache_optimization needs to be provided as a field with the value 'delayed', 'immediate', 'manual' or 'wipe', e.g.: 'cache_optimization: delayed'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "copy_link" => 'copy_link: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match value {
                                // TODO: Maybe we can somehow improve this, so that
                                //       we can just use the same generic implementation for copy_link
                                //       manifest retrieval for artist/catalog/release and set it afterwards
                                //       for the catalog somehow.
                                "enabled" => {
                                    catalog.copy_link = true;
                                    overrides.copy_link = true;
                                }
                                "disabled" => {
                                    catalog.copy_link = false;
                                    overrides.copy_link = false;
                                }
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
            "disable_feed" => {
                if element.is_flag() {
                    catalog.feed_enabled = false;
                } else {
                    let error = "disable_feed needs to be provided as a flag, that is, exactly as 'disable_feed' (without colon and without value)";
                    element_error_with_snippet(element, &manifest_path, error);
                }
            }
            "favicon" => 'favicon: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            if value == "none" {
                                catalog.favicon = Favicon::None;
                            } else {
                                let absolute_path = dir.join(&value);
                                if absolute_path.exists() {
                                    match Favicon::custom(absolute_path) {
                                        Ok(favicon) => catalog.favicon = favicon,
                                        Err(error) => element_error_with_snippet(element, &manifest_path, &error)
                                    }
                                } else {
                                    let error = format!("The referenced file {} was not found", absolute_path.display());
                                    element_error_with_snippet(element, &manifest_path, &error);
                                }
                            }
                        }

                        break 'favicon;
                    }
                }

                let error = "favicon needs to be provided as a field with a value (relative path to an .ico/.png file), e.g.: 'favicon: favicon.png'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "feature_support_artists" => {
                if element.is_flag() {
                    catalog.feature_support_artists = true;
                } else {
                    let error = "feature_support_artists needs to be provided as a flag, that is, exactly as 'feature_support_artists' (without colon and without value)";
                    element_error_with_snippet(element, &manifest_path, error);
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

                let error = "freeze_download_urls needs to be provided as a field with a value, e.g.: 'freeze_download_urls: April 2024'";
                element_error_with_snippet(element, &manifest_path, error);
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
                                        let absolute_path = dir.join(&value);
                                        if absolute_path.exists() {
                                            path_relative_to_catalog = Some(absolute_path.strip_prefix(&build.catalog_dir).unwrap().to_path_buf());
                                        } else {
                                            let error = format!("The referenced file was not found ({})", absolute_path.display());
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }

                                    }
                                }
                                _ => {
                                    let error = format!("The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside an home_image field");
                                    element_error_with_snippet(element, &manifest_path, &error);
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

                let error = "home_image needs to be provided as a field with attributes, e.g.:\n\nhome_image:\ndescription = Alice, looking amused\nfile = alice.jpg";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "label_mode" => {
                if element.is_flag() {
                    catalog.label_mode = true;
                } else {
                    let error = "label_mode needs to be provided as a flag, that is, exactly as 'label_mode' (without colon and without value)";
                    element_error_with_snippet(element, &manifest_path, error);
                }
            }
            "language" => 'language: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            build.locale = Locale::from_code(&value);
                        }

                        break 'language;
                    }
                }

                let error = "language needs to be provided as a field with a value, e.g.: 'language: fr'";
                element_error_with_snippet(element, &manifest_path, error);
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
                                    let error = "This m3u setting was not recognized (supported values are 'catalog', 'disabled', 'enabled' and 'relases')";
                                    element_error_with_snippet(element, &manifest_path, error);
                                }
                            }
                        }

                        break 'm3u;
                    }
                }

                let error = "m3u needs to be provided as a field with the value 'catalog', 'disabled', 'enabled' or 'relases', e.g.: 'm3u: disable'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "rotate_download_urls" => {
                // TODO: Would make sense to report if both rotate_download_urls and
                // freeze_download_urls are set (or the latter twice e.g.), as this
                // could lead to unexpected, frustrating behavior for users (and it
                // can happen by accident).
                if element.is_flag() {
                    build.url_salt = uid();
                } else {
                    let error = "rotate_download_urls needs to be provided as a flag, that is, exactly as 'rotate_download_urls' (without colon and without value)";
                    element_error_with_snippet(element, &manifest_path, error);
                }
            }
            "show_support_artists" => {
                if element.is_flag() {
                    catalog.show_support_artists = true;
                } else {
                    let error = "show_support_artists needs to be provided as a flag, that is, exactly as 'show_support_artists' (without colon and without value)";
                    element_error_with_snippet(element, &manifest_path, error);
                }
            }
            "synopsis" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        let synopsis_chars = value.chars().count();

                        if synopsis_chars <= MAX_SYNOPSIS_CHARS {
                            let synopsis_escaped = html_escape_outside_attribute(&value);
                            catalog.synopsis = Some(synopsis_escaped);
                        } else {
                            let error = format!("Synopsis is too long ({synopsis_chars}/{MAX_SYNOPSIS_CHARS} characters)");
                            element_error_with_snippet(element, &manifest_path, &error);
                        }
                    } else {
                        catalog.synopsis = None;
                    }
                } else {
                    let error = "synopsis needs to be provided as an embed, e.g.:\n-- synopsis\nThe synopsis for the catalog\n--synopsis";
                    element_error_with_snippet(element, &manifest_path, error);
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

                let error = "title needs to be provided as a field with a value, e.g.: 'title: My music'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "text" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        catalog.text = Some(markdown::to_html_and_stripped(&build.base_url, value));
                    }
                } else {
                    let error = "text needs to be provided as an embed, e.g.:\n-- text\nThe text about the catalog\n--text";
                    element_error_with_snippet(element, &manifest_path, error);
                }
            }
            _ if read_artist_catalog_release_option(build, cache, element, local_options, &manifest_path, overrides) => (),
            _ if read_catalog_release_option(catalog, element, &manifest_path) => (),
            _ => {
                let error = format!("The key/name of this option was not recognized, maybe there is a typo, or it appears in a manifest that does not support that option?");
                element_error_with_snippet(element, &manifest_path, &error);
            }
        }
    }
}
