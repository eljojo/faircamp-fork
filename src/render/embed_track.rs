// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use url::Url;

use crate::{Build, Release, Track};
use crate::icons;
use crate::render::{embed_layout, player_icon_templates};
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

pub fn embed_track_html(
    base_url: &Url,
    build: &Build,
    release: &Release,
    track: &Track,
    track_number: usize
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let audio_sources = release.streaming_quality
        .formats()
        .iter()
        .map(|format| {
            let format_dir = format.asset_dirname();
            let format_extension = format.extension();

            let track_filename = format!(
                "{basename}{format_extension}",
                basename = track.asset_basename.as_ref().unwrap()
            );

            let track_hash = build.hash_path_with_salt(
                &release.permalink.slug,
                format_dir,
                &track_filename
            );

            let source_type = format.source_type();
            let src = format!("{release_prefix}{format_dir}/{track_hash}/{track_filename}");

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
            <input aria-valuetext="" autocomplete="off" max="{track_duration_seconds}" min="0" step="any" type="range" value="0">
        </div>
    "#);

    let templates = player_icon_templates(build);

    let play_icon = icons::play(&build.locale.translations.play);
    let volume_icon = icons::volume();
    let t_dimmed = &build.locale.translations.dimmed;
    let t_muted = &build.locale.translations.muted;
    let body = formatdoc!(r##"
        {track_rendered}
        <div class="player">
            <div class="timeline">
                <input aria-valuetext="" autocomplete="off" max="" min="0" step="any" type="range" value="0">
                <div class="base"></div>
                <div class="progress" style="width: 0%;"></div>
            </div>
            <div class="elements">
                <button class="playback">
                    {play_icon}
                </button>
                <div class="volume">
                    <button>
                        {volume_icon}
                    </button>
                    <span class="slider">
                        <input aria-valuetext="" autocomplete="off" max="1" min="0" step="any" type="range" value="1">
                    </span>
                </div>
                <span class="track_info">
                    <span class="title_wrapper"></span>
                    <span class="time"></span>
                </span>
                <span class="volume_hint dimmed">{t_dimmed}</span>
                <span class="volume_hint muted">{t_muted}</span>
            </div>
        </div>
        {templates}
    "##);

    let release_slug = &release.permalink.slug;
    let link_url = base_url.join(&format!("{release_slug}/{track_number}{index_suffix}")).unwrap();

    embed_layout(
        root_prefix,
        &body,
        build,
        &link_url,
        &release.theme,
        &release.title
    )
}
