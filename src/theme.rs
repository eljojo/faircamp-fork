// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

// According to https://evilmartians.com/chronicles/oklch-in-css-why-quit-rgb-hsl the
// chroma component in oklch does never exceed 0.37 in P3 or sRGB.

use std::hash::Hash;
use std::ops::Range;
use std::path::PathBuf;

use indoc::formatdoc;

use crate::{CoverGenerator, ImageRcView};
use crate::util::url_safe_hash_base64;

mod dark;
mod light;

pub use dark::DARK;
pub use light::LIGHT;

#[derive(Clone, Debug, Hash)]
pub struct Theme {
    pub accent_brightening: u8,
    pub accent_chroma: Option<u8>,
    pub accent_hue: Option<u16>,
    pub background_alpha: u8,
    pub background_image: Option<ImageRcView>,
    pub base: ThemeBase,
    pub base_chroma: u8,
    pub base_hue: u16,
    pub cover_generator: CoverGenerator,
    pub dynamic_range: u8,
    pub font: ThemeFont,
    pub relative_waveforms: bool,
    pub round_corners: bool,
    pub waveforms: bool
}

#[derive(Clone, Debug, Hash)]
pub enum ThemeBase {
    Dark,
    Light
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

pub struct ThemeVarsOklch {
    pub background_1_lightness_range: Range<f32>,
    pub background_2_lightness_range: Range<f32>,
    pub background_3_lightness_range: Range<f32>,
    pub background_accent_lightness_range: Range<f32>,
    pub background_middleground_lightness_range: Range<f32>,
    pub foreground_1_focus_variable: &'static str,
    pub foreground_1_lightness: f32,
    pub foreground_2_lightness: f32,
    pub foreground_3_focus_variable: &'static str,
    pub foreground_3_lightness: f32,
    pub foreground_accent_lightness: f32,
    pub foreground_middleground_lightness: f32,
    pub middleground_accent_lightness_range: Range<f32>,
    pub middleground_lightness: f32,
    pub veil_alpha_range: Range<f32>
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
            base: ThemeBase::Dark,
            cover_generator: CoverGenerator::LooneyTunes,
            dynamic_range: 0,
            font: ThemeFont::Default,
            relative_waveforms: true,
            round_corners: false,
            waveforms: true
        }
    }

    pub fn print_vars(&self) -> String {
        let oklch = |l: f32, c: f32, h: u16| format!("oklch({l}% {c}% {h})");
        let oklcha = |l: f32, c: f32, h: u16, a: f32| format!("oklch({l}% {c}% {h} / {a}%)");

        fn pick_from_range(factor: u8, variable: &Range<f32>) -> f32 {
            variable.start + (factor as f32 / 100.0) * (variable.end - variable.start)
        }

        let background_1_lightness = pick_from_range(self.dynamic_range, &self.base.vars().background_1_lightness_range);
        let background_2_lightness = pick_from_range(self.dynamic_range, &self.base.vars().background_2_lightness_range);
        let background_3_lightness = pick_from_range(self.dynamic_range, &self.base.vars().background_3_lightness_range);
        let background_accent_lightness = pick_from_range(self.accent_brightening, &self.base.vars().background_accent_lightness_range);
        let background_middleground_lightness = pick_from_range(self.dynamic_range, &self.base.vars().background_middleground_lightness_range);
        let foreground_1_lightness = self.base.vars().foreground_1_lightness;
        let foreground_2_lightness = self.base.vars().foreground_2_lightness;
        let foreground_3_lightness = self.base.vars().foreground_3_lightness;
        let foreground_accent_lightness = self.base.vars().foreground_accent_lightness;
        let foreground_middleground_lightness = self.base.vars().foreground_middleground_lightness;
        let middleground_accent_lightness = pick_from_range(self.accent_brightening, &self.base.vars().middleground_accent_lightness_range);
        let middleground_lightness = self.base.vars().middleground_lightness;
        let veil_alpha = pick_from_range(self.dynamic_range, &self.base.vars().veil_alpha_range);

        let bg_overlay = match self.background_image.is_some() {
            true => {
                let background_overlay_alpha = 100 - self.background_alpha;
                let bg_overlay = oklcha(background_1_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_1_lightness), self.base_hue, background_overlay_alpha as f32);
                format!("--bg-overlay: {bg_overlay};")
            }
            false => String::new()
        };
        let bg_1 = oklch(background_1_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_1_lightness), self.base_hue);
        let bg_1_90 = oklcha(background_1_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_1_lightness), self.base_hue, 90.0);
        let bg_1_overlay = oklcha(background_1_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_1_lightness), self.base_hue, 80.0);
        let bg_2 = oklch(background_2_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_2_lightness), self.base_hue);
        let bg_2_overlay = oklcha(background_2_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_2_lightness), self.base_hue, 80.0);
        let bg_3 = oklch(background_3_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_3_lightness), self.base_hue);
        let bg_acc = oklch(background_accent_lightness, self.accent_chroma.map(|chroma| chroma as f32).unwrap_or(self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_accent_lightness)), self.accent_hue.unwrap_or(self.base_hue));
        let bg_acc_overlay = oklcha(background_accent_lightness, self.accent_chroma.map(|chroma| chroma as f32).unwrap_or(self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_accent_lightness)), self.accent_hue.unwrap_or(self.base_hue), 80.0);
        let bg_mg = oklch(background_middleground_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(background_middleground_lightness), self.base_hue);
        let fg_1 = oklch(foreground_1_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(foreground_1_lightness), self.base_hue);
        let fg_1_focus = format!("var({})", self.base.vars().foreground_1_focus_variable);
        let fg_1_veil = oklcha(foreground_1_lightness, 0.0, 0, veil_alpha);
        let fg_2 = oklch(foreground_2_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(foreground_2_lightness), self.base_hue);
        let fg_3 = oklch(foreground_3_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(foreground_3_lightness), self.base_hue);
        let fg_3_focus = format!("var({})", self.base.vars().foreground_3_focus_variable);
        let fg_acc = oklch(foreground_accent_lightness, 0.0, 0);
        let fg_mg = oklch(foreground_middleground_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(foreground_middleground_lightness), self.base_hue);
        let mg = oklch(middleground_lightness, self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(middleground_lightness), self.base_hue);
        let mg_acc = oklch(middleground_accent_lightness, self.accent_chroma.map(|chroma| chroma as f32).unwrap_or(self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(middleground_accent_lightness)), self.accent_hue.unwrap_or(self.base_hue));
        let mg_acc_overlay = oklcha(middleground_accent_lightness, self.accent_chroma.map(|chroma| chroma as f32).unwrap_or(self.base_chroma as f32 * ThemeVarsOklch::chroma_attenuator(middleground_accent_lightness)), self.accent_hue.unwrap_or(self.base_hue), 80.0);

        formatdoc!(r#"
            :root {{
                {bg_overlay}
                --bg-1: {bg_1};
                --bg-1-90: {bg_1_90};
                --bg-1-overlay: {bg_1_overlay};
                --bg-2: {bg_2};
                --bg-2-overlay: {bg_2_overlay};
                --bg-3: {bg_3};
                --bg-acc: {bg_acc};
                --bg-acc-overlay: {bg_acc_overlay};
                --bg-mg: {bg_mg};
                --fg-1: {fg_1};
                --fg-1-focus: {fg_1_focus};
                --fg-1-veil: {fg_1_veil};
                --fg-2: {fg_2};
                --fg-3: {fg_3};
                --fg-3-focus: {fg_3_focus};
                --fg-acc: {fg_acc};
                --fg-mg: {fg_mg};
                --mg: {mg};
                --mg-acc: {mg_acc};
                --mg-acc-overlay: {mg_acc_overlay};
            }}
        "#)
    }

    /// Procedural covers are raster images generated in RGB color space,
    /// hence its generation can not utilize the regular OKLCH-based theme
    /// variables. For the only currently relevant color in procedural cover
    /// generation (stroke color) we therefore manually specify it in "RGB
    /// lightness" (the value to be used for the R, G and B component alike).
    pub fn procedural_cover_stroke_lightness(&self) -> f32 {
        match self.base {
            ThemeBase::Dark => 1.0,
            ThemeBase::Light => 0.0
        }
    }

    pub fn stylesheet_filename(&self) -> String {
        format!("theme-{}.css", url_safe_hash_base64(self))
    }
}

