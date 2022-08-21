use std::path::PathBuf;

/// hue         0-360 degrees
/// hue_spread  0+ degrees
/// tint_back   0-100 percent
/// tint_front  0-100 percent
pub struct Theme {
    /// Contains an absolute path to the file (validity is checked when reading manifests)
    pub background_image: Option<PathBuf>,
    pub base: ThemeBase,
    pub customized: bool,
    pub font: ThemeFont,
    pub hue: u16,
    pub hue_spread: i16,
    pub tint_back: u8,
    pub tint_front: u8
}

/// h(ue)         0-360 degrees
/// s(aturation)  0-100 percent
/// l(ightness)   0-100 percent
/// a(lpha)       0-100 percent (gets converted to 0.0-1.0 in css)
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
    Custom { extension: String, path: PathBuf },
    Default,
    SystemMono,
    SystemSans,
    System(String)
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            background_image: None,
            base: ThemeBase::DARK,
            customized: false,
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

impl ThemeFont {
    pub fn custom(path: PathBuf) -> Result<ThemeFont, String> {
        match path.extension() {
            Some(extension) => {
                if extension == "woff" || extension == "woff2" {
                    let theme_font = ThemeFont::Custom {
                        extension: extension.to_str().unwrap().to_string(),
                        path
                    };

                    Ok(theme_font)
                } else {
                    Err(format!("Theme font extension {:?} not supported (only .woff/.woff2 is supported)", extension))
                }
            }
            None => Err(format!("Custom theme font file needs to have a file extension"))
        }
    }
}
