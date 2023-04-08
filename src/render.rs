use indoc::formatdoc;
use std::cell::RefCell;
use std::cmp::Ordering;
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
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    permalink: &str,
    image: &Rc<RefCell<Image>>
) -> String {
    let image_ref = image.borrow();
    let image_assets = image_ref.assets.borrow();

    let alt = match &image_ref.description {
        Some(description) => format!(r#" alt="{}""#, html_escape_inside_attribute(description)),
        None => String::new()
    };

    let poster_fixed_img = image_assets.artist.as_ref().unwrap().img_attributes_fixed(permalink, root_prefix);
    let poster_fluid_img = image_assets.artist.as_ref().unwrap().img_attributes_fluid(permalink, root_prefix);
    let poster = formatdoc!(
        r##"
            <span class="home_image">
                <picture>
                    <source media="(min-width: 60rem)"
                            sizes="27rem"
                            srcset="{srcset_fixed}" />
                    <source media="(min-width: 30rem)"
                            sizes="100vw"
                            srcset="{srcset_fluid}" />
                    <img{alt}
                        class="home_image"
                        sizes="100vw"
                        src="{src_fixed}"
                        srcset="{srcset_fixed}">
                </picture>
            </span>
        "##,
        src_fixed = poster_fixed_img.src,
        srcset_fixed = poster_fixed_img.srcset,
        srcset_fluid = poster_fluid_img.srcset
    );

    if image_ref.description.is_some() {
        poster
    } else {
        wrap_undescribed_image(build, index_suffix, root_prefix, &poster, "", "home_image")
    }
}

fn compact_release_identifier(
    catalog: &Catalog,
    index_suffix: &str,
    release: &Release,
    release_link: &str,
    release_prefix: &str,
    root_prefix: &str
) -> String {
    let artists = list_artists(index_suffix, root_prefix, catalog, release);
    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let cover = cover_image_tiny(release_prefix, &release.cover, release_link);

    format!(r#"
        <div style="align-items: center; column-gap: .8rem; display: flex; margin: 2em 0;">
            <div style="background-color: var(--color-cover); max-width: 4rem">
                {cover}
            </div>
            <div>
                <div style="font-size: var(--boldly-larger);">{release_title_escaped}</div>
                <div style="font-size: var(--subtly-larger);">{artists}</div>
            </div>
        </div>
    "#)
}


pub fn copy_button(build: &Build, content: Option<&str>) -> String {
    let data_content = match content {
        Some(content) => format!(r#"data-content="{content}""#),
        None => String::new()
    };

    let t_copied = &build.locale.translations.copied;
    let t_copy = &build.locale.translations.copy;
    let t_failed = &build.locale.translations.failed;
    let t_share_not_available_navigator_clipboard = &build.locale.translations.share_not_available_navigator_clipboard;

    formatdoc!(r#"
        <a class="button disabled" {data_content}data-copy title="{t_share_not_available_navigator_clipboard}">
            <span class="action">{t_copy}</span>
            <span class="success">{t_copied}</span>
            <span class="error">{t_failed}</span>
        </a>
    "#)
}

fn cover_image(
    build: &Build,
    index_suffix: &str,
    release_prefix: &str,
    root_prefix: &str,
    release: &Release
) -> String {
    let image = &release.cover;

    // TODO: Auto-generating cover descriptions that fit the generated cover artwork
    //       would be very cool, but with regards to needing translations too, right
    //       now out of scope. Should be solved generically for the time being.
    if image.is_none() {
        // Use generated cover
        return formatdoc!(r##"
            <span class="image">
                <img alt="Auto-generated cover image" src="cover.svg">
            </span>
        "##);
    }

    let image_ref = image.as_ref().unwrap().borrow();
    let image_assets = image_ref.assets.borrow();

    let alt = match &image_ref.description {
        Some(description) => format!(r#" alt="{}""#, html_escape_inside_attribute(description)),
        None => String::new()
    };

    let thumbnail_img = image_assets.cover.as_ref().unwrap().img_attributes_up_to_480(release_prefix);
    let thumbnail = formatdoc!(
        r##"
            <a class="image" href="#overlay">
                <img{alt} sizes="(min-width: 20rem) 20rem, calc(100vw - 2rem)" src="{src}" srcset="{srcset}">
            </a>
        "##,
        src = thumbnail_img.src,
        srcset = thumbnail_img.srcset
    );

    let overlay_img = image_assets.cover.as_ref().unwrap().img_attributes_up_to_1280(release_prefix);
    let overlay = formatdoc!(
        r##"
            <a id="overlay" href="#">
                <img {alt} loading="lazy" sizes="calc(100vmin - 4rem)" src="{src}" srcset="{srcset}">
            </a>
        "##,
        src = overlay_img.src,
        srcset = overlay_img.srcset
    );

    if image_ref.description.is_some() {
        formatdoc!("
            {thumbnail}
            {overlay}
        ")
    } else {
        wrap_undescribed_image(build, index_suffix, root_prefix, &thumbnail, &overlay, "")
    }
}

fn cover_tile_image(
    build: &Build,
    index_suffix: &str,
    release_prefix: &str,
    root_prefix: &str,
    release: &Release,
    href: &str
) -> String {
    let image = &release.cover;

    if image.is_none() {
        // Use generated cover
        return formatdoc!(r#"
            <a class="image" href="{href}">
                <img src="{release_prefix}cover.svg"/>
            </a>
        "#);
    }

    let image_ref = image.as_ref().unwrap().borrow();
    let image_assets = image_ref.assets.borrow();

    let alt = match &image_ref.description {
        Some(description) => format!(r#" alt="{}""#, html_escape_inside_attribute(description)),
        None => String::new()
    };

    let thumbnail_img = image_assets.cover.as_ref().unwrap().img_attributes_up_to_320(release_prefix);
    let thumbnail = formatdoc!(
        r##"
            <a class="image" href="{href}">
                <img{alt}
                    loading="lazy"
                    sizes="
                        (min-width: 60rem) 20rem,
                        (min-width: 30rem) calc((100vw - 4rem) * 0.333),
                        (min-width: 15rem) calc((100vw - 3rem) * 0.5),
                        calc(100vw - 2rem)
                    "
                    src="{src}"
                    srcset="{srcset}">
            </a>
        "##,
        src = thumbnail_img.src,
        srcset = thumbnail_img.srcset
    );

    if image_ref.description.is_some() {
        thumbnail
    } else {
        wrap_undescribed_image(build, index_suffix, root_prefix, &thumbnail, "", "")
    }
}

fn cover_image_tiny(
    release_prefix: &str,
    image: &Option<Rc<RefCell<Image>>>,
    href_url: &str
) -> String {
    if image.is_none() {
        return formatdoc!(r#"
            <a class="image" href="{href_url}">
                <img alt="Auto-generated cover image" src="{release_prefix}cover.svg">
            </a>
        "#);
    }

    let image_ref = image.as_ref().unwrap().borrow();
    let image_assets = image_ref.assets.borrow();

    let asset = &image_assets.cover.as_ref().unwrap().max_160;
    let src = format!("{release_prefix}cover_{edge_size}.jpg", edge_size = asset.edge_size);

    let alt = if let Some(description) = &image_ref.description {
        let alt = html_escape_inside_attribute(description);
        format!(r#" alt="{alt}""#)
    } else {
        String::new()
    };

    formatdoc!(r#"
        <a class="image" href="{href_url}">
            <img{alt} loading="lazy" src="{src}">
        </a>
    "#)
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
        true => {
            let t_rss_feed = &build.locale.translations.feed;
            format!(r#"<link rel="alternate" type="application/rss+xml" title="{t_rss_feed}" href="{root_prefix}feed.rss">"#)
        }
        false => String::new()
    };

    let dir_attribute = match build.locale.writing_direction {
        WritingDirection::Ltr => "",
        WritingDirection::Rtl => "dir=\"rtl\""
    };

    let theming_widget = if build.theming_widget {
        formatdoc!(
            r#"
                <script>
                    const LINK_H = {link_h};
                    const LINK_L = {link_l};
                    const LINK_S = {link_s};
                    const TEXT_H = {text_h};
                    const TINT_BACK = {tint_back};
                    const TINT_FRONT = {tint_front};
                </script>
                {template}
            "#,
            link_h = build.theme.link_h,
            link_l = build.theme.link_l.unwrap_or(build.theme.base.link_l),
            link_s = build.theme.link_s.unwrap_or(build.theme.base.link_s),
            template = include_str!("templates/theming_widget.html"),
            text_h = build.theme.text_h,
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
        index_suffix = if build.clean_urls { "/" } else { "/index.html" },
        feed_meta_link = feed_meta_link,
        breadcrumbs = breadcrumbs,
        root_prefix = root_prefix,
        theming_widget = theming_widget,
        title = html_escape_outside_attribute(title)
    )
}

/// Render the artists of a release in the style of "Alice, Bob", where each
/// (Alice, Bob) can be a link too, depending on the release and catalog.
/// In *label mode*, all main artists of a release are shown and linked to
/// their artist page. In *artist mode*, only the catalog artist is ever
/// linked (to the site's homepage in this case). Whether support artists are
/// listed depends on the catalog settings, by default they are not. The
/// catalog artist and main artists are always sorted first, in that order.
fn list_artists(
    index_suffix: &str,
    root_prefix: &str,
    catalog: &Catalog,
    release: &Release
) -> String {
    let mut main_artists_sorted: Vec<Rc<RefCell<Artist>>> = release.main_artists.clone();

    // Sort so the catalog artist comes first
    main_artists_sorted.sort_by(|a, b| {
        if let Some(catalog_artist) = &catalog.artist {
            if Rc::ptr_eq(a, catalog_artist) { return Ordering::Less; }
            if Rc::ptr_eq(b, catalog_artist) { return Ordering::Greater; }
        }
        Ordering::Equal
    });

    let main_artists = main_artists_sorted
        .iter()
        .map(|artist| {
            let artist_ref = artist.borrow();
            let name_escaped = html_escape_outside_attribute(&artist_ref.name);

            if catalog.label_mode {
                let permalink = &artist_ref.permalink.slug;
                return format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#);
            }

            if let Some(catalog_artist) = &catalog.artist {
                if Rc::ptr_eq(artist, catalog_artist) {
                    return format!(r#"<a href="{root_prefix}.{index_suffix}">{name_escaped}</a>"#);
                }
            }

            name_escaped
        })
        .collect::<Vec<String>>()
        .join(", ");

    if catalog.feature_support_artists && !release.support_artists.is_empty() {
        let support_artists = release.support_artists
                .iter()
                .map(|artist| {
                    let artist_ref = artist.borrow();
                    let name_escaped = html_escape_outside_attribute(&artist_ref.name);
                    let permalink = &artist_ref.permalink.slug;
                    format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#)
                })
                .collect::<Vec<String>>()
                .join(", ");

        format!("{main_artists}, {support_artists}")
    } else if catalog.show_support_artists && !release.support_artists.is_empty() {
        let support_artists = release.support_artists
                .iter()
                .map(|artist| html_escape_outside_attribute(&artist.borrow().name))
                .collect::<Vec<String>>()
                .join(", ");

        format!("{main_artists}, {support_artists}")
    } else {
        main_artists
    }
}

fn releases(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    catalog: &Catalog,
    releases: &[Rc<RefCell<Release>>]
) -> String {
    let mut releases_desc_by_date = releases.to_vec();

    releases_desc_by_date.sort_by_key(|release| release.borrow().date);

    releases_desc_by_date
        .iter()
        .rev()
        .map(|release| {
            let release_ref = release.borrow();
            let permalink = &release_ref.permalink.slug;

            let href = format!("{root_prefix}{permalink}{index_suffix}");

            let artists = if catalog.label_mode {
                let list = list_artists(index_suffix, root_prefix, catalog, &release_ref);
                format!("<div>{list}</div>")
            } else {
                String::new()
            };

            let release_prefix = format!("{root_prefix}{permalink}/");

            let cover = cover_tile_image(
                build,
                index_suffix,
                &release_prefix,
                root_prefix,
                &release_ref,
                &href
            );
            let release_title = html_escape_outside_attribute(&release_ref.title);

            formatdoc!(r#"
                <div class="release">
                    <div class="cover_listing">
                        {cover}
                    </div>
                    <div>
                        <a href="{href}" style="color: var(--color-text); font-size: var(--subtly-larger);">
                            {release_title}
                        </a>
                        {artists}
                    </div>
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn share_link(build: &Build) -> String {
    let attributes = if build.base_url.is_some() {
        format!(r##"href="#share""##)
    } else {
        let t_share_not_available_requires_javascript = &build.locale.translations.share_not_available_requires_javascript;
        // In a javascript-enabled browser, some bootstrapping happens on DOM load:
        // - class="disabled" is removed
        // - title="..."  is removed
        // - href="#share" is added
        format!(r#"class="disabled" data-disabled-share title="{t_share_not_available_requires_javascript}""#)
    };

    // TODO: Provide all icons as SHARE_ICON etc. through crate::render?
    //       (or share_icon(...) so we can inject translated alt texts or such)
    let icon_share = include_str!("icons/share.svg");
    let t_share = &build.locale.translations.share;
    formatdoc!(r##"
        <a {attributes}>
            {icon_share}
            <span>{t_share}</span>
        </a>
    "##)
}

pub fn share_overlay(build: &Build, url: &str) -> String {
    let r_copy_button = copy_button(build, None);
    let t_close = &build.locale.translations.close;

    formatdoc!(r##"
        <div id="share">
            <div class="inner">
                <a data-url href="{url}">{url}</a>
                {r_copy_button}
                <a class="button" href="#!">{t_close}</a>
            </div>
        </div>
    "##)
}

fn wrap_undescribed_image(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    thumbnail: &str,
    overlay: &str,
    extra_class: &str
) -> String {
    let t_image_descriptions_permalink = &build.locale.translations.image_descriptions_permalink;
    let t_missing_image_description_note = &build.locale.translations.missing_image_description_note;
    formatdoc!(r#"
        <div class="{extra_class} undescribed_wrapper">
            <a class="undescribed_icon" href="{root_prefix}{t_image_descriptions_permalink}{index_suffix}">
                <img alt="Visual Impairment" src="{root_prefix}visual_impairment.svg">
            </a>
            <span class="undescribed_overlay">
                {t_missing_image_description_note}
            </span>
            {thumbnail}
        </div>
        {overlay}
    "#)
}
