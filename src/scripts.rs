// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::Build;

pub fn generate(build: &Build) {
    generate_site_js(build);

    if build.embeds_requested {
        generate_embeds_js(build);
    }
}

pub fn generate_embeds_js(build: &Build) {
    let js = include_str!("assets/embeds.js");

    fs::write(build.build_dir.join("embeds.js"), js).unwrap();
}

pub fn generate_site_js(build: &Build) {
    let t_listen = &build.locale.translations.listen;
    let t_pause = &build.locale.translations.pause;
    let mut js = formatdoc!("
        const T = {{
            listen: '{t_listen}',
            pause: '{t_pause}'
        }};
    ");

    js.push_str(include_str!("assets/scripts.js")); // TODO: Rename source to site.css

    fs::write(build.build_dir.join("site.js"), js).unwrap();
}