impl ThemeBase {
    pub fn from_manifest_key(key: &str) -> Option<ThemeBase> {
        match key {
            "dark" => Some(ThemeBase::Dark),
            "light" => Some(ThemeBase::Light),
            _ => None
        }
    }

    pub fn to_key(&self) -> &'static str {
        match self {
            ThemeBase::Dark => "dark",
            ThemeBase::Light => "light"
        }
    }

    pub fn vars(&self) -> ThemeVarsOklch {
        match self {
            ThemeBase::Dark => DARK,
            ThemeBase::Light => LIGHT
        }
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

    pub fn print_vars(theme: &Theme) -> String {
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

        let bg_overlay = match theme.background_image.is_some() {
            true => {
                let background_overlay_alpha = 100 - theme.background_alpha;
                format!("--bg-overlay: hsl(0 0% {background_1_lightness}% / {background_overlay_alpha}%);")
            }
            false => String::new()
        };

        formatdoc!(r#"
            :root {{
                {bg_overlay}
                --bg-1: hsl(0 0% {background_1_lightness}%);
                --bg-1-90: hsl(0 0% {background_1_lightness}% / 90%);
                --bg-1-overlay: hsl(0 0% {background_1_lightness}% / 80%);
                --bg-2: hsl(0 0% {background_2_lightness}%);
                --bg-2-overlay: hsl(0 0% {background_2_lightness}% / 80%);
                --bg-3: hsl(0 0% {background_3_lightness}%);
                --bg-acc: hsl(0 0% {background_accent_lightness}%);
                --bg-acc-overlay: hsl(0 0% {background_accent_lightness}% / 80%);
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
                --mg-acc-overlay: hsl(0 0% {middleground_accent_lightness}% / 80%);
            }}
        "#)
    }
}

impl ThemeVarsOklch {
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

    pub fn print_js(&self, constant_name: &str) -> String {
        let background_1_lightness_end = &self.background_1_lightness_range.end;
        let background_1_lightness_start = &self.background_1_lightness_range.start;
        let background_2_lightness_end = &self.background_2_lightness_range.end;
        let background_2_lightness_start = &self.background_2_lightness_range.start;
        let background_3_lightness_end = &self.background_3_lightness_range.end;
        let background_3_lightness_start = &self.background_3_lightness_range.start;
        let background_accent_lightness_end = &self.background_accent_lightness_range.end;
        let background_accent_lightness_start = &self.background_accent_lightness_range.start;
        let background_middleground_lightness_end = &self.background_middleground_lightness_range.end;
        let background_middleground_lightness_start = &self.background_middleground_lightness_range.start;
        let foreground_1_focus_variable = &self.foreground_1_focus_variable;
        let foreground_1_lightness = &self.foreground_1_lightness;
        let foreground_2_lightness = &self.foreground_2_lightness;
        let foreground_3_focus_variable = &self.foreground_3_focus_variable;
        let foreground_3_lightness = &self.foreground_3_lightness;
        let foreground_accent_lightness = &self.foreground_accent_lightness;
        let foreground_middleground_lightness = &self.foreground_middleground_lightness;
        let middleground_accent_lightness_end = &self.middleground_accent_lightness_range.end;
        let middleground_accent_lightness_start = &self.middleground_accent_lightness_range.start;
        let middleground_lightness = &self.middleground_lightness;
        let veil_alpha_range_end = &self.veil_alpha_range.end;
        let veil_alpha_range_start = &self.veil_alpha_range.start;

        formatdoc!(r#"
            const {constant_name} = {{
                background1LightnessRange: [{background_1_lightness_start}, {background_1_lightness_end}],
                background2LightnessRange: [{background_2_lightness_start}, {background_2_lightness_end}],
                background3LightnessRange: [{background_3_lightness_start}, {background_3_lightness_end}],
                backgroundAccentLightnessRange: [{background_accent_lightness_start}, {background_accent_lightness_end}],
                backgroundMiddlegroundLightnessRange: [{background_middleground_lightness_start}, {background_middleground_lightness_end}],
                foreground1FocusVariable: '{foreground_1_focus_variable}',
                foreground1Lightness: {foreground_1_lightness},
                foreground2Lightness: {foreground_2_lightness},
                foreground3FocusVariable: '{foreground_3_focus_variable}',
                foreground3Lightness: {foreground_3_lightness},
                foregroundAccentLightness: {foreground_accent_lightness},
                foregroundMiddlegroundLightness: {foreground_middleground_lightness},
                middlegroundAccentLightnessRange: [{middleground_accent_lightness_start}, {middleground_accent_lightness_end}],
                middlegroundLightness: {middleground_lightness},
                veilAlphaRange: [{veil_alpha_range_start}, {veil_alpha_range_end}]
            }};
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
