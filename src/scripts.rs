// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::{Build, Catalog};

pub enum Scripts {
    Clipboard,
    ClipboardAndPlayer,
    None
}

impl Scripts {
    pub fn header_tags(&self, root_prefix: &str) -> String {
        let file_names = match self {
            Scripts::Clipboard => vec!["clipboard.js"],
            Scripts::ClipboardAndPlayer => vec!["clipboard.js", "player.js"],
            Scripts::None => vec![]
        };

        file_names
            .iter()
            .map(|file_name| format!(r#"<script defer src="{root_prefix}{file_name}"></script>"#))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn generate(build: &Build, catalog: &Catalog) {
    generate_browser_js(build, catalog);
    generate_clipboard_js(build);
    generate_player_js(build);

    if build.embeds_requested {
        generate_embeds_js(build);
    }
}

pub fn generate_browser_js(build: &Build, catalog: &Catalog) {
    let index_suffix = build.index_suffix();

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
                .enumerate()
                .map(|(index, track)| {
                    let track_number = index + 1;
                    let track_number_formatted = release_ref.track_numbering.format(track_number);
                    let track_title_escaped = js_escape_inside_single_quoted_string(&track.title());

                    let r_artists = if catalog.label_mode {
                        let joined = track.artists
                            .iter()
                            .map(|artist| {
                                let artist_ref = artist.borrow();
                                let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
                                let artist_slug = &artist_ref.permalink.slug;
                                format!(r#"{{ name: '{artist_name_escaped}', url: '{artist_slug}{index_suffix}' }}"#)
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
                            number: '{track_number_formatted}',
                            title: '{track_title_escaped}',
                            url: '{release_slug}/{track_number}{index_suffix}'
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
                        format!(r#"{{ name: '{artist_name_escaped}', url: '{artist_slug}{index_suffix}' }}"#)
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

            let r_cover = match release_ref.cover_image_micro_src() {
                Some(src) => format!("cover: '{src}',"),
                None => String::new()
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
                    url: '{release_slug}{index_suffix}'
                }}
            "#)
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let r_artists = match catalog.label_mode {
        true => {
            catalog.featured_artists
                .iter()
                .map(|artist| {
                    let artist_ref = artist.borrow();
                    let artist_name_escaped = js_escape_inside_single_quoted_string(&artist_ref.name);
                    let artist_slug = &artist_ref.permalink.slug;

                    formatdoc!(r#"
                        {{
                            name: '{artist_name_escaped}',
                            url: '{artist_slug}{index_suffix}'
                        }}
                    "#)
                })
                .collect::<Vec<String>>()
                .join(",\n")
        }
        false => String::new()
    };

    let t_nothing_found_for_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.nothing_found_for_xxx);
    let t_showing_featured_items = &build.locale.translations.showing_featured_items;
    let t_showing_xxx_results_for_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.showing_xxx_results_for_xxx);
    let mut js = formatdoc!(r#"
        const BROWSER_JS_T = {{
            nothingFoundForXxx: query => '{t_nothing_found_for_xxx}'.replace('{{query}}', query),
            showingFeaturedItems: '{t_showing_featured_items}',
            showingXxxResultsForXxx: (count, query) => '{t_showing_xxx_results_for_xxx}'.replace('{{count}}', count).replace('{{query}}', query)
        }};

        const ARTISTS = [
            {r_artists}
        ];

        const RELEASES = [
            {r_releases}
        ];
    "#);

    js.push_str(include_str!("assets/browser.js"));

    fs::write(build.build_dir.join("browser.js"), js).unwrap();
}

pub fn generate_clipboard_js(build: &Build) {
    let js = include_str!("assets/clipboard.js");
    fs::write(build.build_dir.join("clipboard.js"), js).unwrap();
}

pub fn generate_embeds_js(build: &Build) {
    let t_mute = &build.locale.translations.mute;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let mut js = formatdoc!("
        const EMBEDS_JS_T = {{
            mute: '{t_mute}',
            playbackPosition: '{t_playback_position}',
            unmute: '{t_unmute}',
            volume: '{t_volume}'
        }};
    ");

    js.push_str(include_str!("assets/embeds.js"));

    fs::write(build.build_dir.join("embeds.js"), js).unwrap();
}

pub fn generate_player_js(build: &Build) {
    let t_listen = &build.locale.translations.listen;
    let t_mute = &build.locale.translations.mute;
    let t_pause = &build.locale.translations.pause;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_player_closed = &build.locale.translations.player_closed;
    let t_player_open_playing_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.player_open_playing_xxx);
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let mut js = formatdoc!("
        const PLAYER_JS_T = {{
            listen: '{t_listen}',
            mute: '{t_mute}',
            pause: '{t_pause}',
            playbackPosition: '{t_playback_position}',
            playerClosed: '{t_player_closed}',
            playerOpenPlayingXxx: title => '{t_player_open_playing_xxx}'.replace('{{title}}', title),
            unmute: '{t_unmute}',
            volume: '{t_volume}'
        }};
    ");

    js.push_str(include_str!("assets/player.js"));

    fs::write(build.build_dir.join("player.js"), js).unwrap();
}

/// Escapes `'` as `\'` and `\` as `\\`
fn js_escape_inside_single_quoted_string(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
}
