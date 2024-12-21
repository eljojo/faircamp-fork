// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::SectionElement;

use crate::Build;

use super::element_error_with_snippet;

// TODO: Occasionally remove some of these, ideally at some point remove the
// whole thing when everything becomes completely stable.
// (rough timeline: 0.16.0 was released in summer 2024, 1.0 in december 2024)
pub fn read_obsolete_option(
    build: &Build,
    element: &Box<dyn SectionElement>,
    manifest_path: &Path
) -> bool {

    // IMPORTANT:
    // Make sure that this does not "over-consume"!
    // Only elements that are clearly obsolete (e.g. because it's a section,
    // or because the key is not in use anymore _at all_ can be matched here,
    // everything else needs to be ignored, else it shadows real fields that
    // should be regularly processed)

    match element.key() {
        "artist" if element.is_field() && element.as_field().unwrap().has_value() => {
            let error = "Since faircamp 1.0, the original 'artist' field (used to set the artist of a release) has been renamed to 'release_artist'. If you meant to use the new 'artist' field (which is a short-hand for defining an artist) you need to use a field with attributes, e.g.:\n\nartist:\nname = Alice\nlink = https://example.com\nalias = Älice\nalias = Älicë";
            element_error_with_snippet(element, manifest_path, error);
        }
        "artist" if element.is_section() => {
            if manifest_path.ends_with("artist.eno") {
                let error = "Since faircamp 1.0, '# artist' sections are not required anymore - just remove the line '# artist'";
                element_error_with_snippet(element, manifest_path, error);
            } else {
                let error = "Since faircamp 1.0, '# artist' sections are not used anymore. Remove the line '# artist', and move all options below to a file called 'artist.eno', inside a separate directory dedicated to the artist only";
                element_error_with_snippet(element, manifest_path, error);
            }
        }
        "artists" if element.is_field() => {
            let error = "Since faircamp 1.0, the 'artists' field (used to set the artists of a release) has been renamed to 'release_artists'.";
            element_error_with_snippet(element, manifest_path, error);
        }
        "cache" if element.is_section() => {
            let error = r##"Since faircamp 0.16.0, the "# cache ... " section was merged into the catalog manifest as the "cache_optimization: delayed|immediate|wipe|manual" option, please move and adapt the current definition accordingly."##;
            element_error_with_snippet(element, manifest_path, error);
        }
        "catalog" if element.is_section() => {
            if manifest_path.ends_with("catalog.eno") && manifest_path.parent().unwrap() == build.catalog_dir {
                let error = "Since faircamp 1.0, '# catalog' sections are not required anymore - just remove the line '# catalog'";
                element_error_with_snippet(element, manifest_path, error);
            } else {
                let error = "Since faircamp 1.0, '# catalog' sections are not used anymore. Remove the line '# catalog', and move all options below to a file called 'catalog.eno' in the catalog root folder";
                element_error_with_snippet(element, manifest_path, error);
            }
        }
        "download" if element.is_section() => {
            let error = "Since faircamp 1.0, the '# download' section is obsolete and its options can/must now be put directly into the 'catalog.eno' and 'release.eno' files, please move and adapt the current options accordingly.";
            element_error_with_snippet(element, manifest_path, error);
        }
        "embedding" if element.is_section() => {
            let error = "Since faircamp 1.0 the embedding option must be specified as 'embedding: enabled|disabled' inside an 'artist.eno', 'catalog.eno' or 'release.eno' manifest, please move and adapt the current definiton accordingly.";
            element_error_with_snippet(element, manifest_path, error);
        }
        "link_brightness" => {
            let error = "Since faircamp 0.16.0, theming works differently and the link_brightness setting needs to be replaced (the dynamic_range attribute in the theme field is somewhat related in function now)";
            element_error_with_snippet(element, manifest_path, error);
        }
        "link_hue" => {
            let error = "Since faircamp 0.16.0, theming works differently and the link_hue setting needs to be replaced (the base_hue attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, manifest_path, error);
        }
        "link_saturation" => {
            let error = "Since faircamp 0.16.0, theming works differently and the link_saturation setting needs to be replaced (the base_chroma attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, manifest_path, error);
        }
        "localization" if element.is_section() => {
            let error = "Since faircamp 0.16.0, specify the language directly in the 'catalog.eno' manifest using e.g. 'language: fr' (the writing direction is determined from language automatically now). The localization section must be removed, it's not supported anymore.";
            element_error_with_snippet(element, manifest_path, error);
        }
        "payment" if element.is_section() => {
            let error = "Since faircamp 1.0, specify payment options directly in an artist.eno, catalog.eno or release.eno manifest using the single 'payment_info' field. The payment section is not supported anymore.";
            element_error_with_snippet(element, manifest_path, error);
        }
        "release" if element.is_section() => {
            if manifest_path.ends_with("release.eno") {
                let error = "Since faircamp 1.0, '# release' sections are not required anymore - just remove the line '# release'";
                element_error_with_snippet(element, manifest_path, error);
            } else {
                let error = "Since faircamp 1.0, '# release' sections are not used anymore. Remove the line '# release', and move all options below to a file called 'release.eno'";
                element_error_with_snippet(element, manifest_path, error);
            }
        }
        "rewrite_tags" => {
            let error = "Since faircamp 1.0, 'rewrite_tags: no' must be specified as 'tags: copy', 'rewrite_tags: yes' as 'tags: normalize'";
            element_error_with_snippet(element, manifest_path, error);
        }
        "streaming" if element.is_section() => {
            let error = r##"Since faircamp 0.16.0, the "# streaming" section has been merged directly into the catalog/release manifests as the 'streaming_quality: frugal|standard' option, please adapt and move the setting accordingly."##;
            element_error_with_snippet(element, manifest_path, error);
        }
        "text" => {
            let error = "Since faircamp 1.0, the name of the 'text' option has changed to 'more'.";
            element_error_with_snippet(element, manifest_path, error);
        }
        "text_hue" => {
            let error = "Since faircamp 0.16.0, theming works differently and the text_hue setting needs to be replaced (the base_hue attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, manifest_path, error);
        }
        "tint_back" => {
            let error = "Since faircamp 0.16.0, theming works differently and the tint_back setting needs to be replaced (the base_chroma attribute in the theme field is the closest alternative)";
            element_error_with_snippet(element, manifest_path, error);
        }
        "tint_front" => {
            let error = "Since faircamp 0.16.0, theming works differently and the tint_front setting needs to be replaced (the base_chroma and dynamic_range attributes in the theme field in combination serve a similar purpose)";
            element_error_with_snippet(element, manifest_path, error);
        }
        _ => return false
    }

    true
}
