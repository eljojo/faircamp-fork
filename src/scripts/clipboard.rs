// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use crate::Build;

const CLIPBOARD_JS: &str = include_str!(env!("FAIRCAMP_CLIPBOARD_JS"));
const CLIPBOARD_JS_FILENAME: &str = "clipboard.js";

pub fn generate_clipboard_js(build: &mut Build) {
    fs::write(
        build.build_dir.join(CLIPBOARD_JS_FILENAME),
        CLIPBOARD_JS
    ).unwrap();

    build.reserve_filename(CLIPBOARD_JS_FILENAME);
}
