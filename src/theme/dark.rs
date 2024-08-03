// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ThemeVarsOklch;
use crate::util::HashableF32;

const BACKGROUND_3_LIGHTNESS: f32 = 32.0;

pub const DARK: ThemeVarsOklch = ThemeVarsOklch {
    background_1_lightness: HashableF32(21.56),
    background_2_lightness: HashableF32(26.0),
    background_3_lightness: HashableF32(BACKGROUND_3_LIGHTNESS),
    background_accent_lightness_max: HashableF32(48.0),
    background_accent_lightness_min: HashableF32(BACKGROUND_3_LIGHTNESS),
    background_middleground_lightness: HashableF32(41.0),
    foreground_1_focus_variable: "--fg-3",
    foreground_1_lightness: HashableF32(100.0),
    foreground_2_lightness: HashableF32(86.0),
    foreground_3_focus_variable: "--fg-1",
    foreground_3_lightness: HashableF32(72.0),
    foreground_accent_lightness: HashableF32(100.0),
    foreground_middleground_lightness: HashableF32(61.0),
    label: "dark",
    middleground_accent_lightness_max: HashableF32(60.0),
    middleground_accent_lightness_min: HashableF32(48.0),
    middleground_lightness: HashableF32(50.0),
    veil_alpha: HashableF32(2.0)
};
