use indoc::formatdoc;

use crate::{
    audio_format::prioritized_for_download,
    Build,
    Catalog,
    Release,
    render::{cover_image, layout, list_artists},
    util::html_escape_outside_attribute
};

const DOWNLOAD_LABEL_SEPARATOR: &str = " <span style=\"font-size: .8rem\">_</span> ";

pub fn download_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../../";

    let (primary_format, sorted_formats) = prioritized_for_download(&release.download_formats);

    let cover_download = if let Some(cover) = &release.cover {
        formatdoc!(
            r#"
                <div>
                    <span>Cover Image</span>
                    <span class="download_formats">
                        <a download href="{root_prefix}{permalink}/cover_{edge_size}.jpg">
                            JPEG
                        </a>
                    </span>
                </div>
            "#,
            edge_size = cover.borrow().assets.borrow().cover.as_ref().unwrap().largest().edge_size,
            permalink = &release.permalink.slug
        )
    } else {
        String::new()
    };

    let download_hints = sorted_formats
        .iter()
        .map(|(format, recommended)|
            formatdoc!(
                r#"
                    <small>
                        {user_label}{download_label}: {description}{recommendation}
                    </small>
                "#,
                description = format.description(),
                download_label = if format.download_label() == format.user_label() { String::new() } else { format!(r#" ({})"#, format.download_label()) },
                user_label = format.user_label(),
                recommendation = if *recommended { " (Recommended Format)" } else { "" }
            )
        )
        .collect::<Vec<String>>()
        .join("\n");

    let release_downloads = sorted_formats
        .iter()
        .map(|(format, _recommended)| {
            let archive_filename = format!(
                "{basename}.zip",
                basename = release.asset_basename.as_ref().unwrap()
            );

            let archive_hash = build.hash(
                &release.permalink.slug,
                format.asset_dirname(),
                &archive_filename
            );

            formatdoc!(
                r#"<a download href="{root_prefix}{permalink}/{format_dir}/{archive_hash}/{archive_filename}">{format_label}</a>"#,
                format_dir = format.asset_dirname(),
                format_label = format.download_label(),
                permalink = &release.permalink.slug
            )
        })
        .collect::<Vec<String>>()
        .join(DOWNLOAD_LABEL_SEPARATOR);

    let track_downloads = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_download_columns = sorted_formats
                .iter()
                .map(|(format, _annotation)| {
                    let track_filename = format!(
                        "{basename}{extension}",
                        basename = track.asset_basename.as_ref().unwrap(),
                        extension = format.extension()
                    ); 

                    let track_hash = build.hash(
                        &release.permalink.slug,
                        format.asset_dirname(),
                        &track_filename
                    );

                    format!(
                        r#"<a download href="{root_prefix}{slug}/{format_dir}/{track_hash}/{track_filename}">{format_label}</a>"#,
                        format_dir = format.asset_dirname(),
                        format_label = format.download_label(),
                        slug = &release.permalink.slug
                    )
                })
                .collect::<Vec<String>>()
                .join(DOWNLOAD_LABEL_SEPARATOR);

            formatdoc!(
                r#"
                    <div>
                        <span class="track_download_option">
                            <span class="track_number">{number}</span> {title}
                        </span>
                        <span class="download_formats">
                            {track_download_columns}
                        </span>
                    </div>
                "#,
                number = release.track_numbering.format(index + 1),
                title = html_escape_outside_attribute(&track.title)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = "../../";

    let primary_download_filename = format!(
        "{basename}.zip",
        basename = release.asset_basename.as_ref().unwrap()
    );

    let primary_download_hash = build.hash(
        &release.permalink.slug,
        primary_format.0.asset_dirname(),
        &primary_download_filename
    );

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let release_link = format!("../..{explicit_index}");

    let body = formatdoc!(
        r##"
            <div class="center_release mobile_hpadding">
                <h1>Download Release</h1>

                <br><br>

                <div style="align-items: center; display: flex;">
                    <div style="margin-right: .8rem; max-width: 4rem">
                        {cover}
                    </div>
                    <div>
                        <div>{release_title_escaped}</div>
                        <div>{artists}</div>
                    </div>
                </div>

                <br><br>

                <a class="download_button" 
                   download
                   href="{root_prefix}{permalink}/{primary_download_format_dirname}/{primary_download_hash}/{primary_download_filename}">
                    <img alt="Download" class="download_icon" src="{root_prefix}download.svg">
                    <div>
                        <span class="large_type">Entire Release</span><br>
                        <span class="small_type">{primary_download_format}{primary_download_format_recommendation}</span>
                    </div>
                </a>

                <br><br>

                <p>
                    Single track downloads or downloads in other formats are
                    available below. Not sure what format to pick? See the <a
                    href="#hints">hints</a> below.
                </p>

                <div class="download_options">
                    <div>
                        <span>Entire Release</span>
                        <span class="download_formats">{release_downloads}</span>
                    </div>
                    {cover_download}
                    {track_downloads}
                </div>

                <br><br>

                <div class="download_hints" id="hints">
                    <small>Format Guide:</small>

                    {download_hints}
                </div>

                <br><br><br>
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, Some(&release_link)),
        primary_download_format = primary_format.0.user_label(),
        primary_download_format_dirname = primary_format.0.asset_dirname(),
        primary_download_format_recommendation = if primary_format.1 { " (Recommended Format)" } else { "" },
        permalink = &release.permalink.slug
    );

    let breadcrumbs = &[
        format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#),
        format!("<span>Download</span>")
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}
