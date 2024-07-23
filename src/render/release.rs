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
    Theme,
    Track
};
use crate::render::{
    copy_button,
    cover_image,
    layout,
    list_release_artists,
    player_icon_templates,
    unlisted_badge
};
use crate::icons;
use crate::util::{
    format_time,
    html_escape_inside_attribute,
    html_escape_outside_attribute
};

pub mod checkout;
pub mod download;
pub mod embed;

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
            let t_downloads = &build.locale.translations.downloads;
            formatdoc!(r#"
                <a href="{t_unlock_permalink}/{page_hash}{index_suffix}">
                    {unlock_icon}
                    <span>{t_downloads}</span>
                </a>
            "#)
        },
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free => {
            let t_downloads_permalink = &build.locale.translations.downloads_permalink;
            let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_downloads_permalink]);

            let download_icon = icons::download(&build.locale.translations.download);
            let t_downloads = &build.locale.translations.downloads;
            formatdoc!(r#"
                <a href="{t_downloads_permalink}/{page_hash}{index_suffix}">
                    {download_icon}
                    <span>{t_downloads}</span>
                </a>
            "#)
        },
        DownloadOption::Paid(_currency, _range) => {
            if release.payment_options.is_empty() {
                String::new()
            } else {
                let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_purchase_permalink]);

                let buy_icon = icons::buy(&build.locale.translations.buy);
                let t_downloads = &build.locale.translations.downloads;
                formatdoc!(r#"
                    <a href="{t_purchase_permalink}/{page_hash}{index_suffix}">
                        {buy_icon}
                        <span>{t_downloads}</span>
                    </a>
                "#)
            }
        }
    };

    let release_text = match &release.text {
        Some(html_and_stripped) => {
            let html = &html_and_stripped.html;
            formatdoc!(r#"
                <div class="hcenter_narrow mobile_hpadding text">
                    {html}
                </div>
            "#)
        }
        None => String::new()
    };

    let longest_track_duration = release.longest_track_duration();

    let copy_track_icon = icons::copy(Some(&build.locale.translations.copy_link_to_track));
    let more_icon = icons::more(&build.locale.translations.more);
    let play_icon = icons::play(&build.locale.translations.play);

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
            let track_title_attribute_escaped = html_escape_inside_attribute(&track_title);
            let waveform_svg = waveform(&catalog.theme, track);

            let (copy_track_key, copy_track_value) = match &build.base_url {
                Some(base_url) => {
                    let url = base_url.join(&format!("{}/{track_number}{index_suffix}", &release.permalink.slug)).unwrap().to_string();
                    ("content", url)
                }
                None => ("dynamic-url", format!("{track_number}{index_suffix}"))
            };

            formatdoc!(r#"
                <div class="track">
                    <span class="track_number outer">{track_number_formatted}</span>
                    <span class="track_header">
                        <span class="track_number inner">{track_number_formatted}</span>
                        <a class="track_title" href="{track_number}/" title="{track_title_attribute_escaped}">{track_title_escaped}</a>
                        <span class="track_time">{track_duration_formatted}</span>
                        <button class="more_button" tabindex="-1">
                            {more_icon}
                        </button>
                        <div class="more">
                            <button class="track_playback">
                                {play_icon}
                            </button>
                            <button data-{copy_track_key}="{copy_track_value}" data-copy-track>
                                {copy_track_icon}
                            </button>
                        </div>
                    </span>
                    <audio controls preload="none">
                        {audio_sources}
                    </audio>
                    <div class="waveform">
                        {waveform_svg}
                        <input aria-valuetext="" autocomplete="off" max="{duration_seconds}" min="0" step="any" type="range" value="0">
                        <div class="decoration"></div>
                    </div>
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let mut action_links = Vec::new();

    if !download_link.is_empty() {
        action_links.push(download_link);
    }

    let failed_icon = icons::failure(&build.locale.translations.failed);
    let success_icon = icons::success(&build.locale.translations.copied);
    let mut templates = format!(r#"
        <template id="copy_track_icon">
            {copy_track_icon}
        </template>
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
        action_links.push(r_copy_link);

        templates.push_str(&format!(r#"
            <template id="copy_icon">
                {copy_icon}
            </template>
        "#));
    }

    if release.embedding && build.base_url.is_some() {
        let t_embed = &build.locale.translations.embed;
        let embed_icon = icons::embed(t_embed);

        let embed_link = formatdoc!(r#"
            <a href="embed{index_suffix}">
                {embed_icon}
                <span>{t_embed}</span>
            </a>
        "#);

        action_links.push(embed_link);
    };

    let r_action_links = if action_links.is_empty() {
        String::new()
    } else {
        let joined = action_links.join("");

        formatdoc!(r#"
            <div class="release_actions">
                {joined}
            </div>
        "#)
    };

    let mut links = Vec::new();

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

        links.push(r_link);
    }

    let r_links = if links.is_empty() {
        String::new()
    } else {
        let joined = links.join("");

        formatdoc!(r#"
            <div class="links hcenter_narrow mobile_hpadding">
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

    let artists = list_release_artists(index_suffix, root_prefix, catalog, release);
    let cover = cover_image(build, index_suffix, "", root_prefix, release);

    let release_duration = format_time(release.duration());
    let release_year = match release.date {
        Some(naive_date) => format!("({})", naive_date.year()),
        None => String::new()
    };


    let (first_track_time, first_track_title) = {
        let first_track = &release.tracks.iter().next().unwrap();

        let time = format!("00:00 / {}", format_time(first_track.transcodes.borrow().source_meta.duration_seconds));
        let title = html_escape_outside_attribute(&first_track.title());

        (time, title)
    };

    let next_track_icon = icons::next_track(&build.locale.translations.next_track);
    let previous_track_icon = icons::previous_track(&build.locale.translations.previous_track);
    let volume_icon = icons::volume(&build.locale.translations.play); // TODO: Label, and a few other things
    let t_play = &build.locale.translations.play;
    let body = formatdoc!(r##"
        <div class="vcenter_page_outer">
            <div class="hcenter_narrow mobile_hpadding vcenter_page vpad_adaptive">
                <div class="cover">{cover}</div>

                <div class="release_label">
                    <h1>{release_title_with_unlisted_badge}</h1>
                    <div class="quick_actions">
                        <button class="play_all_button">{t_play} {play_icon}</button>
                        <a href="#details">More {more_icon}</a>
                    </div>
                    <!--div class="release_artists">{artists}</div-->
                </div>

                <div {relative_waveforms}data-longest-duration="{longest_track_duration}"></div>
                {tracks_rendered}
            </div>
            <div class="docked_player">
                <svg class="active_waveform">
                    <path class="area"/>
                </svg>
                <div class="big_bar">
                    <input aria-valuetext="" autocomplete="off" max="" min="0" step="any" type="range" value="0">
                    <div class="progress" style="width: 0%;"></div>
                </div>
                <div style="display: flex;">
                    <button class="big_play_button">
                        {play_icon}
                    </button>
                    <div class="volume">
                        <button class="volume_button">
                            {volume_icon}
                            <span>100%</span>
                        </button>
                        <div class="element volume_slider">
                            <input aria-valuetext="" autocomplete="off" max="1" min="0" step="any" type="range" value="1">
                        </div>
                    </div>
                    <span class="element track_time">{first_track_time}</span>
                    <span class="element track_title">{first_track_title}</span>
                    <button class="previous_track_button">
                        {previous_track_icon}
                    </button>
                    <button class="next_track_button">
                        {next_track_icon}
                    </button>
                </div>
            </div>
            <a id="details"></a>
            <div class="additional">
                <div class="hcenter_narrow mobile_hpadding release_info">
                    <div>
                        <div>
                            {release_title_escaped} {release_year}
                        </div>
                        <div>{artists}</div>
                        {release_duration}
                    </div>
                    {r_action_links}
                </div>
                {release_text}
                {r_links}
            </div>
        </div>
        {templates}
    "##);

    let crawler_meta = if release.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &release.theme,
        &release.title,
        crawler_meta
    )
}

pub fn waveform(theme: &Theme, track: &Track) -> String {
    if let Some(peaks) = &track.transcodes.borrow().source_meta.peaks {
        let peaks_base64 = peaks.iter()
            .map(|peak| {
                // In https://codeberg.org/simonrepp/faircamp/issues/11#issuecomment-858690
                // the "_ => unreachable!()" branch below was hit, probably due to a slight
                // peak overshoot > 1.0 (1.016 already leads to peak64 being assigned 64).
                // We know that some decoders can produce this kind of overshoot, ideally
                // we should be normalizing (=limiting) these peaks to within 0.0-1.0
                // already when we compute/store/cache them. For now we prevent the panic
                // locally here as a patch.
                // TODO:
                // - Implement normalizing/limiting at the point of decoding/caching
                // - Implement an integrity check of all peaks at cache retrieval time (?),
                //   triggering a correction and cache update/removal if found - this is
                //   only meant as a temporary measure, to be phased out in some months/
                //   years.
                //   OR: Better yet use the cache layout versioning
                //   flag to trigger a cache update for all updated faircamp
                //   versions, so all peaks are correctly recalculated for everyone then.
                // - Then also remove this peak_limited correction and rely on the raw
                //   value again.
                let peak_limited = if theme.waveforms {
                    if *peak > 1.0 { 1.0 } else { *peak }
                } else {
                    0.5
                };

                // Limit range to 0-63
                let peak64 = ((peak_limited / 1.0) * 63.0) as u8;
                let base64 = match peak64 {
                    0..=25 => (peak64 + 65) as char, // shift to 65-90 (A-Z)
                    26..=51 => (peak64 + 71) as char, // shift to 97-122 (a-z)
                    52..=61 => (peak64 - 4) as char, // shift to 48-57 (0-9)
                    62 => '+', // map to 43 (+)
                    63 => '/', // map to 48 (/)
                    _ => unreachable!() 
                };
                base64.to_string()
            })
            .collect::<Vec<String>>()
            .join("");

        formatdoc!(r#"
            <svg data-peaks="{peaks_base64}">
                <path class="seek"/>
                <path class="playback"/>
                <path class="base"/>
            </svg>
        "#)
    } else {
        String::new()
    }
}
