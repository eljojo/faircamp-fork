// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    DownloadFormat,
    DownloadsConfig,
    Release,
    Scripts,
    TagMapping
};
use crate::render::{compact_release_identifier, layout};
use crate::util::{format_bytes, generic_hash, html_escape_outside_attribute};

fn download_entry(href: String, label: &str, size: u64) -> String {
    formatdoc!(
        r#"
            <div class="download_entry">
                <a download href="{href}">
                    {label}
                </a>
                <span class="download_underline"></span>
                <span>{size}</span>
            </div>
        "#,
        size = format_bytes(size)
    )
}

/// The download page itself, providing direct links to the (zip) archive
/// files and/or individual tracks download links.
pub fn download_html(
    build: &Build,
    catalog: &Catalog,
    downloads_config: &DownloadsConfig,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../../";

    let mut release_formats_sorted = downloads_config.release_formats.clone();
    release_formats_sorted.sort_by_key(|format| format.download_rank());

    let mut track_formats_sorted = downloads_config.track_formats.clone();
    track_formats_sorted.sort_by_key(|format| format.download_rank());

    let all_formats_sorted = downloads_config.all_formats_sorted();

    let t_recommended_format =  &build.locale.translations.recommended_format;
    let download_hints = DownloadFormat::with_recommendation(&all_formats_sorted)
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

                let archive_hash = build.hash_path_with_salt(
                    release_slug,
                    download_format.as_audio_format().asset_dirname(),
                    &archive_filename
                );

                let archive_filename_urlencoded = urlencoding::encode(&archive_filename);

                let archives = release.archives.as_ref().unwrap();

                download_entry(
                    format!(
                        "{root_prefix}{release_slug}/{format_dir}/{archive_hash}/{archive_filename_urlencoded}",
                        format_dir = download_format.as_audio_format().asset_dirname()
                    ),
                    download_format.user_label(),
                    archives.borrow().get_unchecked(*download_format).asset.filesize_bytes
                )
            })
            .collect::<Vec<String>>()
            .join("");        

        formatdoc!(
            r#"
                <div class="download_formats" style="margin-bottom: 1rem;">
                    {release_downloads}
                </div>
            "#
        )
    } else {
        String::new()
    };

    let extra_downloads = if downloads_config.extra_downloads.separate && (release.cover.is_some() || !release.extras.is_empty()) {
        let cover_entry = if let Some(described_image) = &release.cover {
            let image_ref = described_image.image.borrow();
            let largest_cover_asset = image_ref.cover_assets.as_ref().unwrap().largest();
            download_entry(
                format!(
                    "{root_prefix}{permalink}/cover_{edge_size}.jpg",
                    edge_size = largest_cover_asset.edge_size,
                    permalink = &release.permalink.slug
                ),
                &build.locale.translations.cover_image,
                largest_cover_asset.filesize_bytes
            )
        } else {
            String::new()
        };

        let extra_entries = if !release.extras.is_empty() {
            let release_slug = &release.permalink.slug;

            release.extras
                .iter()
                .map(|extra| {
                    let extra_hash = build.hash_path_with_salt(
                        release_slug,
                        "extras",
                        &extra.sanitized_filename
                    );

                    let extra_filename_urlencoded = urlencoding::encode(&extra.sanitized_filename);

                    download_entry(
                        format!("{root_prefix}{release_slug}/extras/{extra_hash}/{extra_filename_urlencoded}"),
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
        release.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let tag_mapping = TagMapping::new(release, track, track_index + 1);

                let track_download_columns = track_formats_sorted
                    .iter()
                    .map(|download_format| {
                        let release_slug = &release.permalink.slug;

                        let track_filename = format!(
                            "{basename}{extension}",
                            basename = track.asset_basename.as_ref().unwrap(),
                            extension = download_format.as_audio_format().extension()
                        ); 

                        let track_hash = build.hash_path_with_salt(
                            release_slug,
                            download_format.as_audio_format().asset_dirname(),
                            &track_filename
                        );

                        let track_filename_urlencoded = urlencoding::encode(&track_filename);

                        download_entry(
                            format!(
                                "{root_prefix}{release_slug}/{format_dir}/{track_hash}/{track_filename_urlencoded}",
                                format_dir = download_format.as_audio_format().asset_dirname()
                            ),
                            download_format.user_label(),
                            track.transcodes.borrow().get_unchecked(download_format.as_audio_format(), generic_hash(&tag_mapping)).asset.filesize_bytes
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("");

                let track_number_formatted = release.track_numbering.format(track_index + 1);
                let track_title_escaped = html_escape_outside_attribute(&track.title());

                formatdoc!(r#"
                    <div class="download_group">
                        <span class="track_number">{track_number_formatted}</span> {track_title_escaped}
                    </div>

                    <div class="download_formats">
                        {track_download_columns}
                    </div>
                "#)
            })
            .collect::<Vec<String>>()
            .join("\n")
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
                        {track_downloads}
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
    let breadcrumb = Some(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_downloads} – {release_title}");

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::None,
        &release.theme,
        &page_title,
        CrawlerMeta::NoIndexNoFollow,
        breadcrumb
    )
}
