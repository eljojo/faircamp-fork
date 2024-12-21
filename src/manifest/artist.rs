// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::mem;
use std::path::Path;

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
use crate::markdown;

use super::{
    ARTIST_CATALOG_RELEASE_OPTIONS,
    ARTIST_RELEASE_OPTIONS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    not_supported_error,
    platform_printer,
    read_artist_catalog_release_option,
    read_artist_release_option,
    read_obsolete_option
};

const ARTIST_OPTIONS: &[&str] = &[
    "alias",
    "aliases",
    "image",
    "name",
    "text"
];

pub fn read_artist_manifest(
    build: &Build,
    cache: &mut Cache,
    catalog: &mut Catalog,
    dir: &Path,local_options: &mut LocalOptions,
    overrides: &mut Overrides
) {
    let manifest_path = dir.join("artist.eno");

    let content = match fs::read_to_string(&manifest_path) {
        Ok(content) => content,
        Err(err) => {
            error!("Could not read manifest {} ({})", manifest_path.display(), err);
            return
        }
    };

    let document = match enolib::parse_with_printer(&content, platform_printer()) {
        Ok(document) => document,
        Err(err) => {
            error!("Syntax error in {}:{} ({})", manifest_path.display(), err.line, err);
            return
        }
    };

    let mut aliases = Vec::new();
    let mut more = None;
    // By default we use the folder name as name
    let mut name = dir.file_name().unwrap().to_string_lossy().to_string();
    let mut image = None;

    for element in document.elements() {
        match element.key() {
            _ if read_obsolete_option(build, element, &manifest_path) => (),
            "alias" => 'alias: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            aliases = vec![value.to_string()];
                        }

                        break 'alias;
                    }
                }

                let error = "alias needs to be provided as a field with a value, e.g.: 'alias: Älice'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "aliases" => 'aliases: {
                if let Ok(field) = element.as_field() {
                    if let Ok(items) = field.items() {
                        aliases = items.iter().filter_map(|item| item.value().map(|value| value.to_string())).collect();
                        break 'aliases;
                    }
                }

                let error = "aliases needs to be provided as a field containing items, e.g.:\n\naliases:\n- Älice\n- Älicë";
                element_error_with_snippet(element, &manifest_path, error);
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
                                            let error = format!("The referenced file was not found ({})", absolute_path.display());
                                            attribute_error_with_snippet(attribute, &manifest_path, &error);
                                        }

                                    }
                                }
                                _ => {
                                    let error = "The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside an image field";
                                    element_error_with_snippet(element, &manifest_path, error);
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

                let error = "image needs to be provided as a field with attributes, e.g.:\n\nimage:\ndescription = Alice, looking amused\nfile = alice.jpg";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "more" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        more = Some(markdown::to_html_and_stripped(&build.base_url, value));
                    } else {
                        more = None;
                    }
                } else {
                    let error = "The 'more' option to be provided as an embed, e.g.:\n-- more\nA text about the artist\n--more";
                    element_error_with_snippet(element, &manifest_path, error);
                }
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
                let error = "name needs to be provided as a field with a value, e.g.: 'name: Alice'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            _ if read_artist_catalog_release_option(build, cache, element, local_options, &manifest_path, overrides) => (),
            _ if read_artist_release_option(element, local_options, &manifest_path, overrides) => (),
            other => {
                let error = not_supported_error(
                    "artist.eno",
                    other,
                    &[ARTIST_OPTIONS, ARTIST_CATALOG_RELEASE_OPTIONS, ARTIST_RELEASE_OPTIONS]
                );

                element_error_with_snippet(element, &manifest_path, &error);
            }
        }
    }

    let artist = Artist::new_manual(
        aliases,
        overrides.copy_link,
        image,
        mem::take(&mut local_options.links),
        more,
        overrides.more_label.clone(),
        &name,
        local_options.permalink.take(),
        overrides.theme.clone()
    );

    catalog.artists.push(ArtistRc::new(artist));
}