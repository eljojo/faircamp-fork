// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

// According to https://evilmartians.com/chronicles/oklch-in-css-why-quit-rgb-hsl the
// chroma component in oklch does never exceed 0.37 in P3 or sRGB.

use std::hash::Hash;
use std::path::PathBuf;

use indoc::formatdoc;

use crate::{CoverGenerator, ImageRcView};
use crate::util::{HashableF32, url_safe_hash_base64};

mod black;
mod black_alternate;
mod dark;
mod light;
mod white;
mod white_alternate;

#[derive(Clone, Debug, Hash)]
pub struct Theme {
    pub accent_brightening: u8,
    pub accent_chroma: Option<u8>,
    pub accent_hue: Option<u16>,
    pub background_alpha: u8,
    pub background_image: Option<ImageRcView>,
    pub base: ThemeVarsOklch,
    pub base_chroma: u8,
    pub base_hue: u16,
    pub cover_generator: CoverGenerator,
    pub font: ThemeFont,
    pub relative_waveforms: bool,
    pub round_corners: bool,
    pub waveforms: bool
}

#[derive(Clone, Debug, Hash)]
pub enum ThemeFont {
    Custom { extension: String, path: PathBuf },
    Default,
    SystemMono,
    SystemSans,
    System(String)
}

/// A set of static hsl fallback values for the theme, provided for when oklch
/// is not supported in the visitor's browser.
pub struct ThemeVarsHsl;

#[derive(Clone, Debug, Hash)]
pub struct ThemeVarsOklch {
    pub background_1_lightness: HashableF32,
    pub background_2_lightness: HashableF32,
    pub background_3_lightness: HashableF32,
    pub background_accent_lightness_max: HashableF32,
    pub background_accent_lightness_min: HashableF32,
    pub background_middleground_lightness: HashableF32,
    pub foreground_1_focus_variable: &'static str,
    pub foreground_1_lightness: HashableF32,
    pub foreground_2_lightness: HashableF32,
    pub foreground_3_focus_variable: &'static str,
    pub foreground_3_lightness: HashableF32,
    pub foreground_accent_lightness: HashableF32,
    pub foreground_middleground_lightness: HashableF32,
    pub label: &'static str,
    pub middleground_accent_lightness_max: HashableF32,
    pub middleground_accent_lightness_min: HashableF32,
    pub middleground_lightness: HashableF32,
    pub veil_alpha: HashableF32
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
            accent_brightening: 50,
            accent_chroma: None,
            accent_hue: None,
            background_alpha: 10,
            background_image: None,
            base_chroma: 0,
            base_hue: 0,
            base: dark::DARK,
            cover_generator: CoverGenerator::LooneyTunes,
            font: ThemeFont::Default,
            relative_waveforms: true,
            round_corners: false,
            waveforms: true
        }
    }

    pub fn stylesheet_filename(&self) -> String {
        format!("theme-{}.css", url_safe_hash_base64(self))
    }
}

impl ThemeVarsHsl {
    pub const BACKGROUND_1_LIGHTNESS: f32 = 90.0;
    pub const BACKGROUND_2_LIGHTNESS: f32 = 76.83;
    pub const BACKGROUND_3_LIGHTNESS: f32 = 62.06;
    pub const BACKGROUND_ACCENT_LIGHTNESS: f32 = 31.69;
    pub const BACKGROUND_MIDDLEGROUND_LIGHTNESS: f32 = 52.0;
    pub const FOREGROUND_1_LIGHTNESS: f32 = 0.0;
    pub const FOREGROUND_1_FOCUS: &'static str = "--fg-3";
    pub const FOREGROUND_2_LIGHTNESS: f32 = 8.6;
    pub const FOREGROUND_3_LIGHTNESS: f32 = 28.06;
    pub const FOREGROUND_3_FOCUS: &'static str = "--bg-mg";
    pub const FOREGROUND_ACCENT_LIGHTNESS: f32 = 100.0;
    pub const FOREGROUND_MIDDLEGROUND_LIGHTNESS: f32 = 28.0;
    pub const MIDDLEGROUND_LIGHTNESS: f32 = 38.86;

