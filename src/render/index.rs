use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{artist_image, layout, releases, share_link, share_overlay},
    util::html_escape_outside_attribute
};

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let feed_link = match &build.base_url.is_some() {
        true => {
            let t_feed = &build.locale.strings.feed;
            let t_rss_feed = &build.locale.strings.feed;
            format!(r#"<a href="{root_prefix}feed.rss"><img alt="{t_rss_feed}" class="feed_icon" src="{root_prefix}feed.svg" style="display: none;">{t_feed}</a>"#)
        }
        false => String::new()
    };

    let share_link_rendered = share_link(build);

    let catalog_text = match &catalog.text {
        Some(text) => text.clone(),
        None => String::new()
    };

    let artists = if catalog.label_mode && !catalog.artists.is_empty() {
        let artist_links = catalog.artists
            .iter()
            .map(|artist| {
                let artist_ref = artist.borrow();
                let name = &artist_ref.name;
                let permalink = &artist_ref.permalink.slug;

                format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name}</a>"#)
            })
            .collect::<Vec<String>>()
            .join("");

        format!(r#"<div class="artists">{artist_links}</div>"#)
    } else {
        String::new()
    };

    let title_escaped = html_escape_outside_attribute(&catalog_title);

    let home_image = match &catalog.home_image {
        Some(home_image) => artist_image(
            build,
            index_suffix,
            root_prefix,
            "__home__", // TODO: Bad hack, solve properly
            home_image
        ),
        None => String::new()
    };

    let mut action_links = String::new();

    if !feed_link.is_empty() {
        action_links.push_str(&feed_link);
        action_links.push_str(" &nbsp; ");
    }

    action_links.push_str(&share_link_rendered);

    let catalog_info = formatdoc!(r##"
        <div class="catalog">
            {home_image}
            <div class="catalog_info_padded">
                <h1 style="color: #fff; font-size: 1.8rem; margin-top: 1rem;">
                    {title_escaped}
                </h1>
                {catalog_text}
                <div style="font-size: 1.17rem;">
                    {action_links}
                </div>
                {artists}
            </div>
        </div>
    "##);

    let releases_rendered = releases(
        build,
        index_suffix,
        root_prefix,
        &catalog,
        &catalog.releases,
        catalog.label_mode
    );

    // TODO: catalog_text criterium is a bit random because the character
    //       count includes markup, we should improve this. In the end this
    //       criterium will always be a bit arbitrary though probably.
    let index_vcentered = if
        catalog.home_image.is_none() &&
        catalog.releases.len() <= 2 &&
        catalog_text.len() <= 1024 {
        "index_vcentered"
    } else {
        ""
    };

    let share_url = match &build.base_url {
        Some(base_url) => base_url.join(index_suffix).unwrap().to_string(),
        None => String::new()
    };

    let share_overlay_rendered = share_overlay(build, &share_url);

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {catalog_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
        {share_overlay_rendered}
    "##);

    layout(root_prefix, &body, build, catalog, &catalog_title, &[])
}
