// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::Build;

pub fn generate(build: &Build) {
    let t_listen = &build.locale.translations.listen;
    let t_pause = &build.locale.translations.pause;
    let mut js = formatdoc!("
        const T = {{
            listen: '{t_listen}',
            pause: '{t_pause}'
        }};
    ");

    js.push_str(include_str!("assets/scripts.js"));

    fs::write(build.build_dir.join("scripts.js"), js).unwrap();
}