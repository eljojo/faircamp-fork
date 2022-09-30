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

    let more = if let Some(text) = &catalog.text {
        let artists = if catalog.label_mode && !catalog.artists.is_empty()  {
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

            formatdoc!(
                r#"
                    <div style="max-width: 36rem;">
                        <strong>Artists</strong><br>
                        {list}
                    </div>
                "#,
                list = list
            )
        } else {
            String::new()
        };

        formatdoc!(
            r#"
                <div class="additional" id="more">
                    <div style="max-width: 36rem;">
                        <a href=".{explicit_index}" style="color: #fff;">
                            {title}
                        </a>
                        {text}
                    </div>
                    {artists}
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
            <div class="releases {releases_full_height}" id="releases">
                {releases}
            </div>
            {more}
        "#,
        more = more,
        releases = releases(explicit_index, root_prefix, &catalog, &catalog.releases, catalog.label_mode),
        releases_full_height = if more.is_empty() { "releases_full_height" } else { "" }
    );

    let links = if more.is_empty() {
        None
    } else {
        Some(
            formatdoc!(
                r#"
                    <a href=".{explicit_index}#top" style="border-bottom: 1px solid #ccc;">Releases</a>
                    <a href=".{explicit_index}#more">More</a>
                "#,
                explicit_index = explicit_index
            )
        )
    };

    layout(root_prefix, &body, build, catalog, &catalog_title, links)
}
