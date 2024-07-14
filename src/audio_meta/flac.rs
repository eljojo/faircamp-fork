// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use metaflac::Tag;

use crate::decode::flac;

use super::{
    AudioMeta,
    compute_peaks,
    parse_track_number_ignoring_total_tracks,
    trim_and_reject_empty
};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks) = match flac::decode(path) {
        Some(decode_result) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320))
        ),
        None => (0.0, None)
    };

    if let Ok(tag) = Tag::read_from_path(path) {
        // FLAC uses vorbis comments, which support multiple
        // fields with the same key. For the artist key
        // (where this makes sense) we make use of it. All other
        // keys use only the last found (and actually usable, i.e.
        // not empty) field value.

        let album = match tag.get_vorbis("album") {
            Some(fields) => fields.fold(None, |result, field| {
                match trim_and_reject_empty(field) {
                    Some(field) => Some(field),
                    None => result
                }
            }),
            None => None
        };

        let album_artists = tag
            .get_vorbis("albumartist")
            .or_else(|| tag.get_vorbis("album artist"))
            .map(|fields|
                fields.filter_map(trim_and_reject_empty).collect()
            )
            .unwrap_or_else(Vec::new);

        let artists = tag
            .get_vorbis("artist")
            .map(|fields|
                fields.filter_map(trim_and_reject_empty).collect()
            )
            .unwrap_or_else(Vec::new);

        let title = match tag.get_vorbis("title") {
            Some(fields) => fields.fold(None, |result, field| {
                match trim_and_reject_empty(field) {
                    Some(field) => Some(field),
                    None => result
                }
            }),
            None => None
        };

        let track_number = match tag.get_vorbis("tracknumber") {
            Some(fields) => fields.fold(None, |result, field| {
                parse_track_number_ignoring_total_tracks(field)
                    .or(result)
            }),
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
