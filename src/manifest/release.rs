// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use chrono::NaiveDate;

use crate::{
    Build,
    Cache,
    Catalog,
    LocalOptions,
    Overrides
};

use super::{
    ARTIST_CATALOG_RELEASE_OPTIONS,
    ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
    ARTIST_RELEASE_OPTIONS,
    CATALOG_RELEASE_OPTIONS,
    RELEASE_TRACK_OPTIONS,
    element_error_with_snippet,
    not_supported_error,
    platform_printer,
    read_artist_catalog_release_option,
    read_artist_catalog_release_track_option,
    read_artist_release_option,
    read_catalog_release_option,
    read_obsolete_option,
    read_release_track_option
};

const RELEASE_OPTIONS: &[&str] = &[
    "date",
    "release_artist",
    "release_artists",
    "title",
    "unlisted"
];

pub fn read_release_manifest(
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
            _ if read_obsolete_option(build, element, manifest_path) => (),
            "date" => 'date: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                                Ok(date) => local_options.release_date = Some(date),
                                Err(err) => {
                                    let message = format!("Invalid date value '{value}': {err}");
                                    let error = element_error_with_snippet(element, manifest_path, &message);
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
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
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
                let error = element_error_with_snippet(element, manifest_path, message);
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
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "title" => 'title: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            local_options.title = Some(value.to_string());
                        }

                        break 'title;
                    }
                }

                let message = "title needs to be provided as a field with a value, e.g.: 'title: Demotape'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            "unlisted" => {
                if element.is_flag() {
                    local_options.unlisted_release = true;
                } else {
                    let message = "unlisted needs to be provided as a flag, that is, exactly as 'unlisted' (without colon and without value)";
                    let error = element_error_with_snippet(element, manifest_path, message);
                    build.error(&error);
                }
            }
            _ if read_artist_catalog_release_option(build, element, manifest_path, overrides) => (),
            _ if read_artist_catalog_release_track_option(build, cache, element, local_options, manifest_path, overrides) => (),
            _ if read_artist_release_option(build, element, local_options, manifest_path, overrides) => (),
            _ if read_catalog_release_option(build, catalog, element, manifest_path) => (),
            _ if read_release_track_option(build, cache, dir, element, local_options, manifest_path) => (),
            other => {
                let message = not_supported_error(
                    "release.eno",
                    other,
                    &[
                        RELEASE_OPTIONS,
                        ARTIST_CATALOG_RELEASE_OPTIONS,
                        ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
                        ARTIST_RELEASE_OPTIONS,
                        CATALOG_RELEASE_OPTIONS,
                        RELEASE_TRACK_OPTIONS
                    ]
                );

                let error = element_error_with_snippet(element, manifest_path, &message);
                build.error(&error);
            }
        }
    }
}
