// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use url::Url;

use crate::{Build, Catalog, Release};
use crate::icons;
use crate::render::{
    embed_layout,
    list_track_artists,
    player_icon_templates,
    Truncation
};
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

pub fn embed_release_html(
    base_url: &Url,
    build: &Build,
    catalog: &Catalog,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let varying_track_artists = release.varying_track_artists();

    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_number = index + 1;

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

                    let track_filename_urlencoded = urlencoding::encode(&track_filename);
                    let src = format!("{release_prefix}{format_dir}/{track_hash}/{track_filename_urlencoded}");

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
                            <a class="title" href="{track_number}{index_suffix}" title="{track_title_attribute_escaped}">{track_title_escaped}</a>{track_artists}
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

    let templates = player_icon_templates(build);

    let next_track_icon = icons::next_track(&build.locale.translations.next_track);
    let play_icon = icons::play(&build.locale.translations.play);
    let previous_track_icon = icons::previous_track(&build.locale.translations.previous_track);
    let volume_icon = icons::volume();
    let t_playback_position = &build.locale.translations.playback_position;
    let t_volume = &build.locale.translations.volume;
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
                <div class="volume">
                    <button>
                        {volume_icon}
                    </button>
                    <span class="slider">
                        <input aria-label="{t_volume}" aria-valuetext="" autocomplete="off" max="1" min="0" step="any" type="range" value="1">
                    </span>
                </div>
                <span class="track_info">
                    <span class="number"></span>
                    <span class="title_wrapper"></span>
                    <span class="time"></span>
                </span>
            </div>
        </div>
        {templates}
    "##);

    let release_slug = &release.permalink.slug;
    let link_url = base_url.join(&format!("{release_slug}{index_suffix}")).unwrap();

    embed_layout(
        root_prefix,
        &body,
        build,
        &link_url,
        &release.theme,
        &release.title
    )
}