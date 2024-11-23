// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use chrono::Datelike;
use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    DownloadOption,
    Release,
    Scripts
};
use crate::render::{
    copy_button,
    cover_image,
    layout,
    list_release_artists,
    list_track_artists,
    player_icon_templates,
    Truncation,
    unlisted_badge,
    waveform
};
use crate::icons;
use crate::util::{format_time, html_escape_outside_attribute};

pub mod download;
pub mod embed;
pub mod purchase;
pub mod unlock;

/// The actual release page, featuring the track listing and streaming player, links
/// to downloads, embeds, description, etc.
pub fn release_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";

    let download_link = match &release.download_option {
        DownloadOption::Codes { .. } => {
            let t_unlock_permalink = &build.locale.translations.unlock_permalink;
            let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_unlock_permalink]);

            let unlock_icon = icons::unlock(&build.locale.translations.unlock);
            let t_download = &build.locale.translations.download;
            formatdoc!(r#"
                <a href="{t_unlock_permalink}/{page_hash}{index_suffix}">
                    {unlock_icon}
                    <span>{t_download}</span>
                </a>
            "#)
        }
        DownloadOption::Disabled => String::new(),
        DownloadOption::External { link } => {
            let external_icon = icons::external(&build.locale.translations.external_link);
            let t_download = &build.locale.translations.download;
            formatdoc!(r#"
                <a href="{link}" target="_blank">
                    {external_icon}
                    <span>{t_download}</span>
                </a>
            "#)
        }
        DownloadOption::Free => {
            let t_downloads_permalink = &build.locale.translations.downloads_permalink;
            let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_downloads_permalink]);

            let download_icon = icons::download();
            let t_download = &build.locale.translations.download;
            formatdoc!(r#"
                <a href="{t_downloads_permalink}/{page_hash}{index_suffix}">
                    {download_icon}
                    <span>{t_download}</span>
                </a>
            "#)
        }
        DownloadOption::Paid { payment_text, .. } => {
            if payment_text.is_none() {
                String::new()
            } else {
                let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_purchase_permalink]);

                let buy_icon = icons::buy(&build.locale.translations.buy);
                let t_download = &build.locale.translations.download;
                formatdoc!(r#"
                    <a href="{t_purchase_permalink}/{page_hash}{index_suffix}">
                        {buy_icon}
                        <span>{t_download}</span>
                    </a>
                "#)
            }
        }
    };

    let longest_track_duration = release.longest_track_duration();

    let t_play = &build.locale.translations.play;

    let more_icon = icons::more(&build.locale.translations.more);
    let play_icon = icons::play(t_play);

    let varying_track_artists = release.varying_track_artists();

    let r_tracks = release.tracks
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
                    let src = format!("{format_dir}/{track_hash}/{track_filename_urlencoded}");

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
                        <input aria-valuetext="" autocomplete="off" max="{duration_seconds}" min="0" step="any" type="range" value="0">
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

            let r_cover_micro = match release.cover_image_micro_src() {
                Some(src) => format!(r#"<img src="{src}">"#),
                None => String::from(r#"<span class="cover_placeholder"></span>"#)
            };

            // TODO: Implement and use track-level more_label
            let r_more = if track.text.is_some() {
                format!(r#"<a href="track_number{index_suffix}#more">More</a>&nbsp;&nbsp;"#)
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

    let t_listen = &build.locale.translations.listen;
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

    let r_more = if release.text.is_some() || artists_truncated.truncated {
        let more_label = match &release.more_label {
            Some(label) => label,
            None => *build.locale.translations.more
        };
        let more_link = formatdoc!(r##"
            <a class="more" href="#more">
                {more_icon} {more_label}
            </a>
        "##);

        primary_actions.push(more_link);

        let release_year = match release.date {
            Some(naive_date) => format!("({})", naive_date.year()),
            None => String::new()
        };

        let r_text = match &release.text {
            Some(html_and_stripped) => format!(
                r#"<div class="text">{}</div>"#,
                html_and_stripped.html
            ),
            None => String::new()
        };
        formatdoc!(r#"
            <a class="scroll_target" id="more"></a>
            <div class="page">
                <div class="page_center page_50vh">
                    <div class="page_more">
                        <div class="release_info">
                            <h1>{release_title_escaped} {release_year}</h1>
                            <div class="release_artists">{artists}</div>
                        </div>
                        {r_text}
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

    let failed_icon = icons::failure(&build.locale.translations.failed);
    let success_icon = icons::success(&build.locale.translations.copied);
    let mut templates = format!(r#"
        <template id="failed_icon">
            {failed_icon}
        </template>
        <template id="success_icon">
            {success_icon}
        </template>
    "#);

    if release.copy_link {
        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => {
                let url = base_url.join(&format!("{}{index_suffix}", &release.permalink.slug)).unwrap().to_string();
                ("content", url)
            }
            None => ("dynamic-url", String::new())
        };

        let copy_icon = icons::copy(None);
        let t_copy_link = &build.locale.translations.copy_link;
        let r_copy_link = copy_button(content_key, &content_value, &copy_icon, t_copy_link);
        secondary_actions.push(r_copy_link);

        templates.push_str(&format!(r#"
            <template id="copy_icon">
                {copy_icon}
            </template>
        "#));
    }

    if build.base_url.is_some() {
        if release.m3u  {
            let t_m3u_playlist = &build.locale.translations.m3u_playlist;
            let stream_icon = icons::stream();

            let m3u_playlist_link = formatdoc!(r#"
                <a href="playlist.m3u">
                    {stream_icon}
                    <span>{t_m3u_playlist}</span>
                </a>
            "#);

            secondary_actions.push(m3u_playlist_link);
        }

        if release.embedding {
            let t_embed = &build.locale.translations.embed;
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
        let external_icon = icons::external(&build.locale.translations.external_link);

        let rel_me = if link.rel_me { r#"rel="me""# } else { "" };
        let url = &link.url;

        let r_link = if link.hidden {
            format!(r#"<a href="{url}" {rel_me} style="display: none;">hidden</a>"#)
        } else {
            let label = link.pretty_label();
            let e_label = html_escape_outside_attribute(&label);
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

    templates.push_str(&player_icon_templates(build));

    let cover = cover_image(build, index_suffix, "", root_prefix, release);

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

    let next_track_icon = icons::next_track(&build.locale.translations.next_track);
    let volume_icon = icons::volume();
    let t_dimmed = &build.locale.translations.dimmed;
    let t_muted = &build.locale.translations.muted;
    let body = formatdoc!(r##"
        <div class="page">
            <div class="page_split page_60vh">
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
                <input aria-valuetext="" autocomplete="off" max="" min="0" step="any" type="range" value="0">
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
                <div class="volume">
                    <button>
                        {volume_icon}
                    </button>
                    <span class="slider">
                        <input aria-valuetext="" autocomplete="off" max="1" min="0" step="any" type="range" value="1">
                    </span>
                </div>
                <span class="track_info">
                    <span class="number"></span>
                    <span class="title_wrapper"></span>
                    <span class="time"></span>
                </span>
                <span class="volume_hint dimmed">{t_dimmed}</span>
                <span class="volume_hint muted">{t_muted}</span>
            </div>
        </div>
        <div aria-label="" class="docked_player_status" role="status"></div>
        {templates}
    "##);

    let crawler_meta = if release.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::ClipboardAndPlayer,
        &release.theme,
        &release.title,
        crawler_meta,
        None
    )
}
