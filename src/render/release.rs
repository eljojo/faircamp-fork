// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use indoc::formatdoc;

use crate::{M3U_PLAYLIST_FILENAME, TRACK_NUMBERS};
use crate::{
    Build,
    Catalog,
    DownloadAccess,
    OpenGraphMeta,
    Release
};
use crate::icons;
use crate::util::{format_time, html_escape_outside_attribute};

use super::SPEED_CONTROLS;
use super::{Layout, Truncation};
use super::{
    copy_button,
    list_release_artists,
    list_track_artists,
    release_cover_image,
    unlisted_badge,
    volume_controls,
    waveform
};

/// The actual release page, featuring the track listing and streaming player, links
/// to downloads, embeds, description, etc.
pub fn release_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";
    let translations = &build.locale.translations;

    let mut layout = Layout::new();

    layout.add_player_script();

    let download_link = match &release.download_access {
        DownloadAccess::Code { .. } => {
            if release.download_assets_available() {
                let t_unlock_permalink = &translations.unlock_permalink;
                let page_hash = build.hash_with_salt(|hasher| {
                    release.permalink.slug.hash(hasher);
                    t_unlock_permalink.hash(hasher);
                });

                let unlock_icon = icons::unlock(&translations.unlock);
                let t_download = &translations.download;

                formatdoc!(r#"
                    <a href="{t_unlock_permalink}/{page_hash}{index_suffix}">
                        {unlock_icon}
                        <span>{t_download}</span>
                    </a>
                "#)
            } else {
                String::new()
            }
        }
        DownloadAccess::Disabled => String::new(),
        DownloadAccess::External { link } => {
            let external_icon = icons::external(&translations.external_link);
            let t_download = &translations.download;
            formatdoc!(r#"
                <a href="{link}" target="_blank">
                    {external_icon}
                    <span>{t_download}</span>
                </a>
            "#)
        }
        DownloadAccess::Free => {
            if release.download_assets_available() {
                let t_downloads_permalink = &translations.downloads_permalink;
                let page_hash = build.hash_with_salt(|hasher| {
                    release.permalink.slug.hash(hasher);
                    t_downloads_permalink.hash(hasher);
                });

                let download_icon = icons::DOWNLOAD;
                let t_download = &translations.download;
                formatdoc!(r#"
                    <a href="{t_downloads_permalink}/{page_hash}{index_suffix}">
                        {download_icon}
                        <span>{t_download}</span>
                    </a>
                "#)
            } else {
                String::new()
            }
        }
        DownloadAccess::Paycurtain { payment_info, .. } => {
            if release.download_assets_available() && payment_info.is_some() {
                let t_purchase_permalink = &translations.purchase_permalink;
                let page_hash = build.hash_with_salt(|hasher| {
                    release.permalink.slug.hash(hasher);
                    t_purchase_permalink.hash(hasher);
                });

                let buy_icon = icons::buy(&translations.buy);
                let t_download = &translations.download;
                formatdoc!(r#"
                    <a href="{t_purchase_permalink}/{page_hash}{index_suffix}">
                        {buy_icon}
                        <span>{t_download}</span>
                    </a>
                "#)
            } else {
                String::new()
            }
        }
    };

    let longest_track_duration = release.longest_track_duration();

    let t_play = &translations.play;

    let more_icon = icons::more(&translations.more);
    let play_icon = icons::play(t_play);

    let varying_track_artists = release.varying_track_artists();

    let t_playback_position = &translations.playback_position;
    let r_tracks = release.tracks
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
                    let src = format!("{track_number}/{format_dir}/{track_hash}/{track_filename_urlencoded}");

                    let source_type = format.source_type();
                    format!(r#"<source src="{src}" type="{source_type}">"#)
                })
                .collect::<Vec<String>>()
                .join("\n");

            let duration_seconds = track.transcodes.borrow().source_meta.duration_seconds;
            let track_title = track.title();

            let track_duration_formatted = format_time(duration_seconds);
            let track_number_formatted = release.track_numbering.format(track_number);
            let track_title_escaped = html_escape_outside_attribute(&track_title);

            let r_waveform = if release.theme.waveforms {
                let waveform_svg = waveform(track);

                formatdoc!(r#"
                    <div class="waveform">
                        {waveform_svg}
                        <input aria-label="{t_playback_position}" aria-valuetext="" autocomplete="off" max="{duration_seconds}" min="0" step="any" type="range" value="0">
                        <div class="decoration"></div>
                    </div>
                "#)
            } else {
                String::new()
            };

            let track_artists = match varying_track_artists {
                true => {
                    let artists_truncation = Truncation::Truncate {
                        max_chars: 80,
                        others_link: format!("{track_number}/")
                    };
                    let artists_truncated = list_track_artists(build, index_suffix, root_prefix, catalog, artists_truncation, track);
                    format!(r#"<div class="artists">{artists_truncated}</div>"#)
                }
                false => String::new()
            };

            let r_cover_micro = if let Some(filename) = track.cover_160_filename() {
                format!(r#"<img aria-hidden="true" src="{track_number}/{filename}">"#)
            } else if let Some(filename) = release.cover_160_filename() {
                format!(r#"<img aria-hidden="true" src="{filename}">"#)
            } else {
                let filename = release.procedural_cover_120_filename_unchecked();
                format!(r#"<img aria-hidden="true" class="procedural" src="{filename}">"#)
            };

            let r_more = if track.more.is_some() {
                let more_label = match &track.more_label {
                    Some(label) => label,
                    None => *translations.more
                };
                format!(r#"<a href="{track_number}{index_suffix}#more">{more_label}</a>&nbsp;&nbsp;"#)
            } else {
                String::new()
            };

            formatdoc!(r#"
                <div class="track" data-duration="{duration_seconds}">
                    <button class="track_playback" tabindex="-1">
                        <span class="icon">
                            {play_icon}
                        </span>
                        {r_cover_micro}
                    </button>
                    <div>
                        <div>
                            <span class="number">{track_number_formatted}</span>
                            <a class="title" href="{track_number}{index_suffix}">{track_title_escaped}</a>
                        </div>
                        {track_artists}
                        {r_waveform}
                        <audio controls preload="none">
                            {audio_sources}
                        </audio>
                    </div>
                    <div>
                        {r_more} <span class="time">{track_duration_formatted}</span>
                    </div>
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let mut primary_actions = Vec::new();
    let mut secondary_actions = Vec::new();

    let t_listen = &translations.listen;
    let listen_button = formatdoc!(r#"
        <button class="emphasized listen">
            <span class="icon">{play_icon}</span>
            <span class="label">{t_listen}</span>
        </button>
    "#);

    primary_actions.push(listen_button);

    if !download_link.is_empty() {
        primary_actions.push(download_link);
    }

    let artists = list_release_artists(build, index_suffix, root_prefix, catalog, Truncation::Pass, release);
    let artists_truncation = Truncation::Truncate {
        max_chars: 80,
        others_link: String::from("#more")
    };
    let artists_truncated = list_release_artists(build, index_suffix, root_prefix, catalog, artists_truncation, release);

    let r_more = if release.more.is_some() || artists_truncated.truncated {
        let more_label = match &release.more_label {
            Some(label) => label,
            None => *translations.more
        };
        let more_link = formatdoc!(r##"
            <a class="more" href="#more">
                {more_icon} {more_label}
            </a>
        "##);

        primary_actions.push(more_link);

        let r_more = match &release.more {
            Some(html_and_stripped) => format!(
                r#"<div class="text">{}</div>"#,
                html_and_stripped.html
            ),
            None => String::new()
        };
        formatdoc!(r#"
            <a class="scroll_target" id="more"></a>
            <div class="page">
                <div class="page_center">
                    <div class="page_more">
                        <div class="release_info">
                            <h1>{release_title_escaped}</h1>
                            <div class="release_artists">{artists}</div>
                        </div>
                        {r_more}
                    </div>
                </div>
            </div>
        "#)
    } else {
        String::new()
    };


    let r_primary_actions = if primary_actions.is_empty() {
        String::new()
    } else {
        let joined = primary_actions.join("");

        formatdoc!(r#"
            <div class="actions primary">
                {joined}
            </div>
        "#)
    };

    if release.copy_link {
        layout.add_clipboard_script();

        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => ("content", base_url.join_index(build, &release.permalink.slug)),
            None => ("dynamic-url", String::new())
        };

        let r_copy_link = copy_button(content_key, &content_value, &translations.copy_link);
        secondary_actions.push(r_copy_link);
    }

    if build.base_url.is_some() {
        if release.m3u  {
            let t_m3u_playlist = &translations.m3u_playlist;
            let stream_icon = icons::STREAM;

            let m3u_playlist_link = formatdoc!(r#"
                <a href="{M3U_PLAYLIST_FILENAME}">
                    {stream_icon}
                    <span>{t_m3u_playlist}</span>
                </a>
            "#);

            secondary_actions.push(m3u_playlist_link);
        }

        if release.embedding {
            let t_embed = &translations.embed;
            let embed_icon = icons::embed(t_embed);

            let embed_link = formatdoc!(r#"
                <a href="embed{index_suffix}">
                    {embed_icon}
                    <span>{t_embed}</span>
                </a>
            "#);

            secondary_actions.push(embed_link);
        }
    }

    for link in &release.links {
        let external_icon = icons::external(&translations.external_link);

        let rel_me = if link.rel_me { r#"rel="me""# } else { "" };
        let url = &link.url;

        let r_link = if link.hidden {
            format!(r#"<a href="{url}" {rel_me} style="display: none;">hidden</a>"#)
        } else {
            let label = link.pretty_label();
            let e_label = html_escape_outside_attribute(label);
            formatdoc!(r#"
                <a href="{url}" {rel_me} target="_blank">{external_icon} <span>{e_label}</span></a>
            "#)
        };

        secondary_actions.push(r_link);
    }

    let r_secondary_actions = if secondary_actions.is_empty() {
        String::new()
    } else {
        let joined = secondary_actions.join("");

        formatdoc!(r#"
            <div class="actions">
                {joined}
            </div>
        "#)
    };

    let relative_waveforms = if release.theme.relative_waveforms { "" } else { "data-disable-relative-waveforms " };

    let release_title_with_unlisted_badge = if release.unlisted {
        format!("{release_title_escaped} {}", unlisted_badge(build))
    } else {
        release_title_escaped.clone()
    };

    let cover = release_cover_image(build, release, "", root_prefix);

    let synopsis = match &release.synopsis {
        Some(synopsis) => {
            formatdoc!(r#"
                <div style="margin-bottom: 1rem; margin-top: 1rem;">
                    {synopsis}
                </div>
            "#)
        }
        None => String::new()
    };

    let tall = if varying_track_artists { "tall" } else { "" };

    let compact_tall = match (release.theme.waveforms, varying_track_artists) {
        (true, true) => "tall",
        (true, false) | (false, true) => "",
        (false, false) => "compact"
    };

    let speed_controls = if release.speed_controls { SPEED_CONTROLS } else { "" };
    let r_volume_controls = volume_controls(translations);

    let next_track_icon = icons::next_track(&translations.next_track);
    let body = formatdoc!(r##"
        <div class="page">
            <div class="page_split">
                <div class="cover">{cover}</div>
                <div class="abstract">
                    <h1>{release_title_with_unlisted_badge}</h1>
                    <div class="release_artists">{artists_truncated}</div>
                    {r_primary_actions}
                    {synopsis}
                    {r_secondary_actions}
                </div>
            </div>
        </div>
        <div class="page">
            <div class="page_center">
                <div class="{compact_tall} tracks" data-longest-duration="{longest_track_duration}" {relative_waveforms}>
                    {r_tracks}
                </div>
            </div>
        </div>
        {r_more}
        <div class="docked_player {tall}">
            <div class="timeline">
                <input aria-label="{t_playback_position}" aria-valuetext="" autocomplete="off" max="" min="0" step="any" type="range" value="0">
                <div class="base"></div>
                <div class="progress" style="width: 0%;"></div>
            </div>
            <div class="elements">
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
        <div aria-label="" class="docked_player_status" role="status"></div>
    "##);

    if release.unlisted {
        layout.no_indexing();
    }

    if catalog.opengraph {
        if let Some(base_url) = &build.base_url {
            let release_slug = &release.permalink.slug;
            let release_url = base_url.join_index(build, release_slug);
            let mut meta = OpenGraphMeta::new(release.title.clone(), release_url);

            if let Some(synopsis) = &release.synopsis {
                meta.description(synopsis);
            }

            if let Some(described_image) = &release.cover {
                let release_prefix = base_url.join_prefix(release_slug);
                let opengraph_image = described_image
                    .borrow()
                    .cover_opengraph_image_unchecked(&release_prefix);

                meta.image(opengraph_image);

                if let Some(description) = &described_image.description {
                    meta.image_alt(description);
                }
            }
            // TODO: Should(n't) we also provide a procedural cover as a
            // fallback here? (also applies for the same spot in the track
            // page render code)

            layout.add_opengraph_meta(meta);
        }
    }

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &release.theme,
        &release.title
    )
}
