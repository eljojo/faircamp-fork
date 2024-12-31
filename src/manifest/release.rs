// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use chrono::NaiveDate;

use crate::{
    Build,
    Cache,
    Catalog,
    DescribedImage,
    LocalOptions,
    Overrides,
    TagAgenda
};
use crate::markdown;
use crate::util::html_escape_outside_attribute;

use super::{
    ARTIST_CATALOG_RELEASE_OPTIONS,
    ARTIST_RELEASE_OPTIONS,
    CATALOG_RELEASE_OPTIONS,
    MAX_SYNOPSIS_CHARS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    not_supported_error,
    platform_printer,
    read_artist_catalog_release_option,
    read_artist_release_option,
    read_catalog_release_option,
    read_obsolete_option
};

const RELEASE_OPTIONS: &[&str] = &[
    "cover",
    "date",
    "m3u",
    "more",
    "release_artist",
    "release_artists",
    "synopsis",
    "tags",
    "title",
    "unlisted"
];

pub fn read_release_manifest(
    build: &mut Build,
    cache: &mut Cache,
    catalog: &mut Catalog,
    dir: &Path,
    local_options: &mut LocalOptions,
    overrides: &mut Overrides
) {
    let manifest_path = dir.join("release.eno");

    let content = match fs::read_to_string(&manifest_path) {
        Ok(content) => content,
        Err(err) => {
            let error = format!("Could not read manifest {} ({})", manifest_path.display(), err);
            build.error(&error);
            return
        }
    };

    let document = match enolib::parse_with_printer(&content, platform_printer()) {
        Ok(document) => document,
        Err(err) => {
            let error = format!("Syntax error in {}:{} ({})", manifest_path.display(), err.line, err);
            build.error(&error);
            return
        }
    };

    for element in document.elements() {
        match element.key() {
            _ if read_obsolete_option(build, element, &manifest_path) => (),
            "cover" => 'cover: {
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
                                            let error = attribute_error_with_snippet(attribute, &manifest_path, &message);
                                            build.error(&error);
                                        }

                                    }
                                }
                                _ => {
                                    let message = "The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside a cover field";
                                    let error = element_error_with_snippet(element, &manifest_path, message);
                                    build.error(&error);
                                }
                            }
                        }

                        if let Some(path) = path_relative_to_catalog {
                            let image = cache.get_or_create_image(build, &path);
                            overrides.release_cover = Some(DescribedImage::new(description, image));
                        }

                        break 'cover;
                    }
                }

                let message = "cover needs to be provided as a field with attributes, e.g.:\n\ncover:\ndescription = Alice, looking amused\nfile = alice.jpg";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "date" => 'date: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                                Ok(date) => local_options.release_date = Some(date),
                                Err(err) => {
                                    let message = format!("Invalid date value '{value}': {err}");
                                    let error = element_error_with_snippet(element, &manifest_path, &message);
                                    build.error(&error);
                                }
                            }
                        } else {
                            local_options.release_date = None;
                        }

                        break 'date;
                    }
                }

                let message = "date needs to be provided as a field with a value following the pattern YYYY-MM-DD, e.g.: 'date: 1999-31-12'";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "m3u" => 'm3u: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match value {
                                "disabled" => overrides.m3u_enabled = false,
                                "enabled" => overrides.m3u_enabled = true,
                                _ => {
                                    let message = format!("The value '{value}' is not recognized for the m3u option, allowed values are 'enabled' and 'disabled'");
                                    let error = element_error_with_snippet(element, &manifest_path, &message);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'm3u;
                    }
                }

                let message = "m3u needs to be provided as a field with the value 'enabled' or 'disabled', e.g.: 'm3u: enabled'";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "more" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        overrides.release_more = Some(markdown::to_html_and_stripped(&build.base_url, value));
                    } else {
                        overrides.release_more = None;
                    }
                } else {
                    let message = "The 'more' option to be provided as an embed, e.g.:\n-- more\nA text about the release\n--more";
                    let error = element_error_with_snippet(element, &manifest_path, message);
                    build.error(&error);
                }
            }
            "release_artist" => 'release_artist: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            overrides.release_artists = vec![value.to_string()];
                        }

                        break 'release_artist;
                    }
                }

                let message = "release_artist needs to be provided as a field with a value, e.g.: 'release_artist: Alice'\n\nFor multiple artists specify the release_artists field:\n\nrelease_artists:\n- Alice\n- Bob";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "release_artists" => 'release_artists: {
                if let Ok(field) = element.as_field() {
                    if let Ok(items) = field.items() {
                        overrides.release_artists = items
                            .iter()
                            .filter_map(|item| item.optional_value().ok().flatten())
                            .collect();

                        break 'release_artists;
                    }
                }

                let message = "release_artists needs to be provided as a field with items, e.g.:\n\nrelease_artists:\n- Alice\n- Bob";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "synopsis" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        let synopsis_chars = value.chars().count();

                        if synopsis_chars <= MAX_SYNOPSIS_CHARS {
                            let synopsis_escaped = html_escape_outside_attribute(value);
                            overrides.release_synopsis = Some(synopsis_escaped);
                        } else {
                            let message = format!("Synopsis is too long ({synopsis_chars}/{MAX_SYNOPSIS_CHARS} characters)");
                            let error = element_error_with_snippet(element, &manifest_path, &message);
                            build.error(&error);
                        }
                    } else {
                        overrides.release_synopsis = None;
                    }
                } else {
                    let message = "synopsis needs to be provided as an embed, e.g.:\n-- synopsis\nThe synopsis for the release\n--synopsis";
                    let error = element_error_with_snippet(element, &manifest_path, message);
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
                                    let error = element_error_with_snippet(element, &manifest_path, &message);
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
                                    let error = attribute_error_with_snippet(attribute, &manifest_path, &err);
                                    build.error(&error);
                                }
                            }
                        }

                        break 'tags;
                    }
                }

                let message = "tags needs to be provided either as a field with a value (allowed are 'copy', 'normalize' and 'remove') - e.g.: 'tags: copy' - or as a field with attributes, e.g.:\n\ntags:\ntitle = copy\nartist = rewrite\nalbum_artist = remove";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "title" => 'title: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            local_options.release_title = Some(value.to_string());
                        }

                        break 'title;
                    }
                }

                let message = "title needs to be provided as a field with a value, e.g.: 'title: Demotape'";
                let error = element_error_with_snippet(element, &manifest_path, message);
                build.error(&error);
            }
            "unlisted" => {
                if element.is_flag() {
                    local_options.unlisted_release = true;
                } else {
                    let message = "unlisted needs to be provided as a flag, that is, exactly as 'unlisted' (without colon and without value)";
                    let error = element_error_with_snippet(element, &manifest_path, message);
                    build.error(&error);
                }
            }
            _ if read_artist_catalog_release_option(build, cache, element, local_options, &manifest_path, overrides) => (),
            _ if read_artist_release_option(build, element, local_options, &manifest_path) => (),
            _ if read_catalog_release_option(build, catalog, element, &manifest_path) => (),
            other => {
                let message = not_supported_error(
                    "release.eno",
                    other,
                    &[RELEASE_OPTIONS, ARTIST_CATALOG_RELEASE_OPTIONS, ARTIST_RELEASE_OPTIONS, CATALOG_RELEASE_OPTIONS]
                );

                let error = element_error_with_snippet(element, &manifest_path, &message);
                build.error(&error);
            }
        }
    }
}
