// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use crate::{
    Build,
    Cache,
    LocalOptions,
    Overrides
};

use super::{
    ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
    RELEASE_TRACK_OPTIONS,
    element_error_with_snippet,
    not_supported_error,
    platform_printer,
    read_artist_catalog_release_track_option,
    read_obsolete_option,
    read_release_track_option
};

const TRACK_OPTIONS: &[&str] = &[
    "title"
];

pub fn read_track_manifest(
    build: &mut Build,
    cache: &mut Cache,
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
            "title" => 'title: {
                if let Ok(field) = element.as_field() {
                    if let Ok(result) = field.value() {
                        if let Some(value) = result {
                            local_options.title = Some(value.to_string());
                        }

                        break 'title;
                    }
                }

                let message = "title needs to be provided as a field with a value, e.g.: 'title: Interlude'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
            _ if read_artist_catalog_release_track_option(build, cache, element, local_options, manifest_path, overrides) => (),
            _ if read_release_track_option(build, cache, dir, element, local_options, manifest_path) => (),
            other => {
                let message = not_supported_error(
                    "track.eno",
                    other,
                    &[
                        ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
                        RELEASE_TRACK_OPTIONS,
                        TRACK_OPTIONS
                    ]
                );

                let error = element_error_with_snippet(element, manifest_path, &message);
                build.error(&error);
            }
        }
    }
}
