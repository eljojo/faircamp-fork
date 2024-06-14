// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Build, Catalog, CrawlerMeta, ReleaseRc};
use crate::icons;
use crate::render::{
    artist_image,
    copy_button,
    layout,
    releases
};
use crate::util::html_escape_outside_attribute;

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let catalog_text = match &catalog.text {
        Some(html_and_stripped) => format!(
            r#"<div class="text">{}</div>"#,
            &html_and_stripped.html
        ),
        None => String::new()
    };

    // catalog.featured_artists is only populated in label mode, otherwise empty
    let featured_artists = if catalog.featured_artists.iter().any(|artist| !artist.borrow().unlisted) {
        let artist_links = catalog.featured_artists
            .iter()
            .filter(|artist| !artist.borrow().unlisted)
            .map(|artist| {
                let artist_ref = artist.borrow();
                let name = &artist_ref.name;
                let permalink = &artist_ref.permalink.slug;

                format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name}</a>"#)
            })
            .collect::<Vec<String>>()
            .join("");

        format!(r#"<div class="artists">{artist_links}</div>"#)
    } else {
        String::new()
    };

    let title_escaped = html_escape_outside_attribute(&catalog_title);

    let home_image = match &catalog.home_image {
        Some(home_image) => artist_image(
            build,
            index_suffix,
            root_prefix,
            "__home__", // TODO: Bad hack, solve properly
            home_image
        ),
        None => String::new()
    };

    let mut action_links = Vec::new();

    if build.base_url.is_some() && catalog.feed_enabled {
        let t_feed = &build.locale.translations.feed;
        let feed_icon = icons::feed(&build.locale.translations.rss_feed);

        let feed_link = format!(r#"
            <a href="{root_prefix}feed.rss">
                {feed_icon}
                <span>{t_feed}</span>
            </a>
        "#);

        action_links.push(feed_link);
    };

    if catalog.copy_link {
        let content = match &build.base_url {
            Some(base_url) => Some(base_url.join(build.index_suffix_file_only()).unwrap().to_string()),
            None => None
        };

        let t_copy_link = &build.locale.translations.copy_link;
        let r_copy_link = copy_button(build, content.as_deref(), t_copy_link);
        action_links.push(r_copy_link);
    }

    for link in &catalog.links {
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
            <div class="action_links">
                {joined}
            </div>
        "#)
    };

    let catalog_info = formatdoc!(r##"
        <div class="catalog">
            {home_image}
            <div class="catalog_info_padded">
                <h1 style="font-size: var(--largest); margin-top: 1rem;">
                    {title_escaped}
                </h1>
                {catalog_text}
                {r_action_links}
                {featured_artists}
            </div>
        </div>
    "##);

    let listed_releases: Vec<ReleaseRc> = catalog.releases
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
        &listed_releases
    );

    let index_vcentered = if
        catalog.home_image.is_none() &&
        listed_releases.len() <= 2 &&
        catalog.text.as_ref().is_some_and(|html_and_stripped| html_and_stripped.stripped.len() <= 1024) {
        "index_vcentered"
    } else {
        ""
    };

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {catalog_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
    "##);

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &catalog.theme,
        &catalog_title,
        &[],
        CrawlerMeta::None
    )
}
