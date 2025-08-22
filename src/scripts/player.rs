// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use indoc::formatdoc;

use crate::Build;
use crate::util::url_safe_hash_base64;

use super::js_escape_inside_single_quoted_string;

const PLAYER_JS: &str = include_str!(env!("FAIRCAMP_PLAYER_JS"));
const PLAYER_JS_FILENAME: &str = "player.js";

pub fn generate_player_js(build: &mut Build) {
    let t_listen = &build.locale.translations.listen;
    let t_mute = &build.locale.translations.mute;
    let t_pause = &build.locale.translations.pause;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_player_closed = &build.locale.translations.player_closed;
    let t_player_open_playing_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.player_open_playing_xxx);
    let t_player_open_with_xxx = js_escape_inside_single_quoted_string(&build.locale.translations.player_open_with_xxx);
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let t_xxx_hours = &build.locale.translations.xxx_hours;
    let t_xxx_minutes = &build.locale.translations.xxx_minutes;
    let t_xxx_seconds = &build.locale.translations.xxx_seconds;
    let mut js = formatdoc!("
        const PLAYER_JS_T = {{
            listen: '{t_listen}',
            mute: '{t_mute}',
            pause: '{t_pause}',
            playbackPosition: '{t_playback_position}',
            playerClosed: '{t_player_closed}',
            playerOpenPlayingXxx: title => '{t_player_open_playing_xxx}'.replace('{{title}}', title),
            playerOpenWithXxx: title => '{t_player_open_with_xxx}'.replace('{{title}}', title),
            unmute: '{t_unmute}',
            volume: '{t_volume}',
            xxxHours: hours => '{t_xxx_hours}'.replace('{{xxx}}', hours),
            xxxMinutes: minutes => '{t_xxx_minutes}'.replace('{{xxx}}', minutes),
            xxxSeconds: seconds => '{t_xxx_seconds}'.replace('{{xxx}}', seconds)
        }};
    ");

    js.push_str(PLAYER_JS);

    build.asset_hashes.player_js = Some(url_safe_hash_base64(&js));

    fs::write(
        build.build_dir.join(PLAYER_JS_FILENAME),
        js
    ).unwrap();

    build.reserve_filename(PLAYER_JS_FILENAME);
}
