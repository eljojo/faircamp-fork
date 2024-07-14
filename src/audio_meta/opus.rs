// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use opus_headers::parse_from_path;

use crate::decode::opus;

use super::{
    AudioMeta,
    compute_peaks,
    parse_track_number_ignoring_total_tracks,
    trim_and_reject_empty
};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match opus::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Ok(headers) = parse_from_path(path) {
        let user_comments = headers.comments.user_comments;

        let album = match user_comments.get("album") {
            Some(album) => trim_and_reject_empty(album),
            None => None
        };

        let album_artists = match user_comments.get("albumartist")
            .or_else(|| user_comments.get("album artist")) {
            Some(album_artist) => match trim_and_reject_empty(album_artist) {
                Some(album_artist) => vec![album_artist],
                None => Vec::new()
            },
            None => Vec::new()
        };

        let artists = match user_comments.get("artist") {
            Some(artist) => match trim_and_reject_empty(artist) {
                Some(artist) => vec![artist],
                None => Vec::new()
            },
            None => Vec::new()
        };

        let title = match user_comments.get("title") {
            Some(title) => trim_and_reject_empty(title),
            None => None
        };

        let track_number = match user_comments.get("tracknumber") {
            Some(track_number) => parse_track_number_ignoring_total_tracks(track_number),
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
            track_number
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
