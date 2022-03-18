use indoc::formatdoc;

use crate::{
    audio_format,
    build::Build,
    catalog::Catalog,
    release::Release,
    render::{cover, layout, list_artists}
};

pub fn download_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let root_prefix = &"../".repeat(2);

    let download_links = audio_format::sorted_and_annotated_for_download(&release.download_formats)
        .iter()
        .map(|(format, annotation)|
            formatdoc!(
                r#"
                    <div>
                        <a download href="../../{filename}">Download {label}{annotation}</a>
                    </div>
                "#,
                annotation=annotation.as_ref().map(|annotation| annotation.as_str()).unwrap_or(""),
                filename=release.cached_assets.get(format).as_ref().unwrap().filename,
                label=format.user_label()
            )
        )
        .collect::<Vec<String>>()
        .join("\n");

    let body = formatdoc!(
        r#"
            {cover}

            <h1>Download {title}</h1>
            <div>{artists}</div>

            {download_links}
        "#,
        artists = list_artists(root_prefix, &release.artists),
        cover = cover(root_prefix, release),
        download_links = download_links,
        title = release.title
    );

    layout(root_prefix, &body, build, catalog, &release.title)
}
