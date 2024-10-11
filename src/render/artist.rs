// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Artist, Build, Catalog, CrawlerMeta, ReleaseRc};
use crate::icons;
use crate::render::{
    artist_image,
    copy_button,
    layout,
    releases,
    unlisted_badge
};
use crate::util::html_escape_outside_attribute;

pub fn artist_html(build: &Build, artist: &Artist, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";

    let artist_text = match &artist.text {
        Some(html_and_stripped) => format!(
            r#"<div class="text">{}</div>"#,
            &html_and_stripped.html
        ),
        None => String::new()
    };

    let artist_name_escaped = html_escape_outside_attribute(&artist.name);

    let mut secondary_actions = Vec::new();

    let templates;
    if artist.copy_link {
        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => {
                let url = base_url.join(&format!("{}{index_suffix}", &artist.permalink.slug)).unwrap().to_string();
                ("content", url)
            }
            None => ("dynamic-url", String::new())
        };


        let copy_icon = icons::copy(None);
        let t_copy_link = &build.locale.translations.copy_link;
        let r_copy_link = copy_button(content_key, &content_value, &copy_icon, t_copy_link);
        secondary_actions.push(r_copy_link);

        let failed_icon = icons::failure(&build.locale.translations.failed);
        let success_icon = icons::success(&build.locale.translations.copied);
        templates = format!(r#"
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
    } else {
        templates = String::new();
    };

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

    let r_artist_image = match &artist.image {
        Some(artist_image_unpacked) => artist_image(
            build,
            index_suffix,
            root_prefix,
            &artist.permalink.slug,
            artist_image_unpacked
        ),
        None => String::new()
    };

    let name_unlisted = if artist.unlisted {
        format!("{artist_name_escaped} {}", unlisted_badge(build))
    } else {
        artist_name_escaped.clone()
    };

    let public_releases: Vec<ReleaseRc> = artist.releases
        .iter()
        .filter_map(|release| {
            match release.borrow().unlisted {
                true => None,
                false => Some(release.clone())
            }
        })
        .collect();

    let r_releases = releases(
        build,
        index_suffix,
        root_prefix,
        catalog,
        &public_releases
    );

    let artist_synopsis: Option<String> = None; // TODO: Think through/unmock/implement
    let synopsis = match artist_synopsis {
        Some(synopsis) => {
            formatdoc!(r#"
                <div style="margin-bottom: 1rem; margin-top: 1rem;">
                    {synopsis}
                </div>
            "#)
        }
        None => String::new()
    };

    let grid_icon = icons::grid();
    let more_icon = icons::more(&build.locale.translations.more);
    let scroll_icon = icons::scroll();
    let t_more = &build.locale.translations.more;
    let t_releases = &build.locale.translations.releases;
    let t_top = &build.locale.translations.top;
    let body = formatdoc!(r##"
        <div class="page" data-overview>
            <div class="page_split">
                {r_artist_image}
                <div style="max-width: 26rem;">
                    <h1>{name_unlisted}</h1>
                    <div class="actions primary">
                        <a class="emphasized" href="#releases">
                            {grid_icon}
                            {t_releases}
                        </a>
                        <a class="more" href="#description">
                            {more_icon} {t_more}
                        </a>
                    </div>
                    {synopsis}
                </div>
            </div>
        </div>
        <a class="scroll_target" id="releases"></a>
        <div class="additional page">
            <div class="page_grid">
                <div>
                    {r_releases}
                </div>
            </div>
        </div>
        <a class="scroll_target" id="description"></a>
        <div class="page" data-description>
            <div class="page_center">
                <div style="max-width: 32rem;">
                    <div style="font-size: 1.4rem;">{artist_name_escaped}</div>
                    {r_secondary_actions}
                    {artist_text}
                </div>
            </div>
        </div>
        <div class="scroll_hints">
            <a class="up" href="#">
                {scroll_icon} {t_top}
            </a>
            <a class="down" href="#description">
                <span>{scroll_icon}</span> {t_more}
            </a>
        </div>
        {templates}
    "##);

    let crawler_meta = if artist.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &artist.theme,
        &artist.name,
        crawler_meta,
        None
    )
}
