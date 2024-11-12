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
    Scripts,
    Theme,
    Track
};
use crate::icons;
use crate::util::{html_escape_inside_attribute, html_escape_outside_attribute};

pub mod artist;
pub mod image_descriptions;
pub mod index;
pub mod release;
pub mod embed_release;
pub mod embed_track;
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
        Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
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
                    <img
                        {alt}
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

fn browser(build: &Build, root_prefix: &str) -> String {
    let close_icon = icons::failure(&build.locale.translations.close);
    let t_search = &build.locale.translations.search;

    formatdoc!(r#"
        <div id="browser" data-root-prefix="{root_prefix}">
            <div>
                <input autocomplete="off" placeholder="{t_search}" type="search">
                <div role="status"></div>
                <div id="results"></div>
            </div>
            <button>
                {close_icon}
            </button>
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
    let cover = cover_image_tiny_decorative(release_prefix, release, Some(release_link));

    format!(r#"
        <div class="release_compact">
            {cover}
            <div>
                <div style="font-size: 1.17rem;">
                    <a href="{release_link}">
                        {release_title_escaped}
                    </a>
                </div>
                <div class="artists" style="font-size: 1.14rem;">
                    {artists}
                </div>
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
        Some(described_image) => {
            let image_ref = described_image.image.borrow();

            let alt = match &described_image.description {
                Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
                None => String::new()
            };

            let thumbnail_img = image_ref.cover_assets.as_ref().unwrap().img_attributes_up_to_480(release_prefix);
            let thumbnail = formatdoc!(
                r##"
                    <a class="image" href="#overlay">
                        <img
                            {alt}
                            sizes="(min-width: 20rem) 20rem, calc(100vw - 2rem)"
                            src="{src}"
                            srcset="{srcset}">
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
            let procedural_cover_svg = release.procedural_cover.as_ref().unwrap();
            formatdoc!(r#"
                <span aria-hidden="true" class="image">
                    {procedural_cover_svg}
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
            let procedural_cover_svg = release.procedural_cover.as_ref().unwrap();
            formatdoc!(r#"
                <a aria-hidden="true" href="{href}">
                    {procedural_cover_svg}
                </a>
            "#)
        }
    }
}

fn cover_image_tiny_decorative(
    release_prefix: &str,
    release: &Release,
    release_link: Option<&str>
) -> String {
    let image = match &release.cover {
        Some(described_image) => {
            let image_ref = described_image.image.borrow();
            let asset = &image_ref.cover_assets.as_ref().unwrap().max_160;
            let src = format!("{release_prefix}cover_{edge_size}.jpg", edge_size = asset.edge_size);

            format!(r#"<img loading="lazy" src="{src}">"#)
        }
        None => {
            release.procedural_cover.as_ref().unwrap().to_string()
        }
    };

    match release_link {
        Some(release_link) => formatdoc!(r#"
            <a aria-hidden="true" href="{release_link}" tabindex="-1">
                {image}
            </a>
        "#),
        None => format!(r#"<span aria-hidden="true">{image}</span>"#)
    }
}

fn embed_layout(
    root_prefix: &str,
    body: &str,
    build: &Build,
    theme: &Theme,
    title: &str
) -> String {
    let dir_attribute = if build.locale.text_direction.is_rtl() { r#"dir="rtl""# } else { "" };

    format!(
        include_str!("templates/embed.html"),
        body = body,
        crawler_meta = CrawlerMeta::NoIndexNoFollow.tag(),
        dir_attribute = dir_attribute,
        lang = &build.locale.language,
        root_prefix = root_prefix,
        theme_stylesheet_filename = theme.stylesheet_filename(),
        title = html_escape_outside_attribute(title)
    )
}

/// For pages that should not be indexed by crawlers (search engines etc.),
/// pass CrawlerMeta::NoIndexNoFollow, this adds a noindex and nofollow meta tag for crawlers.
fn layout(
    root_prefix: &str,
    body: &str,
    build: &Build,
    catalog: &Catalog,
    extra_scripts: Scripts,
    theme: &Theme,
    title: &str,
    crawler_meta: CrawlerMeta,
    breadcrumb_option: Option<String>
) -> String {
    let r_browser = browser(build, root_prefix);

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
        let base = catalog.theme.base.to_key();
        let base_chroma = &catalog.theme.base_chroma;
        let base_hue = catalog.theme.base_hue;
        let build_begin = build.build_begin;
        let dynamic_range = catalog.theme.dynamic_range;

        let mut script = formatdoc!(r#"
            const BUILD_OPTIONS = {{
                'accent_brightening': {accent_brightening},
                'accent_chroma': {accent_chroma},
                'accent_hue': {accent_hue},
                'background_alpha': {background_alpha},
                'base': '{base}',
                'base_chroma': {base_chroma},
                'base_hue': {base_hue},
                'build_time': '{build_begin}',
                'dynamic_range': {dynamic_range}
            }};
        "#);

        let dark_js = crate::theme::DARK.print_js("DARK_THEME");
        let light_js = crate::theme::LIGHT.print_js("LIGHT_THEME");

        script.push_str(&dark_js);
        script.push_str(&light_js);
        script.push_str(include_str!("assets/theming_widget.js"));

        format!(include_str!("templates/theming_widget.html"), script = script)
    } else {
        String::new()
    };

    let breadcrumb = match breadcrumb_option {
        Some(link) => format!(" <span>›</span> {link}"),
        None => String::from("")
    };

    format!(
        include_str!("templates/layout.html"),
        body = body,
        breadcrumb = breadcrumb,
        browse_icon = icons::browse(),
        browser = r_browser,
        catalog_title = html_escape_outside_attribute(&catalog.title()),
        crawler_meta = crawler_meta.tag(),
        dir_attribute = dir_attribute,
        extra_scripts = extra_scripts.header_tags(root_prefix),
        faircamp_icon = icons::faircamp(),
        favicon_links = catalog.favicon.header_tags(root_prefix),
        feed_meta_link = feed_meta_link,
        index_suffix = if build.clean_urls { "/" } else { "/index.html" },
        lang = &build.locale.language,
        root_prefix = root_prefix,
        t_browse = &build.locale.translations.browse,
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

fn waveform(track: &Track) -> String {
    let peaks_base64 = track.transcodes.borrow().source_meta.peaks
        .iter()
        .map(|peak| {
            // In https://codeberg.org/simonrepp/faircamp/issues/11#issuecomment-858690
            // the "_ => unreachable!()" branch below was hit, probably due to a slight
            // peak overshoot > 1.0 (1.016 already leads to peak64 being assigned 64).
            // We know that some decoders can produce this kind of overshoot, ideally
            // we should be normalizing (=limiting) these peaks to within 0.0-1.0
            // already when we compute/store/cache them. For now we prevent the panic
            // locally here as a patch.
            // TODO:
            // - Implement normalizing/limiting at the point of decoding/caching
            // - Implement an integrity check of all peaks at cache retrieval time (?),
            //   triggering a correction and cache update/removal if found - this is
            //   only meant as a temporary measure, to be phased out in some months/
            //   years.
            //   OR: Better yet use the cache layout versioning
            //   flag to trigger a cache update for all updated faircamp
            //   versions, so all peaks are correctly recalculated for everyone then.
            // - Then also remove this peak_limited correction and rely on the raw
            //   value again.
            let peak_limited = if *peak > 1.0 { 1.0 } else { *peak };

            // Limit range to 0-63
            let peak64 = ((peak_limited / 1.0) * 63.0) as u8;
            let base64 = match peak64 {
                0..=25 => (peak64 + 65) as char, // shift to 65-90 (A-Z)
                26..=51 => (peak64 + 71) as char, // shift to 97-122 (a-z)
                52..=61 => (peak64 - 4) as char, // shift to 48-57 (0-9)
                62 => '+', // map to 43 (+)
                63 => '/', // map to 48 (/)
                _ => unreachable!()
            };
            base64.to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    formatdoc!(r#"
        <svg data-peaks="{peaks_base64}">
            <path class="seek"/>
            <path class="playback"/>
            <path class="base"/>
        </svg>
    "#)
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
        <div aria-hidden="true" class="{extra_class} undescribed_wrapper">
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
