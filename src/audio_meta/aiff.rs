// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use id3::{Tag, TagLike};

use crate::decode::aiff;

use super::{AudioMeta, compute_peaks, Id3Util};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match aiff::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Ok(tag) = Tag::read_from_path(path) {
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
            lossless: true,
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
            lossless: true,
            peaks,
            title: None,
            track_number: None
        }
    }
}
