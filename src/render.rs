// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-FileCopyrightText: 2023 James Fenn
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use indoc::{formatdoc, indoc};

use translations::Translations;

use crate::{
    ArtistRc,
    Build,
    Catalog,
    DescribedImage,
    ImgAttributes,
    Release,
    ReleaseRc,
    Track
};
use crate::icons;
use crate::util::{
    format_bytes,
    html_double_escape_inside_attribute,
    html_escape_inside_attribute,
    html_escape_outside_attribute
};

pub mod artist;
pub mod image_descriptions;
pub mod index;
pub mod release;
pub mod release_download;
pub mod release_embed;
pub mod release_embed_codes;
pub mod release_purchase;
pub mod release_unlock;
pub mod subscribe;
pub mod track;
pub mod track_download;
pub mod track_embed;
pub mod track_embed_codes;
pub mod track_purchase;
pub mod track_unlock;

mod embed_layout;
mod layout;

use embed_layout::EmbedLayout;
use layout::Layout;

/// Static reusable markup for a speed button we put into the release, track
/// and embedded players
pub const SPEED_CONTROLS: &str = indoc!(r#"
    <button class="speed">
        <span class="multiplier">1.0</span><span class="x">x</span>
    </button>
"#);

struct TruncatedList {
    pub html: String,
    pub truncated: bool
}

enum Truncation {
    Pass,
    Truncate {
        max_chars: usize,
        others_link: String
    }
}

impl Display for TruncatedList {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.html)
    }
}

