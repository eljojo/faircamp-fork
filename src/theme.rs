// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::Image;

pub enum CoverGenerator {
    BestRillen,
    GlassSplinters,
    LooneyTunes,
    ScratchyFaintRillen,
    SpaceTimeRupture
}

/// background_alpha    0-100 percent
/// h(ue)               0-360 degrees
/// l(ightness)         0-100 percent
/// s(aturation)        0-100 percent
/// tint_back           0-100 percent
/// tint_front          0-100 percent
pub struct Theme {
    pub background_alpha: u8,
    /// Contains an absolute path to the file (validity is checked when reading manifests)
    pub background_image: Option<Rc<RefCell<Image>>>,
    pub base: ThemeBase,
    pub cover_generator: CoverGenerator,
    pub customized: bool,
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
pub struct ThemeBase {
    pub background_l: u8,
    pub faint_l: u8,
    pub header_a: f32,
    pub header_l: u8,
    pub header_link_l: u8,
    pub header_shadow_a: f32,
    pub header_text_l: u8,
    pub link_l: u8,
    pub link_s: u8,
    pub link_hover_l: u8,
    pub muted_l: u8,
    pub release_additional_a: f32,
    pub text_l: u8
}

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

impl Theme {
    pub fn new() -> Theme {
        Theme {
            background_alpha: 10,
            background_image: None,
            base: ThemeBase::DARK,
            cover_generator: CoverGenerator::LooneyTunes,
            customized: false,
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
        header_a: 0.8,
        header_l: 0,
        header_link_l: 86,
        header_shadow_a: 0.0,
        header_text_l: 68,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        release_additional_a: 0.06,
        text_l: 72
    };

    pub const BLACK_ALTERNATE: ThemeBase = ThemeBase {
        background_l: 0,
        faint_l: 15,
        header_a: 0.9,
        header_l: 10,
        header_link_l: 86,
        header_shadow_a: 0.2,
        header_text_l: 72,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        release_additional_a: 0.06,
        text_l: 72
    };

    pub const DARK: ThemeBase = ThemeBase {
        background_l: 10,
        faint_l: 15,
        header_a: 0.8,
        header_l: 10,
        header_link_l: 86,
        header_shadow_a: 0.0,
        header_text_l: 72,
        link_hover_l: 82,
        link_l: 68,
        link_s: 62,
        muted_l: 23,
        release_additional_a: 0.02,
        text_l: 86
    };

    pub const LIGHT: ThemeBase = ThemeBase {
        background_l: 90,
        faint_l: 85,
        header_a: 0.9,
        header_l: 90,
        header_link_l: 14,
        header_shadow_a: 0.0,
        header_text_l: 14,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        release_additional_a: 0.03,
        text_l: 14
    };

    pub const WHITE: ThemeBase = ThemeBase {
        background_l: 100,
        faint_l: 87,
        header_a: 0.9,
        header_l: 100,
        header_link_l: 14,
        header_shadow_a: 0.0,
        header_text_l: 14,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        release_additional_a: 0.04,
        text_l: 14
    };

    pub const WHITE_ALTERNATE: ThemeBase = ThemeBase {
        background_l: 100,
        faint_l: 87,
        header_a: 0.82,
        header_l: 0,
        header_link_l: 100,
        header_shadow_a: 0.2,
        header_text_l: 85,
        link_hover_l: 48,
        link_l: 42,
        link_s: 100,
        muted_l: 68,
        release_additional_a: 0.04,
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
