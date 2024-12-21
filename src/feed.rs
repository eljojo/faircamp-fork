// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// RSS 2.0 Specification for reference:
/// https://www.rssboard.org/rss-specification

use std::fs;

use url::Url;

use crate::{Build, Catalog};
use crate::util::html_escape_outside_attribute;

pub fn generate(base_url: &Url, build: &Build, catalog: &Catalog) {
    let channel_items = catalog.releases
        .iter()
        .filter(|release| !release.borrow().unlisted)
        .map(|release| {
            let release_ref = release.borrow();

            let main_artists = release_ref.main_artists
                .iter()
                .map(|artist| artist.borrow().name.clone())
                .collect::<Vec<String>>()
                .join(", ");

            let artists_list = if catalog.show_support_artists && !release_ref.support_artists.is_empty() {
                let support_artists = release_ref.support_artists
                    .iter()
                    .map(|artist| artist.borrow().name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("{main_artists}, {support_artists}")
            } else {
                main_artists
            };

            let item_description = release_ref.more
                .as_ref()
                .map(|html_and_stripped|
                    format!(
                        "<description>{}</description>",
                        html_escape_outside_attribute(html_and_stripped.stripped.as_str())
                    )
                )
                .unwrap_or(String::new());

            let item_title = format!("{artists_list} – {}", release_ref.title);

            let index_suffix = build.index_suffix();
            let link = base_url.join(&format!("{}{index_suffix}", release_ref.permalink.slug)).unwrap();

            format!(
                include_str!("templates/feed/item.xml"),
                description = item_description,
                link = link,
                title = html_escape_outside_attribute(&item_title)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let channel_description = catalog.more
        .as_ref()
        .map(|html_and_stripped| html_escape_outside_attribute(html_and_stripped.stripped.as_str()))
        // TODO: Translate and/or reconsider content (e.g. integrate artist list in label mode?)
        // Note that this is a mandatory field in RSS (https://www.rssboard.org/rss-specification#requiredChannelElements)
        .unwrap_or(String::from("A faircamp-based music catalog"));

    let channel_title = catalog.title();

    let link = match build.clean_urls {
        true => base_url.clone(),
        false => base_url.join("index.html").unwrap()
    };

    let channel_image = if let Some(home_image) = &catalog.home_image {
        let hash = home_image.image.borrow().hash.as_url_safe_base64();

        format!(
            include_str!("templates/feed/image.xml"),
            image_url = base_url.join(&format!("feed.jpg?{hash}")).unwrap(),
            link = link,
            title = html_escape_outside_attribute(&channel_title)
        )
    } else {
        String::new()
    };

    let xml = format!(
        include_str!("templates/feed/channel.xml"),
        description = channel_description,
        feed_url = base_url.join("feed.rss").unwrap(),
        image = channel_image,
        items = channel_items,
        last_build_date = build.build_begin.to_rfc2822(),
        link = link,
        language = build.locale.language,
        title = html_escape_outside_attribute(&channel_title)
    );

    fs::write(build.build_dir.join("feed.rss"), xml).unwrap();
}
