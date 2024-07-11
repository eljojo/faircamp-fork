// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

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
                <div class="hcenter_narrow mobile_hpadding">
                    <div class="text vpad" style="margin-top: 1.5rem;">
                        {html}
                    </div>
                </div>
            "#)
        }
        None => String::new()
    };

    let longest_track_duration = release.longest_track_duration();

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

            let duration_formatted = format_time(duration_seconds);
            let track_number_formatted = release.track_numbering.format(track_number);
            let track_title_escaped = html_escape_outside_attribute(&track_title);
            let track_title_attribute_escaped = html_escape_inside_attribute(&track_title);
            let waveform_svg = waveform(&catalog.theme, track);

            let copy_icon = icons::copy(&build.locale.translations.copy_link);
            let more_icon = icons::more(&build.locale.translations.more);
            let play_icon = icons::play(&build.locale.translations.play);
            formatdoc!(r#"
                <div class="track">
                    <span class="track_number outer">{track_number_formatted}</span>
                    <span class="track_header">
                        <span class="track_number inner">{track_number_formatted}</span>
                        <a class="track_title" href="{track_number}/" title="{track_title_attribute_escaped}">{track_title_escaped}</a>
                        <span class="duration">{duration_formatted}</span>
                        <div class="more">
                            <button class="track_playback">
                                {play_icon}
                            </button>
                            <button>
                                {copy_icon}
                            </button>
                        </div>
                        <button class="more_button">
                            {more_icon}
                        </button>
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

    if release.copy_link {
        let content = match &build.base_url {
            Some(base_url) => Some(
                base_url
                    .join(&format!("{}{index_suffix}", &release.permalink.slug))
                    .unwrap()
                    .to_string()
            ),
            None => None
        };

        let t_copy_link = &build.locale.translations.copy_link;
        let r_copy_link = copy_button(build, content.as_deref(), t_copy_link);
        action_links.push(r_copy_link);
    }

    for link in &release.links {
        let external_icon = icons::external(&build.locale.translations.share);

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

        action_links.push(r_link);
    }

    let r_action_links = if action_links.is_empty() {
        String::new()
    } else {
        let joined = action_links.join(" &nbsp; ");

        formatdoc!(r#"
            <div class="action_links hcenter_narrow mobile_hpadding">
                {joined}
            </div>
        "#)
    };

    let relative_waveforms = if release.theme.relative_waveforms { "" } else { "data-disable-relative-waveforms " };

    let release_title_unlisted = if release.unlisted {
        format!("{release_title_escaped} {}", unlisted_badge(build))
    } else {
        release_title_escaped.clone()
    };

    let r_player_icon_templates = player_icon_templates(build);

    let play_icon = icons::play(&build.locale.translations.play);
    let body = formatdoc!(
        r##"
            <div class="vcenter_page_outer">
                <div class="hcenter_narrow mobile_hpadding vcenter_page vpad_adaptive">
                    <div class="cover">{cover}</div>

                    <div class="release_label">
                        <button class="big_play_button">
                            {play_icon}
                        </button>
                        <h1>{release_title_unlisted}</h1>
                        <div class="release_artists">{artists}</div>
                    </div>

                    <div {relative_waveforms}data-longest-duration="{longest_track_duration}"></div>
                    {tracks_rendered}
                    {r_player_icon_templates}
                </div>
                <div class="additional">
                    {r_action_links}
                    {release_text}
                </div>
            </div>
        "##,
        artists = list_release_artists(index_suffix, root_prefix, catalog, release),
        cover = cover_image(build, index_suffix, "", root_prefix, release)
    );

    let breadcrumbs = &[
        format!(r#"<a href=".{index_suffix}">{release_title_escaped}</a>"#)
    ];

    let crawler_meta = if release.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &release.theme,
        &release.title,
        breadcrumbs,
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

        let duration_seconds = track.transcodes.borrow().source_meta.duration_seconds;

        formatdoc!(r#"
            <svg data-duration="{duration_seconds}"
                 data-peaks="{peaks_base64}">
                <path class="seek"/>
                <path class="playback"/>
                <path class="base"/>
            </svg>
        "#)
    } else {
        String::new()
    }
}
