// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2024 James Fenn
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::{
    Build,
    Catalog,
    Theme,
    ThemeFont,
    ThemeVarsHsl
};
use crate::util::url_safe_hash_base64;

const FALLBACK_FONT_STACK_SANS: &str = r#"-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif"#;
const FONT_ELEMENTS_SELECTOR: &str = "body, button, input";

pub fn generate(build: &Build, catalog: &Catalog) {
    if build.embeds_requested {
        generate_embeds_css(build);
    }

    generate_site_css(build);

    generate_theme_css(build, &catalog.theme);

    for artist in &catalog.featured_artists {
        generate_theme_css(build, &artist.borrow().theme);
    }

    for release in &catalog.releases {
        let release_ref = release.borrow();

        generate_theme_css(build, &release_ref.theme);

        for track in &release_ref.tracks {
            generate_theme_css(build, &track.theme);
        }
    }
}

fn generate_embeds_css(build: &Build) {
    let css = include_str!("assets/embeds.css");
    fs::write(build.build_dir.join("embeds.css"), css).unwrap();
}

fn generate_site_css(build: &Build) {
    let mut css = String::from(include_str!("assets/site.css"));

    if build.missing_image_descriptions {
        css.push_str(include_str!("assets/missing_image_descriptions.css"));
    }

    if build.theming_widget {
        css.push_str(include_str!("assets/theming_widget.css"));
    }

    fs::write(build.build_dir.join("site.css"), css).unwrap();
}

fn generate_theme_css(build: &Build, theme: &Theme) {
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

    let mut css = generate_vars(theme);

    css.push_str(&font_declaration);

    if let Some(image) = &theme.background_image {
        let image_ref = image.borrow();
        let filename = &image_ref.background_asset.as_ref().unwrap().filename;
        let hashed_filename = format!("background-{}.jpg", url_safe_hash_base64(filename));

        // We are using a pseudo-element floating behind all other page content
        // to display the background image. A more straight-forward way would
        // be to use "fixed" background positioning on body itself, but Apple
        // is seemingly not willing to implement/support this standard in their
        // Safari browser, leaving us stuck with this work-around.
        // See e.g. https://stackoverflow.com/questions/26372127/background-fixed-no-repeat-not-working-on-mobile
        let background_override = formatdoc!("
            body::before {{
                background: linear-gradient(var(--bg-overlay), var(--bg-overlay)), url({hashed_filename}) center / cover;
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

fn generate_vars(theme: &Theme) -> String {
    let cover_border_radius = if theme.round_corners { ".8rem" } else { "0" };
    let vars_hsl = ThemeVarsHsl::print_vars(theme);
    let vars_oklch = &theme.print_vars();

    formatdoc!(r#"
        :root {{
            --cover-border-radius: {cover_border_radius};
        }}
        {vars_hsl}
        @supports (color: oklch(0% 0% 0)) {{
            {vars_oklch}
        }}
    "#)
}
