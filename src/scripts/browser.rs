// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use indoc::formatdoc;

use crate::{ArtistRc, Build, Catalog};
use crate::TRACK_NUMBERS;
use crate::util::url_safe_hash_base64;

use super::js_escape_inside_single_quoted_string;

const BROWSER_JS: &str = include_str!(env!("FAIRCAMP_BROWSER_JS"));
const BROWSER_JS_FILENAME: &str = "browser.js";

fn artist_js_object(artist: &ArtistRc) -> String {
    let artist_ref = artist.borrow();

    let mut artist_props = Vec::new();

    if let Some(url) = &artist_ref.external_page {
        artist_props.push(format!("externalPage:'{url}'"));
    }

    let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
    artist_props.push(format!("name:'{artist_name_escaped}'"));

    let artist_slug = &artist_ref.permalink.slug;
    artist_props.push(format!("url:'{artist_slug}/'"));

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
                    .map(artist_js_object)
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
                            .map(artist_js_object)
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

    let r_artists = if catalog.label_mode {
        catalog.featured_artists
            .iter()
            .filter(|artist| !artist.borrow().unlisted)
            .map(|artist| {
                let artist_ref = artist.borrow();

                let mut artist_props = Vec::new();

                if let Some(src) = artist_ref.thumbnail_image_src() {
                    artist_props.push(format!("image:'{src}'"));
                }

                let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
                artist_props.push(format!("name:'{artist_name_escaped}'"));

                let artist_slug = &artist_ref.permalink.slug;
                artist_props.push(format!("url:'{artist_slug}/'"));

                let artist_props_joined = artist_props.join(",");
                format!("{{{artist_props_joined}}}")
            })
            .collect::<Vec<String>>()
            .join(",")
    } else {
        String::new()
    };

    let label_mode_bool = if catalog.label_mode { "true" } else { "false" };

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
        const ARTISTS = [{r_artists}];
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
