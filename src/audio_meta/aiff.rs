// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use id3::{Tag, TagLike};

use crate::AudioFormatFamily;
use crate::decode::aiff;

use super::{AudioMeta, compute_peaks, Id3Util};

pub fn extract(path: &Path) -> Result<AudioMeta, String> {
    let format_family = AudioFormatFamily::Aiff;
    let lossless = true;

    let (duration_seconds, peaks) = match aiff::decode(path) {
        Ok(decode_result) => (
            decode_result.duration,
            compute_peaks(decode_result, 320)
        ),
        Err(err) => return Err(err)
    };

    let audio_meta = if let Ok(tag) = Tag::read_from_path(path) {
        let id3_util = Id3Util::new(&tag);

        let album = id3_util.album();
        let album_artists = id3_util.album_artists();
        let artists = id3_util.artists();
        let title = id3_util.title();

        AudioMeta {
            album,
            album_artists,
            artists,
            duration_seconds,
            format_family,
            lossless,
            peaks,
            title,
            track_number: tag.track()
        }
    } else {
        AudioMeta {
            album: None,
            album_artists: Vec::new(),
            artists: Vec::new(),
            duration_seconds,
            format_family,
            lossless,
            peaks,
            title: None,
            track_number: None
        }
    };

    Ok(audio_meta)
}
