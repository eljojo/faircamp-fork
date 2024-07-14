// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use crate::decode::alac;

use mp4parse::TryString;

use super::{AudioMeta, compute_peaks};

/// Extract peaks and tag data using mp4parse
pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match alac::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Some(meta) = alac::decode_meta(path) {
        let album = extract_single(meta.album); // '©alb'
        let album_artists = extract_multiple(meta.album_artist); // 'aART'
        let artists = extract_multiple(meta.artist); // '©art' or '©ART'
        let title = extract_single(meta.title); // '©nam'

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

fn extract_multiple(metadata_option: Option<TryString>) -> Vec<String> {
    match metadata_option {
        Some(try_string) => match String::from_utf8(try_string.to_vec()) {
            Ok(string) => vec![string],
            Err(_) => Vec::new()
        }
        None => Vec::new()
    }
}

fn extract_single (metadata_option: Option<TryString>) -> Option<String> {
    match metadata_option {
        Some(try_string) => match String::from_utf8(try_string.to_vec()) {
            Ok(string) => Some(string),
            Err(_) => None
        }
        None => None
    }
}
