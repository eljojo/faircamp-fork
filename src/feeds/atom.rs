// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Atom specification for reference:
/// - https://validator.w3.org/feed/docs/atom.html (more approachable)
/// - https://datatracker.ietf.org/doc/html/rfc4287 (more formal)

use std::fs;

use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    FeedImageAsset,
    GENERATOR_INFO,
    Release,
    SiteUrl
};
use crate::util::html_escape_outside_attribute;

use super::Feeds;

pub fn atom(build: &Build, catalog: &Catalog) {
    let base_url = build.base_url_unchecked();
    let atom_feed_url = base_url.join_file(Feeds::ATOM_FILENAME);

    let author = if catalog.label_mode {
        String::new()
    } else if let Some(artist) = &catalog.artist {
        let name_escaped = html_escape_outside_attribute(&artist.borrow().name);

        formatdoc!(r#"
            <author>
                <name>{name_escaped}</name>
            </author>
        "#)
    } else {
        String::new()
    };

    // TODO: The atom standard specifies atom:updated to be an "instant in
    // time when an entry or feed was modified in a way the publisher
    // considers significant", however right now we only know about the build
    // time, and we never know when significant changes happened. Some form
    // of statefulness in faircamp could change this (for instance) - this
    // would also tie in with similar questions around RSS feeds - e.g. the
    // "published" fields there. (however in general we of course would prefer
    // to avoid statefulness, so this should be carefully weighed off)
    let build_begin = build.build_begin.to_rfc3339();

    let entries = catalog.public_releases()
        .iter()
        .map(|release| {
            entry(
                base_url,
                build,
                catalog,
                &release.borrow()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    // TODO: icon (optional field where we could use a faircamp favicon)
    // "Identifies a small image which provides iconic visual identification for the feed. Icons should be square."
    // <icon>/icon.jpg</icon>
    // (see https://validator.w3.org/feed/docs/atom.html#optionalFeedElements)

    let site_url = base_url.index(build);

    let logo = if let Some(home_image) = &catalog.home_image {
        let image_ref = home_image.borrow();

        let hash = image_ref.hash.as_url_safe_base64();
        let filename = FeedImageAsset::TARGET_FILENAME;
        let url = base_url.join_file(format!("{filename}?{hash}"));

        format!("<logo>{url}</logo>")
    } else {
        String::new()
    };

    let subtitle = if let Some(synopsis) = &catalog.synopsis {
        let synopsis_escaped = html_escape_outside_attribute(synopsis);
        format!("<subtitle>{synopsis_escaped}</subtitle>")
    } else {
        String::new()
    };

    let title_escaped = html_escape_outside_attribute(&catalog.title());

    let version_detailed = env!("FAIRCAMP_VERSION_DETAILED");
    let xml = formatdoc!(r#"
        <?xml version="1.0" encoding="utf-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
            {author}
            <generator uri="https://simonrepp.com/faircamp" version="{version_detailed}">
                {GENERATOR_INFO}
            </generator>
            <id>{site_url}</id>
            <link href="{atom_feed_url}" rel="self"/>
            {logo}
            {subtitle}
            <title>{title_escaped}</title>
            <updated>{build_begin}</updated>
            {entries}
        </feed>
    "#);

    let path = build.build_dir.join(Feeds::ATOM_FILENAME);
    fs::write(path, xml).unwrap();
}

fn entry(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog,
    release: &Release
) -> String {
    let release_slug = &release.permalink.slug;

    // TODO: Same as with the updated field on the feed itself - but yet more
    // critical here - we don't know the published date of releases
    // (published as in "first appearance on the site/feed" or "something
    // fundamental about the release was changed so it should be marked
    // as 'updated on xxx'" rather than "first published anywhere
    // (else potentially)"), and this again would require some form of
    // statefulness (if we don't rely on the site operator adding their own
    // published dates, which we probably shouldn't).
    let build_begin_rfc3339 = build.build_begin.to_rfc3339();

    let author = |artist: &Artist| -> String {
        let name_escaped = html_escape_outside_attribute(&artist.name);
        let uri = if artist.featured {
            let link = base_url.join_index(build, &artist.permalink.slug);
            format!("<uri>{link}</uri>")
        } else if let Some(link) = &artist.external_page {
            format!("<uri>{link}</uri>")
        } else {
            String::new()
        };

        formatdoc!(r#"
            <author>
                <name>{name_escaped}</name>
                {uri}
            </author>
        "#)
    };

    let mut authors = Vec::new();

    for artist in &release.main_artists {
        authors.push(author(&artist.borrow()));
    }

    if catalog.show_support_artists {
        for artist in &release.support_artists {
            authors.push(author(&artist.borrow()));
        }
    }

    let authors = authors.join("\n");

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

    // TODO: published (optional field which could either use the release `date` or a stateful/user-supplied publishing date we don't have yet)
    // "Contains the time of the initial creation or first availability of the entry."
    // <published>2003-12-13T09:17:51-08:00</published>
    // (see https://validator.w3.org/feed/docs/atom.html#optionalEntryElements)

    let release_url = base_url.join_index(build, release_slug);

    let summary = if let Some(synopsis) = &release.synopsis {
        let synopsis_escaped = html_escape_outside_attribute(synopsis);
        format!("<summary>{synopsis_escaped}</summary>")
    } else if let Some(html_and_stripped) = &release.more {
        let more_html_escaped = html_escape_outside_attribute(html_and_stripped.html.as_str());
        format!(r#"<summary type="html">{more_html_escaped}</summary>"#)
    } else {
        String::new()
    };

    let title_escaped = html_escape_outside_attribute(&artists_and_title);

    // TODO: Link can include things like language, type, enclosure (!), etc.,
    // we maybe should make use of that
    // (see https://validator.w3.org/feed/docs/atom.html#link)

    // TODO: Consider further fields to include in the future
    // See https://validator.w3.org/feed/docs/atom.html#recommendedEntryElements

    formatdoc!(r#"
        <entry>
            {authors}
            <id>{release_url}</id>
            <link href="{release_url}" rel="alternate"/>
            <title>{title_escaped}</title>
            {summary}
            <updated>{build_begin_rfc3339}</updated>
        </entry>
    "#)
}
