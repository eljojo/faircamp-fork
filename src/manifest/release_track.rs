// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;

use crate::{
    Build,
    Cache,
    DescribedImage,
    LocalOptions
};

use super::{
    attribute_error_with_snippet,
    element_error_with_snippet
};

pub const RELEASE_TRACK_OPTIONS: &[&str] = &[
    "cover"
];

/// Try to read a single option from the passed element. Processes
/// options that are present in release and track manifests.
pub fn read_release_track_option(
    build: &mut Build,
    cache: &mut Cache,
    dir: &Path,
    element: &Box<dyn SectionElement>,
    local_options: &mut LocalOptions,
    manifest_path: &Path
) -> bool {
    match element.key() {
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
                                        let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                        build.error(&error);
                                    }

                                }
                            }
                            _ => {
                                let message = "The key/name of this attribute was not recognized, only 'description' and 'file' are recognized inside a cover field";
                                let error = element_error_with_snippet(element, manifest_path, message);
                                build.error(&error);
                            }
                        }
                    }

                    if let Some(path) = path_relative_to_catalog {
                        let image = cache.get_or_create_image(build, &path);
                        local_options.cover = Some(DescribedImage::new(description, image));
                    }

                    break 'cover;
                }
            }

            let message = "cover needs to be provided as a field with attributes, e.g.:\n\ncover:\ndescription = Alice, looking amused\nfile = alice.jpg";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        _ => return false
    }

    true
}
