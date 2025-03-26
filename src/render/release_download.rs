// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadFormat,
    Release
};
use crate::util::html_escape_outside_attribute;

use super::Layout;
use super::{compact_release_identifier, download_entry};

/// The download page itself, providing direct links to the (zip) archive
/// files and/or individual tracks download links.
pub fn release_download_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../../";

    let mut layout = Layout::new();

    layout.no_indexing();

    let mut release_formats_sorted = release.download_formats.clone();
    release_formats_sorted.sort_by_key(|format| format.download_rank());

    let t_recommended_format =  &build.locale.translations.recommended_format;
    let download_hints = DownloadFormat::with_recommendation(&release_formats_sorted)
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

    let release_prefix = "../../";
    let release_link = format!("../..{index_suffix}");

    let compact_release_identifier_rendered = compact_release_identifier(
        build,
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let release_downloads = if !release_formats_sorted.is_empty() {
        let release_downloads = release_formats_sorted
            .iter()
            .map(|download_format| {
                let release_slug = &release.permalink.slug;

                let archive_filename = format!("{}.zip", release.asset_basename.as_ref().unwrap());

                let archive_hash = build.hash_with_salt(|hasher| {
                    release_slug.hash(hasher);
                    download_format.as_audio_format().asset_dirname().hash(hasher);
                    archive_filename.hash(hasher);
                });

                let archive_filename_urlencoded = urlencoding::encode(&archive_filename);

                let archives = release.archives.as_ref().unwrap();
                let format_dir = download_format.as_audio_format().asset_dirname().to_string();

                download_entry(
                    format!("{release_prefix}{format_dir}/{archive_hash}/{archive_filename_urlencoded}"),
                    download_format.user_label(),
                    archives.borrow().get_unchecked(*download_format).asset.filesize_bytes
                )
            })
            .collect::<Vec<String>>()
            .join("");

        formatdoc!(r#"
            <div class="download_formats" style="margin-bottom: 1rem;">
                {release_downloads}
            </div>
        "#)
    } else {
        String::new()
    };

    let extra_downloads = if release.extra_downloads.separate && (release.cover.is_some() || !release.extras.is_empty()) {
        let cover_entry = if let Some(described_image) = &release.cover {
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

        let extra_entries = if !release.extras.is_empty() {
            release.extras
                .iter()
                .map(|extra| {
                    let extra_hash = build.hash_with_salt(|hasher| {
                        release.permalink.slug.hash(hasher);
                        "extras".hash(hasher);
                        extra.sanitized_filename.hash(hasher);
                    });

                    let extra_filename_urlencoded = urlencoding::encode(&extra.sanitized_filename);

                    download_entry(
                        format!("{release_prefix}extras/{extra_hash}/{extra_filename_urlencoded}"),
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
        formatdoc!(r#"
            <div class="download_group">{t_extras}</div>

            <div class="download_formats" style="margin-bottom: 1rem;">
                {cover_entry}
                {extra_entries}
            </div>
        "#)
    } else {
        String::new()
    };

    let t_downloads = &build.locale.translations.downloads;
    let body = formatdoc!(
        r##"
            <div class="page">
                <div class="page_center">
                    <div style="max-width: 28rem;">
                        <h1>{t_downloads}</h1>

                        {compact_release_identifier_rendered}
                        {release_downloads}
                        {extra_downloads}

                        <div class="download_hints" id="hints">
                            {download_hints}
                        </div>
                    </div>
                </div>
            </div>
        "##
    );

    let release_title = &release.title;
    let release_title_escaped = html_escape_outside_attribute(release_title);

    layout.add_breadcrumb(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_downloads} – {release_title}");

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &release.theme,
        &page_title
    )
}
