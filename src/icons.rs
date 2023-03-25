//! The svg files in `icons/` are actually/also format strings and directly imported
//! here to generate the icons. Look out for fill="{text_color}" and such in the
//! svg files when working on existing/new icons.

use std::fs;

use crate::Build;

pub fn generate(build: &Build) {
    let text_color = format!("hsl(0, 0%, {l}%)", l = build.theme.base.text_l);

    if build.base_url.is_some() {   
        let feed_svg = format!(
            include_str!("icons/feed.svg"),
            text_color = text_color
        );
        fs::write(build.build_dir.join("feed.svg"), feed_svg).unwrap();
    }

    if build.missing_image_descriptions {
        let visual_impairment_svg = format!(
            include_str!("icons/visual_impairment.svg"),
            text_color = text_color
        );
        fs::write(build.build_dir.join("visual_impairment.svg"), visual_impairment_svg).unwrap();
    }

    let logo_svg = format!(
        include_str!("icons/logo.svg"),
        text_color = text_color
    );
    fs::write(build.build_dir.join("logo.svg"), logo_svg).unwrap();

    let pause_svg = format!(
        include_str!("icons/pause.svg"),
        text_color = text_color
    );
    fs::write(build.build_dir.join("pause.svg"), pause_svg).unwrap();

    let play_svg = format!(
        include_str!("icons/play.svg"),
        text_color = text_color
    );
    fs::write(build.build_dir.join("play.svg"), play_svg).unwrap();    
}