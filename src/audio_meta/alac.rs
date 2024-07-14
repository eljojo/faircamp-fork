// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use crate::decode::alac;

use super::{AudioMeta, compute_peaks};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match alac::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Some(meta) = alac::decode_meta(path) {
        let album = match meta.album {
            Some(try_string) => match String::from_utf8(try_string.to_vec()) {
                Ok(string) => Some(string),
                Err(_) => None
            }
            None => None
        };

        let album_artists = match meta.album_artist {
            Some(try_string) => match String::from_utf8(try_string.to_vec()) {
                Ok(string) => vec![string],
                Err(_) => Vec::new()
            }
            None => Vec::new()
        };

        let artists = match meta.artist {
            Some(try_string) => match String::from_utf8(try_string.to_vec()) {
                Ok(string) => vec![string],
                Err(_) => Vec::new()
            }
            None => Vec::new()
        };

        let title = match meta.title {
            Some(try_string) => match String::from_utf8(try_string.to_vec()) {
                Ok(string) => Some(string),
                Err(_) => None
            }
            None => None
        };

        let track_number = meta.track_number.map(|number| number as u32);

        AudioMeta {
            album,
            album_artists,
            artists,
            duration_seconds,
            lossless: true,
            peaks,
            title,
            track_number
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
