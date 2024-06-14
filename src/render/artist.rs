// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Artist, Build, Catalog, CrawlerMeta, ReleaseRc};
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
        Some(html_and_stripped) => html_and_stripped.html.as_str(),
        None => ""
    };

    let artist_name_escaped = html_escape_outside_attribute(&artist.name);

    let mut action_links = Vec::new();

    if artist.copy_link {
        let content = match &build.base_url {
            Some(base_url) => Some(
                base_url
                    .join(&format!("{}{index_suffix}", &artist.permalink.slug))
                    .unwrap()
                    .to_string()
            ),
            None => None
        };

        let t_copy_link = &build.locale.translations.copy_link;
        let r_copy_link = copy_button(build, content.as_deref(), t_copy_link);
        action_links.push(r_copy_link);
    }

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

    // TODO: See note in index.rs and sync the solution between there and here
    let index_vcentered = if
        artist.image.is_none() &&
        artist.releases.len() <= 2 &&
        artist_text.len() <= 1024 {
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
    "##);

    let breadcrumbs = &[
        format!(r#"<a href=".{index_suffix}">{artist_name_escaped}</a>"#)
    ];

    let crawler_meta = if artist.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &catalog.theme, // TODO: Should be artist.theme but we don't have that yet!
        &artist.name,
        breadcrumbs,
        crawler_meta
    )
}