fn artist_image(
    artist_prefix: &str,
    build: &Build,
    described_image: &DescribedImage,
    root_prefix: &str
) -> String {
    let image_ref = described_image.borrow();

    let alt = match &described_image.description {
        Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
        None => String::new()
    };

    let hash = image_ref.hash.as_url_safe_base64();

    let ImgAttributes { src: src_fixed, srcset: srcset_fixed } = image_ref.artist_assets
        .as_ref()
        .unwrap()
        .img_attributes_fixed(&hash, artist_prefix);

    let ImgAttributes { srcset: srcset_fluid, .. } = image_ref.artist_assets
        .as_ref()
        .unwrap()
        .img_attributes_fluid(&hash, artist_prefix);

    let poster = formatdoc!(r#"
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
    "#);

    if described_image.description.is_some() {
        poster
    } else {
        wrap_undescribed_image(build, root_prefix, &poster, "", "home_image")
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
    let artists_truncation = Truncation::Truncate {
        max_chars: 40,
        others_link: format!("{release_prefix}#more")
    };
    let artists = list_release_artists(build, index_suffix, root_prefix, catalog, artists_truncation, release);
    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let cover = release_cover_image_tiny_decorative(
        release,
        release_link,
        release_prefix
    );

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

fn compact_track_identifier(
    build: &Build,
    catalog: &Catalog,
    index_suffix: &str,
    release: &Release,
    release_prefix: &str,
    root_prefix: &str,
    track: &Track,
    track_link: &str,
    track_prefix: &str
) -> String {
    let artists_truncation = Truncation::Truncate {
        max_chars: 40,
        others_link: format!("{track_prefix}#more")
    };
    let artists = list_track_artists(build, index_suffix, root_prefix, catalog, artists_truncation, track);
    let track_title_escaped = html_escape_outside_attribute(&track.title());
    let cover = track_cover_image_tiny_decorative(
        release,
        release_prefix,
        track,
        track_link,
        track_prefix
    );

    format!(r#"
        <div class="release_compact">
            {cover}
            <div>
                <div style="font-size: 1.17rem;">
                    <a href="{track_link}">
                        {track_title_escaped}
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
pub fn copy_button(content_key: &str, content_value: &str, label: &str) -> String {
    let copy_icon = icons::COPY;
    formatdoc!(r##"
        <button data-{content_key}="{content_value}" data-copy>
            <span class="icon">{copy_icon}</span>
            <span>{label}</span>
        </button>
    "##)
}

fn cover_tile_image(
    build: &Build,
    release_prefix: &str,
    root_prefix: &str,
    release: &Release,
    href: &str
) -> String {
    match &release.cover {
        Some(described_image) => {
            let image_ref = described_image.borrow();

            let alt = match &described_image.description {
                Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
                None => String::new()
            };

            let hash = image_ref.hash.as_url_safe_base64();

            let ImgAttributes { src, srcset } = image_ref.cover_assets_unchecked()
                .img_attributes_up_to_320(&hash, release_prefix);

            // TODO: Re-evaluate if the 'sizes' attribute still reflects circumstances of the current layout
            let thumbnail = formatdoc!(r#"
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
            "#);

            if described_image.description.is_some() {
                thumbnail
            } else {
                wrap_undescribed_image(build, root_prefix, &thumbnail, "", "")
            }
        }
        None => {
            let ImgAttributes { src, srcset } = release.procedural_cover_unchecked()
                .borrow()
                .img_attributes_all_sizes(release_prefix);

            // TODO: Re-evaluate if the 'sizes' attribute still reflects circumstances of the current layout
            formatdoc!(r#"
                <a aria-hidden="true" href="{href}">
                    <img
                        class="procedural"
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
            "#)
        }
    }
}

fn download_entry(href: String, label: &str, size: u64) -> String {
    formatdoc!(
        r#"
            <div class="download_entry">
                <a download href="{href}">
                    {label}
                </a>
                <span class="download_underline"></span>
                <span>{size}</span>
            </div>
        "#,
        size = format_bytes(size)
    )
}

/// Returns a two-field tuple where the fields have the following use:
/// .0 => Intended to be directly copied via copy-to-clipboard
/// .1 => Intended to be rendered to the page, so people can copy it themselves.
/// The title parameter provides a text that indicates to screen-reader users
/// what to expect inside the iframe. See description at
/// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#accessibility_concerns
fn embed_code(embed_url: &str, title: &str) -> (String, String) {
    let title_double_escaped = html_double_escape_inside_attribute(title);
    let title_escaped = html_escape_inside_attribute(title);

    let inline_style = "border: none; min-width: 480px;";

    let copy_code = html_escape_inside_attribute(
        &format!(r#"<iframe loading="lazy" src="{embed_url}" style="{inline_style}" title="{title_escaped}"></iframe>"#)
    );

    let display_code = formatdoc!(r#"
        <div class="embed_code_wrapper">
            <pre class="embed_code"><span class="embed_syntax_special">&lt;</span>iframe
            loading<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"lazy"</span>
            src<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"{embed_url}"</span>
            style<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"{inline_style}"</span>
            title<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"{title_double_escaped}"</span><span class="embed_syntax_special">&gt;</span>
        <span class="embed_syntax_special">&lt;/</span>iframe<span class="embed_syntax_special">&gt;</span></pre>
        </div>
    "#);

    (copy_code, display_code)
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
    truncation: Truncation,
    release: &Release
) -> TruncatedList {
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
            if let Some(link) = &artist_ref.external_page {
                let external_artist_link = format!(r#"<a href="{link}" target="_blank">{name_escaped}</a>"#);
                items.push((name_chars, external_artist_link));
                continue;
            }

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

            if let Some(link) = &artist_ref.external_page {
                let external_artist_link = format!(r#"<a href="{link}" target="_blank">{name_escaped}</a>"#);
                items.push((name_chars, external_artist_link));
            } else if artist_ref.unlisted {
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

            if let Some(link) = &artist_ref.external_page {
                let external_artist_link = format!(r#"<a href="{link}" target="_blank">{name_escaped}</a>"#);
                items.push((name_chars, external_artist_link));
                continue;
            }

            items.push((name_chars, name_escaped));
        }
    }

    truncate_artist_list(build, catalog, items, truncation)
}

/// Render the artists of a track in the style of "Alice, Bob", where each
/// (Alice, Bob) can be a link too, depending on the track and catalog.
/// In *label mode*, all artists of a track are shown and linked to their
/// artist page, if they have one. In *artist mode*, only the catalog artist
/// is ever linked (to the site's homepage in this case). The catalog artist
/// is always sorted first.
fn list_track_artists(
    build: &Build,
    index_suffix: &str,
    root_prefix: &str,
    catalog: &Catalog,
    truncation: Truncation,
    track: &Track
) -> TruncatedList {
    // .1 is the char count of the name, .2 is either the plain name or a link to the artist
    let mut items: Vec<(usize, String)> = Vec::new();

    let mut track_artists_sorted: Vec<ArtistRc> = track.artists.clone();

    // Sort so the catalog artist comes first
    if let Some(catalog_artist) = &catalog.artist {
        track_artists_sorted.sort_by(|a, b| {
            if ArtistRc::ptr_eq(a, catalog_artist) {
                Ordering::Less
            } else if ArtistRc::ptr_eq(b, catalog_artist) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    for artist in &track_artists_sorted {
        let artist_ref = artist.borrow();

        let name_chars = artist_ref.name.chars().count();
        let name_escaped = html_escape_outside_attribute(&artist_ref.name);

        if !artist_ref.unlisted {
            if let Some(link) = &artist_ref.external_page {
                let external_artist_link = format!(r#"<a href="{link}" target="_blank">{name_escaped}</a>"#);
                items.push((name_chars, external_artist_link));
                continue;
            }

            if artist_ref.featured {
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
pub fn player_icon_templates(translations: &Translations) -> String {
    let pause_icon = icons::pause(&translations.pause);
    let play_icon = icons::play(&translations.play);
    let loading_icon = icons::loading(&translations.loading);

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

/// Used on release/tracks pages to display a large-size cover for the release
fn release_cover_image(
    build: &Build,
    release: &Release,
    release_prefix: &str,
    root_prefix: &str,
) -> String {
    match &release.cover {
        Some(described_image) => {
            let image_ref = described_image.borrow();

            let alt = match &described_image.description {
                Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
                None => String::new()
            };

            let hash = image_ref.hash.as_url_safe_base64();

            let ImgAttributes { src: thumb_src, srcset: thumb_srcset } = image_ref.cover_assets
                .as_ref()
                .unwrap()
                .img_attributes_up_to_480(&hash, release_prefix);

            let thumbnail = formatdoc!(r#"
                <a class="image" href="{thumb_src}" target="_blank">
                    <img
                        {alt}
                        sizes="(min-width: 20rem) 20rem, calc(100vw - 2rem)"
                        src="{thumb_src}"
                        srcset="{thumb_srcset}">
                </a>
            "#);

            let cover_ref = image_ref.cover_assets_unchecked();

            let ImgAttributes { src: overlay_src, srcset: overlay_srcset } = cover_ref
                .img_attributes_up_to_1280(&hash, release_prefix);

            let largest_edge_size = cover_ref.largest().edge_size;

            let t_close = &build.locale.translations.close;
            let overlay = formatdoc!(r#"
                <dialog id="overlay">
                    <form method="dialog">
                        <button aria-label="{t_close}"></button>
                    </form>
                    <img
                        {alt}
                        height="{largest_edge_size}"
                        loading="lazy"
                        sizes="calc(100vmin - 4rem)"
                        src="{overlay_src}"
                        srcset="{overlay_srcset}"
                        width="{largest_edge_size}">
                </dialog>
                <script>
                    const overlay = document.querySelector('dialog#overlay');
                    const thumbnailButton = document.querySelector('a.image');

                    overlay.addEventListener('click', () => {{
                        overlay.close();
                    }});

                    thumbnailButton.addEventListener('click', event => {{
                        overlay.showModal();
                        event.preventDefault();
                    }});
                </script>
            "#);

            if described_image.description.is_some() {
                formatdoc!("
                    {thumbnail}
                    {overlay}
                ")
            } else {
                wrap_undescribed_image(build, root_prefix, &thumbnail, &overlay, "")
            }
        }
        None => {
            let ImgAttributes { src, srcset } = release.procedural_cover
                .as_ref()
                .unwrap()
                .borrow()
                .img_attributes_all_sizes(release_prefix);

            // TODO: Re-evaluate if the 'sizes' attribute still reflects circumstances of the current layout
            formatdoc!(r#"
                <span aria-hidden="true" class="image">
                    <img
                        class="procedural"
                        loading="lazy"
                        sizes="
                            (min-width: 60rem) 20rem,
                            (min-width: 30rem) calc((100vw - 4rem) * 0.333),
                            (min-width: 15rem) calc((100vw - 3rem) * 0.5),
                            calc(100vw - 2rem)
                        "
                        src="{src}"
                        srcset="{srcset}">
                </span>
            "#)
        }
    }
}

fn release_cover_image_tiny_decorative(
    release: &Release,
    release_link: &str,
    release_prefix: &str
) -> String {
    let image = match &release.cover {
        Some(described_image) => {
            let filename = described_image.borrow().cover_160_filename_unchecked();
            format!(r#"<img loading="lazy" src="{release_prefix}{filename}">"#)
        }
        None => {
            let filename = release.procedural_cover_120_filename_unchecked();
            format!(r#"<img class="procedural" loading="lazy" src="{release_prefix}{filename}">"#)
        }
    };

    formatdoc!(r#"
        <a aria-hidden="true" href="{release_link}" tabindex="-1">
            {image}
        </a>
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
                let artists_truncation = Truncation::Truncate {
                    max_chars: 40,
                    others_link: format!("{href}#more")
                };
                let list = list_release_artists(build, index_suffix, root_prefix, catalog, artists_truncation, &release_ref);
                format!(r#"<div class="release_artists">{list}</div>"#)
            } else {
                String::new()
            };

            let release_prefix = format!("{root_prefix}{permalink}/");

            let cover = cover_tile_image(
                build,
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

/// Used on track pages to display a large-size cover for the track
fn track_cover_image(
    build: &Build,
    cover: &DescribedImage,
    root_prefix: &str
) -> String {
    let image_ref = cover.image.borrow();
    let track_prefix = "";

    let alt = match &cover.description {
        Some(description) => format!(r#"alt="{}""#, html_escape_inside_attribute(description)),
        None => String::new()
    };

    let hash = image_ref.hash.as_url_safe_base64();

    let ImgAttributes { src: thumb_src, srcset: thumb_srcset } = image_ref.cover_assets
        .as_ref()
        .unwrap()
        .img_attributes_up_to_480(&hash, track_prefix);

    let thumbnail = formatdoc!(r#"
        <a class="image" href="{thumb_src}" target="_blank">
            <img
                {alt}
                sizes="(min-width: 20rem) 20rem, calc(100vw - 2rem)"
                src="{thumb_src}"
                srcset="{thumb_srcset}">
        </a>
    "#);

    let cover_ref = image_ref.cover_assets_unchecked();

    let ImgAttributes { src: overlay_src, srcset: overlay_srcset } = cover_ref
        .img_attributes_up_to_1280(&hash, track_prefix);

    let largest_edge_size = cover_ref.largest().edge_size;

    let t_close = &build.locale.translations.close;
    let overlay = formatdoc!(r#"
        <dialog id="overlay">
            <form method="dialog">
                <button aria-label="{t_close}"></button>
            </form>
            <img
                {alt}
                height="{largest_edge_size}"
                loading="lazy"
                sizes="calc(100vmin - 4rem)"
                src="{overlay_src}"
                srcset="{overlay_srcset}"
                width="{largest_edge_size}">
        </dialog>
        <script>
            const overlay = document.querySelector('dialog#overlay');
            const thumbnailButton = document.querySelector('a.image');

            overlay.addEventListener('click', () => {{
                overlay.close();
            }});

            thumbnailButton.addEventListener('click', event => {{
                overlay.showModal();
                event.preventDefault();
            }});
        </script>
    "#);

    if cover.description.is_some() {
        formatdoc!("
            {thumbnail}
            {overlay}
        ")
    } else {
        wrap_undescribed_image(build, root_prefix, &thumbnail, &overlay, "")
    }
}

fn track_cover_image_tiny_decorative(
    release: &Release,
    release_prefix: &str,
    track: &Track,
    track_link: &str,
    track_prefix: &str
) -> String {
    let image = if let Some(described_image) = &track.cover {
        let filename = described_image.borrow().cover_160_filename_unchecked();
        format!(r#"<img loading="lazy" src="{track_prefix}{filename}">"#)
    } else if let Some(described_image) = &release.cover {
        let filename = described_image.borrow().cover_160_filename_unchecked();
        let src = format!("{release_prefix}{filename}");

        format!(r#"<img loading="lazy" src="{src}">"#)
    } else {
        // TODO: Do we want procedural track covers? (would be used here e.g.)
        let filename = release.procedural_cover_120_filename_unchecked();
        format!(r#"<img class="procedural" loading="lazy" src="{release_prefix}{filename}">"#)
    };

    formatdoc!(r#"
        <a aria-hidden="true" href="{track_link}" tabindex="-1">
            {image}
        </a>
    "#)
}

/// Pass in a Vec holding tuples containing the char count and plain name or
/// link to an artist each, alongside truncation settings. The list then gets
/// truncated (if needed) and joined with ", ".
fn truncate_artist_list(
    build: &Build,
    catalog: &Catalog,
    items: Vec<(usize, String)>,
    truncation: Truncation
) -> TruncatedList {
    if items.len() > 2 {
        if let Truncation::Truncate { max_chars, others_link } = truncation {
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

                    return TruncatedList {
                        html: build.locale.translations.xxx_and_others(&r_items, &others_link),
                        truncated: true
                    };
                }

                // In artist mode we show only "[catalog artist] and others".
                // Our sorting ensures the catalog artist is the first one,
                // so we can just take that.
                return TruncatedList {
                    html: build.locale.translations.xxx_and_others(&items[0].1, &others_link),
                    truncated: true
                };
            }
        }
    }

    TruncatedList {
        html: items.into_iter().map(|item| item.1).collect::<Vec<String>>().join(", "),
        truncated: false
    }
}

pub fn unlisted_badge(build: &Build) -> String {
    let t_unlisted = &build.locale.translations.unlisted;
    format!(r#"<span class="unlisted">{t_unlisted}</span>"#)
}

/// Markup for volume controls as used/shared by the release, track and
/// embedded players
fn volume_controls(translations: &Translations) -> String {
    let volume_icon = icons::VOLUME;
    let t_volume = &translations.volume;

    formatdoc!(r#"
        <div class="volume">
            <button>
                {volume_icon}
            </button>
            <span class="slider">
                <input aria-label="{t_volume}" aria-valuetext="" autocomplete="off" max="1" min="0" step="any" type="range" value="1">
                <svg width="3em" height=".75em" version="1.1" viewBox="0 0 600 150" xmlns="http://www.w3.org/2000/svg">
                    <defs>
                        <linearGradient id="gradient_level">
                            <stop offset="0.999"></stop>
                            <stop offset="1"></stop>
                        </linearGradient>
                        <linearGradient id="gradient_level_decrease">
                            <stop offset="0"></stop>
                            <stop offset="0.0001"></stop>
                        </linearGradient>
                        <linearGradient id="gradient_level_increase">
                            <stop offset="0"></stop>
                            <stop offset="0.0001"></stop>
                        </linearGradient>
                    </defs>
                    <path class="base" d="M0,140H600V10Z"/>
                    <path class="level_increase" d="M0,150H600V10Z" fill="url(#gradient_level_increase)"/>
                    <path d="M0,150H600V10Z" fill="url(#gradient_level)"/>
                    <path class="level_decrease" d="M0,150H600V10Z" fill="url(#gradient_level_decrease)"/>
                </svg>
            </span>
        </div>
    "#)
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
    root_prefix: &str,
    thumbnail: &str,
    overlay: &str,
    extra_class: &str
) -> String {
    let index_suffix = build.index_suffix();

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