    pub fn print_vars() -> String {
        let background_1_lightness = ThemeVarsHsl::BACKGROUND_1_LIGHTNESS;
        let background_2_lightness = ThemeVarsHsl::BACKGROUND_2_LIGHTNESS;
        let background_3_lightness = ThemeVarsHsl::BACKGROUND_3_LIGHTNESS;
        let background_accent_lightness = ThemeVarsHsl::BACKGROUND_ACCENT_LIGHTNESS;
        let background_middleground_lightness = ThemeVarsHsl::BACKGROUND_MIDDLEGROUND_LIGHTNESS;
        let foreground_1_focus_variable = ThemeVarsHsl::FOREGROUND_1_FOCUS;
        let foreground_1_lightness = ThemeVarsHsl::FOREGROUND_1_LIGHTNESS;
        let foreground_2_lightness = ThemeVarsHsl::FOREGROUND_2_LIGHTNESS;
        let foreground_3_focus_variable = ThemeVarsHsl::FOREGROUND_3_FOCUS;
        let foreground_3_lightness = ThemeVarsHsl::FOREGROUND_3_LIGHTNESS;
        let foreground_accent_lightness = ThemeVarsHsl::FOREGROUND_ACCENT_LIGHTNESS;
        let foreground_middleground_lightness = ThemeVarsHsl::FOREGROUND_MIDDLEGROUND_LIGHTNESS;
        let middleground_lightness = ThemeVarsHsl::MIDDLEGROUND_LIGHTNESS;
        let middleground_accent_lightness = ThemeVarsHsl::BACKGROUND_ACCENT_LIGHTNESS;

        formatdoc!(r#"
            :root {{
                --bg-1: hsl(0 0% {background_1_lightness}%);
                --bg-1-overlay: hsl(0 0% {background_1_lightness}% / 80%);
                --bg-2: hsl(0 0% {background_2_lightness}%);
                --bg-2-overlay: hsl(0 0% {background_2_lightness}% / 80%);
                --bg-3: hsl(0 0% {background_3_lightness}%);
                --bg-acc: hsl(0 0% {background_accent_lightness}%);
                --bg-mg: hsl(0 0% {background_middleground_lightness}%);
                --fg-1: hsl(0 0% {foreground_1_lightness}%);
                --fg-1-focus: var({foreground_1_focus_variable});
                --fg-1-veil: hsl(0 0% {foreground_1_lightness}% / var(--veil-a));
                --fg-2: hsl(0 0% {foreground_2_lightness}%);
                --fg-3: hsl(0 0% {foreground_3_lightness}%);
                --fg-3-focus: var({foreground_3_focus_variable});
                --fg-acc: hsl(0 0% {foreground_accent_lightness}%);
                --fg-mg: hsl(0 0% {foreground_middleground_lightness}%);
                --mg: hsl(0 0% {middleground_lightness}%);
                --mg-acc: hsl(0 0% {middleground_accent_lightness}%);
            }}
        "#)
    }
}

impl ThemeVarsOklch {
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

    pub fn from_manifest_key(key: &str) -> Option<ThemeVarsOklch> {
        match key {
            "black" => Some(black::BLACK),
            "black_alternate" => Some(black_alternate::BLACK_ALTERNATE),
            "dark" => Some(dark::DARK),
            "light" => Some(light::LIGHT),
            "white" => Some(white::WHITE),
            "white_alternate" => Some(white_alternate::WHITE_ALTERNATE),
            _ => None
        }
    }

    pub fn chroma_attenuator(lightness: f32) -> f32 {
        // This can be tuned to modify the attenuation ramp, where a
        // minimum value (> 0%) means the attenuation only happens
        // right before black/white, and the maximum value of 50% means the
        // attenuation happens gradually througout the entire gray range,
        // only being inactive at exactly 50% lightness.
        let ramp = 50.0;

        // Shaps the ramp with a sine function (slope as it occurs between 0-90 degrees)
        let shape = |attenuator: f32| (attenuator * std::f32::consts::FRAC_PI_2).sin();

        if lightness < ramp {
            let attenuator = lightness / ramp; // 0.0 (full attenuation) - 1.0 (no attenuation)
            shape(attenuator)
        } else if lightness > 100.0 - ramp {
            let attenuator = (100.0 - lightness) / ramp; // 1.0 (no attenuation) - 0.0 (full attenuation)
            shape(attenuator)
        } else {
            // Lightness lies within the unattenuated mid-range, no attenuation
            1.0
        }
    }

