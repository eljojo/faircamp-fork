// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use indoc::formatdoc;

use crate::{Build, Catalog, TRACK_NUMBERS};
use crate::util::url_safe_hash_base64;

const BROWSER_JS: &str = include_str!(env!("FAIRCAMP_BROWSER_JS"));
const BROWSER_JS_FILENAME: &str = "browser.js";

const CLIPBOARD_JS: &str = include_str!(env!("FAIRCAMP_CLIPBOARD_JS"));
const CLIPBOARD_JS_FILENAME: &str = "clipboard.js";

const EMBEDS_JS: &str = include_str!(env!("FAIRCAMP_EMBEDS_JS"));
const EMBEDS_JS_FILENAME: &str = "embeds.js";

const PLAYER_JS: &str = include_str!(env!("FAIRCAMP_PLAYER_JS"));
const PLAYER_JS_FILENAME: &str = "player.js";

pub fn generate(build: &mut Build, catalog: &Catalog) {
    generate_browser_js(build, catalog);
    generate_clipboard_js(build);
    generate_player_js(build);

    if build.embeds_requested {
        generate_embeds_js(build);
    }
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
                            .map(|artist| {
                                let artist_ref = artist.borrow();
                                let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
                                let artist_slug = &artist_ref.permalink.slug;
                                format!(r#"{{ name: '{artist_name_escaped}', url: '{artist_slug}/' }}"#)
                            })
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
                    .map(|artist| {
                        let artist_ref = artist.borrow();
                        let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
                        let artist_slug = &artist_ref.permalink.slug;
                        formatdoc!(r#"
                            {{
                                name: '{artist_name_escaped}',
                                url: '{artist_slug}/'
                            }}
                        "#)
                    })
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

        const ARTISTS = [
            {r_artists}
        ];

        const RELEASES = [
            {r_releases}
        ];
    "#);

    js.push_str(BROWSER_JS);

    build.asset_hashes.browser_js = Some(url_safe_hash_base64(&js));

    fs::write(
        build.build_dir.join(BROWSER_JS_FILENAME),
        js
    ).unwrap();

    build.reserve_filename(BROWSER_JS_FILENAME);
}

pub fn generate_clipboard_js(build: &mut Build) {
    fs::write(
        build.build_dir.join(CLIPBOARD_JS_FILENAME),
        CLIPBOARD_JS
    ).unwrap();

    build.reserve_filename(CLIPBOARD_JS_FILENAME);
}

pub fn generate_embeds_js(build: &mut Build) {
    let t_mute = &build.locale.translations.mute;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let t_xxx_hours = &build.locale.translations.xxx_hours;
    let t_xxx_minutes = &build.locale.translations.xxx_minutes;
    let t_xxx_seconds = &build.locale.translations.xxx_seconds;
    let mut js = formatdoc!("
        const EMBEDS_JS_T = {{
            mute: '{t_mute}',
            playbackPosition: '{t_playback_position}',
            unmute: '{t_unmute}',
            volume: '{t_volume}',
            xxxHours: hours => '{t_xxx_hours}'.replace('{{xxx}}', hours),
            xxxMinutes: minutes => '{t_xxx_minutes}'.replace('{{xxx}}', minutes),
            xxxSeconds: seconds => '{t_xxx_seconds}'.replace('{{xxx}}', seconds)
        }};
    ");

    js.push_str(EMBEDS_JS);

    build.asset_hashes.embeds_js = Some(url_safe_hash_base64(&js));

    fs::write(
        build.build_dir.join(EMBEDS_JS_FILENAME),
        js
    ).unwrap();

    build.reserve_filename(EMBEDS_JS_FILENAME);
}

pub fn generate_player_js(build: &mut Build) {
    let t_listen = &build.locale.translations.listen;
    let t_mute = &build.locale.translations.mute;
    let t_pause = &build.locale.translations.pause;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_player_closed = &build.locale.translations.player_closed;
    let t_player_open_playing_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.player_open_playing_xxx);
    let t_player_open_with_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.player_open_with_xxx);
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let t_xxx_hours = &build.locale.translations.xxx_hours;
    let t_xxx_minutes = &build.locale.translations.xxx_minutes;
    let t_xxx_seconds = &build.locale.translations.xxx_seconds;
    let mut js = formatdoc!("
        const PLAYER_JS_T = {{
            listen: '{t_listen}',
            mute: '{t_mute}',
            pause: '{t_pause}',
            playbackPosition: '{t_playback_position}',
            playerClosed: '{t_player_closed}',
            playerOpenPlayingXxx: title => '{t_player_open_playing_xxx}'.replace('{{title}}', title),
            playerOpenWithXxx: title => '{t_player_open_with_xxx}'.replace('{{title}}', title),
            unmute: '{t_unmute}',
            volume: '{t_volume}',
            xxxHours: hours => '{t_xxx_hours}'.replace('{{xxx}}', hours),
            xxxMinutes: minutes => '{t_xxx_minutes}'.replace('{{xxx}}', minutes),
            xxxSeconds: seconds => '{t_xxx_seconds}'.replace('{{xxx}}', seconds)
        }};
    ");

    js.push_str(PLAYER_JS);

    build.asset_hashes.player_js = Some(url_safe_hash_base64(&js));

    fs::write(
        build.build_dir.join(PLAYER_JS_FILENAME),
        js
    ).unwrap();

    build.reserve_filename(PLAYER_JS_FILENAME);
}

/// Escapes `'` as `\'` and `\` as `\\`
fn js_escape_inside_single_quoted_string(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
}
