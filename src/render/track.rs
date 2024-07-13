// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    DownloadOption,
    Release,
    Track
};
use crate::icons;
use crate::render::{
    copy_button,
    cover_image,
    layout,
    list_track_artists,
    player_icon_templates
};
use crate::render::release::waveform;
use crate::util::{
    format_time,
    html_escape_inside_attribute,
    html_escape_outside_attribute
};

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
                <a href="{t_unlock_permalink}/{page_hash}{index_suffix}">
                    {unlock_icon}
                    <span>{t_downloads}</span>
                </a>
            "#))
        },
        DownloadOption::Disabled => None,
        DownloadOption::Free => {
            let t_downloads_permalink = &build.locale.translations.downloads_permalink;
            let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_downloads_permalink]);

            let download_icon = icons::download(&build.locale.translations.download);
            let t_downloads = &build.locale.translations.downloads;
            Some(formatdoc!(r#"
                <a href="{t_downloads_permalink}/{page_hash}{index_suffix}">
                    {download_icon}
                    <span>{t_downloads}</span>
                </a>
            "#))
        },
        DownloadOption::Paid(_currency, _range) => {
            if release.payment_options.is_empty() {
                None
            } else {
                let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_purchase_permalink]);

                let buy_icon = icons::buy(&build.locale.translations.buy);
                let t_downloads = &build.locale.translations.downloads;
                Some(formatdoc!(r#"
                    <a href="{t_purchase_permalink}/{page_hash}{index_suffix}">
                        {buy_icon}
                        <span>{t_downloads}</span>
                    </a>
                "#))
            }
        }
    };

    let track_text = match &track.text {
        Some(html_and_stripped) => format!(
            r#"<div class="vpad" style="margin-top: 1.5rem;">{}</div>"#,
            &html_and_stripped.html
        ),
        None => String::new()
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

    let duration_formatted = format_time(duration_seconds);
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

    let copy_track_icon = icons::copy(Some(&build.locale.translations.copy_link_to_track));
    let more_icon = icons::more(&build.locale.translations.more);
    let play_icon = icons::play(&build.locale.translations.play);
    let track_rendered = formatdoc!(r#"
        <div class="track">
            <span class="track_number outer">{track_number_formatted}</span>
            <span class="track_header">
                <span class="track_number inner">{track_number_formatted}</span>
                <span class="track_title" title="{track_title_attribute_escaped}">{track_title_escaped}</span>
                <span class="duration">{duration_formatted}</span>
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
    "#);

    let mut action_links = Vec::new();

    if let Some(download_link) = download_link {
        action_links.push(download_link);
    }

    if release.embedding && build.base_url.is_some() {
        let embed_icon = icons::embed(&build.locale.translations.embed);
        let t_embed = &build.locale.translations.embed;
        let embed_link = formatdoc!(r#"
            <a href="embed{index_suffix}">
                {embed_icon}
                <span>{t_embed}</span>
            </a>
        "#);
        action_links.push(embed_link);
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
        action_links.push(r_copy_link);

        templates.push_str(&format!(r#"
            <template id="copy_icon">
                {copy_icon}
            </template>
        "#));
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

    let relative_waveforms = if track.theme.relative_waveforms { "" } else { "data-disable-relative-waveforms " };
    let track_duration = track.transcodes.borrow().source_meta.duration_seconds;

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
                        <h1>{track_title_escaped}</h1>
                        <div class="release_artists">{artists}</div>
                    </div>

                    <div {relative_waveforms}data-longest-duration="{track_duration}"></div>
                    {track_rendered}
                    {r_player_icon_templates}
                </div>
                <div class="additional">
                    {r_action_links}
                    {track_text}
                </div>
            </div>
            {templates}
        "##,
        artists = list_track_artists(index_suffix, root_prefix, catalog, track),
        // TODO: Track-level cover support
        cover = cover_image(build, index_suffix, "../", root_prefix, release)
    );

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let breadcrumbs = &[
        format!(r#"<a href="..{index_suffix}">{release_title_escaped}</a>"#),
        format!(r#"<a href=".{index_suffix}">{track_title_escaped}</a>"#)
    ];

    // TODO: Track-level unlisted properties?
    let crawler_meta = if release.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &track.theme,
        &track_title,
        breadcrumbs,
        crawler_meta
    )
}
