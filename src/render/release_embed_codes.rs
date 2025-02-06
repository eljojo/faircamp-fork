// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use url::Url;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    Release,
    Scripts
};
use crate::icons;
use crate::render::{compact_release_identifier, copy_button, embed_code, layout};
use crate::util::html_escape_outside_attribute;

/// Renders the page that lets the visitor copy embed codes for the release.
pub fn release_embed_codes_html(
    base_url: &Url,
    build: &Build,
    catalog: &Catalog,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../";
    let root_prefix = "../../";

    let release_link = format!("..{index_suffix}");

    let r_compact_release_identifier = compact_release_identifier(
        build,
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let copy_icon = icons::copy(None);
    let failed_icon = icons::failure(&build.locale.translations.failed);
    let success_icon = icons::success(&build.locale.translations.copied);
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

    let t_audio_player_widget_for_xxx =
        build.locale.translations.audio_player_widget_for_xxx(&release.title);

    let release_slug = &release.permalink.slug;

    let embed_url = base_url
        .join(&format!("{release_slug}/embed/all{index_suffix}"))
        .unwrap();

    let (embed_copy_code, embed_display_code) = embed_code(&embed_url, &t_audio_player_widget_for_xxx);

    let t_copy = &build.locale.translations.copy;
    let r_copy_button = copy_button("content", &embed_copy_code, &copy_icon, t_copy);

    let t_embed = &build.locale.translations.embed;
    let t_embed_entire_release = &build.locale.translations.embed_entire_release;
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center">
                <div>
                    <h1>{t_embed}</h1>
                    {r_compact_release_identifier}
                    <div style="margin-top: 2rem;">
                        <div class="embed_split">
                            <span>{t_embed_entire_release}</span>
                            {r_copy_button}
                        </div>
                        {embed_display_code}
                    </div>
                </div>
            </div>
        </div>
        {templates}
    "#);

    let release_title = &release.title;
    let release_title_escaped = html_escape_outside_attribute(release_title);
    let breadcrumb = Some(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_embed} – {release_title}");

    layout(
        root_prefix,
        &body,
        breadcrumb,
        build,
        catalog,
        CrawlerMeta::NoIndexNoFollow,
        Scripts::Clipboard,
        None,
        &release.theme,
        &page_title
    )
}
