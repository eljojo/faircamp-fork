// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use indoc::formatdoc;

use crate::{Artist, Build, Catalog};
use crate::TRACK_NUMBERS;
use crate::util::url_safe_hash_base64;

use super::js_escape_inside_single_quoted_string;

const BROWSER_JS: &str = include_str!(env!("FAIRCAMP_BROWSER_JS"));
const BROWSER_JS_FILENAME: &str = "browser.js";

// TODO: When an artist appears on multiple releases, we currently duplicate
// all artist data for each time they appear on an release, plus for the one
// time we list the artist on its own. This could be optimized using a global
// map of artists, whose entries we we merely reference from releases.

/// Regarding `include_image`: Images are included where we specify an artist
/// as a stand-alone entry, and they are not included when we specify an
/// artist as an artist on a specific release (there we obviously show the
/// release image and therefore don't need an artist image).
fn artist_js_object(artist: &Artist, include_image: bool) -> String {
    let mut artist_props = Vec::new();

    if let Some(url) = &artist.external_page {
        artist_props.push(format!("externalPage:'{url}'"));
    }

    if include_image {
        if let Some(src) = artist.thumbnail_image_src() {
            artist_props.push(format!("image:'{src}'"));
        }
    }

    let artist_name_escaped = js_escape_inside_single_quoted_string(&artist.name);
    artist_props.push(format!("name:'{artist_name_escaped}'"));

    // Client-side we construct the final artist image url by concatenating
    // the url + image props, therefore even if the artist has an external
    // page url we need to include the (internal) url in these cases.
    if include_image || !artist.external_page.is_some() {
        let artist_slug = &artist.permalink.slug;
        artist_props.push(format!("url:'{artist_slug}/'"));
    }

    let artist_props_joined = artist_props.join(",");
    format!("{{{artist_props_joined}}}")
}

pub fn generate_browser_js(build: &mut Build, catalog: &Catalog) {
    let mut releases_desc_by_date = catalog.public_releases();

    releases_desc_by_date.sort_by_key(|release| release.borrow().date);

    let r_releases = releases_desc_by_date
        .iter()
        .rev()
        .map(|release| {
            let release_ref = release.borrow();
            let release_slug = &release_ref.permalink.slug;

            let mut release_props = Vec::new();

            if catalog.label_mode {
                let artists_joined = release_ref.main_artists
                    .iter()
                    .map(|artist| artist_js_object(&artist.borrow(), false))
                    .collect::<Vec<String>>()
                    .join(",");

                release_props.push(format!("artists:[{artists_joined}]"));
            }

            if let Some(src) = release_ref.cover_160_filename() {
                release_props.push(format!("cover:'{src}'"));
            } else {
                let src = release_ref.procedural_cover_120_filename_unchecked();
                release_props.push(format!("coverProcedural:'{src}'"));
            }

            let release_title_escaped = js_escape_inside_single_quoted_string(&release_ref.title);
            release_props.push(format!("title:'{release_title_escaped}'"));

            let tracks = release_ref.tracks
                .iter()
                .zip(TRACK_NUMBERS)
                .map(|(track, track_number)| {
                    let mut track_props = Vec::new();

                    if catalog.label_mode {
                        let artists_joined = track.artists
                            .iter()
                            .map(|artist| artist_js_object(&artist.borrow(), false))
                            .collect::<Vec<String>>()
                            .join(",");

                        track_props.push(format!("artists:[{artists_joined}]"));
                    }

                    if let Some(src) = track.cover_160_filename() {
                        track_props.push(format!("cover:'{src}'"));
                    }

                    let track_number_formatted = release_ref.track_numbering.format(track_number);
                    track_props.push(format!("number:'{track_number_formatted}'"));

                    let track_title_escaped = js_escape_inside_single_quoted_string(&track.title());
                    track_props.push(format!("title:'{track_title_escaped}'"));

                    track_props.push(format!("url:'{release_slug}/{track_number}/'"));

                    let track_props_joined = track_props.join(",");
                    format!("{{{track_props_joined}}}")
                })
                .collect::<Vec<String>>()
                .join(",");

            release_props.push(format!("tracks:[{tracks}]"));

            release_props.push(format!("url:'{release_slug}/'"));

            let release_props_joined = release_props.join(",");
            format!("{{{release_props_joined}}}")
        })
        .collect::<Vec<String>>()
        .join(",");

    let artists_joined;
    let label_mode_bool;

    if catalog.label_mode {
        let mut artists = Vec::new();

        let mut public_artists_asc_by_name = catalog.public_artists();

        public_artists_asc_by_name.sort_by(|a, b| {
            // Eventually we should probably use proper icu/unicode-based
            // collation,(or for instance the `caseless` crate).
            let a_name_lowercase = a.borrow().name.to_lowercase();
            let b_name_lowercase = b.borrow().name.to_lowercase();

            a_name_lowercase.cmp(&b_name_lowercase)
        });

        for artist in public_artists_asc_by_name {
            let artist_ref = artist.borrow();

            if artist_ref.featured || artist_ref.external_page.is_some() {
                let artist_js = artist_js_object(&artist_ref, true);
                artists.push(artist_js);
            }
        }

        artists_joined = artists.join(",");

        label_mode_bool = "true";
    } else {
        artists_joined = String::new();
        label_mode_bool = "false";
    }

    let t_nothing_found_for_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.nothing_found_for_xxx);
    let t_showing_featured_items = &build.locale.translations.showing_featured_items;
    let t_showing_xxx_results_for_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.showing_xxx_results_for_xxx);
    let t_xxx_and_others = &build.locale.translations.xxx_and_others;
    let mut js = formatdoc!(r#"
        const BROWSER_JS_T = {{
            nothingFoundForXxx: query => '{t_nothing_found_for_xxx}'.replace('{{query}}',query),
            showingFeaturedItems: '{t_showing_featured_items}',
            showingXxxResultsForXxx: (count,query) => '{t_showing_xxx_results_for_xxx}'.replace('{{count}}',count).replace('{{query}}',query),
            xxxAndOthers: (xxx,othersLink) => '{t_xxx_and_others}'.replace('{{xxx}}',xxx).replace('{{others_link}}',othersLink)
        }};
        const LABEL_MODE = {label_mode_bool};
        const ARTISTS = [{artists_joined}];
        const RELEASES = [{r_releases}];
    "#);

    js.push_str(BROWSER_JS);

    build.asset_hashes.browser_js = Some(url_safe_hash_base64(&js));

    fs::write(
        build.build_dir.join(BROWSER_JS_FILENAME),
        js
    ).unwrap();

    build.reserve_filename(BROWSER_JS_FILENAME);
}
