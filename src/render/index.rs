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

    let feed_link = match &build.base_url.is_some() {
        true => format!(r#"<a href="{root_prefix}feed.rss"><img alt="RSS Feed" class="feed_icon" src="{root_prefix}feed.svg" style="display: none;">Feed</a>"#),
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

    let home_image = match &catalog.home_image {
        Some(_home_image) => {
            // TODO: sizes/srcset, alt etc.
            format!(r#"
                <img alt="TODO" class="home_image" src="home.jpg">
            "#)
        }
        None => String::new()
    };

    let catalog_info = formatdoc!(r##"
        <div class="catalog">
            {home_image}
            <div class="catalog_info_padded">
                <h1 style="color: #fff;">
                    {title_escaped}
                </h1>
                {catalog_text}
                {artists}
                {feed_link} &nbsp; <a href="#share">Share</a>
            </div>
        </div>
    "##);

    let releases_rendered = releases(
        explicit_index,
        root_prefix,
        &catalog,
        &catalog.releases,
        catalog.label_mode
    );

    // TODO: catalog_text criterium is a bit random because the character
    //       count includes markup, we should improve this. In the end this
    //       criterium will always be a bit arbitrary though probably.
    let index_vcentered = if catalog.releases.len() <= 2 &&
        catalog_text.len() <= 1024 {
        "index_vcentered"
    } else {
        ""
    };

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {catalog_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
        <a id="share" href="#">
            <span>https://example.org/share/example</span>
        </a>
    "##);

    layout(root_prefix, &body, build, catalog, &catalog_title, &[])
}
