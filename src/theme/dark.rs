// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ThemeVarsOklch;

const BACKGROUND_3_LIGHTNESS: f32 = 32.0;

pub const DARK: ThemeVarsOklch = ThemeVarsOklch {
    background_1_lightness_range: 21.56..0.0,
    background_2_lightness_range: 26.0..21.56,
    background_3_lightness_range: BACKGROUND_3_LIGHTNESS..BACKGROUND_3_LIGHTNESS,
    background_accent_lightness_range: BACKGROUND_3_LIGHTNESS..48.0,
    background_middleground_lightness_range: 41.0..41.0,
    foreground_1_focus_variable: "--fg-3",
    foreground_1_lightness: 100.0,
    foreground_2_lightness: 86.0,
    foreground_3_focus_variable: "--fg-1",
    foreground_3_lightness: 72.0,
    foreground_accent_lightness: 100.0,
    foreground_middleground_lightness: 61.0,
    middleground_accent_lightness_range: 48.0..60.0,
    middleground_lightness: 50.0,
    veil_alpha_range: 2.0..6.0
};
