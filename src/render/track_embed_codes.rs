// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    Release,
    Scripts,
    SiteUrl,
    Track
};
use crate::icons;
use crate::render::{compact_track_identifier, copy_button, embed_code, layout};
use crate::util::html_escape_outside_attribute;

/// Renders the page that lets the visitor copy embed codes for the track.
pub fn track_embed_codes_html(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    track: &Track,
    track_number: usize
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";
    let track_prefix = "../";
    let translations = &build.locale.translations;

    let track_link = format!("..{index_suffix}");

    let r_compact_track_identifier = compact_track_identifier(
        build,
        catalog,
        index_suffix,
        release,
        release_prefix,
        root_prefix,
        track,
        &track_link,
        track_prefix
    );

    let copy_icon = icons::copy();
    let failed_icon = icons::failure(&translations.failed);
    let success_icon = icons::success(&translations.copied);
    let templates = format!(r#"
        <template id="copy_icon">
            {copy_icon}
        </template>
        <template id="failed_icon">
            {failed_icon}
        </template>
        <template id="success_icon">
            {success_icon}
        </template>
    "#);

    let track_title = track.title();

    let t_audio_player_widget_for_xxx =
        translations.audio_player_widget_for_xxx(&track_title);

    let release_slug = &release.permalink.slug;

    let embed_url = base_url.join_index(build, &format!("{release_slug}/embed/{track_number}"));

    let (embed_copy_code, embed_display_code) = embed_code(&embed_url, &t_audio_player_widget_for_xxx);

    let r_copy_button = copy_button("content", &embed_copy_code, &translations.copy);
    let track_number_formatted = release.track_numbering.format(track_number);
    let track_title_escaped = html_escape_outside_attribute(&track_title);

    let t_embed = &translations.embed;
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center">
                <div>
                    <h1>{t_embed}</h1>
                    {r_compact_track_identifier}
                    <div class="embed_split" style="margin-top: 2rem; position: relative;">
                        <div>
                            <span style="color: var(--fg-3);">{track_number_formatted}</span>
                            <span>{track_title_escaped}</span>
                        </div>
                        {r_copy_button}
                    </div>
                    {embed_display_code}
                </div>
            </div>
        </div>
        {templates}
    "#);

    let release_link = format!("../../..{index_suffix}");
    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let breadcrumb = Some(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_embed} – {track_title}");

    layout(
        root_prefix,
        &body,
        breadcrumb,
        build,
        catalog,
        CrawlerMeta::NoIndexNoFollow,
        Scripts::Clipboard,
        None,
        &track.theme,
        &page_title
    )
}
