// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

//! This module contains the building blocks for generating the generic rss,
//! media rss and podcast rss feeds. Each of those respective modules calls
//! this module's functions to render the shared base markup, while adding
//! specific extension markup through closures operating at the channel and
//! item level.
//!
//! RSS 2.0 specification for reference:
//! https://www.rssboard.org/rss-specification

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    FeedImageAsset,
    GENERATOR_INFO,
    Release,
    SiteUrl
};
use crate::util::{
    html_double_escape_outside_attribute,
    html_escape_outside_attribute
};

pub fn rss(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog,
    // Specific channel-level extension markup added by the caller (e.g. media
    // rss or podcast rss specific markup).
    channel_extensions: &str,
    // E.g. to pass specific media or podcast namespaces
    extra_namespaces: &[&str],
    // Specific item-level extension markup added by the caller (e.g. media
    // rss or podcast rss specific markup).
    item_extensions: &mut impl FnMut(&SiteUrl, &Build, &Release) -> String,
    feed_url: &str
) -> String {
    let items = catalog.public_releases()
        .iter()
        .map(|release| {
            item(
                base_url,
                build,
                catalog,
                item_extensions,
                &release.borrow()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let description = if let Some(synopsis) = &catalog.synopsis {
        html_double_escape_outside_attribute(synopsis)
    } else if let Some(html_and_stripped) = &catalog.more {
        html_escape_outside_attribute(html_and_stripped.html.as_str())
    } else {
        // TODO: Eventually find something better to fallback to.
        // Note that this is a mandatory field in RSS (https://www.rssboard.org/rss-specification#requiredChannelElements)
        format!("Faircamp {}", env!("FAIRCAMP_VERSION_DISPLAY"))
    };

    let link = base_url.index(build);

    let channel_title = html_double_escape_outside_attribute(&catalog.title());

    let image = if let Some(home_image) = &catalog.home_image {
        let image_ref = home_image.borrow();

        let hash = image_ref.hash.as_url_safe_base64();
        let feed_asset = image_ref.feed_asset_unchecked();

        let filename = FeedImageAsset::TARGET_FILENAME;
        let url = base_url.join_file(format!("{filename}?{hash}"));
        let edge_size = feed_asset.edge_size;

        let image_title = match &home_image.description {
            Some(description) => html_double_escape_outside_attribute(description),
            None => channel_title.clone()
        };

        formatdoc!(r#"
            <image>
                <height>{edge_size}</height>
                <link>{link}</link>
                <title>{image_title}</title>
                <url>{url}</url>
                <width>{edge_size}</width>
            </image>
        "#)
    } else {
        String::new()
    };

    let last_build_date = build.build_begin.to_rfc2822();
    let language = &build.locale.language;

    let extra_namespaces = extra_namespaces.join(" ");

    // Note that atom:link inside <channel> is not something that points to an
    // atom feed (we use it to point back to an RSS feed after all), but
    // instead just a piece of functionality we borrow from the atom
    // specification to include a link to the feed itself inside an RSS feed
    // (which natively does not offer such functionality). See also
    // https://www.rssboard.org/rss-profile#namespace-elements-atom-link
    formatdoc!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" {extra_namespaces}>
            <channel>
                <atom:link href="{feed_url}" rel="self" type="application/rss+xml"/>
                <description>{description}</description>
                <generator>{GENERATOR_INFO}</generator>
                {image}
                <language>{language}</language>
                <lastBuildDate>{last_build_date}</lastBuildDate>
                <link>{link}</link>
                <title>{channel_title}</title>
                {channel_extensions}
                {items}
            </channel>
        </rss>
    "#)
}

fn item(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog,
    // Specific item-level extension markup added by the caller (e.g. media
    // rss or podcast rss specific markup).
    item_extensions: &mut impl FnMut(&SiteUrl, &Build, &Release) -> String,
    release: &Release
) -> String {
    let release_slug = &release.permalink.slug;

    let main_artists = release.main_artists
        .iter()
        .map(|artist| artist.borrow().name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let artists_list = if catalog.show_support_artists && !release.support_artists.is_empty() {
        let support_artists = release.support_artists
            .iter()
            .map(|artist| artist.borrow().name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{main_artists}, {support_artists}")
    } else {
        main_artists
    };

    let artists_and_title = format!("{artists_list} – {}", release.title);

    let description = if let Some(synopsis) = &release.synopsis {
        let synopsis_escaped = html_double_escape_outside_attribute(synopsis);
        format!("<description>{synopsis_escaped}</description>")
    } else if let Some(html_and_stripped) = &release.more {
        let more_html_escaped = html_escape_outside_attribute(html_and_stripped.html.as_str());
        format!("<description>{more_html_escaped}</description>")
    } else {
        String::new()
    };

    let link = base_url.join_index(build, release_slug);

    let title = html_double_escape_outside_attribute(&artists_and_title);

    // Execute closure that may add e.g. media rss or podcast rss specific
    // markup.
    let extensions = item_extensions(base_url, build, release);

    formatdoc!(r#"
        <item>
            {description}
            <guid>{link}</guid>
            <link>{link}</link>
            <title>{title}</title>
            {extensions}
        </item>
    "#)
}
