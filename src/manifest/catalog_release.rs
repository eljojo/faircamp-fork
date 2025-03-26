// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;
use url::Url;

use crate::{
    Artist,
    ArtistRc,
    Build,
    Catalog,
    Permalink
};

use super::{
    attribute_error_with_snippet,
    element_error_with_snippet
};

pub const CATALOG_RELEASE_OPTIONS: &[&str] = &[
    "artist"
];

/// Try to read a single option from the passed element. Processes
/// options that are present in catalog and release manifests.
pub fn read_catalog_release_option(
    build: &mut Build,
    catalog: &mut Catalog,
    element: &Box<dyn SectionElement>,
    manifest_path: &Path
) -> bool {
    match element.key() {
        "artist" => 'artist: {
            if let Ok(field) = element.as_field() {
                if let Ok(attributes) = field.attributes() {
                    let mut aliases = Vec::new();
                    let mut external_page = None;
                    let mut name = None;
                    let mut permalink = None;

                    for attribute in attributes {
                        match attribute.key() {
                            "alias" => {
                                if let Some(value) = attribute.value() {
                                    aliases.push(value.to_string());
                                }
                            }
                            "external_page" => {
                                if let Some(value) = attribute.value() {
                                    match Url::parse(value) {
                                        Ok(_) => external_page = Some(value.to_string()),
                                        Err(err) => {
                                            let message = format!("The url supplied for the external_page option seems to be malformed ({err})");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            "name" => {
                                if let Some(value) = attribute.value() {
                                    name = Some(value.to_string());
                                }
                            }
                            "permalink" => {
                                if let Some(value) = attribute.value() {
                                    match Permalink::new(value) {
                                        Ok(custom_permalink) => permalink = Some(custom_permalink),
                                        Err(err) => {
                                            let message = format!("There is a problem with the permalink '{value}': {err}");
                                            let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                            build.error(&error);
                                        }
                                    }
                                }
                            }
                            other => {
                                let message = format!("The attribute '{other}' is not recognized here (supported attributes are 'alias', 'name' and 'link'");
                                let error = attribute_error_with_snippet(attribute, manifest_path, &message);
                                build.error(&error);
                            }
                        }
                    }

                    if let Some(name) = name {
                        let artist = Artist::new_shortcut(
                            aliases,
                            catalog,
                            external_page,
                            &name,
                            permalink
                        );

                        catalog.artists.push(ArtistRc::new(artist));
                    } else {
                        let message = "The artist option must supply a name attribute at least, e.g.:\n\nartist:\nname = Alice";
                        let error = element_error_with_snippet(element, manifest_path, message);
                        build.error(&error);
                    }

                    break 'artist;
                }
            }

            let message = "artist must be provided as a field with attributes, e.g.:\n\nartist:\nname = Alice\nlink = https://example.com\nalias = Älice\nalias = Älicë";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        _ => return false
    }

    true
}
