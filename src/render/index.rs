use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{layout, releases},
    util::html_escape_outside_attribute
};

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let more = if build.base_url.is_none() && catalog.text.is_none() {
        String::new()
    } else {
        let feed_link = match &build.base_url.is_some() {
            true => format!(r#"<a href="{root_prefix}feed.rss"><img alt="RSS Feed" class="feed_icon" src="{root_prefix}feed.svg"> RSS Feed</a>"#),
            false => String::new()
        };

        let catalog_text = match &catalog.text {
            Some(text) => text.clone(),
            None => String::new()
        };

        let artists = if catalog.label_mode && !catalog.artists.is_empty()  {
            let list = catalog.artists
                .iter()
                .map(|artist| {
                    let artist_ref = artist.borrow();
                    let name = &artist_ref.name;
                    let permalink = &artist_ref.permalink.slug;

                    format!(r#"<a href="{root_prefix}{permalink}{explicit_index}">{name}</a>"#)
                })
                .collect::<Vec<String>>()
                .join("<br>\n");

            formatdoc!(r#"
                <div style="max-width: 36rem;">
                    <strong>Artists</strong><br>
                    {list}
                </div>
            "#)
        } else {
            String::new()
        };

        let title_escaped = html_escape_outside_attribute(&catalog_title);

        formatdoc!(r#"
            <div class="additional">
                <div style="max-width: 36rem;">
                    <h1 style="color: #fff;">
                        {title_escaped}
                    </h1>
                    {catalog_text}
                    {artists}
                    {feed_link}
                </div>
            </div>
        "#)
    };

    let body = formatdoc!(
        r#"
            <div class="releases {releases_full_height}" id="releases">
                {releases}
            </div>
            {more}
        "#,
        releases = releases(explicit_index, root_prefix, &catalog, &catalog.releases, catalog.label_mode),
        releases_full_height = if more.is_empty() { "releases_full_height" } else { "" }
    );

    layout(root_prefix, &body, build, catalog, &catalog_title, &[])
}
