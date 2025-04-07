// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

//! RSS 2.0 specification for reference:
//! https://www.rssboard.org/rss-specification

use std::fs;

use crate::{
    Build,
    Catalog,
    Release,
    SiteUrl
};

use super::Feeds;
use super::rss::rss;

pub fn generic_rss(build: &Build, catalog: &Catalog) {
    let base_url = build.base_url_unchecked();
    let url = base_url.join_file(Feeds::GENERIC_RSS_FILENAME);

    // The generic RSS feed just re-uses the generic RSS base markup in the
    // rss module, adding nothing at all.
    let channel_extensions = "";

    let extra_namespaces = &[];

    let xml = rss(
        base_url,
        build,
        catalog,
        channel_extensions,
        extra_namespaces,
        &mut item_extensions,
        &url
    );

    let path = build.build_dir.join(Feeds::GENERIC_RSS_FILENAME);
    fs::write(path, xml).unwrap();
}

pub fn item_extensions(
    _base_url: &SiteUrl,
    _build: &Build,
    _release: &Release
) -> String {
    // The generic RSS feed just re-uses the generic RSS base markup in the
    // rss module, adding nothing at all.
    String::new()
}
