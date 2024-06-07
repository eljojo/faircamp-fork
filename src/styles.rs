// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2024 James Fenn
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::{Build, Catalog, Theme, ThemeFont};
use crate::util::url_safe_hash;

const FALLBACK_FONT_STACK_SANS: &str = r#"-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif"#;

/// Might need adjustment at a later point in development, if we don't use
/// button/input anymore we can drop that again.
const FONT_ELEMENTS_SELECTOR: &str = "body, button, input";

pub fn generate(build: &Build, catalog: &Catalog) {
    generate_common(build);
    generate_theme(build, &catalog.theme);

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
    
    let mut css = formatdoc!(
        r#"
            :root {{
                --link-h: {link_h}deg;
                --tint-back: {tint_back};
                --tint-front: {tint_front};
                
                --background-h: var(--link-h);
                --background-l: {background_l}%;
                --background-s: calc({background_s}% * (var(--tint-back) / 100));
                --cover-border-radius: {cover_border_radius};
                --faint-l: {faint_l}%;
                --header-a: {header_a};
                --header-l: {header_l}%;
                --header-link-l: {header_link_l}%;
                --header-shadow-a: {header_shadow_a};
                --header-text-l: {header_text_l}%;
                --link-l: {link_l}%;
                --link-s: {link_s}%;
                --link-hover-l: {link_hover_l}%;
                --muted-h: var(--link-h);
                --muted-l: {muted_l}%;
                --muted-s: calc({muted_s}% * (var(--tint-front) / 100));
                --nav-s: calc({nav_s}% * (var(--tint-front) / 100));
                --overlay-a: {overlay_a};
                --release-additional-a: {release_additional_a};
                --text-h: {text_h}deg;
                --text-l: {text_l}%;
                --text-s: calc({text_s}% * (var(--tint-front) / 100));
            }}
            {font_declaration}
        "#,
        tint_back = theme.tint_back,
        tint_front = theme.tint_front,
        background_l = theme.base.background_l,
        background_s = 41,
        cover_border_radius = if theme.round_corners { ".8rem" } else { "0" },
        faint_l = theme.base.faint_l,
        header_a = theme.base.header_a,
        header_l = theme.base.header_l,
        header_link_l = theme.base.header_link_l,
        header_shadow_a = theme.base.header_shadow_a,
        header_text_l = theme.base.header_text_l,
        link_h = theme.link_h,
        link_l = theme.link_l.unwrap_or(theme.base.link_l),
        link_s = theme.link_s.unwrap_or(theme.base.link_s),
        link_hover_l = theme.base.link_hover_l,
        muted_l = theme.base.muted_l,
        muted_s = 35,
        nav_s = 17,
        // To the user it's exposed as background alpha, technically it's solved
        // the other way round though. Not the image is overlayed transparently
        // over the background, but a solid color layer with the background color is
        // transparently overlayed over the background image. Here we convert from
        // background alpha to overlay alpha (simply the inverse).
        overlay_a = 100 - theme.background_alpha,
        release_additional_a = theme.base.release_additional_a,
        text_h = theme.text_h,
        text_l = theme.base.text_l,
        text_s = 94 // TODO: Dynamic or elsewhere defined
    );

    if let Some(image) = &theme.background_image {
        let image_ref = image.borrow();
        let filename = &image_ref.background_asset.as_ref().unwrap().filename;
        let hashed_filename = format!("background-{}.jpg", url_safe_hash(filename));

        // We are using a pseudo-element floating behind all other page content
        // to display the background image. A more straight-forward way would
        // be to use "fixed" background positioning on body itself, but Apple
        // is seemingly not willing to implement/support this standard in their
        // Safari browser, leaving us stuck with this work-around.
        // See e.g. https://stackoverflow.com/questions/26372127/background-fixed-no-repeat-not-working-on-mobile
        let background_override = formatdoc!("
            body::before {{
                background:
                    linear-gradient(
                        hsla(var(--background-h), var(--background-s), var(--background-l), calc(var(--overlay-a) / 100)),
                        hsla(var(--background-h), var(--background-s), var(--background-l), calc(var(--overlay-a) / 100))
                    ),
                    url({hashed_filename}) center / cover;
                content: '';
                display: block;
                height: 100vh;
                left: 0;
                position: fixed;
                top: 0;
                width: 100vw;
                z-index: -1;
            }}
        ");

        css.push_str(&background_override);
    }
    
    fs::write(stylesheet_path, css).unwrap();
}