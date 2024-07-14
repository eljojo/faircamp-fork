// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use id3::{Tag, TagLike};

use crate::decode::aiff;

use super::{AudioMeta, compute_peaks, trim_and_reject_empty};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match aiff::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Ok(tag) = Tag::read_from_path(path) {
        let album = match tag.album() {
            Some(album) => trim_and_reject_empty(album),
            None => None
        };

        let album_artists = match tag.album_artist() {
            Some(album_artist) => match trim_and_reject_empty(album_artist) {
                Some(album_artist) => vec![album_artist],
                None => Vec::new()
            },
            None => Vec::new()
        };

        let artists = match tag.artist() {
            Some(artist) => match trim_and_reject_empty(artist) {
                Some(artist) => vec![artist],
                None => Vec::new()
            },
            None => Vec::new()
        };

        let title = match tag.title() {
            Some(title) => trim_and_reject_empty(title),
            None => None
        };

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
