// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// RSS 2.0 Specification for reference:
/// https://www.rssboard.org/rss-specification

use std::fs;

use crate::{
    Build,
    Catalog,
    FeedImageAsset,
    GENERATOR_INFO,
    SiteUrl
};
use crate::util::html_escape_outside_attribute;

pub fn generate(base_url: &SiteUrl, build: &Build, catalog: &Catalog) {
    let channel_items = catalog.releases
        .iter()
        .filter(|release| !release.borrow().unlisted)
        .map(|release| {
            let release_ref = release.borrow();
            let release_slug = &release_ref.permalink.slug;

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

            let artists_and_title = format!("{artists_list} – {}", release_ref.title);

            let item_description = if let Some(synopsis) = &release_ref.synopsis {
                format!("<description>{synopsis}</description>")
            } else if let Some(html_and_stripped) = &release_ref.more {
                let more_stripped = html_escape_outside_attribute(html_and_stripped.stripped.as_str());
                format!("<description>{more_stripped}</description>")
            } else {
                String::new()
            };



            let link = base_url.join_index(build, release_slug);

            format!(
                include_str!("templates/feed/item.xml"),
                description = item_description,
                link = link,
                title = html_escape_outside_attribute(&artists_and_title)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let channel_description = if let Some(synopsis) = &catalog.synopsis {
        format!("<description>{synopsis}</description>")
    } else if let Some(html_and_stripped) = &catalog.more {
        let more_stripped = html_escape_outside_attribute(html_and_stripped.stripped.as_str());
        format!("<description>{more_stripped}</description>")
    } else {
        // TODO: Eventually find something better to fallback to.
        // Note that this is a mandatory field in RSS (https://www.rssboard.org/rss-specification#requiredChannelElements)
        let version_id = env!("FAIRCAMP_VERSION_DISPLAY");
        format!("Faircamp {version_id}")
    };

    let channel_title = catalog.title();
    let channel_title_escaped = html_escape_outside_attribute(&channel_title);

    let link = base_url.index(build);

    let channel_image = if let Some(home_image) = &catalog.home_image {
        let description_or_channel_title = match &home_image.description {
            Some(description) => html_escape_outside_attribute(description),
            None => channel_title_escaped.clone()
        };

        let image_ref = home_image.borrow();

        let hash = image_ref.hash.as_url_safe_base64();
        let feed_asset = image_ref.feed_asset_unchecked();

        let filename = FeedImageAsset::TARGET_FILENAME;
        let edge_size = feed_asset.edge_size;

        format!(
            include_str!("templates/feed/image.xml"),
            height = edge_size,
            link = link,
            title = description_or_channel_title,
            url = base_url.join_file(&format!("{filename}?{hash}")),
            width = edge_size
        )
    } else {
        String::new()
    };

    let xml = format!(
        include_str!("templates/feed/channel.xml"),
        description = channel_description,
        feed_url = base_url.join_file("feed.rss"),
        GENERATOR_INFO = GENERATOR_INFO,
        image = channel_image,
        items = channel_items,
        last_build_date = build.build_begin.to_rfc2822(),
        link = link,
        language = build.locale.language,
        title = channel_title_escaped
    );

    fs::write(build.build_dir.join("feed.rss"), xml).unwrap();
}
