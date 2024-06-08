// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 James Fenn
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cmp::Ordering;

use indoc::formatdoc;

use crate::{
    ArtistRc,
    Build,
    Catalog,
    DescribedImage,
    Release,
    ReleaseRc,
    Theme
};
use crate::icons;
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

pub mod artist;
pub mod image_descriptions;
pub mod index;
pub mod release;

pub enum CrawlerMeta {
    None,
    NoIndexNoFollow,
}

impl CrawlerMeta {
    pub fn tag(&self) -> &str {
        match self {
            CrawlerMeta::None => "",
            CrawlerMeta::NoIndexNoFollow => r#"<meta name="robots" content="noindex, nofollow">"#
        }
    }
}

fn artist_image(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    permalink: &str,
    described_image: &DescribedImage
) -> String {
    let image_ref = described_image.image.borrow();

    let alt = match &described_image.description {
        Some(description) => format!(r#" alt="{}""#, html_escape_inside_attribute(description)),
        None => String::new()
    };

    let poster_fixed_img = image_ref.artist_assets.as_ref().unwrap().img_attributes_fixed(permalink, root_prefix);
    let poster_fluid_img = image_ref.artist_assets.as_ref().unwrap().img_attributes_fluid(permalink, root_prefix);
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

    if described_image.description.is_some() {
        poster
    } else {
        wrap_undescribed_image(build, index_suffix, root_prefix, &poster, "", "home_image")
    }
}

fn compact_release_identifier(
    build: &Build,
    catalog: &Catalog,
    index_suffix: &str,
    release: &Release,
    release_link: &str,
    release_prefix: &str,
    root_prefix: &str
) -> String {
    let artists = list_artists(index_suffix, root_prefix, catalog, release);
    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let cover = cover_image_tiny(build, release_prefix, &release.cover, release_link);

    format!(r#"
        <div style="align-items: center; column-gap: .8rem; display: flex; margin: 2em 0;">
            <div style="max-width: 4rem">
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
    match &release.cover {
        Some (described_image) => {
            let image_ref = described_image.image.borrow();

            let alt = match &described_image.description {
                Some(description) => format!(r#" alt="{}""#, html_escape_inside_attribute(description)),
                None => String::new()
            };

            let thumbnail_img = image_ref.cover_assets.as_ref().unwrap().img_attributes_up_to_480(release_prefix);
            let thumbnail = formatdoc!(
                r##"
                    <a class="image" href="#overlay">
                        <img{alt} sizes="(min-width: 20rem) 20rem, calc(100vw - 2rem)" src="{src}" srcset="{srcset}">
                    </a>
                "##,
                src = thumbnail_img.src,
                srcset = thumbnail_img.srcset
            );

            let cover_ref = image_ref.cover_assets.as_ref().unwrap();
            let overlay_img = cover_ref.img_attributes_up_to_1280(release_prefix);
            let largest_edge_size = cover_ref.largest().edge_size;
            let overlay = formatdoc!(
                r##"
                    <a id="overlay" href="#">
                        <img
                            {alt}
                            height="{largest_edge_size}"
                            loading="lazy"
                            sizes="calc(100vmin - 4rem)"
                            src="{src}"
                            srcset="{srcset}"
                            width="{largest_edge_size}">
                    </a>
                "##,
                src = overlay_img.src,
                srcset = overlay_img.srcset
            );

            if described_image.description.is_some() {
                formatdoc!("
                    {thumbnail}
                    {overlay}
                ")
            } else {
                wrap_undescribed_image(build, index_suffix, root_prefix, &thumbnail, &overlay, "")
            }
        }
        None => {
            let t_auto_generated_cover = &build.locale.translations.auto_generated_cover;
            formatdoc!(r#"
                <span class="image">
                    <img alt="{t_auto_generated_cover}" src="{release_prefix}cover.svg">
                </span>
            "#)
        }
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
    match &release.cover {
        Some(described_image) => {
            let image_ref = described_image.image.borrow();

            let alt = match &described_image.description {
                Some(description) => format!(r#" alt="{}""#, html_escape_inside_attribute(description)),
                None => String::new()
            };

            let thumbnail_img = image_ref.cover_assets.as_ref().unwrap().img_attributes_up_to_320(release_prefix);
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

            if described_image.description.is_some() {
                thumbnail
            } else {
                wrap_undescribed_image(build, index_suffix, root_prefix, &thumbnail, "", "")
            }
        }
        None => {
            let t_auto_generated_cover = &build.locale.translations.auto_generated_cover;
            formatdoc!(r#"
                <a class="image" href="{href}">
                    <img alt="{t_auto_generated_cover}" src="{release_prefix}cover.svg"/>
                </a>
            "#)
        }
    }
}

fn cover_image_tiny(
    build: &Build,
    release_prefix: &str,
    image: &Option<DescribedImage>,
    href_url: &str
) -> String {
    match image {
        Some(described_image) => {
            let image_ref = described_image.image.borrow();
            let asset = &image_ref.cover_assets.as_ref().unwrap().max_160;
            let src = format!("{release_prefix}cover_{edge_size}.jpg", edge_size = asset.edge_size);

            let alt = if let Some(description) = &described_image.description {
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
        None => {
            let t_auto_generated_cover = &build.locale.translations.auto_generated_cover;
            formatdoc!(r#"
                <a class="image" href="{href_url}">
                    <img alt="{t_auto_generated_cover}" src="{release_prefix}cover.svg">
                </a>
            "#)
        }
    }
}

/// For pages that should not be indexed by crawlers (search engines etc.),
/// pass CrawlerMeta::NoIndexNoFollow, this adds a noindex and nofollow meta tag for crawlers.
fn layout(
    root_prefix: &str,
    body: &str,
    build: &Build,
    catalog: &Catalog,
    theme: &Theme,
    title: &str,
    breadcrumbs: &[String],
    crawler_meta: CrawlerMeta
) -> String {
    let feed_meta_link = match build.base_url.is_some() && catalog.feed_enabled {
        true => {
            let t_rss_feed = &build.locale.translations.rss_feed;
            format!(r#"<link rel="alternate" type="application/rss+xml" title="{t_rss_feed}" href="{root_prefix}feed.rss">"#)
        }
        false => String::new()
    };

    let dir_attribute = if build.locale.text_direction.is_rtl() { r#"dir="rtl""# } else { "" };

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
            link_h = catalog.theme.link_h,
            link_l = catalog.theme.link_l.unwrap_or(catalog.theme.base.link_l),
            link_s = catalog.theme.link_s.unwrap_or(catalog.theme.base.link_s),
            template = include_str!("templates/theming_widget.html"),
            text_h = catalog.theme.text_h,
            tint_back = catalog.theme.tint_back,
            tint_front = catalog.theme.tint_front
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
        breadcrumbs = breadcrumbs,
        catalog_title = html_escape_outside_attribute(&catalog.title()),
        crawler_meta = crawler_meta.tag(),
        dir_attribute = dir_attribute,
        favicon_links = catalog.favicon.header_tags(root_prefix),
        feed_meta_link = feed_meta_link,
        index_suffix = if build.clean_urls { "/" } else { "/index.html" },
        lang = &build.locale.language,
        root_prefix = root_prefix,
        theme_stylesheet_filename = theme.stylesheet_filename(),
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
    let mut main_artists_sorted: Vec<ArtistRc> = release.main_artists.clone();

    // Sort so the catalog artist comes first
    main_artists_sorted.sort_by(|a, b| {
        if let Some(catalog_artist) = &catalog.artist {
            if ArtistRc::ptr_eq(a, catalog_artist) { return Ordering::Less; }
            if ArtistRc::ptr_eq(b, catalog_artist) { return Ordering::Greater; }
        }
        Ordering::Equal
    });

    let main_artists = main_artists_sorted
        .iter()
        .map(|artist| {
            let artist_ref = artist.borrow();
            let name_escaped = html_escape_outside_attribute(&artist_ref.name);

            if artist_ref.unlisted {
                name_escaped
            } else if catalog.label_mode {
                let permalink = &artist_ref.permalink.slug;
                format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#)
            } else if let Some(catalog_artist) = &catalog.artist {
                if ArtistRc::ptr_eq(artist, catalog_artist) {
                    format!(r#"<a href="{root_prefix}.{index_suffix}">{name_escaped}</a>"#)
                } else {
                    name_escaped
                }
            } else {
                name_escaped
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    if catalog.feature_support_artists && !release.support_artists.is_empty() {
        let support_artists = release.support_artists
            .iter()
            .map(|artist| {
                let artist_ref = artist.borrow();
                let name_escaped = html_escape_outside_attribute(&artist_ref.name);

                if artist_ref.unlisted {
                    name_escaped
                } else {
                    let permalink = &artist_ref.permalink.slug;
                    format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#)
                }
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

/// These are rendered alongside the release player and provide prepared and translated
/// icons for the client side script to use.
pub fn player_icon_templates(build: &Build) -> String {
    let pause_icon = icons::pause(&build.locale.translations.pause);
    let play_icon = icons::play(&build.locale.translations.play);
    let loading_icon = icons::loading(&build.locale.translations.loading);

    formatdoc!(r#"
        <template id="pause_icon">
            {pause_icon}
        </template>
        <template id="play_icon">
            {play_icon}
        </template>
        <template id="loading_icon">
            {loading_icon}
        </template>
    "#)
}

fn releases(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    catalog: &Catalog,
    releases: &[ReleaseRc]
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
            let release_title_escaped = html_escape_outside_attribute(&release_ref.title);

            formatdoc!(r#"
                <div class="release">
                    <div class="cover_listing">
                        {cover}
                    </div>
                    <div>
                        <a href="{href}" style="color: var(--color-text); font-size: var(--subtly-larger);">
                            {release_title_escaped}
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
        String::from(r##"href="#share""##)
    } else {
        let t_share_not_available_requires_javascript = &build.locale.translations.share_not_available_requires_javascript;
        // In a javascript-enabled browser, some bootstrapping happens on DOM load:
        // - class="disabled" is removed
        // - title="..."  is removed
        // - href="#share" is added
        format!(r#"class="disabled" data-disabled-share title="{t_share_not_available_requires_javascript}""#)
    };

    let t_share = &build.locale.translations.share;
    let share_icon = icons::share(t_share);
    formatdoc!(r##"
        <a {attributes}>
            {share_icon}
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

pub fn unlisted_badge(build: &Build) -> String {
    let t_unlisted = &build.locale.translations.unlisted;
    format!(r#"<span class="unlisted">{t_unlisted}</span>"#)
}

fn wrap_undescribed_image(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    thumbnail: &str,
    overlay: &str,
    extra_class: &str
) -> String {
    let visual_impairment_icon = icons::visual_impairment(&build.locale.translations.visual_impairment);

    let t_image_descriptions_permalink = &build.locale.translations.image_descriptions_permalink;
    let t_missing_image_description_note = &build.locale.translations.missing_image_description_note;
    formatdoc!(r#"
        <div class="{extra_class} undescribed_wrapper">
            <a class="undescribed_icon" href="{root_prefix}{t_image_descriptions_permalink}{index_suffix}">
                {visual_impairment_icon}
            </a>
            <span class="undescribed_overlay">
                {t_missing_image_description_note}
            </span>
            {thumbnail}
        </div>
        {overlay}
    "#)
}
