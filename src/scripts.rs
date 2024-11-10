// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::fs;

use crate::Build;

pub enum Scripts {
    Clipboard,
    ClipboardAndPlayer,
    None
}

impl Scripts {
    pub fn header_tags(&self, root_prefix: &str) -> String {
        let file_names = match self {
            Scripts::Clipboard => vec!["clipboard.js"],
            Scripts::ClipboardAndPlayer => vec!["clipboard.js", "player.js"],
            Scripts::None => vec![]
        };

        file_names
            .iter()
            .map(|file_name| format!(r#"<script defer src="{root_prefix}{file_name}"></script>"#))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn generate(build: &Build) {
    generate_clipboard_js(build);
    generate_player_js(build);

    if build.embeds_requested {
        generate_embeds_js(build);
    }
}

pub fn generate_clipboard_js(build: &Build) {
    let js = include_str!("assets/clipboard.js");
    fs::write(build.build_dir.join("clipboard.js"), js).unwrap();
}

pub fn generate_embeds_js(build: &Build) {
    let t_mute = &build.locale.translations.mute;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let mut js = formatdoc!("
        const T = {{
            mute: '{t_mute}',
            playbackPosition: '{t_playback_position}',
            unmute: '{t_unmute}',
            volume: '{t_volume}'
        }};
    ");

    js.push_str(include_str!("assets/embeds.js"));

    fs::write(build.build_dir.join("embeds.js"), js).unwrap();
}

pub fn generate_player_js(build: &Build) {
    let t_listen = &build.locale.translations.listen;
    let t_mute = &build.locale.translations.mute;
    let t_pause = &build.locale.translations.pause;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_player_closed = &build.locale.translations.player_closed;
    let t_player_open_playing_xxx = &build.locale.translations.player_open_playing_xxx;
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let mut js = formatdoc!("
        const T = {{
            listen: '{t_listen}',
            mute: '{t_mute}',
            pause: '{t_pause}',
            playbackPosition: '{t_playback_position}',
            playerClosed: '{t_player_closed}',
            playerOpenPlayingXxx: title => '{t_player_open_playing_xxx}'.replace('{{title}}', title),
            unmute: '{t_unmute}',
            volume: '{t_volume}'
        }};
    ");

    js.push_str(include_str!("assets/player.js"));

    fs::write(build.build_dir.join("player.js"), js).unwrap();
}
