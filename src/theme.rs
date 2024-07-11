// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use crate::{CoverGenerator, ImageRcView};
use crate::util::url_safe_hash_base64;

/// We need float values in our themes to be automatically hashable (which f32
/// isn't), so we use the newtype pattern to ensure this property.
#[derive(Clone, Debug)]
pub struct HashableF32(f32);

/// Lightness for both hsl and oklch given in 0-100%
#[derive(Clone, Debug, Hash)]
pub struct Lightness {
    pub hsl_l: HashableF32,
    pub oklch_l: HashableF32
}

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
    pub bg_1: Lightness,
    pub bg_2: Lightness,
    pub background_l: u8,
    pub faint_l: u8,
    pub fg_1: Lightness,
    pub fg_2: Lightness,
    pub fg_3: Lightness,
    pub header_a: HashableF32,
    pub header_l: u8,
    pub header_link_l: u8,
    pub header_shadow_a: HashableF32,
    pub header_text_l: u8,
    pub link_l: u8,
    pub link_s: u8,
    pub link_hover_l: u8,
    pub mg: Lightness,
    pub muted_l: u8,
    pub release_additional_a: HashableF32,
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

impl Display for HashableF32 {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Hash for HashableF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl Lightness {
    pub fn to_gray_hsl(&self) -> String {
        format!("hsl(0 0% {}%)", self.hsl_l)
    }

    pub fn to_gray_oklch(&self) -> String {
        format!("oklch({}% 0 0)", self.oklch_l)
    }

    pub fn to_transparent_gray_hsl(&self, a: f32) -> String {
        format!("hsl(0 0% {}% / {a}%)", self.hsl_l)
    }

    pub fn to_transparent_gray_oklch(&self, a: f32) -> String {
        format!("oklch({}% 0 0 / {a}%)", self.oklch_l)
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
        bg_1: Lightness {
            hsl_l: HashableF32(0.0),
            oklch_l: HashableF32(0.0)
        },
        bg_2: Lightness {
            hsl_l: HashableF32(10.0),
            oklch_l: HashableF32(21.56)
        },
        background_l: 0,
        faint_l: 15,
        fg_1: Lightness {
            hsl_l: HashableF32(100.0),
            oklch_l: HashableF32(100.0)
        },
        fg_2: Lightness {
            hsl_l: HashableF32(81.88),
            oklch_l: HashableF32(86.0)
        },
        fg_3: Lightness {
            hsl_l: HashableF32(64.48),
            oklch_l: HashableF32(72.0)
        },
        header_a: HashableF32(0.8),
        header_l: 0,
        header_link_l: 86,
        header_shadow_a: HashableF32(0.0),
        header_text_l: 68,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        mg: Lightness {
            hsl_l: HashableF32(38.86),
            oklch_l: HashableF32(50.0)
        },
        muted_l: 23,
        release_additional_a: HashableF32(0.06),
        text_l: 72
    };

    pub const BLACK_ALTERNATE: ThemeBase = ThemeBase {
        bg_1: Lightness {
            hsl_l: HashableF32(0.0),
            oklch_l: HashableF32(0.0)
        },
        bg_2: Lightness {
            hsl_l: HashableF32(10.0),
            oklch_l: HashableF32(21.56)
        },
        background_l: 0,
        faint_l: 15,
        fg_1: Lightness {
            hsl_l: HashableF32(100.0),
            oklch_l: HashableF32(100.0)
        },
        fg_2: Lightness {
            hsl_l: HashableF32(81.88),
            oklch_l: HashableF32(86.0)
        },
        fg_3: Lightness {
            hsl_l: HashableF32(64.48),
            oklch_l: HashableF32(72.0)
        },
        header_a: HashableF32(0.9),
        header_l: 10,
        header_link_l: 86,
        header_shadow_a: HashableF32(0.2),
        header_text_l: 72,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        mg: Lightness {
            hsl_l: HashableF32(38.86),
            oklch_l: HashableF32(50.0)
        },
        muted_l: 23,
        release_additional_a: HashableF32(0.06),
        text_l: 72
    };

