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
    let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
    let artist_slug = &artist_ref.permalink.slug;

    let mut external_page = String::new();
    if let Some(url) = &artist_ref.external_page {
        external_page.push_str(&format!("externalPage:'{url}',"));
    }

    format!("{{{external_page}name:'{artist_name_escaped}',url:'{artist_slug}/'}}")
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

            let r_tracks = release_ref.tracks
                .iter()
                .zip(TRACK_NUMBERS)
                .map(|(track, track_number)| {
                    let track_number_formatted = release_ref.track_numbering.format(track_number);

                    let r_cover = if let Some(src) = track.cover_160_filename() {
                        format!("cover: '{src}',")
                    } else {
                        String::new()
                    };

                    let track_title_escaped = js_escape_inside_single_quoted_string(&track.title());

                    let r_artists = if catalog.label_mode {
                        let joined = track.artists
                            .iter()
                            .map(artist_js_object)
                            .collect::<Vec<String>>()
                            .join(",\n");

                        format!("artists:[{joined}],")
                    } else {
                        String::new()
                    };

                    formatdoc!(r#"
                        {{
                            {r_artists}
                            {r_cover}
                            number: '{track_number_formatted}',
                            title: '{track_title_escaped}',
                            url: '{release_slug}/{track_number}/'
                        }}
                    "#)
                })
                .collect::<Vec<String>>()
                .join(",\n");

            let r_artists = if catalog.label_mode {
                let joined = release_ref.main_artists
                    .iter()
                    .map(artist_js_object)
                    .collect::<Vec<String>>()
                    .join(",\n");

                formatdoc!(r#"
                    artists: [
                        {joined}
                    ],
                "#)
            } else {
                String::new()
            };

            let r_cover = if let Some(src) = release_ref.cover_160_filename() {
                format!("cover: '{src}',")
            } else {
                let src = release_ref.procedural_cover_120_filename_unchecked();
                format!("coverProcedural: '{src}',")
            };
            let release_title_escaped = js_escape_inside_single_quoted_string(&release_ref.title);

            formatdoc!(r#"
                {{
                    {r_artists}
                    {r_cover}
                    title: '{release_title_escaped}',
                    tracks: [
                        {r_tracks}
                    ],
                    url: '{release_slug}/'
                }}
            "#)
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let r_artists = match catalog.label_mode {
        true => {
            catalog.featured_artists
                .iter()
                .filter(|artist| !artist.borrow().unlisted)
                .map(|artist| {
                    let artist_ref = artist.borrow();
                    let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
                    let artist_slug = &artist_ref.permalink.slug;

                    let image = if let Some(src) = artist_ref.thumbnail_image_src() {
                        format!("image: '{src}',")
                    } else {
                        String::new()
                    };

                    formatdoc!(r#"
                        {{
                            {image}
                            name: '{artist_name_escaped}',
                            url: '{artist_slug}/'
                        }}
                    "#)
                })
                .collect::<Vec<String>>()
                .join(",\n")
        }
        false => String::new()
    };

    let label_mode_bool = if catalog.label_mode { "true" } else { "false" };

    let t_nothing_found_for_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.nothing_found_for_xxx);
    let t_showing_featured_items = &build.locale.translations.showing_featured_items;
    let t_showing_xxx_results_for_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.showing_xxx_results_for_xxx);
    let t_xxx_and_others = &build.locale.translations.xxx_and_others;
    let mut js = formatdoc!(r#"
        const BROWSER_JS_T = {{
            nothingFoundForXxx: query => '{t_nothing_found_for_xxx}'.replace('{{query}}', query),
            showingFeaturedItems: '{t_showing_featured_items}',
            showingXxxResultsForXxx: (count, query) => '{t_showing_xxx_results_for_xxx}'.replace('{{count}}', count).replace('{{query}}', query),
            xxxAndOthers: (xxx, othersLink) => '{t_xxx_and_others}'.replace('{{xxx}}', xxx).replace('{{others_link}}', othersLink)
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
