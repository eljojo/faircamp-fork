//! Note that the files `play.svg` and `pause.svg` in `src/assets/` are
//! not actually included anywhere, their contents are **copy-pasted** inside
//! the `generate` function inside this module. The reason the files exist is
//! to allow conveniently modifying the icons with an editor (e.g. inkscape)
//! and then copying the code, without having to create the files first.

use indoc::formatdoc;
use std::fs;

use crate::Build;

pub fn generate(build: &Build) {
    let fill = format!(
        "hsl({h}, {s}%, {l}%)",
        h = build.theme.hue,
        l = build.theme.base.link_l,
        s = build.theme.base.link_s
    );

    let pause_svg = formatdoc!(
        r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <path d="m36.915 56.995v-49.985h15.592v49.985zm-25.421 0v-49.985h15.592v49.985z" fill="{fill}"/>
            </svg>
        "#,
        fill = fill
    );

    fs::write(build.build_dir.join("pause.svg"), pause_svg).unwrap();

    let play_svg = formatdoc!(
        r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <path d="m11.116 59.677 45.476-26.453-45.544-28.093z" fill="{fill}"/>
            </svg>
        "#,
        fill = fill
    );

    fs::write(build.build_dir.join("play.svg"), play_svg).unwrap();
}