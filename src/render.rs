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
    Theme,
    Track
};
use crate::icons;
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

pub mod artist;
pub mod image_descriptions;
pub mod index;
pub mod release;
pub mod track;

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

fn browser(catalog: &Catalog) -> String {
    // let public_releases: Vec<ReleaseRc> = catalog.releases
    //     .iter()
    //     .filter_map(|release| {
    //         match release.borrow().unlisted {
    //             true => None,
    //             false => Some(release.clone())
    //         }
    //     })
    //     .collect();

    // let r_releases = public_releases
    //     .iter()
    //     .map(|release| {
    //         release.borrow().title.clone()
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");

    let close_icon = icons::failure("Close"); // TODO: Revert removal of close translation and use
    formatdoc!(r#"
        <div id="browser">
            <div class="page">
                <div class="page_center">
                    <div>
                        <input type="search"> {close_icon}
                    </div>
                </div>
            </div>
        </div>
    "#)
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
    let artists_truncation = Some((40, format!("{release_prefix}#description")));
    let artists = list_release_artists(build, index_suffix, root_prefix, catalog, artists_truncation, release);
    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let cover = cover_image_tiny(build, release_prefix, &release.cover, release_link);

    format!(r#"
        <div style="align-items: center; column-gap: .8rem; display: flex; margin: 2em 0;">
            <div style="max-width: 4rem">
                {cover}
            </div>
            <div>
                <div style="font-size: 1.17rem;">{release_title_escaped}</div>
                <div style="font-size: 1.14rem;">{artists}</div>
            </div>
        </div>
    "#)
}

/// A button enriched with data attributes that client scripting can use
/// to copy the content (embed code or link) to clipboard and display success/failure state.
pub fn copy_button(content_key: &str, content_value: &str, copy_icon: &str, label: &str) -> String {
    formatdoc!(r##"
        <button data-{content_key}="{content_value}" data-copy>
            <span class="icon">{copy_icon}</span>
            <span>{label}</span>
        </button>
    "##)
}

/// Used on release/tracks pages to display a large-size cover
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
                Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
                None => String::new()
            };

            let thumbnail_img = image_ref.cover_assets.as_ref().unwrap().img_attributes_up_to_320(release_prefix);
            let thumbnail = formatdoc!(
                r##"
                    <a href="{href}">
                        <img
                            {alt}
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
                <a href="{href}">
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
                <a href="{href_url}">
                    <img{alt} loading="lazy" src="{src}">
                </a>
            "#)
        }
        None => {
            let t_auto_generated_cover = &build.locale.translations.auto_generated_cover;
            formatdoc!(r#"
                <a href="{href_url}">
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
    crawler_meta: CrawlerMeta
) -> String {
    let r_browser = browser(catalog);

    let feed_meta_link = match build.base_url.is_some() && catalog.feed_enabled {
        true => {
            let t_rss_feed = &build.locale.translations.rss_feed;
            format!(r#"<link rel="alternate" type="application/rss+xml" title="{t_rss_feed}" href="{root_prefix}feed.rss">"#)
        }
        false => String::new()
    };

    let dir_attribute = if build.locale.text_direction.is_rtl() { r#"dir="rtl""# } else { "" };

    let theming_widget = if build.theming_widget {
        let accent_brightening = &catalog.theme.accent_brightening;
        let accent_chroma = match &catalog.theme.accent_chroma {
            Some(chroma) => chroma.to_string(),
            None => String::from("null")
        };
        let accent_hue = match catalog.theme.accent_hue {
            Some(hue) => hue.to_string(),
            None => String::from("null")
        };
        let background_alpha = &catalog.theme.background_alpha;
        let base_chroma = &catalog.theme.base_chroma;
        let base_hue = catalog.theme.base_hue;

        let r_template = format!(
            include_str!("templates/theming_widget.html"),
            base = theme.base.label,
            background_1_lightness = theme.base.background_1_lightness,
            background_2_lightness = theme.base.background_2_lightness,
            background_3_lightness = theme.base.background_3_lightness,
            background_middleground_lightness = theme.base.background_middleground_lightness,
            foreground_1_lightness = theme.base.foreground_1_lightness,
            foreground_2_lightness = theme.base.foreground_2_lightness,
            foreground_3_lightness = theme.base.foreground_3_lightness,
            foreground_middleground_lightness = theme.base.foreground_middleground_lightness,
            middleground_lightness = theme.base.middleground_lightness,
            script = include_str!("assets/theming_widget.js")
        );

        formatdoc!(r#"
            <script>
                const ACCENT_BRIGHTENING = {accent_brightening};
                const ACCENT_CHROMA = {accent_chroma};
                const ACCENT_HUE = {accent_hue};
                const BACKGROUND_ALPHA = {background_alpha};
                const BASE_CHROMA = {base_chroma};
                const BASE_HUE = {base_hue};
            </script>
            {r_template}
        "#)
    } else {
        String::new()
    };

    format!(
        include_str!("templates/layout.html"),
        body = body,
        browser = r_browser,
        catalog_title = html_escape_outside_attribute(&catalog.title()),
        crawler_meta = crawler_meta.tag(),
        dir_attribute = dir_attribute,
        faircamp_icon = icons::faircamp(),
        favicon_links = catalog.favicon.header_tags(root_prefix),
        feed_meta_link = feed_meta_link,
        grid_icon = icons::grid(),
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
fn list_release_artists(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    catalog: &Catalog,
    truncation: Option<(usize, String)>,
    release: &Release
) -> String {
    // .1 is the char count of the name, .2 is either the plain name or a link to the artist
    let mut items: Vec<(usize, String)> = Vec::new();

    let mut main_artists_sorted: Vec<ArtistRc> = release.main_artists.clone();

    // Sort so the catalog artist comes first
    main_artists_sorted.sort_by(|a, b| {
        if let Some(catalog_artist) = &catalog.artist {
            if ArtistRc::ptr_eq(a, catalog_artist) { return Ordering::Less; }
            if ArtistRc::ptr_eq(b, catalog_artist) { return Ordering::Greater; }
        }
        Ordering::Equal
    });

    for artist in &main_artists_sorted {
        let artist_ref = artist.borrow();

        let name_chars = artist_ref.name.chars().count();
        let name_escaped = html_escape_outside_attribute(&artist_ref.name);

        if !artist_ref.unlisted {
            if catalog.label_mode {
                let permalink = &artist_ref.permalink.slug;
                let artist_link = format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#);
                items.push((name_chars, artist_link));
                continue;
            }

            if let Some(catalog_artist) = &catalog.artist {
                if ArtistRc::ptr_eq(artist, catalog_artist) {
                    let catalog_artist_link = format!(r#"<a href="{root_prefix}.{index_suffix}">{name_escaped}</a>"#);
                    items.push((name_chars, catalog_artist_link));
                    continue;
                }
            }
        }

        items.push((name_chars, name_escaped));
    }

    if catalog.feature_support_artists {
        for artist in &release.support_artists {
            let artist_ref = artist.borrow();
            let name_chars = artist_ref.name.chars().count();
            let name_escaped = html_escape_outside_attribute(&artist_ref.name);

            if artist_ref.unlisted {
                items.push((name_chars, name_escaped));
            } else {
                let permalink = &artist_ref.permalink.slug;
                let artist_link = format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#);
                items.push((name_chars, artist_link));
            }
        }
    } else if catalog.show_support_artists {
        for artist in &release.support_artists {
            let artist_ref = artist.borrow();
            let name_chars = artist_ref.name.chars().count();
            let name_escaped = html_escape_outside_attribute(&artist_ref.name);
            items.push((name_chars, name_escaped));
        }
    }

    truncate_artist_list(build, catalog, items, truncation)
}

/// Render the artists of a track in the style of "Alice, Bob", where each
/// (Alice, Bob) can be a link too, depending on the track and catalog.
/// In *label mode*, all artists of a track are shown and linked to
/// their artist page. In *artist mode*, only the catalog artist is ever
/// linked (to the site's homepage in this case). The catalog artist is
/// always sorted first.
fn list_track_artists(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    catalog: &Catalog,
    truncation: Option<(usize, String)>,
    track: &Track
) -> String {
    // .1 is the char count of the name, .2 is either the plain name or a link to the artist
    let mut items: Vec<(usize, String)> = Vec::new();

    let mut track_artists_sorted: Vec<ArtistRc> = track.artists.clone();

    // Sort so the catalog artist comes first
    track_artists_sorted.sort_by(|a, b| {
        if let Some(catalog_artist) = &catalog.artist {
            if ArtistRc::ptr_eq(a, catalog_artist) { return Ordering::Less; }
            if ArtistRc::ptr_eq(b, catalog_artist) { return Ordering::Greater; }
        }
        Ordering::Equal
    });

    for artist in &track_artists_sorted {
        let artist_ref = artist.borrow();

        let name_chars = artist_ref.name.chars().count();
        let name_escaped = html_escape_outside_attribute(&artist_ref.name);

        if !artist_ref.unlisted {
            if catalog.label_mode {
                let permalink = &artist_ref.permalink.slug;
                let artist_link = format!(r#"<a href="{root_prefix}{permalink}{index_suffix}">{name_escaped}</a>"#);
                items.push((name_chars, artist_link));
                continue;
            }

            if let Some(catalog_artist) = &catalog.artist {
                if ArtistRc::ptr_eq(artist, catalog_artist) {
                    let catalog_artist_link = format!(r#"<a href="{root_prefix}.{index_suffix}">{name_escaped}</a>"#);
                    items.push((name_chars, catalog_artist_link));
                    continue;
                }
            }
        }

        items.push((name_chars, name_escaped));
    }

    truncate_artist_list(build, catalog, items, truncation)
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
                let artists_truncation = Some((40, format!("{href}#description")));
                let list = list_release_artists(build, index_suffix, root_prefix, catalog, artists_truncation, &release_ref);
                format!(r#"<div class="release_artists">{list}</div>"#)
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
                    {cover}
                    <a href="{href}">
                        {release_title_escaped}
                    </a>
                    {artists}
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Pass in a Vec holding tuples containing the char count and plain name or
/// link to an artist each, alongside a possible truncation option
/// (character limit and a link that can be followed to see the truncated
/// artists). The list then gets truncated (if needed) and joined with ", ".
fn truncate_artist_list(
    build: &Build,
    catalog: &Catalog,
    items: Vec<(usize, String)>,
    truncation: Option<(usize, String)>
) -> String {
    if items.len() > 2 {
        if let Some((max_chars, others_link)) = truncation {
            let name_chars: usize = items.iter().map(|item| item.0).sum();
            let separator_chars = (items.len() - 1) * 2; // All separating ", " between the artists

            if name_chars + separator_chars > max_chars {
                // Here we have more than two artists, we have a char limit,
                // and we cannot fit all artists within the limit, thus
                // we truncate the list.

                if catalog.label_mode {
                    // In label mode we show at least one artist, then as many
                    // additional ones as fit, e.g. "[artist],[artist] and
                    // more"
                    let mut chars_used = 0;
                    let truncated_items = items
                        .into_iter()
                        .filter(|item| {
                            if chars_used == 0 {
                                chars_used += item.0;
                                return true;
                            }

                            chars_used += item.0;
                            chars_used < max_chars
                        });

                    let r_items = truncated_items.into_iter().map(|item| item.1).collect::<Vec<String>>().join(", ");
                    return build.locale.translations.xxx_and_others(&r_items, &others_link)
                }

                // In artist mode we show only "[catalog artist] and others".
                // Our sorting ensures the catalog artist is the first one,
                // so we can just take that.
                return build.locale.translations.xxx_and_others(&items[0].1, &others_link)
            }
        }
    }

    items.into_iter().map(|item| item.1).collect::<Vec<String>>().join(", ")
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
