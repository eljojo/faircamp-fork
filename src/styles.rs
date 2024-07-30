// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2024 James Fenn
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::{Build, Catalog, Theme, ThemeFont};
use crate::util::url_safe_hash_base64;

const FALLBACK_FONT_STACK_SANS: &str = r#"-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif"#;

/// Might need adjustment at a later point in development, if we don't use
/// button/input anymore we can drop that again.
const FONT_ELEMENTS_SELECTOR: &str = "body, button, input";

pub fn generate(build: &Build, catalog: &Catalog) {
    generate_common(build);
    generate_theme(build, &catalog.theme);

    for artist in &catalog.featured_artists {
        generate_theme(build, &artist.borrow().theme);
    }

    for release in &catalog.releases {
        let release_ref = release.borrow();

        generate_theme(build, &release_ref.theme);

        for track in &release_ref.tracks {
            generate_theme(build, &track.theme);
        }
    }
}

fn generate_common(build: &Build) {
    let mut css = String::from(include_str!("assets/styles.css"));

    if build.missing_image_descriptions {
        css.push_str(include_str!("assets/missing_image_descriptions.css"));
    }

    if build.theming_widget {
        css.push_str(include_str!("assets/theming_widget.css"));
    }

    fs::write(build.build_dir.join("styles.css"), css).unwrap();
}

