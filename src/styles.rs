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
            {included_static_css}
        "#,
        tint_back = theme.tint_back,
        tint_front = theme.tint_front,
        background_l = theme.base.background_l,
        background_s = 41,
        cover_border_radius = if theme.round_corners { "5%" } else { "0" },
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
        text_s = 94, // TODO: Dynamic or elsewhere defined
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
                    url(background.jpg) center / cover fixed;
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