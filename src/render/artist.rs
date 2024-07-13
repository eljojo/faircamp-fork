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

    let mut action_links = Vec::new();

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
        action_links.push(r_copy_link);

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

    let r_action_links = if action_links.is_empty() {
        String::new()
    } else {
        let joined = action_links.join(" &nbsp; ");

        formatdoc!(r#"
            <div class="action_links">
                {joined}
            </div>
        "#)
    };

    let artist_image_rendered = match &artist.image {
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

    let artist_info = formatdoc!(r##"
        <div class="catalog">
            {artist_image_rendered}
            <div class="catalog_info_padded">
                <h1>{name_unlisted}</h1>
                {artist_text}
                {r_action_links}
            </div>
        </div>
    "##);

    let visible_releases: Vec<ReleaseRc> = artist.releases
        .iter()
        .filter_map(|release| {
            match release.borrow().unlisted {
                true => None,
                false => Some(release.clone())
            }
        })
        .collect();

    let releases_rendered = releases(
        build,
        index_suffix,
        root_prefix,
        catalog,
        &visible_releases
    );

    let index_vcentered = if
        artist.image.is_none() &&
        artist.releases.len() <= 2 &&
        artist.text.as_ref().is_some_and(|html_and_stripped| html_and_stripped.stripped.len() <= 1024) {
        "index_vcentered"
    } else {
        ""
    };

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {artist_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
        {templates}
    "##);

    let breadcrumbs = &[
        format!(r#"<a href="">{artist_name_escaped}</a>"#)
    ];

    let crawler_meta = if artist.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &artist.theme,
        &artist.name,
        breadcrumbs,
        crawler_meta
    )
}
