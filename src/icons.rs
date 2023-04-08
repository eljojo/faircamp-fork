//! The svg files in `icons/` are actually/also format strings and directly imported
//! here to generate the icons. Look out for fill="{text_color}" and such in the
//! svg files when working on existing/new icons.

use std::fs;

use crate::Build;

pub fn generate(build: &Build) {
    let link_color = format!(
        "hsl({h}deg, {s}%, {l}%)",
        h = build.theme.link_h,
        s = build.theme.base.link_s,
        l = build.theme.base.link_l
    );

    // TODO: h and s are calculated from tint_front etc., revisit/change
    let text_color = format!(
        "hsl({h}deg, {s}%, {l}%)",
        h = build.theme.text_h,
        s = 0,
        l = build.theme.base.text_l
    );

    // TODO: h and s are calculated from tint_front etc., revisit/change
    let header_link_color = format!(
        "hsl({h}deg, {s}%, {l}%)",
        h = build.theme.text_h,
        s = 0,
        l = build.theme.base.header_link_l
    );

    // TODO: Provision should be conditional on some "purchases_requested" flag (?) (similar to embeds_requested)
    let buy_svg = format!(include_str!("icons/buy.svg"), link_color = link_color);
    fs::write(build.build_dir.join("buy.svg"), buy_svg).unwrap();

    // TODO: Provision should be conditional on some "downloads_requested" flag (?) (similar to embeds_requested)
    let download_svg = format!(include_str!("icons/download.svg"), link_color = link_color);
    fs::write(build.build_dir.join("download.svg"), download_svg).unwrap();

    if build.base_url.is_some() {   
        let feed_svg = format!(include_str!("icons/feed.svg"), link_color = link_color);
        fs::write(build.build_dir.join("feed.svg"), feed_svg).unwrap();

        if build.embeds_requested {
            let embed_svg = format!(include_str!("icons/embed.svg"), link_color = link_color);
            fs::write(build.build_dir.join("embed.svg"), embed_svg).unwrap();
        }
    }

    if build.missing_image_descriptions {
        let visual_impairment_svg = format!(include_str!("icons/visual_impairment.svg"), text_color = text_color);
        fs::write(build.build_dir.join("visual_impairment.svg"), visual_impairment_svg).unwrap();
    }

    let logo_svg = format!(include_str!("icons/logo.svg"), header_link_color = header_link_color);
    fs::write(build.build_dir.join("logo.svg"), logo_svg).unwrap();

    let pause_svg = format!(include_str!("icons/pause.svg"), text_color = text_color);
    fs::write(build.build_dir.join("pause.svg"), pause_svg).unwrap();

    let play_svg = format!(include_str!("icons/play.svg"), text_color = text_color);
    fs::write(build.build_dir.join("play.svg"), play_svg).unwrap();    

    let share_svg = format!(include_str!("icons/share.svg"), link_color = link_color);
    fs::write(build.build_dir.join("share.svg"), share_svg).unwrap();

    // TODO: Provision should be conditional on some "unlocks_requested" flag (?) (similar to embeds_requested)
    let unlock_svg = format!(include_str!("icons/unlock.svg"), link_color = link_color);
    fs::write(build.build_dir.join("unlock.svg"), unlock_svg).unwrap();
}