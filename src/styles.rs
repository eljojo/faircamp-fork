use indoc::formatdoc;
use std::fs;

use crate::{
    build::Build,
    ffmpeg::{self, MediaFormat},
    image_format::ImageFormat
};

// TODO: Turn Theme into enum (like with CacheOptimization) so we can model the global
//       as Theme::Default on Build and thereby differentiate repeated global setting of theme

// hue         0-360 degrees
// hue_spread  0+ degrees
// tint_back   0-100 percent
// tint_front  0-100 percent
pub struct Theme {
    pub background_image: Option<String>,
    pub base: ThemeBase,
    pub font: ThemeFont,
    pub hue: u16,
    pub hue_spread: i16,
    pub tint_back: u8,
    pub tint_front: u8
}

// h(ue)         0-360 degrees
// s(aturation)  0-100 percent
// l(ightness)   0-100 percent
// a(lpha)       0-100 percent (gets converted to 0.0-1.0 in css)
pub struct ThemeBase {
    pub background_l: u8,
    pub cover_l: u8,
    pub link_l: u8,
    pub link_s: u8,
    pub link_hover_l: u8,
    pub muted_l: u8,
    pub overlay_a: u8,
    pub text_l: u8
}

pub enum ThemeFont {
    Custom(String),
    Default,
    SystemMono,
    SystemSans,
    System(String)
}

pub fn generate(build: &mut Build) {
    let theme = &build.theme.take().unwrap_or_else(|| Theme::defaults());    
    
    let background_override = match &theme.background_image {
        Some(background_image) => {
            // TODO: Go through asset cache with this
            ffmpeg::transcode(
                &build.catalog_dir.join(background_image),
                &build.build_dir.join("background.jpg"),
                MediaFormat::Image(&ImageFormat::Jpeg)
            ).unwrap();
            
            formatdoc!(r#"
                body {{
                    background:
                        linear-gradient(
                            hsla(var(--background-h), var(--background-s), var(--background-l), calc(var(--overlay-a) / 100)),
                            hsla(var(--background-h), var(--background-s), var(--background-l), calc(var(--overlay-a) / 100))
                        ),
                        url(background.jpg) center / cover;
                }}
            "#)
        }
        None => String::new()
    };
    
    let font_declaration = match &theme.font {
        ThemeFont::Custom(path) => {
            // TODO: Safe-guard against file not existing in that location
            fs::copy(
                build.catalog_dir.join(path),
                build.build_dir.join("custom.woff2") // TODO: Support .woff (/whatever is supplied?) too
            ).unwrap();
            
            formatdoc!(r#"
                @font-face {{
                    font-family: 'Custom';
                    font-style: normal;
                    font-weight: 400;
                    src: url('custom.woff2') format('woff2');
                }}
                body {{ font-family: 'Custom'; }}
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
                body {{ font-family: 'Barlow'; }}
            "#)
        }
        ThemeFont::SystemMono => {
            String::from(r#"body { font-family: SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }"#)
        }
        ThemeFont::SystemSans => {
            String::from(r#"body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif; }"#)
        }
        ThemeFont::System(fonts) => {
            format!("body {{ font-family: {}; }}", fonts)
        }
    };
    
    let css = formatdoc!(
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
                --text-h: calc(var(--hue) + 1 * var(--hue-spread));
                --text-l: {text_l}%;
                --text-s: calc({text_s}% * (var(--tint-front) / 100));
            }}
            {font_declaration}
            {included_static_css}
            {background_override}
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
        text_l = theme.base.text_l,
        text_s = 94,
        font_declaration = font_declaration,
        included_static_css = include_str!("assets/styles.css"),
        background_override = background_override
    );
    
    fs::write(build.build_dir.join("styles.css"), css).unwrap();
}
    
impl Theme {
    pub fn defaults() -> Theme {
        Theme {
            background_image: None,
            base: ThemeBase::DARK,
            font: ThemeFont::Default,
            hue: 0,
            hue_spread: 0,
            tint_back: 0,
            tint_front: 0
        }
    }
}

impl ThemeBase {
    pub const DARK: ThemeBase = ThemeBase {
        background_l: 10,
        cover_l: 13,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        overlay_a: 90,
        text_l: 86
    };

    pub const LIGHT: ThemeBase = ThemeBase {
        background_l: 90,
        cover_l: 87,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        overlay_a: 90,
        text_l: 14
    };
    
    pub fn from_manifest_key(key: &str) -> Option<ThemeBase> {
        match key {
            "dark" => Some(ThemeBase::DARK),
            "light" => Some(ThemeBase::LIGHT),
            _ => None
        }
    }
}