// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::path::Path;

use opus_headers::parse_from_path;

use crate::AudioFormatFamily;
use crate::decode::opus;

use super::{
    AudioMeta,
    compute_peaks,
    parse_track_number_ignoring_total_tracks,
    trim_and_reject_empty
};

pub fn extract(path: &Path) -> Result<AudioMeta, String> {
    let format_family = AudioFormatFamily::Opus;
    let lossless = false;

    let (duration_seconds, peaks) = match opus::decode(path) {
        Ok(decode_result) => (
            decode_result.duration,
            compute_peaks(decode_result, 320)
        ),
        Err(err) => return Err(err)
    };

    let audio_meta = if let Ok(headers) = parse_from_path(path) {
        let user_comments = headers.comments.user_comments;

        let album = extract_single("album", &user_comments);
        let album_artists = extract_multiple_alternatives(&["albumartist", "album artist"], &user_comments);
        let artists = extract_multiple("artist", &user_comments);
        let title = extract_single("title", &user_comments);

        let track_number = match user_comments.get("tracknumber") {
            Some(track_number) => parse_track_number_ignoring_total_tracks(track_number),
            None => None
        };

        AudioMeta {
            album,
            album_artists,
            artists,
            duration_seconds,
            format_family,
            lossless,
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
            format_family,
            lossless,
            peaks,
            title: None,
            track_number: None
        }
    };

    Ok(audio_meta)
}

fn extract_multiple(key: &str, user_comments: &HashMap<String, String>) -> Vec<String> {
    match user_comments.get(key) {
        Some(value) => match trim_and_reject_empty(value) {
            Some(value) => vec![value],
            None => Vec::new()
        },
        None => Vec::new()
    }
}

fn extract_multiple_alternatives(keys: &[&str], user_comments: &HashMap<String, String>) -> Vec<String> {
    for key in keys {
        if let Some(value) = user_comments.get(*key) {
            if let Some(value) = trim_and_reject_empty(value) {
                return vec![value];
            }
        }
    }

    Vec::new()
}

fn extract_single (key: &str, user_comments: &HashMap<String, String>) -> Option<String> {
    match user_comments.get(key) {
        Some(value) => trim_and_reject_empty(value),
        None => None
    }
}
