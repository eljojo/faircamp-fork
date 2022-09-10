use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    Artist,
    Build,
    Catalog,
    Image,
    ImageFormat,
    Release,
    util::{format_time, html_escape_inside_attribute, html_escape_outside_attribute},
    WritingDirection
};

pub mod about;
pub mod artist;
pub mod image_descriptions;
pub mod release;
pub mod releases;

fn play_icon(root_prefix: &str) -> String {
    formatdoc!(
        r#"<img alt="Play" src="{root_prefix}play.svg" style="max-width: 1em;">"#,
        root_prefix = root_prefix
    )
}

fn image(explicit_index: &str, root_prefix: &str, image: &Option<Rc<RefCell<Image>>>, format: ImageFormat) -> String {
    match image {
        Some(image) => {
            let image_ref = image.borrow();

            if let Some(description) = &image_ref.description {
                formatdoc!(
                    r#"
                        <a class="image" href="{root_prefix}{filename}">
                            <img alt="{alt}" loading="lazy" src="{root_prefix}{filename}">
                        </a>
                    "#,
                    alt = html_escape_inside_attribute(description),
                    filename = image_ref.get_as(format).as_ref().unwrap().filename,
                    root_prefix = root_prefix
                )
            } else {
                formatdoc!(
                    r#"
                        <a class="image missing_image_description" href="{root_prefix}image-descriptions{explicit_index}">
                            <span class="missing_image_description_overlay">Missing image description.<br>Click to learn more</span>
                            <img loading="lazy" src="{root_prefix}{filename}">
                        </a>
                    "#,
                    explicit_index = explicit_index,
                    filename = image_ref.get_as(format).as_ref().unwrap().filename,
                    root_prefix = root_prefix
                )
            }
        },
        None => String::from(r#"<div></div>"#)
    }
}

fn layout(root_prefix: &str, body: &str, build: &Build, catalog: &Catalog, title: &str) -> String {
    let (feed_meta_link, feed_user_link) = match &build.base_url.is_some() {
        true => (
            format!(
                r#"<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="{root_prefix}feed.rss">"#,
                root_prefix = root_prefix
            ),
            format!(
                r#"<a href="{root_prefix}feed.rss">RSS</a>"#,
                root_prefix = root_prefix
            ),
        ),
        false => (String::new(), String::new())
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

    format!(
        include_str!("templates/layout.html"),
        body = body,
        catalog_title = html_escape_outside_attribute(&catalog.title()),
        dir_attribute = dir_attribute,
        explicit_index = if build.clean_urls { "/" } else { "/index.html" },
        feed_meta_link = feed_meta_link,
        feed_user_link = feed_user_link,
        root_prefix = root_prefix,
        theming_widget = theming_widget,
        title = html_escape_outside_attribute(title)
    )
}

fn list_artists(explicit_index: &str, root_prefix: &str, artists: &Vec<Rc<RefCell<Artist>>>) -> String {
    artists
        .iter()
        .map(|artist| {
            let artist_ref = artist.borrow();
            format!(
                r#"<a href="{root_prefix}{permalink}{explicit_index}">{name}</a>"#,
                explicit_index = explicit_index,
                name = html_escape_outside_attribute(&artist_ref.name),
                permalink = artist_ref.permalink.slug,
                root_prefix = root_prefix
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn releases(explicit_index: &str, root_prefix: &str, releases: &Vec<Rc<RefCell<Release>>>) -> String {
    releases
        .iter()
        .map(|release| {
            let release_ref = release.borrow();

            formatdoc!(
                r#"
                    <div class="vpad" style="display: flex;">
                        <a class="cover_listing" href="{root_prefix}{permalink}{explicit_index}">
                            {cover}
                        </a>
                        <div>
                            <a class="large" href="{root_prefix}{permalink}{explicit_index}" style="color: #fff;">{title} <span class="runtime">{runtime}</span></a>
                            <div>{artists}</div>
                        </div>
                    </div>
                "#,
                artists = list_artists(explicit_index, root_prefix, &release_ref.artists),
                cover = image(explicit_index, root_prefix, &release_ref.cover, ImageFormat::Cover),
                explicit_index = explicit_index,
                permalink = release_ref.permalink.slug,
                root_prefix = root_prefix,
                runtime = format_time(release_ref.runtime),
                title = html_escape_outside_attribute(&release_ref.title)
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
