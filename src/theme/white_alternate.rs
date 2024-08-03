// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ThemeVarsOklch;
use crate::util::HashableF32;

pub const WHITE_ALTERNATE: ThemeVarsOklch = ThemeVarsOklch {
    background_1_lightness: HashableF32(100.0),
    background_2_lightness: HashableF32(92.34),
    background_3_lightness: HashableF32(84.0),
    background_accent_lightness_max: HashableF32(48.0),
    background_accent_lightness_min: HashableF32(28.0),
    background_middleground_lightness: HashableF32(67.0),
    foreground_1_focus_variable: "--fg-3",
    foreground_1_lightness: HashableF32(0.0),
    foreground_2_lightness: HashableF32(20.0),
    foreground_3_focus_variable: "--bg-mg",
    foreground_3_lightness: HashableF32(40.0),
    foreground_accent_lightness: HashableF32(100.0),
    foreground_middleground_lightness: HashableF32(45.0),
    label: "white_alternate",
    middleground_accent_lightness_max: HashableF32(48.0),
    middleground_accent_lightness_min: HashableF32(28.0),
    middleground_lightness: HashableF32(50.0),
    veil_alpha: HashableF32(4.0)
};
