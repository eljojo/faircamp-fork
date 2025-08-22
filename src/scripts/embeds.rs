// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use indoc::formatdoc;

use crate::Build;
use crate::util::url_safe_hash_base64;

const EMBEDS_JS: &str = include_str!(env!("FAIRCAMP_EMBEDS_JS"));
const EMBEDS_JS_FILENAME: &str = "embeds.js";

pub fn generate_embeds_js(build: &mut Build) {
    let t_mute = &build.locale.translations.mute;
    let t_playback_position = &build.locale.translations.playback_position;
    let t_unmute = &build.locale.translations.unmute;
    let t_volume = &build.locale.translations.volume;
    let t_xxx_hours = &build.locale.translations.xxx_hours;
    let t_xxx_minutes = &build.locale.translations.xxx_minutes;
    let t_xxx_seconds = &build.locale.translations.xxx_seconds;
    let mut js = formatdoc!("
        const EMBEDS_JS_T = {{
            mute: '{t_mute}',
            playbackPosition: '{t_playback_position}',
            unmute: '{t_unmute}',
            volume: '{t_volume}',
            xxxHours: hours => '{t_xxx_hours}'.replace('{{xxx}}', hours),
            xxxMinutes: minutes => '{t_xxx_minutes}'.replace('{{xxx}}', minutes),
            xxxSeconds: seconds => '{t_xxx_seconds}'.replace('{{xxx}}', seconds)
        }};
    ");

    js.push_str(EMBEDS_JS);

    build.asset_hashes.embeds_js = Some(url_safe_hash_base64(&js));

    fs::write(
        build.build_dir.join(EMBEDS_JS_FILENAME),
        js
    ).unwrap();

    build.reserve_filename(EMBEDS_JS_FILENAME);
}
