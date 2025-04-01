// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use indoc::formatdoc;

use crate::{
    Build,
    Release,
    SiteUrl,
    Track
};
use crate::icons;
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

use super::SPEED_CONTROLS;
use super::EmbedLayout;
use super::volume_controls;

pub fn track_embed_html(
    base_url: &SiteUrl,
    build: &Build,
    release: &Release,
    track: &Track,
    track_number: usize
) -> String {
    let release_prefix = "../../";
    let root_prefix = "../../../";
    let translations = &build.locale.translations;

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

            let source_type = format.source_type();
            let src = format!("{release_prefix}{track_number}/{format_dir}/{track_hash}/{track_filename}");

            format!(r#"<source src="{src}" type="{source_type}">"#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let track_title = track.title();
    let track_duration_seconds = track.transcodes.borrow().source_meta.duration_seconds;
    let track_title_attribute_escaped = html_escape_inside_attribute(&track_title);
    let track_title_escaped = html_escape_outside_attribute(&track_title);

    let track_rendered = formatdoc!(r#"
        <div class="track" data-duration="{track_duration_seconds}">
            <span class="track_header">
                <span class="title" title="{track_title_attribute_escaped}">{track_title_escaped}</span>
            </span>
            <audio controls preload="none">
                {audio_sources}
            </audio>
            <input autocomplete="off" max="{track_duration_seconds}" min="0" step="any" type="range" value="0">
        </div>
    "#);

    let speed_controls = if track.speed_controls { SPEED_CONTROLS } else { "" };
    let r_volume_controls = volume_controls(translations);

    let play_icon = icons::play(&translations.play);
    let t_playback_position = &translations.playback_position;
    let body = formatdoc!(r##"
        {track_rendered}
        <div class="player">
            <div class="timeline">
                <input aria-label="{t_playback_position}" aria-valuetext="" autocomplete="off" max="" min="0" step="any" type="range" value="0">
                <div class="base"></div>
                <div class="progress" style="width: 0%;"></div>
            </div>
            <div class="elements">
                <button class="playback">
                    {play_icon}
                </button>
                {speed_controls}
                {r_volume_controls}
                <span class="track_info">
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
    let link_url = base_url.join_index(build, format!("{release_slug}/{track_number}"));

    EmbedLayout::render(
        &body,
        build,
        &link_url,
        root_prefix,
        &release.theme,
        &release.title
    )
}
