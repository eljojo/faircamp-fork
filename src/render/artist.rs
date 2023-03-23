use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    render::{artist_image, layout, releases, share_link, share_overlay},
    util::html_escape_outside_attribute
};

pub fn artist_html(build: &Build, artist: &Artist, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";

    let artist_text = match &artist.text {
        Some(text) => text.clone(),
        None => String::new()
    };

    let artist_name_escaped = html_escape_outside_attribute(&artist.name);
    let share_link_rendered = share_link(build);

    let artist_image_rendered = match &artist.image {
        Some(artist_image_unpacked) => artist_image(
            build,
            index_suffix,
            root_prefix,
            &artist.permalink.slug,
            artist_image_unpacked
        ),
        None => String::new()
    };

    let artist_info = formatdoc!(r##"
        <div class="catalog">
            {artist_image_rendered}
            <div class="catalog_info_padded">
                <h1>{artist_name_escaped}</h1>
                {artist_text}
                {share_link_rendered}
            </div>
        </div>
    "##);

    let releases_rendered = releases(
        build,
        index_suffix,
        root_prefix,
        catalog,
        &artist.releases,
        false
    );

    // TODO: See note in index.rs and sync the solution between there and here
    let index_vcentered = if
        artist.image.is_none() &&
        artist.releases.len() <= 2 &&
        artist_text.len() <= 1024 {
        "index_vcentered"
    } else {
        ""
    };

    let share_url = match &build.base_url {
        Some(base_url) => base_url
            .join(&format!("{}{}", &artist.permalink.slug, index_suffix))
            .unwrap()
            .to_string(),
        None => String::new()
    };

    let share_overlay_rendered = share_overlay(build, &share_url);

    let body = formatdoc!(r##"
        <div class="index_split {index_vcentered}">
            {artist_info}
            <div class="releases" id="releases">
                {releases_rendered}
            </div>
        </div>
        {share_overlay_rendered}
    "##);

    let breadcrumbs = &[
        format!(r#"<a href=".{index_suffix}">{artist_name_escaped}</a>"#)
    ];

    layout(root_prefix, &body, build, catalog, &artist.name, breadcrumbs)
}
