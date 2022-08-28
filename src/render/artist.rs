use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    ImageFormat,
    render::{SHARE_WIDGET, image, layout, releases}
};

pub fn artist_html(build: &Build, artist: &Artist, catalog: &Catalog) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../";

    let text = if let Some(text) = &artist.text {
        formatdoc!(
            r#"
                <div class="vpad">
                    {text}
                </div>
            "#,
            text = text
        )
    } else {
        String::new()
    };

    let body = formatdoc!(
        r#"
            <div class="center">
                <div class="cover">
                    {artist_image}
                </div>

                <div class="vpad">
                    <h1><a href="{root_prefix}.{explicit_index}">All Releases</a> &gt; {artist_name}</h1>
                </div>

                {text}

                {share_widget}

                {releases}
            </div>
        "#,
        artist_image = image(explicit_index, root_prefix, &artist.image, ImageFormat::Artist),
        artist_name = artist.name,
        explicit_index = explicit_index,
        releases = releases(explicit_index, root_prefix, &artist.releases),
        root_prefix = root_prefix,
        share_widget = SHARE_WIDGET,
        text = text
    );

    layout(root_prefix, &body, build, catalog, &artist.name)
}
