// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Media RSS 1.5.1 Specification for reference:
//! https://www.rssboard.org/media-rss

use std::fs;

use crate::{
    Build,
    Catalog,
    Release,
    SiteUrl
};

use super::Feeds;
use super::rss::rss;

pub fn item_extensions(
    _base_url: &SiteUrl,
    _build: &Build,
    _release: &Release
) -> String {
    // TODO: Implement
    String::new()
}

pub fn media_rss(
    base_url: &SiteUrl,
    build: &Build,
    catalog: &Catalog
) {
    let url = base_url.join_file(Feeds::MEDIA_RSS_FILENAME);

    // TODO: Implement
    let channel_extensions = "";

    let extra_namespaces = &[r#"xmlns:media="http://search.yahoo.com/mrss/""#];

    let xml = rss(
        base_url,
        build,
        catalog,
        channel_extensions,
        extra_namespaces,
        &mut item_extensions,
        &url
    );

    let path = build.build_dir.join(Feeds::MEDIA_RSS_FILENAME);
    fs::write(path, xml).unwrap();
}
