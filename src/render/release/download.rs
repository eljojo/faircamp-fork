use indoc::formatdoc;

use crate::{
    audio_format::prioritized_for_download,
    Build,
    Catalog,
    Release,
    render::{compact_release_identifier, layout},
    util::html_escape_outside_attribute
};

const DOWNLOAD_LABEL_SEPARATOR: &str = " <small>/</small> ";

pub fn download_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../../";

    let t_recommended_format =  &build.locale.translations.recommended_format;

    let (primary_format, sorted_formats) = prioritized_for_download(&release.download_formats);

    let t_cover_image = &build.locale.translations.cover_image;
    let cover_download = if let Some(cover) = &release.cover {
        formatdoc!(
            r#"
                <div>
                    <span>{t_cover_image}</span>
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
                description = format.description(build),
                download_label = if format.download_label() == format.user_label() { String::new() } else { format!(r#" ({})"#, format.download_label()) },
                user_label = format.user_label(),
                recommendation = if *recommended { format!(" ({t_recommended_format})") } else { String::new() }
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

    let release_link = format!("../..{index_suffix}");

    let compact_release_identifier_rendered = compact_release_identifier(
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let t_download = &build.locale.translations.download;
    let t_download_choice_hints = &build.locale.translations.download_choice_hints;
    let t_download_release = &build.locale.translations.download_release;
    let t_entire_release = &build.locale.translations.entire_release;
    let t_format_guide = &build.locale.translations.format_guide;
    let body = formatdoc!(
        r##"
            <div class="center_medium margin_page_bottom margin_page_top mobile_hpadding">
                <h1>{t_download_release}</h1>

                {compact_release_identifier_rendered}

                <div style="align-items: center; column-gap: .3rem; display: flex; justify-content: space-between; margin-bottom: 2rem;">
                    <div>
                        <span style="font-size: var(--subtly-larger);">{t_entire_release}</span><br>
                        <small>{primary_download_format}{primary_download_format_recommendation}</small>
                    </div>
                    <a class="button" 
                       download
                       href="{root_prefix}{permalink}/{primary_download_format_dirname}/{primary_download_hash}/{primary_download_filename}">
                       Download
                    </a>
                </div>

                <p>
                    {t_download_choice_hints}
                </p>

                <div class="download_options">
                    <div>
                        <span>{t_entire_release}</span>
                        <span class="download_formats">{release_downloads}</span>
                    </div>
                    {cover_download}

                    <br>

                    {track_downloads}
                </div>

                <br><br>

                <div class="download_hints" id="hints">
                    <small>{t_format_guide}</small>

                    {download_hints}
                </div>

                <br><br><br>
            </div>
        "##,
        primary_download_format = primary_format.0.user_label(),
        primary_download_format_dirname = primary_format.0.asset_dirname(),
        primary_download_format_recommendation = if primary_format.1 { format!(" ({t_recommended_format})") } else { String::new() },
        permalink = &release.permalink.slug
    );

    let breadcrumbs = &[
        format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#),
        format!("<span>{t_download}</span>")
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}
