use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    Artist,
    Build,
    Catalog,
    Image,
    Release,
    util::{html_escape_inside_attribute, html_escape_outside_attribute},
    WritingDirection
};

pub mod artist;
pub mod image_descriptions;
pub mod index;
pub mod release;

fn play_icon(root_prefix: &str) -> String {
    format!(r#"<img alt="Play" src="{root_prefix}play.svg" style="max-width: 1rem;">"#)
}

fn artist_image(
    explicit_index: &str,
    root_prefix: &str,
    image: &Option<Rc<RefCell<Image>>>,
    href_url: Option<&str>
) -> String {
    if image.is_none() { return format!("<div></div>"); }

    let image_ref = image.as_ref().unwrap().borrow();

    let assets = image_ref.assets.borrow();
    let filename = &assets.artist.as_ref().unwrap().filename;

    let src_url = format!("{root_prefix}{filename}");
    let href_or_src_url = href_url.unwrap_or(&src_url);

    if let Some(description) = &image_ref.description {
        let alt = html_escape_inside_attribute(description);

        formatdoc!(r#"
            <a class="image" href="{href_or_src_url}">
                <img alt="{alt}" loading="lazy" src="{src_url}">
            </a>
        "#)
    } else {
        formatdoc!(r#"
            <div class="undescribed_wrapper">
                <div class="undescribed_corner_tag">
                    <img src="{root_prefix}corner_tag.svg">
                </div>
                <a class="undescribed_icon" href="{root_prefix}image-descriptions{explicit_index}">
                    <img alt="Visual Impairment"  src="{root_prefix}visual_impairment.svg">
                </a>
                <a class="undescribed_overlay" href="{root_prefix}image-descriptions{explicit_index}">
                    <span>Missing image description.<br>Click to learn more</span>
                </a>
                <a class="image" href="{href_or_src_url}">
                    <img loading="lazy" src="{src_url}">
                </a>
            </div>
        "#)
    }
}

fn cover_image(
    explicit_index: &str,
    release_prefix: &str,
    root_prefix: &str,
    image: &Option<Rc<RefCell<Image>>>,
    href_url: Option<&str>
) -> String {
    if image.is_none() { return format!("<div></div>"); }

    let image_ref = image.as_ref().unwrap().borrow();
    let image_assets = image_ref.assets.borrow();

    let img = image_assets.cover.as_ref().unwrap().img_attributes_up_to_360(release_prefix);

    let href_or_overlay_anchor = href_url.unwrap_or("#overlay");

    if let Some(description) = &image_ref.description {
        let alt = html_escape_inside_attribute(description);
        let overlay = if href_url.is_none() {
            let overlay_img = image_assets.cover.as_ref().unwrap().img_attributes_up_to_1080(release_prefix);

            formatdoc!(
                r##"
                    <a id="overlay" href="#">
                        <img alt="{alt}" loading="lazy" sizes="{sizes}" src="{src}" srcset="{srcset}">
                    </a>
                "##,
                sizes = overlay_img.sizes,
                src = overlay_img.src,
                srcset = overlay_img.srcset
            )
        } else {
            String::new()
        };

        formatdoc!(
            r#"
                <a class="image" href="{href_or_overlay_anchor}">
                    <img alt="{alt}" loading="lazy" sizes="{sizes}" src="{src}" srcset="{srcset}">
                </a>
                {overlay}
            "#,
            sizes = img.sizes,
            src = img.src,
            srcset = img.srcset
        )
    } else {
        let overlay = if href_url.is_none() {
            let overlay_img = image_assets.cover.as_ref().unwrap().img_attributes_up_to_1080(release_prefix);

            formatdoc!(
                r##"
                    <a id="overlay" href="#">
                        <img loading="lazy" sizes="{sizes}" src="{src}" srcset="{srcset}">
                    </a>
                "##,
                sizes = overlay_img.sizes,
                src = overlay_img.src,
                srcset = overlay_img.srcset
            )
        } else {
            String::new()
        };

        formatdoc!(
            r#"
                <div class="undescribed_wrapper">
                    <div class="undescribed_corner_tag">
                        <img src="{root_prefix}corner_tag.svg">
                    </div>
                    <a class="undescribed_icon" href="{root_prefix}image-descriptions{explicit_index}">
                        <img alt="Visual Impairment"  src="{root_prefix}visual_impairment.svg">
                    </a>
                    <a class="undescribed_overlay" href="{root_prefix}image-descriptions{explicit_index}">
                        <span>Missing image description.<br>Click to learn more</span>
                    </a>
                    <a class="image" href="{href_or_overlay_anchor}">
                        <img loading="lazy" sizes="{sizes}" src="{src}" srcset="{srcset}">
                    </a>
                </div>
                {overlay}
            "#,
            sizes = img.sizes,
            src = img.src,
            srcset = img.srcset
        )
    }
}

fn layout(
    root_prefix: &str,
    body: &str,
    build: &Build,
    catalog: &Catalog,
    title: &str,
    breadcrumbs: &[String]
) -> String {
    let feed_meta_link = match &build.base_url.is_some() {
        true => format!(r#"<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="{root_prefix}feed.rss">"#),
        false => String::new()
    };

    let dir_attribute = match build.localization.writing_direction {
        WritingDirection::Ltr => "",
        WritingDirection::Rtl => "dir=\"rtl\""
    };

    let theming_widget = if build.theming_widget {
        formatdoc!(
            r#"
                <script>
                    const HUE = {hue};
                    const HUE_SPREAD = {hue_spread};
                    const TINT_BACK = {tint_back};
                    const TINT_FRONT = {tint_front};
                </script>
                {template}
            "#,
            hue = build.theme.hue,
            hue_spread = build.theme.hue_spread,
            template = include_str!("templates/theming_widget.html"),
            tint_back = build.theme.tint_back,
            tint_front = build.theme.tint_front
        )
    } else {
        String::new()
    };

    let breadcrumbs = breadcrumbs
        .iter()
        .map(|link| format!(" <span>›</span> {link}"))
        .collect::<Vec<String>>()
        .join("");

    format!(
        include_str!("templates/layout.html"),
        body = body,
        catalog_title = html_escape_outside_attribute(&catalog.title()),
        dir_attribute = dir_attribute,
        explicit_index = if build.clean_urls { "/" } else { "/index.html" },
        feed_meta_link = feed_meta_link,
        breadcrumbs = breadcrumbs,
        root_prefix = root_prefix,
        theming_widget = theming_widget,
        title = html_escape_outside_attribute(title)
    )
}

fn list_artists(
    explicit_index: &str,
    root_prefix: &str,
    catalog: &Catalog,
    artists: &Vec<Rc<RefCell<Artist>>>
) -> String {
    artists
        .iter()
        .map(|artist| {
            let artist_ref = artist.borrow();
            let name_escaped = html_escape_outside_attribute(&artist_ref.name);

            if catalog.label_mode {
                let permalink = &artist_ref.permalink.slug;
                return format!(r#"<a href="{root_prefix}{permalink}{explicit_index}">{name_escaped}</a>"#);
            }

            if let Some(catalog_artist) = &catalog.artist {
                if Rc::ptr_eq(artist, catalog_artist) {
                    return format!(r#"<a href="{root_prefix}.{explicit_index}">{name_escaped}</a>"#);
                }
            }

            name_escaped
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn releases(
    explicit_index: &str,
    root_prefix: &str,
    catalog: &Catalog,
    releases: &Vec<Rc<RefCell<Release>>>,
    show_artists: bool
) -> String {
    releases
        .iter()
        .map(|release| {
            let release_ref = release.borrow();
            let permalink = &release_ref.permalink.slug;

            let href = format!("{root_prefix}{permalink}{explicit_index}");

            let artists = if show_artists {
                let list = list_artists(explicit_index, root_prefix, &catalog, &release_ref.artists);
                format!("<div>{list}</div>")
            } else {
                String::new()
            };

            let release_prefix = format!("{root_prefix}{permalink}/");

            let cover = cover_image(explicit_index, &release_prefix, root_prefix, &release_ref.cover, Some(&href));
            let release_title = html_escape_outside_attribute(&release_ref.title);

            formatdoc!(r#"
                <div class="release">
                    <div class="cover_listing">
                        {cover}
                    </div>
                    <div>
                        <a href="{href}" style="color: #fff;">{release_title}</a>
                        <div>{artists}</div>
                    </div>
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn share_link(build: &Build) -> String {
    match &build.base_url.is_some() {
        true => format!(r##"<a href="#share">Share</a>"##),
        // In a javascript-enabled browser, some bootstrapping happens on DOM load:
        // - class="disabled" is removed
        // - title="..."  is removed
        // - href="#share" is added
        false => format!(r##"<a class="disabled" data-disabled-share title="Not available in your browser (requires JavaScript)">Share</a>"##)
    }
}

pub fn share_overlay(url: &str) -> String {
    formatdoc!(r##"
        <div id="share">
            <div class="inner">
                <input data-url value="{url}">
                <a class="button disabled" data-copy title="Not available in your browser (navigator.clipboard is not supported)"><span class="action">Copy</span><span class="success">Copied</span><span class="error">Failed</span></a>
                <a class="button" href="#">Close</a>
            </div>
        </div>
    "##)
}