// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use id3::{Tag, TagLike, Version};

use crate::decode::mp3;

use super::{AudioMeta, compute_peaks, trim_and_reject_empty};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match mp3::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Ok(tag) = Tag::read_from_path(path) {
        // Due to a bug in the id3 crate, in ID3v2.2 and ID3v2.3 tags
        // the character '/' (slash) is replaced with '\0' (null byte).
        // The issue is a bit more complex than that, hence unresolved,
        // but as a practical workaround we are for the time being re-
        // replacing '\0' with '/' when we encounter it. A bugreport
        // for the underlying issue is found at the following url:
        // https://github.com/polyfloyd/rust-id3/issues/103
        let trim_and_reject_empty_override = match tag.version() {
            Version::Id3v22 |
            Version::Id3v23 => |string: &str| -> Option<String> {
                let repaired_string = string.replace('\0', "/");
                trim_and_reject_empty(&repaired_string)
            },
            Version::Id3v24 => trim_and_reject_empty
        };

        let album = match tag.album() {
            Some(album) => trim_and_reject_empty_override(album),
            None => None
        };

        let album_artists = match tag.album_artist() {
            Some(album_artist) => match trim_and_reject_empty_override(album_artist) {
                Some(album_artist) => vec![album_artist],
                None => Vec::new()
            },
            None => Vec::new()
        };

        let artists = match tag.artist() {
            Some(artist) => match trim_and_reject_empty_override(artist) {
                Some(artist) => vec![artist],
                None => Vec::new()
            },
            None => Vec::new()
        };

        let title = match tag.title() {
            Some(title) => trim_and_reject_empty_override(title),
            None => None
        };

        AudioMeta {
            album,
            album_artists,
            artists,
            duration_seconds,
            lossless: false,
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
            lossless: false,
            peaks,
            title: None,
            track_number: None
        }
    }
}
