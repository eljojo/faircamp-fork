// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Build, Catalog};

mod browser;
mod clipboard;
mod embeds;
mod player;

pub fn generate(build: &mut Build, catalog: &Catalog) {
    browser::generate_browser_js(build, catalog);
    clipboard::generate_clipboard_js(build);
    player::generate_player_js(build);

    if build.embeds_requested {
        embeds::generate_embeds_js(build);
    }
}

/// Escapes `'` as `\'` and `\` as `\\`
fn js_escape_inside_single_quoted_string(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('\'', "\\'")
}
