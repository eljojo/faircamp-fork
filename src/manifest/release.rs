// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use enolib::TerminalPrinter;

use crate::{
    Build,
    Cache,
    DescribedImage,
    LocalOptions,
    Overrides,
    TagAgenda
};
use crate::markdown;
use crate::util::html_escape_outside_attribute;

use super::{
    MAX_SYNOPSIS_CHARS,
    attribute_error_with_snippet,
    element_error_with_snippet,
    read_artist_catalog_release_option,
    read_artist_release_option,
    read_obsolete_option
};

pub fn read_release_manifest(
    build: &Build,
    cache: &mut Cache,
    dir: &Path,
    local_options: &mut LocalOptions,
    overrides: &mut Overrides
) {
    let manifest_path = dir.join("release.eno");

    let content = match fs::read_to_string(&manifest_path) {
        Ok(content) => content,
        Err(err) => {
            error!("Could not read manifest {} ({})", manifest_path.display(), err);
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
            "artist" => 'artist: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            // TODO: Investigate if release_artists can simply be a Vec (without the Option wrapper)
                            overrides.release_artists = Some(vec![value.to_string()]);
                        }

                        break 'artist;
                    }
                }

                let error = "artist needs to be provided as a field with a value, e.g.: 'artist: Alice'\n\nFor multiple artists specify the artists field:\n\nartists:\n- Alice\n- Bob";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "artists" => 'artists: {
                if let Ok(field) = element.as_field() {
                    if let Ok(items) = field.items() {
                        overrides.release_artists = Some(
                            items
                                .iter()
                                .filter_map(|item| item.optional_value().ok().flatten())
                                .collect()
                        );

                        break 'artists;
                    }
                }

                let error = "artists needs to be provided as a field with items, e.g.:\n\nartists:\n- Alice\n- Bob";
                element_error_with_snippet(element, &manifest_path, error);
            }
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
                                    let error = format!("The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside a cover field");
                                    element_error_with_snippet(element, &manifest_path, &error);
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

                let error = "cover needs to be provided as a field with attributes, e.g.:\n\ncover:\ndescription = Alice, looking amused\nfile = alice.jpg";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "date" => 'date: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                                Ok(date) => local_options.release_date = Some(date),
                                Err(err) => {
                                    let error = format!("Invalid date value '{value}': {err}");
                                    element_error_with_snippet(element, &manifest_path, &error);
                                }
                            }
                        } else {
                            local_options.release_date = None;
                        }

                        break 'date;
                    }
                }

                let error = "date needs to be provided as a field with a value following the pattern YYYY-MM-DD, e.g.: 'date: 1999-31-12'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "m3u" => 'm3u: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match value {
                                "disabled" => overrides.m3u_enabled = false,
                                "enabled" => overrides.m3u_enabled = true,
                                _ => {
                                    let error = format!("The value '{value}' is not recognized for the m3u option, allowed values are 'enabled' and 'disabled'");
                                    element_error_with_snippet(element, &manifest_path, &error);
                                }
                            }
                        }

                        break 'm3u;
                    }
                }

                let error = "m3u needs to be provided as a field with the value 'enabled' or 'disabled', e.g.: 'm3u: enabled'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "synopsis" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        let synopsis_chars = value.chars().count();

                        if synopsis_chars <= MAX_SYNOPSIS_CHARS {
                            let synopsis_escaped = html_escape_outside_attribute(&value);
                            overrides.release_synopsis = Some(synopsis_escaped);
                        } else {
                            let error = format!("Synopsis is too long ({synopsis_chars}/{MAX_SYNOPSIS_CHARS} characters)");
                            element_error_with_snippet(element, &manifest_path, &error);
                        }
                    } else {
                        overrides.release_synopsis = None;
                    }
                } else {
                    let error = "synopsis needs to be provided as an embed, e.g.:\n-- synopsis\nThe synopsis for the release\n--synopsis";
                    element_error_with_snippet(element, &manifest_path, error);
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
                                    let error = format!("The value '{value}' is not recognized for the tags option, allowed values are 'copy', 'normalize' and 'remove'");
                                    element_error_with_snippet(element, &manifest_path, &error);
                                }
                            }
                        }

                        break 'tags;
                    } else if let Ok(attributes) = field.attributes() {
                        overrides.tag_agenda = TagAgenda::Remove;
                        for attribute in attributes {
                            if let Some(value) = attribute.value() {
                                if let Err(error) = overrides.tag_agenda.set(attribute.key(), value) {
                                    attribute_error_with_snippet(attribute, &manifest_path, &error);
                                }
                            }
                        }

                        break 'tags;
                    }
                }

                let error = "tags needs to be provided either as a field with a value (allowed are 'copy', 'normalize' and 'remove') - e.g.: 'tags: copy' - or as a field with attributes, e.g.:\n\ntags:\ntitle = copy\nartist = rewrite\nalbum_artist = remove";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "text" => {
                if let Ok(embed) = element.as_embed() {
                    if let Some(value) = embed.value() {
                        overrides.release_text = Some(markdown::to_html_and_stripped(&build.base_url, value));
                    } else {
                        overrides.release_text = None;
                    }
                } else {
                    let error = "text needs to be provided as an embed, e.g.:\n-- text\nThe text about the release\n--text";
                    element_error_with_snippet(element, &manifest_path, error);
                }
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

                let error = "title needs to be provided as a field with a value, e.g.: 'title: Demotape'";
                element_error_with_snippet(element, &manifest_path, error);
            }
            "unlisted" => {
                if element.is_flag() {
                    local_options.unlisted_release = true;
                } else {
                    let error = "unlisted needs to be provided as a flag, that is, exactly as 'unlisted' (without colon and without value)";
                    element_error_with_snippet(element, &manifest_path, error);
                }
            }
            _ if read_artist_catalog_release_option(build, cache, element, local_options, &manifest_path, overrides) => (),
            _ if read_artist_release_option(element, local_options, &manifest_path, overrides) => (),
            _ => {
                let error = format!("The key/name of this option was not recognized, maybe there is a typo, or it appears in a manifest that does not support that option?");
                element_error_with_snippet(element, &manifest_path, &error);
            }
        }
    }
}
