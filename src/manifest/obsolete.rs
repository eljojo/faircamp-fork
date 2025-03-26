// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use enolib::{Attribute, SectionElement};
use indoc::indoc;

use crate::Build;

use super::{
    attribute_error_with_snippet,
    element_error_with_snippet
};

// TODO: Occasionally remove some of these, ideally at some point remove the
// whole thing when everything becomes completely stable.
// (rough timeline: 0.16.0 was released in summer 2024, 1.0 in december 2024)

pub fn read_obsolete_option(
    build: &mut Build,
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
            let message = "Since faircamp 1.0, the original 'artist' field (used to set the artist of a release) has been renamed to 'release_artist'. If you meant to use the new 'artist' field (which is a short-hand for defining an artist) you need to use a field with attributes, e.g.:\n\nartist:\nname = Alice\nlink = https://example.com\nalias = Älice\nalias = Älicë";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "artist" if element.is_section() => {
            if manifest_path.ends_with("artist.eno") {
                let message = "Since faircamp 1.0, '# artist' sections are not required anymore - just remove the line '# artist'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            } else {
                let message = indoc!("
                    Since faircamp 1.0, '# artist' sections are not used anymore. Remove the line '# artist' and pick one of these two options:

                    1) If you only need to set the name, permalink, and alias(es) you can use the new artist field inside a 'catalog.eno' or 'release.eno' manifest file (this only defines the artist, assigning an artist to a release happens with the 'release_artist(s)' field now):

                      artist:
                      name = Alice
                      permalink = alice
                      alias = alice
                      alias = Älicë

                    2) For a full-fledged artist definition (including image, long text etc.), move all options you had inside the '# artist' section to a file called 'artist.eno', inside a separate directory dedicated to the artist only.
                ");
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
        }
        "artists" if element.is_field() => {
            let message = "Since faircamp 1.0, the 'artists' field (used to set the artists of a release) has been renamed to 'release_artists'.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "cache" if element.is_section() => {
            let message = r##"Since faircamp 0.16.0, the "# cache ... " section was merged into the catalog manifest as the "cache_optimization: delayed|immediate|wipe|manual" option, please move and adapt the current definition accordingly."##;
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "catalog" if element.is_section() => {
            if manifest_path.ends_with("catalog.eno") && manifest_path.parent().unwrap() == build.catalog_dir {
                let message = "Since faircamp 1.0, '# catalog' sections are not required anymore - just remove the line '# catalog'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            } else {
                let message = "Since faircamp 1.0, '# catalog' sections are not used anymore. Remove the line '# catalog', and move all options below to a file called 'catalog.eno' in the catalog root folder";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
        }
        "code" |
        "codes" => {
            let message = "Since faircamp 1.0, the 'code' and 'codes' fields have been renamed to 'download_code' and 'download_codes'.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "download" if element.is_section() => {
            let message = indoc!("
                Since faircamp 1.0, the '# download' section is obsolete - its options have changed a bit and are now put directly into the 'catalog.eno' and 'release.eno' manifest files. Some examples follow:

                *) Free downloads in mp3 and opus (for free downloads you only need to set the download formats!):

                  release_downloads:
                  - mp3
                  - opus

                *) Flac downloads protected with a download code:

                  release_downloads: flac
                  downloads: code
                  download_code: thesecret
                  -- unlock_info
                  Get a download code at my [shop](https://example.com)
                  -- unlock_info

                *) Mp3 and flac downloads with a paycurtain:

                  release_downloads:
                  - mp3
                  - flac
                  downloads: paycurtain
                  price: USD 0+
                  -- payment_info
                  Pay via my [donations page](https://example.com)
                  -- payment_info

                *) Enabling single file downloads (this is now fine-controlled by setting formats explicitly):

                  track_downloads:
                  - mp3
                  - flac

                *) External downloads:

                  downloads: https://example.com

                *) Disabled downloads (only ever needed when you somehow enabled them in a parent manifest):

                  downloads: disabled
            ");
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "embedding" if element.is_section() => {
            let message = "Since faircamp 1.0 the embedding option must be specified as 'embedding: enabled|disabled' inside an 'artist.eno', 'catalog.eno', 'release.eno' or 'track.eno' manifest, please move and adapt the current definiton accordingly.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "external" => {
            let message = "Since faircamp 1.0, external download options such as 'external: https://example.com' are instead specified as 'downloads: https://example.com'.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "format" |
        "formats" => {
            let message = "Since faircamp 1.0, the 'format' and 'formats' fields have been renamed to 'release_downloads' (respectively 'track_downloads' to now separately specify the format(s) for single track downloads).";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "free" => {
            let message = "Since faircamp 1.0, free downloads have become the default (but explicit declaration is still possible with 'downloads: free') - you now only need to set download formats with 'release_downloads' (replaces the previous 'format(s)' option) or 'track_downloads' (now enables specific format choices for single track downloads) to enable free downloads.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "include_extras" => {
            let message = "Since faircamp 1.0, 'include_extras: no' is now specified as 'extra_downloads: disabled', 'include_extras: yes' as 'extra_downloads: bundled' (the default) and there's also an additional 'extra_downloads: separate' option, as well as the possiblity for enabling both, like this:\nextra_downloads:\n- bundled\n- separate.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "link_brightness" => {
            let message = "Since faircamp 0.16.0, theming works differently and the link_brightness setting needs to be replaced (the dynamic_range attribute in the theme field is somewhat related in function now)";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "link_hue" => {
            let message = "Since faircamp 0.16.0, theming works differently and the link_hue setting needs to be replaced (the base_hue attribute in the theme field is the closest alternative)";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "link_saturation" => {
            let message = "Since faircamp 0.16.0, theming works differently and the link_saturation setting needs to be replaced (the base_chroma attribute in the theme field is the closest alternative)";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "localization" if element.is_section() => {
            let message = "Since faircamp 0.16.0, specify the language directly in the 'catalog.eno' manifest using e.g. 'language: fr' (the writing direction is determined from language automatically now). The localization section must be removed, it's not supported anymore.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "payment" if element.is_section() => {
            let message = indoc!("
                Since faircamp 1.0, specify payment options directly in an artist.eno, catalog.eno or release.eno manifest using the single 'payment_info' field. Example with context:

                  downloads: paycurtain
                  price: EUR 4+
                  release_downloads:
                  - mp3
                  - flac

                  -- payment_info
                  Pay via my [donations page](https://example.com)
                  -- payment_info
            ");
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "payment_text" => {
            let message = "Since faircamp 1.0, the name of the 'payment_text' option has changed to 'payment_info'.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "release" if element.is_section() => {
            if manifest_path.ends_with("release.eno") {
                let message = "Since faircamp 1.0, '# release' sections are not required anymore - just remove the line '# release'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            } else {
                let message = "Since faircamp 1.0, '# release' sections are not used anymore. Remove the line '# release', and move all options below to a file called 'release.eno'";
                let error = element_error_with_snippet(element, manifest_path, message);
                build.error(&error);
            }
        }
        "rewrite_tags" => {
            let message = "Since faircamp 1.0, 'rewrite_tags: no' must be specified as 'tags: copy', 'rewrite_tags: yes' as 'tags: normalize'";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "round_corners" => {
            let message = "Since faircamp 1.0, 'round_corners' must be specified inside a theme field as an attribute with the value 'enabled', e.g.:'\ntheme:\nround_corners = enabled";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "single_files" => {
            let message = "Since faircamp 1.0, the 'single_files' option has been removed, instead you now can use the 'track_downloads' option to directly enable/pick the formats in which you want to offer single file downloads, e.g. 'track_downloads: mp3' or for multiple:\ntrack_downloads:\n- flac\n- mp3\n- opus";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "streaming" if element.is_section() => {
            let message = r##"Since faircamp 0.16.0, the "# streaming" section has been merged directly into the catalog/release manifests as the 'streaming_quality: frugal|standard' option, please adapt and move the setting accordingly."##;
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "text" => {
            let message = "Since faircamp 1.0, the name of the 'text' option has changed to 'more'.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "text_hue" => {
            let message = "Since faircamp 0.16.0, theming works differently and the text_hue setting needs to be replaced (the base_hue attribute in the theme field is the closest alternative)";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "theme" if element.is_section() => {
            let message = "Since faircamp 1.0, the \"# theme\" section needs to be provided as a field with attributes, e.g:\n\ntheme:\naccent_chroma = 50\nbackground_image = example.jpg\nbase = light\ndynamic_range = 13\nround_corners = enabled";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "tint_back" => {
            let message = "Since faircamp 0.16.0, theming works differently and the tint_back setting needs to be replaced (the base_chroma attribute in the theme field is the closest alternative)";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "tint_front" => {
            let message = "Since faircamp 0.16.0, theming works differently and the tint_front setting needs to be replaced (the base_chroma and dynamic_range attributes in the theme field in combination serve a similar purpose)";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        "unlock_text" => {
            let message = "Since faircamp 1.0, the name of the 'unlock_text' option has changed to 'unlock_info'.";
            let error = element_error_with_snippet(element, manifest_path, message);
            build.error(&error);
        }
        _ => return false
    }

    true
}

pub fn read_obsolete_theme_attribute(
    build: &mut Build,
    attribute: &Attribute,
    manifest_path: &Path
) -> bool {

    // IMPORTANT:
    // Make sure that this does not "over-consume"!
    // Only elements that are clearly obsolete (e.g. because it's a section,
    // or because the key is not in use anymore _at all_ can be matched here,
    // everything else needs to be ignored, else it shadows real fields that
    // should be regularly processed)

    match attribute.key() {
        "link_brightness" => {
            let message = "Since faircamp 0.16.0, theming works differently and the link_brightness setting needs to be replaced (the dynamic_range attribute is somewhat similar in function now)";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        "link_hue" => {
            let message = "Since faircamp 0.16.0, theming works differently and the link_hue setting needs to be replaced (the base_hue attribute is the closest alternative)";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        "link_saturation" => {
            let message = "Since faircamp 0.16.0, theming works differently and the link_saturation setting needs to be replaced (the base_chroma attribute is the closest alternative)";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        "round_corners" if attribute.value().is_none() => {
            let message = "Since faircamp 1.0, 'round_corners' must be specified with the value 'enabled', e.g.:'\ntheme:\nround_corners = enabled";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        "text_hue" => {
            let message = "Since faircamp 0.16.0, theming works differently and the text_hue setting needs to be replaced (the base_hue attribute is the closest alternative)";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        "tint_back" => {
            let message = "Since faircamp 0.16.0, theming works differently and the tint_back setting needs to be replaced (the base_chroma attribute is the closest alternative)";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        "tint_front" => {
            let message = "Since faircamp 0.16.0, theming works differently and the tint_front setting needs to be replaced (the base_chroma and dynamic_range attributes in combination serve a similar purpose)";
            let error = attribute_error_with_snippet(attribute, manifest_path, message);
            build.error(&error);
        }
        _ => return false
    }

    true
}
