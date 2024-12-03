// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use chrono::Datelike;
use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    DownloadOption,
    Release,
    Scripts,
    Track
};
use crate::icons;
use crate::render::{
    copy_button,
    cover_image,
    layout,
    list_track_artists,
    player_icon_templates,
    Truncation,
    waveform
};
use crate::util::{format_time, html_escape_outside_attribute};

pub fn track_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    track: &Track,
    track_number: usize
) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../";

    // TODO: Could we/should we have a track-only flow here?
    //       (Also implies track-level download configuration?)
    let download_link = match &release.download_option {
        DownloadOption::Codes { .. } => {
            let t_unlock_permalink = &build.locale.translations.unlock_permalink;
            let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_unlock_permalink]);

            let unlock_icon = icons::unlock(&build.locale.translations.unlock);
            let t_downloads = &build.locale.translations.downloads;
            Some(formatdoc!(r#"
                <a href="../{t_unlock_permalink}/{page_hash}{index_suffix}">
                    {unlock_icon}
                    <span>{t_downloads}</span>
                </a>
            "#))
        }
        DownloadOption::Disabled => None,
        DownloadOption::External { link } => {
            let external_icon = icons::external(&build.locale.translations.external_link);
            let t_download = &build.locale.translations.download;
            Some(formatdoc!(r#"
                <a href="{link}" target="_blank">
                    {external_icon}
                    <span>{t_download}</span>
                </a>
            "#))
        }
        DownloadOption::Free => {
            let t_downloads_permalink = &build.locale.translations.downloads_permalink;
            let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_downloads_permalink]);

            let download_icon = icons::download();
            let t_downloads = &build.locale.translations.downloads;
            Some(formatdoc!(r#"
                <a href="../{t_downloads_permalink}/{page_hash}{index_suffix}">
                    {download_icon}
                    <span>{t_downloads}</span>
                </a>
            "#))
        }
        DownloadOption::Paid { payment_text, .. } => {
            if payment_text.is_none() {
                None
            } else {
                let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_purchase_permalink]);

                let buy_icon = icons::buy(&build.locale.translations.buy);
                let t_downloads = &build.locale.translations.downloads;
                Some(formatdoc!(r#"
                    <a href="../{t_purchase_permalink}/{page_hash}{index_suffix}">
                        {buy_icon}
                        <span>{t_downloads}</span>
                    </a>
                "#))
            }
        }
    };

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
            let src = format!("../{format_dir}/{track_hash}/{track_filename_urlencoded}");

            let source_type = format.source_type();
             format!(r#"<source src="{src}" type="{source_type}">"#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let duration_seconds = track.transcodes.borrow().source_meta.duration_seconds;
    let track_title = track.title();

    let track_duration_formatted = format_time(duration_seconds);
    let track_title_escaped = html_escape_outside_attribute(&track_title);

    let compact;
    let r_waveform;
    if release.theme.waveforms {
        let waveform_svg = waveform(track);

        compact = "";
        r_waveform = formatdoc!(r#"
            <div class="waveform">
                {waveform_svg}
                <input aria-valuetext="" autocomplete="off" max="{duration_seconds}" min="0" step="any" type="range" value="0">
                <div class="decoration"></div>
            </div>
        "#);
    } else {
        compact = "compact";
        r_waveform = String::new();
    };

    let r_cover_micro = match release.cover_image_micro_src() {
        Some(src) => format!(r#"<img aria-hidden="true" src="../{src}">"#),
        None => String::from(r#"<span class="cover_placeholder"></span>"#)
    };
    let r_more = if track.text.is_some() {
        format!(r#"<a href="">More</a>&nbsp;&nbsp;"#)
    } else {
        String::new()
    };

    let play_icon = icons::play(&build.locale.translations.play);
    let r_track = formatdoc!(r#"
        <div class="track" data-duration="{duration_seconds}">
            <button class="track_playback" tabindex="-1">
                <span class="icon">
                    {play_icon}
                </span>
                {r_cover_micro}
            </button>
            <div>
                <div>
                    <span class="title" href="{track_number}{index_suffix}">{track_title_escaped}</span>
                </div>
                {r_waveform}
                <audio controls preload="none">
                    {audio_sources}
                </audio>
            </div>
            </span>
            <div>
                {r_more} <span class="time">{track_duration_formatted}</span>
            </div>
        </div>
    "#);

    let mut primary_actions = Vec::new();

    let t_listen = &build.locale.translations.listen;
    let listen_button = formatdoc!(r#"
        <button class="emphasized listen">
            <span class="icon">{play_icon}</span>
            <span class="label">{t_listen}</span>
        </button>
    "#);

    primary_actions.push(listen_button);

    if let Some(download_link) = download_link {
        primary_actions.push(download_link);
    }

    let artists = list_track_artists(build, index_suffix, root_prefix, catalog, Truncation::Pass, track);
    let artists_truncation = Truncation::Truncate {
        max_chars: 80,
        others_link: String::from("#more")
    };
    let artists_truncated = list_track_artists(build, index_suffix, root_prefix, catalog, artists_truncation, track);

    let r_more = if track.text.is_some() || artists_truncated.truncated {
        let t_more = &build.locale.translations.more;
        let more_icon = icons::more(&build.locale.translations.more);
        let more_link = formatdoc!(r##"
            <a class="more" href="#more">
                {more_icon} {t_more}
            </a>
        "##);

        primary_actions.push(more_link);

        let release_year = match release.date {
            Some(naive_date) => format!("({})", naive_date.year()),
            None => String::new()
        };

        let r_text = match &track.text {
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
                            <h1>{track_title_escaped} {release_year}</h1>
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

    let mut secondary_actions = Vec::new();

    if track.copy_link {
        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => {
                let url = base_url.join(&format!("{}/{track_number}{index_suffix}", &release.permalink.slug)).unwrap().to_string();
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

    if release.embedding && build.base_url.is_some() {
        let embed_icon = icons::embed(&build.locale.translations.embed);
        let t_embed = &build.locale.translations.embed;
        // TODO: /embed/ uses no embed_permalink translation? Should(n't) it?
        let embed_link = formatdoc!(r#"
            <a href="../embed{index_suffix}">
                {embed_icon}
                <span>{t_embed}</span>
            </a>
        "#);
        secondary_actions.push(embed_link);
    }

    // TODO: Get these from track (respectively think through if/how we want that implemented)
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

    let relative_waveforms = if track.theme.relative_waveforms { "" } else { "data-disable-relative-waveforms " };
    let track_duration = track.transcodes.borrow().source_meta.duration_seconds;

    templates.push_str(&player_icon_templates(build));

    // TODO: Track-level cover support
    let cover = cover_image(build, index_suffix, "../", root_prefix, release);

    let track_synopsis: Option<String> = None; // TODO: Think through/unmock/implement
    let synopsis = match track_synopsis {
        Some(synopsis) => {
            formatdoc!(r#"
                <div style="margin-bottom: 1rem; margin-top: 1rem;">
                    {synopsis}
                </div>
            "#)
        }
        None => String::new()
    };

    let volume_icon = icons::volume();
    let t_dimmed = &build.locale.translations.dimmed;
    let t_muted = &build.locale.translations.muted;
    let body = formatdoc!(r##"
        <div class="page">
            <div class="page_split page_60vh">
                <div class="cover">{cover}</div>
                <div class="abstract">
                    <h1>{track_title_escaped}</h1> <!-- TODO: Unlisted badge -->
                    <div class="release_artists">{artists_truncated}</div>
                    {r_primary_actions}
                    {synopsis}
                    {r_secondary_actions}
                </div>
            </div>
        </div>
        <div class="page">
            <div class="page_center">
                <div class="{compact} tracks" data-longest-duration="{track_duration}" {relative_waveforms}>
                    {r_track}
                </div>
            </div>
        </div>
        {r_more}
        <div class="docked_player">
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
        <div aria-label="" class="docked_player_status" role="status"></div>
        {templates}
    "##);

    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let breadcrumb = Some(format!(r#"<a href="..{index_suffix}">{release_title_escaped}</a>"#));

    // TODO: Track-level unlisted properties?
    let crawler_meta = if release.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::ClipboardAndPlayer,
        &track.theme,
        &track_title,
        crawler_meta,
        breadcrumb
    )
}
