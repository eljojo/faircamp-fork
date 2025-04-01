// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    Release,
    SiteUrl,
    TRACK_NUMBERS
};
use crate::icons;
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

use super::SPEED_CONTROLS;
use super::{EmbedLayout, Truncation};
use super::{list_track_artists, volume_controls};

pub fn release_embed_html(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";
    let translations = &build.locale.translations;

    let varying_track_artists = release.varying_track_artists();

    let tracks_rendered = release.tracks
        .iter()
        .zip(TRACK_NUMBERS)
        .map(|(track, track_number)| {
            let audio_sources = track.streaming_quality
                .formats()
                .iter()
                .map(|format| {
                    let format_dir = format.asset_dirname();
                    let format_extension = format.extension();

                    let track_filename = format!(
                        "{basename}{format_extension}",
                        basename = track.asset_basename.as_ref().unwrap()
                    );

                    let track_hash = build.hash_with_salt(|hasher| {
                        release.permalink.slug.hash(hasher);
                        track_number.hash(hasher);
                        format_dir.hash(hasher);
                        track_filename.hash(hasher);
                    });

                    let track_filename_urlencoded = urlencoding::encode(&track_filename);
                    let src = format!("{release_prefix}{track_number}/{format_dir}/{track_hash}/{track_filename_urlencoded}");

                    let source_type = format.source_type();
                    format!(r#"<source src="{src}" type="{source_type}">"#)
                })
                .collect::<Vec<String>>()
                .join("\n");

            let track_title = track.title();

            let track_duration_seconds = track.transcodes.borrow().source_meta.duration_seconds;
            let track_number_formatted = release.track_numbering.format(track_number);
            let track_title_escaped = html_escape_outside_attribute(&track_title);
            let track_title_attribute_escaped = html_escape_inside_attribute(&track_title);

            let track_artists = match varying_track_artists {
                true => {
                    let artists_truncation = Truncation::Truncate {
                        max_chars: 80,
                        others_link: format!("{track_number}/")
                    };
                    let artists_truncated = list_track_artists(build, index_suffix, root_prefix, catalog, artists_truncation, track);
                    format!(r#"&nbsp;&nbsp;/&nbsp;&nbsp;<span class="artists">{artists_truncated}</span>"#)
                }
                false => String::new()
            };

            formatdoc!(r#"
                <div class="track" data-duration="{track_duration_seconds}">
                    <div class="track_header">
                        <span class="number">{track_number_formatted}</span>
                        <span>
                            <a class="title" href="{release_prefix}{track_number}{index_suffix}" target="_parent" title="{track_title_attribute_escaped}">{track_title_escaped}</a>{track_artists}
                        </span>
                    </div>
                    <audio controls preload="none">
                        {audio_sources}
                    </audio>
                    <input autocomplete="off" max="{track_duration_seconds}" min="0" step="any" type="range" value="0">
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let tall = if release.varying_track_artists() { "tall" } else { "" };

    let speed_controls = if release.speed_controls { SPEED_CONTROLS } else { "" };
    let r_volume_controls = volume_controls(translations);

    let next_track_icon = icons::next_track(&translations.next_track);
    let play_icon = icons::play(&translations.play);
    let previous_track_icon = icons::previous_track(&translations.previous_track);
    let t_playback_position = &translations.playback_position;
    let body = formatdoc!(r##"
        {tracks_rendered}
        <div class="player {tall}">
            <div class="timeline">
                <input aria-label="{t_playback_position}" aria-valuetext="" autocomplete="off" max="" min="0" step="any" type="range" value="0">
                <div class="base"></div>
                <div class="progress" style="width: 0%;"></div>
            </div>
            <div class="elements">
                <button class="previous_track" disabled>
                    {previous_track_icon}
                </button>
                <button class="playback">
                    {play_icon}
                </button>
                <button class="next_track">
                    {next_track_icon}
                </button>
                {speed_controls}
                {r_volume_controls}
                <span class="track_info">
                    <span class="number"></span>
                    <span class="title_wrapper"></span>
                </span>
                <span class="time">
                    <span class="current"></span>
                    <span>/</span>
                    <span class="total"></span>
                </span>
            </div>
        </div>
    "##);

    let release_slug = &release.permalink.slug;
    let link_url = base_url.join_index(build, release_slug);

    EmbedLayout::render(
        &body,
        build,
        &link_url,
        root_prefix,
        &release.theme,
        &release.title
    )
}