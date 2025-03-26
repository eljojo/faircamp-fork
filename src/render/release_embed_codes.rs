// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    Release,
    SiteUrl
};
use crate::util::html_escape_outside_attribute;

use super::Layout;
use super::{compact_release_identifier, copy_button, embed_code};

/// Renders the page that lets the visitor copy embed codes for the release.
pub fn release_embed_codes_html(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../";
    let root_prefix = "../../";
    let translations = &build.locale.translations;

    let mut layout = Layout::new();

    layout.add_clipboard_script();
    layout.no_indexing();

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

    let t_audio_player_widget_for_xxx =
        translations.audio_player_widget_for_xxx(&release.title);

    let release_slug = &release.permalink.slug;

    let embed_url = base_url.join_index(build, format!("{release_slug}/embed/all"));

    let (embed_copy_code, embed_display_code) = embed_code(&embed_url, &t_audio_player_widget_for_xxx);

    let r_copy_button = copy_button("content", &embed_copy_code, &translations.copy);

    let t_embed = &translations.embed;
    let t_embed_entire_release = &translations.embed_entire_release;
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
    "#);

    let release_title = &release.title;
    let release_title_escaped = html_escape_outside_attribute(release_title);

    layout.add_breadcrumb(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_embed} – {release_title}");

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &release.theme,
        &page_title
    )
}
