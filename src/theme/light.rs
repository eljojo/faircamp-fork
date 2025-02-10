// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ThemeVarsOklch;

pub const LIGHT: ThemeVarsOklch = ThemeVarsOklch {
    background_1_lightness_range: 92.34..100.0,
    background_2_lightness_range: 82.0..92.34,
    background_3_lightness_range: 70.0..84.0,
    background_accent_lightness_range: 28.0..48.0,
    background_middleground_lightness_range: 60.0..67.0,
    foreground_1_focus_variable: "--fg-3",
    foreground_1_lightness: 0.0,
    foreground_2_lightness: 20.0,
    foreground_3_focus_variable: "--bg-mg",
    foreground_3_lightness: 40.0,
    foreground_accent_lightness: 100.0,
    foreground_middleground_lightness: 45.0,
    middleground_accent_lightness_range: 28.0..48.0,
    middleground_lightness: 50.0,
    veil_alpha_range: 3.0..4.0
};
