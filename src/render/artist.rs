use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    ImageFormat,
    render::{artist_image, layout, releases},
    util::html_escape_outside_attribute
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
                {releases}
            </div>
            <div class="additional">
                <div class="center">
                    <div class="cover">
                        {artist_image}
                    </div>

                    {artist_name}

                    <br><br>

                    {text}
                </div>
            </div>
        "#,
        artist_image = artist_image(explicit_index, root_prefix, &artist.image, ImageFormat::Artist, None),
        artist_name = html_escape_outside_attribute(&artist.name),
        releases = releases(explicit_index, root_prefix, &catalog, &artist.releases, false),
        text = text
    );

    layout(root_prefix, &body, build, catalog, &artist.name, None)
}
