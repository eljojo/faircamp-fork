// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Specification from the Podcast Standards Project as reference:
//! https://github.com/Podcast-Standards-Project/PSP-1-Podcast-RSS-Specification
//!
//! "A Podcaster’s Guide to RSS" from apple as reference:
//! https://help.apple.com/itc/podcasts_connect/#/itcb54353390

use std::fs;
use std::hash::Hash;

use uuid::Uuid;

use crate::{
    Build,
    Catalog,
    FeedImageAsset,
    Release,
    SiteUrl,
    TagMapping
};
use crate::util::{generic_hash, html_escape_outside_attribute};

use super::Feeds;
use super::rss::rss;

// Used to compute the podcast's guid.
// See https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid
const PODCAST_NAMESPACE_UUID: &str = "ead4c236-bf58-58c6-a2c6-a6b28d128cb6";

pub fn item_extensions(
    base_url: &SiteUrl,
    build: &Build,
    release: &Release
) -> String {
    let mut extensions = Vec::new();

    // enclosure

    let release_slug = &release.permalink.slug;

    // In the context of a podcast ("episode") we assume that the first track
    // is the only track of a release - podcast rss does not support anything
    // else either.
    let track = &release.tracks[0];
    let track_number: usize = 1;

    let format = track.streaming_quality.mp3_format();
    let format_dir = format.asset_dirname();
    let format_extension = format.extension();

    let basename = track.asset_basename.as_ref().unwrap();
    let track_filename = format!("{basename}{format_extension}");

    let track_hash = build.hash_with_salt(|hasher| {
        release_slug.hash(hasher);
        track_number.hash(hasher);
        format_dir.hash(hasher);
        track_filename.hash(hasher);
    });

    let track_filename_urlencoded = urlencoding::encode(&track_filename);
    let filepath = format!("{release_slug}/{track_number}/{format_dir}/{track_hash}/{track_filename_urlencoded}");
    let url = base_url.join_file(filepath);

    let source_type = format.source_type();

    let tag_mapping = TagMapping::new(release, track, track_number);

    let transcodes_ref = track.transcodes.borrow();
    let transcode = transcodes_ref.get_unchecked(format, generic_hash(&tag_mapping));

    let filesize_bytes = transcode.asset.filesize_bytes;

    let enclosure = format!(r#"<enclosure length="{filesize_bytes}" type="{source_type}" url="{url}"/>"#);

    extensions.push(enclosure);

    // itunes:duration

    let duration_seconds = transcodes_ref.source_meta.duration_seconds;
    let itunes_duration = format!(r#"<itunes:duration>{duration_seconds}</itunes:duration>"#);
    extensions.push(itunes_duration);

    // itunes:image

    let image_filename = if let Some(described_image) = &release.cover {
        let image_ref = described_image.borrow();

        // According to apple's specification "Artwork must be a minimum size
        // of 1400 x 1400 pixels and a maximum size of 3000 x 3000 pixels, in
        // JPEG or PNG format".
        let largest_cover_asset = image_ref.cover_assets_unchecked().largest();
        let filename = largest_cover_asset.target_filename();
        let hash = image_ref.hash.as_url_safe_base64();

        format!("{filename}?{hash}")
    } else {
        // TODO: "Confirm your art does not contain an Alpha Channel."
        // (https://help.apple.com/itc/podcasts_connect/#/itcb54353390)
        // In other words we need to ensure we also have versions of our covers
        // that have no alpha and are baked against a background color derived
        // from the theme (e.g.).
        release.procedural_cover_720_filename_unchecked()
    };

    let image_url = base_url.join_file(format!("{release_slug}/{image_filename}"));
    let itunes_image = format!(r#"<itunes:image href="{image_url}"/>"#);

    extensions.push(itunes_image);

    // TODO: Consider further items for future implementation:
    // https://github.com/Podcast-Standards-Project/PSP-1-Podcast-RSS-Specification?tab=readme-ov-file#required-item-elements

    extensions.join("\n")
}

pub fn podcast_rss(build: &Build, catalog: &Catalog) {
    let base_url = build.base_url_unchecked();
    let url = base_url.join_file(Feeds::PODCAST_RSS_FILENAME);

    let mut extensions = Vec::new();

    if !catalog.label_mode {
        if let Some(artist) = &catalog.artist {
            let name_escaped = html_escape_outside_attribute(&artist.borrow().name);

            let itunes_author = format!("<itunes:author>{name_escaped}</itunes:author>");

            extensions.push(itunes_author);
        }
    }

    if let Some(home_image) = &catalog.home_image {
        let image_ref = home_image.borrow();

        let hash = image_ref.hash.as_url_safe_base64();
        let filename = FeedImageAsset::TARGET_FILENAME;
        let url = base_url.join_file(format!("{filename}?{hash}"));

        let itunes_image = format!(r#"<itunes:image href="{url}"/>"#);

        extensions.push(itunes_image);
    }

    let namespace_uuid = Uuid::parse_str(PODCAST_NAMESPACE_UUID).unwrap();
    let normalized_url = base_url.without_scheme_and_trailing_slash();
    let guid = Uuid::new_v5(&namespace_uuid, normalized_url.as_bytes());
    let podcast_guid = format!(r#"<podcast:guid>{guid}</podcast:guid>"#);
    extensions.push(podcast_guid);

    // TODO: Consider futher items for future implementation:
    // https://github.com/Podcast-Standards-Project/PSP-1-Podcast-RSS-Specification?tab=readme-ov-file#required-channel-elements

    let channel_extensions = extensions.join("\n");

    let extra_namespaces = &[
        r#"xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd""#,
        r#"xmlns:podcast="https://podcastindex.org/namespace/1.0""#
    ];

    let xml = rss(
        base_url,
        build,
        catalog,
        &channel_extensions,
        extra_namespaces,
        &mut item_extensions,
        &url
    );

    let path = build.build_dir.join(Feeds::PODCAST_RSS_FILENAME);
    fs::write(path, xml).unwrap();
}
