use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{layout, releases},
    util::html_escape_outside_attribute
};

pub fn releases_html(build: &Build, catalog: &Catalog) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let catalog_pane = if let Some(text) = &catalog.text {
        let artists = if catalog.artists.is_empty() {
            String::new()
        } else {
            let list = catalog.artists
                .iter()
                .map(|artist| {
                    let artist_ref = artist.borrow();

                    format!(
                        r#"<a href="{root_prefix}{permalink}{explicit_index}">{name}</a>"#,
                        explicit_index = explicit_index,
                        name = artist_ref.name,
                        permalink = artist_ref.permalink.slug,
                        root_prefix = root_prefix,
                    )
                })
                .collect::<Vec<String>>()
                .join("<br>\n");

            format!("<br><strong>Artists</strong><br>{}", list)
        };

        formatdoc!(
            r#"
                <div class="split_side">
                    <div style="max-width: 36em; text-align: justify;">
                        <a href=".{explicit_index}" style="color: #fff;">
                            {title}
                        </a>
                        {text}
                        {artists}
                    </div>
                </div>
            "#,
            artists = artists,
            explicit_index = explicit_index,
            text = text,
            title = html_escape_outside_attribute(&catalog_title)
        )
    } else {
        String::new()
    };

    let body = formatdoc!(
        r#"
            <div class="split">
                <div class="releases split_main">
                    {releases}
                </div>
                {catalog_pane}
            </div>
        "#,
        catalog_pane = catalog_pane,
        releases = releases(explicit_index, root_prefix, &catalog.releases)
    );

    layout(root_prefix, &body, build, catalog, &catalog_title)
}
