// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;
use url::Url;

use crate::{
    Artist,
    ArtistRc,
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
    catalog: &mut Catalog,
    element: &Box<dyn SectionElement>,
    manifest_path: &Path
) -> bool {
    match element.key() {
        "artist" => 'artist: {
            if let Ok(field) = element.as_field() {
                if let Ok(attributes) = field.attributes() {
                    let mut aliases = Vec::new();
                    let mut name = None;
                    let mut link = None;
                    let mut permalink = None;

                    for attribute in attributes {
                        match attribute.key() {
                            "alias" => {
                                if let Some(value) = attribute.value() {
                                    aliases.push(value.to_string());
                                }
                            }
                            "link" => {
                                if let Some(value) = attribute.value() {
                                    match Url::parse(value) {
                                        Ok(parsed_url) => link = Some(parsed_url),
                                        Err(err) => {
                                            let error = format!("The url supplied for the link seems to be malformed ({err})");
                                            attribute_error_with_snippet(attribute, manifest_path, &error);
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
                                            let error = format!("There is a problem with the permalink '{value}': {err}");
                                            attribute_error_with_snippet(attribute, manifest_path, &error);
                                        }
                                    }
                                }
                            }
                            other => {
                                let error = format!("The attribute '{other}' is not recognized here (supported attributes are 'alias', 'name' and 'link'");
                                attribute_error_with_snippet(attribute, manifest_path, &error);
                            }
                        }
                    }

                    if let Some(name) = name {
                        let artist = Artist::new_shortcut(
                            aliases,
                            catalog,
                            link,
                            &name,
                            permalink
                        );

                        catalog.artists.push(ArtistRc::new(artist));
                    } else {
                        let error = "The artist option must supply a name attribute at least, e.g.:\n\nartist:\nname = Alice";
                        element_error_with_snippet(element, manifest_path, error);
                    }

                    break 'artist;
                }
            }

            let error = "artist must be provided as a field with attributes, e.g.:\n\nartist:\nname = Alice\nlink = https://example.com\nalias = Älice\nalias = Älicë";
            element_error_with_snippet(element, manifest_path, error);
        }
        _ => return false
    }

    true
}
