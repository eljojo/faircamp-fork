use indoc::formatdoc;

use crate::{
    audio_format::prioritized_for_download,
    Build,
    Catalog,
    ImageFormat,
    Release,
    render::{image, layout, list_artists},
    util::html_escape_outside_attribute
};

pub fn download_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../";

    let (primary_format, sorted_formats) = prioritized_for_download(&release.download_formats);

    let download_column_headers = sorted_formats
        .iter()
        .enumerate()
        .map(|(index, (format, recommended))|
            format!(
                "<th>{name}&nbsp;<sup>{number}</sup>{recommendation}</th>",
                name = format.user_label(),
                number = index + 1,
                recommendation = if *recommended { "<br>(Recommended)" } else { "" }
            )
        )
        .collect::<Vec<String>>()
        .join("\n");

    let release_download_columns = sorted_formats
        .iter()
        .map(|(format, _recommended)|
            format!(
                r#"<td><a download href="{root_prefix}{filename}">⭳</a></td>"#,
                filename = release.cached_assets.get(*format).as_ref().unwrap().filename,
                root_prefix = root_prefix
            )
        )
        .collect::<Vec<String>>()
        .join("\n");

    let download_format_hints = sorted_formats
        .iter()
        .enumerate()
        .map(|(index, (format, recommended))|
            formatdoc!(
                r#"
                    <div class="format_hint">
                        <sup>{number}</sup> {label}: {description}{recommendation}
                    </div>
                "#,
                description = format.description(),
                label = format.user_label(),
                number = index + 1,
                recommendation = if *recommended { " (Recommended Format)" } else { "" }
            )
        )
        .collect::<Vec<String>>()
        .join("\n");

    let track_download_rows = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_download_columns = sorted_formats
                .iter()
                .map(|(format, _annotation)|
                    format!(
                        r#"<td><a download href="{root_prefix}{filename}">⭳</a></td>"#,
                        filename = track.cached_assets.get(*format).as_ref().unwrap().filename,
                        root_prefix = root_prefix
                    )
                )
                .collect::<Vec<String>>()
                .join("\n");

            formatdoc!(
                r#"
                    <tr>
                        <th class="download_option"
                            title="{number} {title}">
                            <span class="track_number">{number}</span> {title}
                        </th>
                        {track_download_columns}
                    </tr>
                "#,
                number = release.track_numbering.format(index + 1),
                title = html_escape_outside_attribute(&track.title),
                track_download_columns = track_download_columns
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let body = formatdoc!(
        r#"
            <div class="center">
                {cover}

                <h1>Download {title}</h1>
                <div>{artists}</div>

                <br><br>

                <!-- TODO: Download icon on button -->
                <a class="download_button" 
                   download
                   href="{root_prefix}{primary_download_filename}">
                    Download Entire Release<br>
                    {primary_download_format}{primary_download_format_recommendation}
                </a>

                <br><br>

                <h2>Other options</h2>

                <table class="download_options">
                    <tr>
                        <th></th>
                        {download_column_headers}
                    </tr>
                    <tr>
                        <th class="download_option">Entire Release (.zip)</th>
                        {release_download_columns}
                    </tr>
                    {track_download_rows}
                </table>

                <br><br>

                {download_format_hints}
            </div>
        "#,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = image(explicit_index, root_prefix, &release.cover, ImageFormat::Cover, None),
        primary_download_filename = release.cached_assets.get(primary_format.0).as_ref().unwrap().filename,
        primary_download_format = primary_format.0.user_label(),
        primary_download_format_recommendation = if primary_format.1 { " (Reommended Format)" } else { "" },
        release_download_columns = release_download_columns,
        root_prefix = root_prefix,
        title = html_escape_outside_attribute(&release.title),
        track_download_rows = track_download_rows
    );

    layout(root_prefix, &body, build, catalog, &release.title, None)
}
