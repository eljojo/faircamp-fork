// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::mem;
use std::path::Path;

use url::Url;

use crate::{
    Artist,
    ArtistRc,
    Build,
    Cache,
    Catalog,
    DescribedImage,
    LocalOptions,
    Overrides
};

use super::{
    ARTIST_CATALOG_RELEASE_OPTIONS,
    ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
    ARTIST_RELEASE_OPTIONS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    not_supported_error,
    platform_printer,
    read_artist_catalog_release_option,
    read_artist_catalog_release_track_option,
    read_artist_release_option,
    read_obsolete_option
};

const ARTIST_OPTIONS: &[&str] = &[
    "alias",
    "aliases",
    "external_page",
    "image",
    "name"
];

pub fn read_artist_manifest(
    build: &mut Build,
    cache: &mut Cache,
    catalog: &mut Catalog,
    dir: &Path,
    manifest_path: &Path,
    overrides: &mut Overrides
) {
    let content = match fs::read_to_string(manifest_path) {
        Ok(content) => content,
        Err(err) => {
            let error =format!("Could not read manifest {} ({})", manifest_path.display(), err);
            build.error(&error);
            return
        }
    };

    let document = match enolib::parse_with_printer(&content, platform_printer()) {
        Ok(document) => document,
        Err(err) => {
            let error =format!("Syntax error in {}:{} ({})", manifest_path.display(), err.line, err);
            build.error(&error);
            return
        }
    };

    let mut local_options = LocalOptions::new();

    let mut aliases = Vec::new();
    let mut external_page = None;
    // By default we use the folder name as name
    let mut name = dir.file_name().unwrap().to_string_lossy().to_string();
    let mut image = None;

    for element in document.elements() {
        match element.key() {
            _ if read_obsolete_option(build, element, manifest_path) => (),
            "alias" => 'alias: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            aliases = vec![value.to_string()];
                        }

                        break 'alias;
                    }
                }

                let message = "alias needs to be provided as a field with a value, e.g.: 'alias: Älice'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "aliases" => 'aliases: {
                if let Ok(field) = element.as_field() {
                    if let Ok(items) = field.items() {
                        aliases = items.iter().filter_map(|item| item.value().map(|value| value.to_string())).collect();
                        break 'aliases;
                    }
                }

                let message = "aliases needs to be provided as a field containing items, e.g.:\n\naliases:\n- Älice\n- Älicë";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "external_page" => 'external_page: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match Url::parse(value) {
                                Ok(_) => external_page = Some(value.to_string()),
                                Err(err) => {
                                    let message = format!("The url supplied for the external_page option seems to be malformed ({err})");
                                    let error = element_error_with_snippet(element, manifest_path, &message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'external_page;
                    }
                }

                let message = "external_page must be provided as a field with a value, e.g. 'external_page: https://example.com'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "image" => 'image: {
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
                                    let message = "The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside an image field";
                                    let error = element_error_with_snippet(element, manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        if let Some(path) = path_relative_to_catalog {
                            let obtained_image = cache.get_or_create_image(build, &path);
                            image = Some(DescribedImage::new(description, obtained_image));
                        }

                        break 'image;
                    }
                }

                let message = "image needs to be provided as a field with attributes, e.g.:\n\nimage:\ndescription = Alice, looking amused\nfile = alice.jpg";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "name" => 'name: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            name = value.to_string();
                        }

                        break 'name;
                    }
                }
                let message = "name needs to be provided as a field with a value, e.g.: 'name: Alice'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            _ if read_artist_catalog_release_option(build, element, manifest_path, overrides) => (),
            _ if read_artist_catalog_release_track_option(build, cache, element, &mut local_options, manifest_path, overrides) => (),
            _ if read_artist_release_option(build, element, &mut local_options, manifest_path, overrides) => (),
            other => {
                let message = not_supported_error(
                    "artist.eno",
                    other,
                    &[
                        ARTIST_OPTIONS,
                        ARTIST_CATALOG_RELEASE_OPTIONS,
                        ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
                        ARTIST_RELEASE_OPTIONS
                    ]
                );

                let error = element_error_with_snippet(element, manifest_path, &message);
                build.error(&error);
            }
        }
    }

    let artist = Artist::new_manual(
        aliases,
        overrides.copy_link,
        external_page,
        image,
        mem::take(&mut local_options.links),
        overrides.m3u_enabled,
        local_options.more.take(),
        overrides.more_label.clone(),
        &name,
        local_options.permalink.take(),
        local_options.synopsis.take(),
        overrides.theme.clone()
    );

    catalog.artists.push(ArtistRc::new(artist));
}