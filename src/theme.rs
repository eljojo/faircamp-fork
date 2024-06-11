// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use crate::{CoverGenerator, ImageRcView};
use crate::util::url_safe_hash_base64;

/// We use the newtype pattern here (instead of just f32) mostly because
/// we need all alpha fields to be automatically hashable (which f32 isn't)
/// Valid range is 0.0-1.0
/// TODO: We could maybe enforce this range in code (macro?)
#[derive(Clone, Debug)]
pub struct Alpha(f32);

/// background_alpha    0-100 percent
/// h(ue)               0-360 degrees
/// l(ightness)         0-100 percent
/// s(aturation)        0-100 percent
/// tint_back           0-100 percent
/// tint_front          0-100 percent
#[derive(Clone, Debug, Hash)]
pub struct Theme {
    pub background_alpha: u8,
    pub background_image: Option<ImageRcView>,
    pub base: ThemeBase,
    pub cover_generator: CoverGenerator,
    pub font: ThemeFont,
    pub link_h: u16,
    pub link_l: Option<u8>,
    pub link_s: Option<u8>,
    pub relative_waveforms: bool,
    pub round_corners: bool,
    pub text_h: u16,
    pub tint_back: u8,
    pub tint_front: u8,
    pub waveforms: bool
}

/// a(lpha)       0.0-1.0
/// h(ue)         0-360 degrees
/// l(ightness)   0-100 percent
/// s(aturation)  0-100 percent
#[derive(Clone, Debug, Hash)]
pub struct ThemeBase {
    pub background_l: u8,
    pub faint_l: u8,
    pub header_a: Alpha,
    pub header_l: u8,
    pub header_link_l: u8,
    pub header_shadow_a: Alpha,
    pub header_text_l: u8,
    pub link_l: u8,
    pub link_s: u8,
    pub link_hover_l: u8,
    pub muted_l: u8,
    pub release_additional_a: Alpha,
    pub text_l: u8
}

#[derive(Clone, Debug, Hash)]
pub enum ThemeFont {
    Custom { extension: String, path: PathBuf },
    Default,
    SystemMono,
    SystemSans,
    System(String)
}

impl Display for Alpha {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Hash for Alpha {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl CoverGenerator {
    pub const ALL_GENERATORS: [&'static str; 5] = [
        "best_rillen",
        "glass_splinters",
        "looney_tunes",
        "scratchy_faint_rillen",
        "space_time_rupture"
    ];

    pub fn from_manifest_key(key: &str) -> Option<CoverGenerator> {
        match key {
            "best_rillen" => Some(CoverGenerator::BestRillen),
            "glass_splinters" => Some(CoverGenerator::GlassSplinters),
            "looney_tunes" => Some(CoverGenerator::LooneyTunes),
            "scratchy_faint_rillen" => Some(CoverGenerator::ScratchyFaintRillen),
            "space_time_rupture" => Some(CoverGenerator::SpaceTimeRupture),
            _ => None
        }
    }
}

impl Theme {
    pub fn new() -> Theme {
        Theme {
            background_alpha: 10,
            background_image: None,
            base: ThemeBase::DARK,
            cover_generator: CoverGenerator::LooneyTunes,
            font: ThemeFont::Default,
            link_h: 0,
            link_l: None,
            link_s: None,
            relative_waveforms: true,
            round_corners: false,
            text_h: 0,
            tint_back: 0,
            tint_front: 0,
            waveforms: true
        }
    }

    pub fn stylesheet_filename(&self) -> String {
        format!("theme-{}.css", url_safe_hash_base64(self))
    }
}

impl ThemeBase {
    /// If a wrong theme base is configured in a manifest, faircamp
    /// uses this to print a hint about which ones are available.
    pub const ALL_PRESETS: [&'static str; 6] = [
        "black",
        "black_alternate",
        "dark",
        "light",
        "white",
        "white_alternate"
    ];

    pub const BLACK: ThemeBase = ThemeBase {
        background_l: 0,
        faint_l: 15,
        header_a: Alpha(0.8),
        header_l: 0,
        header_link_l: 86,
        header_shadow_a: Alpha(0.0),
        header_text_l: 68,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        release_additional_a: Alpha(0.06),
        text_l: 72
    };

    pub const BLACK_ALTERNATE: ThemeBase = ThemeBase {
        background_l: 0,
        faint_l: 15,
        header_a: Alpha(0.9),
        header_l: 10,
        header_link_l: 86,
        header_shadow_a: Alpha(0.2),
        header_text_l: 72,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        release_additional_a: Alpha(0.06),
        text_l: 72
    };

    pub const DARK: ThemeBase = ThemeBase {
        background_l: 10,
        faint_l: 15,
        header_a: Alpha(0.8),
        header_l: 10,
        header_link_l: 86,
        header_shadow_a: Alpha(0.0),
        header_text_l: 72,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        release_additional_a: Alpha(0.02),
        text_l: 86
    };

    pub const LIGHT: ThemeBase = ThemeBase {
        background_l: 90,
        faint_l: 85,
        header_a: Alpha(0.9),
        header_l: 90,
        header_link_l: 14,
        header_shadow_a: Alpha(0.0),
        header_text_l: 14,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        release_additional_a: Alpha(0.03),
        text_l: 14
    };

    pub const WHITE: ThemeBase = ThemeBase {
        background_l: 100,
        faint_l: 87,
        header_a: Alpha(0.9),
        header_l: 100,
        header_link_l: 14,
        header_shadow_a: Alpha(0.0),
        header_text_l: 14,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        release_additional_a: Alpha(0.04),
        text_l: 14
    };

    pub const WHITE_ALTERNATE: ThemeBase = ThemeBase {
        background_l: 100,
        faint_l: 87,
        header_a: Alpha(0.82),
        header_l: 0,
        header_link_l: 100,
        header_shadow_a: Alpha(0.2),
        header_text_l: 85,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        release_additional_a: Alpha(0.04),
        text_l: 14
    };

    pub fn from_manifest_key(key: &str) -> Option<ThemeBase> {
        match key {
            "black" => Some(ThemeBase::BLACK),
            "black_alternate" => Some(ThemeBase::BLACK_ALTERNATE),
            "dark" => Some(ThemeBase::DARK),
            "light" => Some(ThemeBase::LIGHT),
            "white" => Some(ThemeBase::WHITE),
            "white_alternate" => Some(ThemeBase::WHITE_ALTERNATE),
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
            None => Err(String::from("Custom theme font file needs to have a file extension"))
        }
    }
}