    pub fn print_vars(&self) -> String {
        let background_1_lightness = &self.background_1_lightness;
        let background_2_lightness = &self.background_2_lightness;
        let background_3_lightness = &self.background_3_lightness;
        let background_accent_lightness_max = &self.background_accent_lightness_max;
        let background_accent_lightness_min = &self.background_accent_lightness_min;
        let background_middleground_lightness = &self.background_middleground_lightness;
        let foreground_1_focus_variable = &self.foreground_1_focus_variable;
        let foreground_1_lightness = &self.foreground_1_lightness;
        let foreground_2_lightness = &self.foreground_2_lightness;
        let foreground_3_focus_variable = &self.foreground_3_focus_variable;
        let foreground_3_lightness = &self.foreground_3_lightness;
        let foreground_accent_lightness = &self.foreground_accent_lightness;
        let foreground_middleground_lightness = &self.foreground_middleground_lightness;
        let middleground_accent_lightness_max = &self.middleground_accent_lightness_max;
        let middleground_accent_lightness_min = &self.middleground_accent_lightness_min;
        let middleground_lightness = &self.middleground_lightness;

        let background_accent_lightness_delta = (background_accent_lightness_max.0 - background_accent_lightness_min.0) / 100.0;
        let middleground_accent_lightness_delta = (middleground_accent_lightness_max.0 - middleground_accent_lightness_min.0) / 100.0;

        let background_1_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(background_1_lightness.0);
        let background_2_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(background_2_lightness.0);
        let background_3_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(background_3_lightness.0);
        let background_accent_chroma_attenuator = ThemeVarsOklch::chroma_attenuator((background_accent_lightness_min.0 + background_accent_lightness_max.0) * 0.5);
        let background_middleground_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(background_middleground_lightness.0);
        let foreground_1_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(foreground_1_lightness.0);
        let foreground_2_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(foreground_2_lightness.0);
        let foreground_3_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(foreground_3_lightness.0);
        let foreground_middleground_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(foreground_middleground_lightness.0);
        let middleground_accent_chroma_attenuator = ThemeVarsOklch::chroma_attenuator((middleground_accent_lightness_min.0 + middleground_accent_lightness_max.0) * 0.5);
        let middleground_chroma_attenuator = ThemeVarsOklch::chroma_attenuator(middleground_lightness.0);

        formatdoc!(r#"
            :root {{
                --bg-1: oklch({background_1_lightness}% calc(var(--base-c) * {background_1_chroma_attenuator}) var(--base-h));
                --bg-1-overlay: oklch({background_1_lightness}% calc(var(--base-c) * {background_1_chroma_attenuator}) var(--base-h) / 80%);
                --bg-2: oklch({background_2_lightness}% calc(var(--base-c) * {background_2_chroma_attenuator}) var(--base-h));
                --bg-2-overlay: oklch({background_2_lightness}% calc(var(--base-c) * {background_2_chroma_attenuator}) var(--base-h) / 80%);
                --bg-3: oklch({background_3_lightness}% calc(var(--base-c) * {background_3_chroma_attenuator}) var(--base-h));
                --bg-acc: oklch(calc({background_accent_lightness_min}% + {background_accent_lightness_delta} * var(--acc-b)) var(--acc-c, calc(var(--base-c) * {background_accent_chroma_attenuator})) var(--acc-h, var(--base-h)));
                --bg-mg: oklch({background_middleground_lightness}% calc(var(--base-c) * {background_middleground_chroma_attenuator}) var(--base-h));
                --fg-1: oklch({foreground_1_lightness}% calc(var(--base-c) * {foreground_1_chroma_attenuator}) var(--base-h));
                --fg-1-focus: var({foreground_1_focus_variable});
                --fg-1-veil: oklch({foreground_1_lightness}% 0 0 / var(--veil-a));
                --fg-2: oklch({foreground_2_lightness}% calc(var(--base-c) * {foreground_2_chroma_attenuator}) var(--base-h));
                --fg-3: oklch({foreground_3_lightness}% calc(var(--base-c) * {foreground_3_chroma_attenuator}) var(--base-h));
                --fg-3-focus: var({foreground_3_focus_variable});
                --fg-acc: oklch({foreground_accent_lightness}% 0 0);
                --fg-mg: oklch({foreground_middleground_lightness}% calc(var(--base-c) * {foreground_middleground_chroma_attenuator}) var(--base-h));
                --mg: oklch({middleground_lightness}% calc(var(--base-c) * {middleground_chroma_attenuator}) var(--base-h));
                --mg-acc: oklch(calc({middleground_accent_lightness_min}% + {middleground_accent_lightness_delta} * var(--acc-b)) var(--acc-c, calc(var(--base-c) * {middleground_accent_chroma_attenuator})) var(--acc-h, var(--base-h)));
            }}
        "#)
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
