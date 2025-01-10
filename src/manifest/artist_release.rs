// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;

use crate::{Build, LocalOptions, Overrides, Permalink};

use super::element_error_with_snippet;

pub const ARTIST_RELEASE_OPTIONS: &[&str] = &["m3u", "permalink"];

/// Try to read a single option from the passed element. Processes
/// options that are present in artist and release manifests.
pub fn read_artist_release_option(
    build: &mut Build,
    element: &Box<dyn SectionElement>,
    local_options: &mut LocalOptions,
    manifest_path: &Path,
    overrides: &mut Overrides,
) -> bool {
    match element.key() {
        "m3u" => 'm3u: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "disabled" => overrides.m3u_enabled = false,
                            "enabled" => overrides.m3u_enabled = true,
                            _ => {
                                let message = format!("The value '{value}' is not recognized for the m3u option, allowed values are 'enabled' and 'disabled'");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'm3u;
                }
            }

            let message = "m3u needs to be provided as a field with the value 'enabled' or 'disabled', e.g.: 'm3u: enabled'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "permalink" => 'permalink: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Permalink::new(value) {
                            Ok(permalink) => local_options.permalink = Some(permalink),
                            Err(err) => {
                                let message = format!("There is a problem with the permalink '{value}': {err}");
                                let error = element_error_with_snippet(element, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    break 'permalink;
                }
            }

            let message = "permalink needs to be provided as a field with a value, e.g.: 'permalink: such-perma-wow'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        _ => return false
    }

    true
}
