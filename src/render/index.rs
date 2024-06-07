// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Build, Catalog, CrawlerMeta, ReleaseRc};
use crate::icons;
use crate::render::{artist_image, layout, releases, share_link, share_overlay};
use crate::util::html_escape_outside_attribute;

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let feed_link = match &build.base_url.is_some() {
        true => {
            let t_feed = &build.locale.translations.feed;
            let feed_icon = icons::feed(&build.locale.translations.rss_feed);
            format!(r#"
                <a href="{root_prefix}feed.rss">
                    {feed_icon}
                    <span>{t_feed}</span>
                </a>
            "#)
        }
        false => String::new()
    };

    let share_link_rendered = share_link(build);

    let catalog_text = match &catalog.text {
        Some(html_and_stripped) => html_and_stripped.html.as_str(),
        None => ""
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

    let mut action_links = String::new();

    if !feed_link.is_empty() {
        action_links.push_str(&feed_link);
        action_links.push_str(" &nbsp; ");
    }

    action_links.push_str(&share_link_rendered);

    let catalog_info = formatdoc!(r##"
        <div class="catalog">
            {home_image}
            <div class="catalog_info_padded">
                <h1 style="font-size: var(--largest); margin-top: 1rem;">
                    {title_escaped}
                </h1>
                {catalog_text}
                <div class="action_links">
                    {action_links}
                </div>
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

    // TODO: catalog_text criterium is a bit random because the character
    //       count includes markup, we should improve this. In the end this
    //       criterium will always be a bit arbitrary though probably.
    let index_vcentered = if
        catalog.home_image.is_none() &&
        listed_releases.len() <= 2 &&
        catalog_text.len() <= 1024 {
        "index_vcentered"
    } else {
        ""
    };

    let share_url = match &build.base_url {
        Some(base_url) => base_url.join(build.index_suffix_file_only()).unwrap().to_string(),
        None => String::new()
    };

    let share_overlay_rendered = share_overlay(build, &share_url);

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {catalog_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
        {share_overlay_rendered}
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
