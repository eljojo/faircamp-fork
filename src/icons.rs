// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The svg files in `icons/` are actually/also format strings and directly imported
//! here to generate the icons. Look out for fill="{text_color}" and such in the
//! svg files when working on existing/new icons.

use std::fs;

use crate::{Build, Theme};

pub fn generate(build: &Build, theme: &Theme) {
    // TODO: h and s are calculated from tint_front etc., revisit/change
    let text_color = format!(
        "hsl({h}deg, {s}%, {l}%)",
        h = theme.text_h,
        s = 0,
        l = theme.base.text_l
    );

    // TODO: h and s are calculated from tint_front etc., revisit/change
    let header_link_color = format!(
        "hsl({h}deg, {s}%, {l}%)",
        h = theme.text_h,
        s = 0,
        l = theme.base.header_link_l
    );
    if build.missing_image_descriptions {
        let visual_impairment_svg = format!(include_str!("icons/visual_impairment.svg"), text_color = text_color);
        fs::write(build.build_dir.join("visual_impairment.svg"), visual_impairment_svg).unwrap();
    }

    let loading_svg = format!(include_str!("icons/loading.svg"), text_color = text_color);
    fs::write(build.build_dir.join("loading.svg"), loading_svg).unwrap();

    let logo_svg = format!(include_str!("icons/logo.svg"), header_link_color = header_link_color);
    fs::write(build.build_dir.join("logo.svg"), logo_svg).unwrap();

    let pause_svg = format!(include_str!("icons/pause.svg"), text_color = text_color);
    fs::write(build.build_dir.join("pause.svg"), pause_svg).unwrap();

    let play_svg = format!(include_str!("icons/play.svg"), text_color = text_color);
    fs::write(build.build_dir.join("play.svg"), play_svg).unwrap();
}