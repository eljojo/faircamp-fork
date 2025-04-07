// SPDX-FileCopyrightText: 2021-2025 Simon Repp
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

const BARLOW_FONT_FILENAME: &str = "barlow-v12-latin-regular.woff2";

const EMBEDS_CSS: &str = include_str!(env!("FAIRCAMP_EMBEDS_CSS"));
const EMBEDS_CSS_FILENAME: &str = "embeds.css";

const MISSING_IMAGE_DESCRIPTIONS_CSS: &str = include_str!(env!("FAIRCAMP_MISSING_IMAGE_DESCRIPTIONS_CSS"));

const THEMING_WIDGET_CSS: &str = include_str!(env!("FAIRCAMP_THEMING_WIDGET_CSS"));

const SITE_CSS: &str = include_str!(env!("FAIRCAMP_SITE_CSS"));
const SITE_CSS_FILENAME: &str = "site.css";

const FALLBACK_FONT_STACK_SANS: &str = r#"-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif"#;
const FONT_ELEMENTS_SELECTOR: &str = "body, button, input";

pub fn generate(build: &mut Build, catalog: &Catalog) {
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

fn generate_embeds_css(build: &mut Build) {
    fs::write(
        build.build_dir.join(EMBEDS_CSS_FILENAME),
        EMBEDS_CSS
    ).unwrap();

    build.reserve_filename(EMBEDS_CSS_FILENAME);
}

fn generate_site_css(build: &mut Build) {
    let mut css = String::from(SITE_CSS);

    if build.missing_image_descriptions {
        css.push_str(MISSING_IMAGE_DESCRIPTIONS_CSS);
    }

    if build.theming_widget {
        css.push_str(THEMING_WIDGET_CSS);
    }

    build.asset_hashes.site_css = Some(url_safe_hash_base64(&css));

    fs::write(
        build.build_dir.join(SITE_CSS_FILENAME),
        css
    ).unwrap();

    build.reserve_filename(SITE_CSS_FILENAME);
}

fn generate_theme_css(build: &mut Build, theme: &Theme) {
    let stylesheet_filename = theme.stylesheet_filename();

    if build.asset_hashes.theme_css.contains_key(&stylesheet_filename) {
        return;
    }

    let font_declaration = match &theme.font {
        ThemeFont::Custom { extension, path } => {
            let filename = format!("custom.{}", extension);

            fs::copy(path, build.build_dir.join(&filename)).unwrap();

            build.reserve_filename(filename.clone());
            
            formatdoc!(r#"
                @font-face {{
                    font-family: 'Custom';
                    font-style: normal;
                    font-weight: 1 1000;
                    src: url('{filename}') format('{extension}');
                }}
                {FONT_ELEMENTS_SELECTOR} {{ font-family: 'Custom'; }}
            "#)
        }
        ThemeFont::Default => {
            fs::write(
                build.build_dir.join(BARLOW_FONT_FILENAME),
                include_bytes!("assets/barlow-v12-latin-regular.woff2")
            ).unwrap();

            build.reserve_filename(BARLOW_FONT_FILENAME);

            formatdoc!(r#"
                @font-face {{
                    font-display: fallback;
                    font-family: 'Barlow';
                    font-style: normal;
                    font-weight: 400;
                    src: local('Barlow'), url('{BARLOW_FONT_FILENAME}') format('woff2');
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
    
    build.asset_hashes.theme_css.insert(stylesheet_filename.clone(), url_safe_hash_base64(&css));

    fs::write(
        build.build_dir.join(&stylesheet_filename),
        css
    ).unwrap();

    build.reserve_filename(stylesheet_filename);
}

fn generate_vars(theme: &Theme) -> String {
    let cover_border_radius;
    let ul_list_style_type;

    if theme.round_corners {
        cover_border_radius = ".8rem";
        ul_list_style_type = "disc";
    } else {
        cover_border_radius = "0";
        ul_list_style_type = "square";
    };

    let vars_hsl = ThemeVarsHsl::print_vars(theme);
    let vars_oklch = &theme.print_vars();

    formatdoc!(r#"
        :root {{
            --cover-border-radius: {cover_border_radius};
            --ul-list-style-type: {ul_list_style_type};
        }}
        {vars_hsl}
        @supports (color: oklch(0% 0% 0)) {{
            {vars_oklch}
        }}
    "#)
}
