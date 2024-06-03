// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::RefCell;
use std::rc::Rc;

use indoc::formatdoc;

use crate::{Artist, Build, Catalog, CrawlerMeta, Release};
use crate::render::{artist_image, layout, releases, share_link, share_overlay};
use crate::util::html_escape_outside_attribute;

pub fn artist_html(build: &Build, artist: &Artist, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";

    let artist_text = match &artist.text {
        Some(html_and_stripped) => html_and_stripped.html.as_str(),
        None => ""
    };

    let artist_name_escaped = html_escape_outside_attribute(&artist.name);
    let share_link_rendered = share_link(build);

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

    let artist_info = formatdoc!(r##"
        <div class="catalog">
            {artist_image_rendered}
            <div class="catalog_info_padded">
                <h1>{artist_name_escaped}</h1>
                {artist_text}
                <div class="action_links">
                    {share_link_rendered}
                </div>
            </div>
        </div>
    "##);

    // If all releases are unlisted, all releases are visible on the artist page,
    // because the artist page itself is then unlisted itself. If however a single
    // release is listed, all unlisted releases become invisible on the artist page.
    let visible_releases: Vec<Rc<RefCell<Release>>> = if artist.unlisted {
        artist.releases.iter().cloned().collect()
    } else {
        artist.releases
            .iter()
            .filter_map(|release| {
                match release.borrow().unlisted {
                    true => None,
                    false => Some(release.clone())
                }
            })
            .collect()
    };

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

    let share_url = match &build.base_url {
        Some(base_url) => base_url
            .join(&format!("{}{index_suffix}", &artist.permalink.slug))
            .unwrap()
            .to_string(),
        None => String::new()
    };

    let share_overlay_rendered = share_overlay(build, &share_url);

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {artist_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
        {share_overlay_rendered}
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
        &artist.name,
        breadcrumbs,
        crawler_meta
    )
}
