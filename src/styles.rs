use indoc::formatdoc;
use std::fs;

use crate::{Build, theme::ThemeFont};

/// Might need adjustment at a later point in development, if we don't use
/// button/input anymore we can drop that again.
const FONT_ELEMENTS_SELECTOR: &str = "body, button, input";

pub fn generate(build: &Build) {
    let theme = &build.theme;
    
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
                    font-family: 'Barlow';
                    font-style: normal;
                    font-weight: 400;
                    src: local('Barlow'), url('barlow-v5-latin-regular.woff2') format('woff2');
                }}
                {FONT_ELEMENTS_SELECTOR} {{ font-family: 'Barlow'; }}
            "#)
        }
        ThemeFont::SystemMono => {
            format!(r#"{FONT_ELEMENTS_SELECTOR} {{ font-family: SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }}"#)
        }
        ThemeFont::SystemSans => {
            format!(r#"{FONT_ELEMENTS_SELECTOR} {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif; }}"#)
        }
        ThemeFont::System(fonts) => {
            format!("{FONT_ELEMENTS_SELECTOR} {{ font-family: {}; }}", fonts)
        }
    };
    
    let mut css = formatdoc!(
        r#"
            :root {{
                --hue: {hue}deg;
                --hue-spread: {hue_spread}deg;
                --tint-back: {tint_back};
                --tint-front: {tint_front};
                
                --background-h: calc(var(--hue) + 3 * var(--hue-spread));
                --background-l: {background_l}%;
                --background-s: calc({background_s}% * (var(--tint-back) / 100));
                --cover-h: calc(var(--hue) + 2 * var(--hue-spread));
                --cover-l: {cover_l}%;
                --cover-s: calc({cover_s}% * (var(--tint-front) / 100));
                --link-l: {link_l}%;
                --link-s: {link_s}%;
                --link-hover-l: {link_hover_l}%;
                --muted-h: calc(var(--hue) + 2 * var(--hue-spread));
                --muted-l: {muted_l}%;
                --muted-s: calc({muted_s}% * (var(--tint-front) / 100));
                --nav-s: calc({nav_s}% * (var(--tint-front) / 100));
                --overlay-a: {overlay_a};
                --pane-l: {pane_l}%;
                --text-h: calc(var(--hue) + 1 * var(--hue-spread));
                --text-l: {text_l}%;
                --text-s: calc({text_s}% * (var(--tint-front) / 100));
            }}
            {font_declaration}
            {included_static_css}
        "#,
        hue = theme.hue,
        hue_spread = theme.hue_spread,
        tint_back = theme.tint_back,
        tint_front = theme.tint_front,
        background_l = theme.base.background_l,
        background_s = 41,
        cover_l = theme.base.cover_l,
        cover_s = 35,
        link_l = theme.base.link_l,
        link_s = theme.base.link_s,
        link_hover_l = theme.base.link_hover_l,
        muted_l = theme.base.muted_l,
        muted_s = 35,
        nav_s = 17,
        overlay_a = theme.base.overlay_a,
        pane_l = theme.base.pane_l,
        text_l = theme.base.text_l,
        text_s = 94,
        included_static_css = include_str!("assets/styles.css")
    );

    if theme.background_image.is_some() {
        let background_override = formatdoc!("
            body {{
                background:
                    linear-gradient(
                        hsla(var(--background-h), var(--background-s), var(--background-l), calc(var(--overlay-a) / 100)),
                        hsla(var(--background-h), var(--background-s), var(--background-l), calc(var(--overlay-a) / 100))
                    ),
                    background.jpg center / cover;
            }}
        ");

        css.push_str(&background_override);
    }

    if build.missing_image_descriptions {
        css.push_str(include_str!("assets/missing_image_descriptions.css"));
    }

    if build.theming_widget {
        css.push_str(include_str!("assets/theming_widget.css"));
    }
    
    fs::write(build.build_dir.join("styles.css"), css).unwrap();
}