pub fn generate_theme(build: &Build, theme: &Theme) {
    let stylesheet_filename = theme.stylesheet_filename();
    let stylesheet_path = build.build_dir.join(stylesheet_filename);

    if stylesheet_path.exists() {
        return;
    }

    let font_declaration = match &theme.font {
        ThemeFont::Custom { extension, path } => {
            let filename = format!("custom.{}", extension);

            fs::copy(path, build.build_dir.join(&filename)).unwrap();
            
            formatdoc!(r#"
                @font-face {{
                    font-family: 'Custom';
                    font-style: normal;
                    font-weight: 400;
                    src: url('{filename}') format('{extension}');
                }}
                {FONT_ELEMENTS_SELECTOR} {{ font-family: 'Custom'; }}
            "#)
        }
        ThemeFont::Default => {
            fs::write(
                build.build_dir.join("barlow-v5-latin-regular.woff2"),
                include_bytes!("assets/barlow-v5-latin-regular.woff2")
            ).unwrap();
            
            formatdoc!(r#"
                @font-face {{
                    font-display: fallback;
                    font-family: 'Barlow';
                    font-style: normal;
                    font-weight: 400;
                    src: local('Barlow'), url('barlow-v5-latin-regular.woff2') format('woff2');
                }}
                {FONT_ELEMENTS_SELECTOR} {{ font-family: 'Barlow', {FALLBACK_FONT_STACK_SANS}; }}
            "#)
        }
        ThemeFont::SystemMono => {
            format!(r#"{FONT_ELEMENTS_SELECTOR} {{ font-family: SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }}"#)
        }
        ThemeFont::SystemSans => {
            format!(r#"{FONT_ELEMENTS_SELECTOR} {{ font-family: {FALLBACK_FONT_STACK_SANS}; }}"#)
        }
        ThemeFont::System(fonts) => {
            format!("{FONT_ELEMENTS_SELECTOR} {{ font-family: {}; }}", fonts)
        }
    };

    let bg_1_oklch_l = &theme.base.bg_1.oklch_l;
    let bg_2_oklch_l = &theme.base.bg_2.oklch_l;
    let bg_3_oklch_l = &theme.base.bg_3.oklch_l;
    let bg_mg_oklch_l = &theme.base.bg_mg.oklch_l;
    let fg_1_oklch_l = &theme.base.fg_1.oklch_l;
    let fg_2_oklch_l = &theme.base.fg_2.oklch_l;
    let fg_3_oklch_l = &theme.base.fg_3.oklch_l;
    let fg_mg_oklch_l = &theme.base.fg_mg.oklch_l;
    let mg_oklch_l = &theme.base.mg.oklch_l;

    let mut css = {
        let accent_chroma = &theme.accent_chroma;
        let accent_hue = theme.accent_hue;
        let background_chroma = &theme.background_chroma;
        let background_hue = theme.background_hue;
        let tint_front = theme.tint_front;
        let bg_1_hsl = theme.base.bg_1.to_gray_hsl();
        let bg_1_overlay_hsl = theme.base.bg_1.to_transparent_gray_hsl(80.0);
        let bg_2_hsl = theme.base.bg_2.to_gray_hsl();
        let bg_2_overlay_hsl = theme.base.bg_2.to_transparent_gray_hsl(80.0);
        let bg_3_hsl = theme.base.bg_3.to_gray_hsl();
        let bg_mg_hsl = theme.base.bg_mg.to_gray_hsl();
        let background_l = theme.base.background_l;
        let background_s = 41;
        let cover_border_radius = if theme.round_corners { ".8rem" } else { "0" };
        let faint_l = theme.base.faint_l;
        let fg_1_focus = theme.base.fg_1_focus;
        let fg_1_hsl = theme.base.fg_1.to_gray_hsl();
        let fg_2_hsl = theme.base.fg_2.to_gray_hsl();
        let fg_3_focus = theme.base.fg_3_focus;
        let fg_3_hsl = theme.base.fg_3.to_gray_hsl();
        let fg_mg_hsl = theme.base.fg_mg.to_gray_hsl();
        let header_a = &theme.base.header_a;
        let header_l = theme.base.header_l;
        let header_link_l = theme.base.header_link_l;
        let header_text_l = theme.base.header_text_l;
        let link_h = theme.link_h;
        let link_l = theme.base.link_l;
        let link_s = theme.link_s.unwrap_or(theme.base.link_s);
        let link_hover_l = theme.base.link_hover_l;
        let mg_hsl = theme.base.mg.to_gray_hsl();
        let muted_l = theme.base.muted_l;
        let muted_s = 35;
        let nav_s = 17;
        // To the user it's exposed as background alpha, technically it's solved
        // the other way round though. Not the image is overlayed transparently
        // over the background, but a solid color layer with the background color is
        // transparently overlayed over the background image. Here we convert from
        // background alpha to overlay alpha (simply the inverse).
        let overlay_a = 100 - theme.background_alpha;
        let release_additional_a = &theme.base.release_additional_a;
        let text_h = theme.text_h;
        let text_l = theme.base.text_l;
        let text_s = 94; // TODO: Dynamic or elsewhere defined

        formatdoc!(r#"
            :root {{
                --link-h: {link_h}deg;
                --tint-front: {tint_front};
                
                --accent: {mg_hsl};
                --background-h: var(--link-h);
                --background-l: {background_l}%;
                --background-s: calc({background_s}% * (var(--tint-back) / 100));
                --bg-1: {bg_1_hsl};
                --bg-1-overlay: {bg_1_overlay_hsl};
                --bg-2: {bg_2_hsl};
                --bg-2-overlay: {bg_2_overlay_hsl};
                --bg-3: {bg_3_hsl};
                --bg-mg: {bg_mg_hsl};
                --cover-border-radius: {cover_border_radius};
                --faint-l: {faint_l}%;
                --fg-1: {fg_1_hsl};
                --fg-1-focus: var({fg_1_focus});
                --fg-2: {fg_2_hsl};
                --fg-3: {fg_3_hsl};
                --fg-3-focus: var({fg_3_focus});
                --fg-mg: {fg_mg_hsl};
                --header-a: {header_a};
                --header-l: {header_l}%;
                --header-link-l: {header_link_l}%;
                --header-text-l: {header_text_l}%;
                --link-l: {link_l}%;
                --link-s: {link_s}%;
                --link-hover-l: {link_hover_l}%;
                --mg: {mg_hsl};
                --muted-h: var(--link-h);
                --muted-l: {muted_l}%;
                --muted-s: calc({muted_s}% * (var(--tint-front) / 100));
                --nav-s: calc({nav_s}% * (var(--tint-front) / 100));
                --overlay-a: {overlay_a}%;
                --release-additional-a: {release_additional_a};
                --text-h: {text_h}deg;
                --text-l: {text_l}%;
                --text-s: calc({text_s}% * (var(--tint-front) / 100));
            }}
            @supports (color: oklch(0% 0 0)) {{
                :root {{
                    --acc-c: {accent_chroma};
                    --acc-h: {accent_hue};
                    /* Without accent chroma we give the accent mg_l lightness, with max chroma (0.37) we apply a 10% boost in lightness for extra pop */
                    --accent: oklch(calc(var(--acc-c) * (10% / 0.37) + {mg_oklch_l}%) var(--acc-c) var(--acc-h));
                    --bg-c: {background_chroma};
                    --bg-h: {background_hue};
                    --bg-1: oklch({bg_1_oklch_l}% var(--bg-c) var(--bg-h));
                    --bg-1-overlay: oklch({bg_1_oklch_l}% var(--bg-c) var(--bg-h) / 80%);
                    --bg-2: oklch({bg_2_oklch_l}% var(--bg-c) var(--bg-h));
                    --bg-2-overlay: oklch({bg_2_oklch_l}% var(--bg-c) var(--bg-h) / 80%);
                    --bg-3: oklch({bg_3_oklch_l}% var(--bg-c) var(--bg-h));
                    --bg-mg: oklch({bg_mg_oklch_l}% var(--bg-c) var(--bg-h));
                    --fg-c: 0;
                    --fg-h: 0;
                    --fg-1: oklch({fg_1_oklch_l}% 0 var(--bg-h));
                    --fg-2: oklch({fg_2_oklch_l}% calc(var(--bg-c) / 2) var(--bg-h));
                    --fg-3: oklch({fg_3_oklch_l}% calc(var(--bg-c) / 4) var(--bg-h));
                    --fg-mg: oklch({fg_mg_oklch_l}% var(--fg-c) var(--fg-h));
                    --mg: oklch({mg_oklch_l}% var(--bg-c) var(--bg-h));
                }}
            }}
            {font_declaration}
        "#)
    };

    if let Some(image) = &theme.background_image {
        let image_ref = image.borrow();
        let filename = &image_ref.background_asset.as_ref().unwrap().filename;
        let hashed_filename = format!("background-{}.jpg", url_safe_hash_base64(filename));

        let bg_1_hsl_l = &theme.base.bg_1.hsl_l;

        // We are using a pseudo-element floating behind all other page content
        // to display the background image. A more straight-forward way would
        // be to use "fixed" background positioning on body itself, but Apple
        // is seemingly not willing to implement/support this standard in their
        // Safari browser, leaving us stuck with this work-around.
        // See e.g. https://stackoverflow.com/questions/26372127/background-fixed-no-repeat-not-working-on-mobile
        let background_override = formatdoc!("
            body::before {{
                content: '';
                display: block;
                height: 100vh;
                left: 0;
                position: fixed;
                top: 0;
                width: 100vw;
                z-index: -1;
            }}
            body::before {{
                background:
                    linear-gradient(
                        hsl(0 0% {bg_1_hsl_l}% / var(--overlay-a)),
                        hsl(0 0% {bg_1_hsl_l}% / var(--overlay-a))
                    ),
                    url({hashed_filename}) center / cover;
            }}
            @supports (color: oklch(0% 0 0)) {{
                body::before {{
                    background:
                        linear-gradient(
                            oklch({bg_1_oklch_l}% var(--bg-c) var(--bg-h) / var(--overlay-a)),
                            oklch({bg_1_oklch_l}% var(--bg-c) var(--bg-h) / var(--overlay-a))
                        ),
                        url({hashed_filename}) center / cover;
                }}
            }}
        ");

        css.push_str(&background_override);
    }
    
    fs::write(stylesheet_path, css).unwrap();
}