// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;

use crate::{
    LocalOptions,
    Overrides,
    Permalink
};

use super::element_error_with_snippet;

/// Try to read a single option from the passed element. Processes
/// options that are present in artist and release manifests.
pub fn read_artist_release_option(
    element: &Box<dyn SectionElement>,
    local_options: &mut LocalOptions,
    manifest_path: &Path,
    overrides: &mut Overrides
) -> bool {
    match element.key() {
        "copy_link" => 'copy_link: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match value {
                            "enabled" => overrides.copy_link = true,
                            "disabled" => overrides.copy_link = false,
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
        "permalink" => 'permalink: {
            if let Ok(field) = element.as_field() {
                if let Ok(result) = field.value() {
                    if let Some(value) = result {
                        match Permalink::new(&value) {
                            Ok(permalink) => local_options.permalink = Some(permalink),
                            Err(err) => {
                                let error = format!("There is a problem with the permalink '{value}': {err}");
                                element_error_with_snippet(element, &manifest_path, &error);
                            }
                        }
                    }

                    break 'permalink;
                }
            }

            let error = "permalink needs to be provided as a field with a value, e.g.: 'permalink: such-perma-wow'";
            element_error_with_snippet(element, &manifest_path, error);
        }
        _ => return false
    }

    true
}