    pub const DARK: ThemeBase = ThemeBase {
        bg_1: Lightness {
            hsl_l: HashableF32(10.0),
            oklch_l: HashableF32(21.56)
        },
        bg_2: Lightness {
            hsl_l: HashableF32(12.23),
            oklch_l: HashableF32(24.0)
        },
        background_l: 10,
        faint_l: 15,
        fg_1: Lightness {
            hsl_l: HashableF32(100.0),
            oklch_l: HashableF32(100.0)
        },
        fg_2: Lightness {
            hsl_l: HashableF32(81.88),
            oklch_l: HashableF32(86.0)
        },
        fg_3: Lightness {
            hsl_l: HashableF32(64.48),
            oklch_l: HashableF32(72.0)
        },
        header_a: HashableF32(0.8),
        header_l: 10,
        header_link_l: 86,
        header_shadow_a: HashableF32(0.0),
        header_text_l: 72,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        mg: Lightness {
            hsl_l: HashableF32(38.86),
            oklch_l: HashableF32(50.0)
        },
        muted_l: 23,
        release_additional_a: HashableF32(0.02),
        text_l: 86
    };

    pub const LIGHT: ThemeBase = ThemeBase {
        bg_1: Lightness {
            hsl_l: HashableF32(90.0),
            oklch_l: HashableF32(92.34)
        },
        bg_2: Lightness {
            hsl_l: HashableF32(76.83),
            oklch_l: HashableF32(82.0)
        },
        background_l: 90,
        faint_l: 85,
        fg_1: Lightness {
            hsl_l: HashableF32(0.0),
            oklch_l: HashableF32(0.0)
        },
        fg_2: Lightness {
            hsl_l: HashableF32(8.6),
            oklch_l: HashableF32(20.0)
        },
        fg_3: Lightness {
            hsl_l: HashableF32(28.06),
            oklch_l: HashableF32(40.0)
        },
        header_a: HashableF32(0.9),
        header_l: 90,
        header_link_l: 14,
        header_shadow_a: HashableF32(0.0),
        header_text_l: 14,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        mg: Lightness {
            hsl_l: HashableF32(38.86),
            oklch_l: HashableF32(50.0)
        },
        muted_l: 68,
        release_additional_a: HashableF32(0.03),
        text_l: 14
    };

    pub const WHITE: ThemeBase = ThemeBase {
        bg_1: Lightness {
            hsl_l: HashableF32(100.0),
            oklch_l: HashableF32(100.0)
        },
        bg_2: Lightness {
            hsl_l: HashableF32(90.0),
            oklch_l: HashableF32(92.34)
        },
        background_l: 100,
        faint_l: 87,
        fg_1: Lightness {
            hsl_l: HashableF32(0.0),
            oklch_l: HashableF32(0.0)
        },
        fg_2: Lightness {
            hsl_l: HashableF32(8.6),
            oklch_l: HashableF32(20.0)
        },
        fg_3: Lightness {
            hsl_l: HashableF32(28.06),
            oklch_l: HashableF32(40.0)
        },
        header_a: HashableF32(0.9),
        header_l: 100,
        header_link_l: 14,
        header_shadow_a: HashableF32(0.0),
        header_text_l: 14,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        mg: Lightness {
            hsl_l: HashableF32(38.86),
            oklch_l: HashableF32(50.0)
        },
        muted_l: 68,
        release_additional_a: HashableF32(0.04),
        text_l: 14
    };

    pub const WHITE_ALTERNATE: ThemeBase = ThemeBase {
        bg_1: Lightness {
            hsl_l: HashableF32(100.0),
            oklch_l: HashableF32(100.0)
        },
        bg_2: Lightness {
            hsl_l: HashableF32(90.0),
            oklch_l: HashableF32(92.34)
        },
        background_l: 100,
        faint_l: 87,
        fg_1: Lightness {
            hsl_l: HashableF32(0.0),
            oklch_l: HashableF32(0.0)
        },
        fg_2: Lightness {
            hsl_l: HashableF32(8.6),
            oklch_l: HashableF32(20.0)
        },
        fg_3: Lightness {
            hsl_l: HashableF32(28.06),
            oklch_l: HashableF32(40.0)
        },
        header_a: HashableF32(0.82),
        header_l: 0,
        header_link_l: 100,
        header_shadow_a: HashableF32(0.2),
        header_text_l: 85,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        mg: Lightness {
            hsl_l: HashableF32(38.86),
            oklch_l: HashableF32(50.0)
        },
        muted_l: 68,
        release_additional_a: HashableF32(0.04),
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
