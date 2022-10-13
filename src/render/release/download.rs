use indoc::formatdoc;

use crate::{
    audio_format::prioritized_for_download,
    Build,
    Catalog,
    Release,
    render::{cover_image, layout, list_artists},
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
            formatdoc!(
                r#"
                    <td>
                        <a download href="{root_prefix}{permalink}/{format}/{basename}.zip">
                            <img alt="Download" class="download_icon" src="{root_prefix}download.svg">
                        </a>
                    </td>
                "#,
                basename = release.asset_basename.as_ref().unwrap(),
                format = format.asset_dirname(),
                permalink = &release.permalink.slug
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
                    <div class="format_hint small_type">
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
                    formatdoc!(
                        r#"
                            <td>
                                <a download href="{root_prefix}{slug}/{format}/{basename}{extension}">
                                    <img alt="Download" class="download_icon" src="{root_prefix}download.svg">
                                </a>
                            </td>
                        "#,
                        basename = track.asset_basename.as_ref().unwrap(),
                        extension = format.extension(),
                        format = format.asset_dirname(),
                        slug = &release.permalink.slug
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
                title = html_escape_outside_attribute(&track.title)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = format!(
        "{root_prefix}{permalink}/",
        permalink = release.permalink.slug
    );

    let body = formatdoc!(
        r#"
            <div class="center">
                {cover}

                <h1>{title}</h1>
                <div>{artists}</div>

                <br><br>

                <a class="download_button" 
                   download
                   href="{root_prefix}{permalink}/{primary_download_format_dirname}/{primary_download_basename}.zip">
                    <img alt="Download" class="download_icon" src="{root_prefix}download_inverted.svg">
                    <div>
                        <span class="large_type">Download Entire Release</span><br>
                        <span class="small_type">{primary_download_format}{primary_download_format_recommendation}</span>
                    </div>
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
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        primary_download_basename = release.asset_basename.as_ref().unwrap(),
        primary_download_format = primary_format.0.user_label(),
        primary_download_format_dirname = primary_format.0.asset_dirname(),
        primary_download_format_recommendation = if primary_format.1 { " (Recommended Format)" } else { "" },
        permalink = &release.permalink.slug,
        title = html_escape_outside_attribute(&release.title)
    );

    layout(root_prefix, &body, build, catalog, &release.title, None)
}
