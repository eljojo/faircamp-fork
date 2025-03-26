// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadFormat,
    Release,
    TagMapping,
    Track
};
use crate::util::{generic_hash, html_escape_outside_attribute};

use super::Layout;
use super::{compact_track_identifier, download_entry};

/// The download page itself, providing direct links to the (zip) archive
/// files and/or individual tracks download links.
pub fn track_download_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    track: &Track,
    track_number: usize
) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../../../";

    let mut layout = Layout::new();

    layout.no_indexing();

    let mut track_formats_sorted = track.download_formats.clone();
    track_formats_sorted.sort_by_key(|format| format.download_rank());

    let t_recommended_format =  &build.locale.translations.recommended_format;
    let download_hints = DownloadFormat::with_recommendation(&track_formats_sorted)
        .iter()
        .map(|(format, recommended)| {
            let description = format.description(build);
            let user_label = format.user_label();
            let recommendation = if *recommended { format!(" ({t_recommended_format})") } else { String::new() };
            formatdoc!("
                <div>
                    {user_label}: <span>{description}{recommendation}</span>
                </div>
            ")
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = "../../../";
    let track_link = format!("../..{index_suffix}");
    let track_prefix = "../../";

    let r_compact_track_identifier = compact_track_identifier(
        build,
        catalog,
        index_suffix,
        release,
        release_prefix,
        root_prefix,
        track,
        &track_link,
        track_prefix
    );

    let extra_downloads = if track.cover.is_some() || release.cover.is_some() || (track.extra_downloads && !track.extras.is_empty()) {
        let cover_entry = if let Some(described_image) = &track.cover {
            let image_ref = described_image.borrow();
            let largest_cover_asset = image_ref.cover_assets_unchecked().largest();
            let filename = largest_cover_asset.target_filename();

            download_entry(
                format!("{track_prefix}{filename}"),
                &build.locale.translations.cover_image,
                largest_cover_asset.filesize_bytes
            )
        } else if let Some(described_image) = &release.cover {
            let image_ref = described_image.borrow();
            let largest_cover_asset = image_ref.cover_assets_unchecked().largest();
            let filename = largest_cover_asset.target_filename();

            download_entry(
                format!("{release_prefix}{filename}"),
                &build.locale.translations.cover_image,
                largest_cover_asset.filesize_bytes
            )
        } else {
            String::new()
        };

        let extra_entries = if track.extra_downloads && !track.extras.is_empty() {
            track.extras
                .iter()
                .map(|extra| {
                    let extra_hash = build.hash_with_salt(|hasher| {
                        release.permalink.slug.hash(hasher);
                        track_number.hash(hasher);
                        "extras".hash(hasher);
                        extra.sanitized_filename.hash(hasher);
                    });

                    let extra_filename_urlencoded = urlencoding::encode(&extra.sanitized_filename);

                    download_entry(
                        format!("{track_prefix}extras/{extra_hash}/{extra_filename_urlencoded}"),
                        &extra.sanitized_filename,
                        extra.file_meta.size
                    )
                })
                .collect::<Vec<String>>()
                .join("")
        } else {
            String::new()
        };

        let t_extras = &build.locale.translations.extras;
        formatdoc!(
            r#"
                <div class="download_group">{t_extras}</div>

                <div class="download_formats" style="margin-bottom: 1rem;">
                    {cover_entry}
                    {extra_entries}
                </div>
            "#
        )
    } else {
        String::new()
    };

    let track_downloads = if !track_formats_sorted.is_empty() {
        let tag_mapping = TagMapping::new(release, track, track_number);

        let track_download_columns = track_formats_sorted
            .iter()
            .map(|download_format| {
                let track_filename = format!(
                    "{basename}{extension}",
                    basename = track.asset_basename.as_ref().unwrap(),
                    extension = download_format.as_audio_format().extension()
                );

                let track_hash = build.hash_with_salt(|hasher| {
                    release.permalink.slug.hash(hasher);
                    track_number.hash(hasher);
                    download_format.as_audio_format().asset_dirname().hash(hasher);
                    track_filename.hash(hasher);
                });

                let format_dir = download_format.as_audio_format().asset_dirname().to_string();
                let track_filename_urlencoded = urlencoding::encode(&track_filename);

                download_entry(
                    format!("{track_prefix}{format_dir}/{track_hash}/{track_filename_urlencoded}"),
                    download_format.user_label(),
                    track.transcodes.borrow().get_unchecked(download_format.as_audio_format(), generic_hash(&tag_mapping)).asset.filesize_bytes
                )
            })
            .collect::<Vec<String>>()
            .join("");

        let track_number_formatted = release.track_numbering.format(track_number);
        let track_title_escaped = html_escape_outside_attribute(&track.title());

        formatdoc!(r#"
            <div class="download_group">
                <span class="track_number">{track_number_formatted}</span> {track_title_escaped}
            </div>

            <div class="download_formats">
                {track_download_columns}
            </div>
        "#)
    } else {
        String::new()
    };

    let t_downloads = &build.locale.translations.downloads;
    let body = formatdoc!(r##"
        <div class="page">
            <div class="page_center">
                <div style="max-width: 28rem;">
                    <h1>{t_downloads}</h1>

                    {r_compact_track_identifier}
                    {track_downloads}
                    {extra_downloads}

                    <div class="download_hints" id="hints">
                        {download_hints}
                    </div>
                </div>
            </div>
        </div>
    "##);

    let release_link = format!("../../..{index_suffix}");
    let release_title_escaped = html_escape_outside_attribute(&release.title);

    layout.add_breadcrumb(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let track_title = track.title();
    let page_title = format!("{t_downloads} – {track_title}");

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &track.theme,
        &page_title
    )
}
