// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use metaflac::Tag;

use crate::AudioFormatFamily;
use crate::decode::flac;

use super::{
    AudioMeta,
    compute_peaks,
    parse_track_number_ignoring_total_tracks,
    trim_and_reject_empty
};

pub fn extract(path: &Path) -> Result<AudioMeta, String> {
    let format_family = AudioFormatFamily::Flac;
    let lossless = true;

    let (duration_seconds, peaks) = match flac::decode(path) {
        Ok(decode_result) => (
            decode_result.duration,
            compute_peaks(decode_result, 320)
        ),
        Err(err) => return Err(err)
    };

    let audio_meta = if let Ok(tag) = Tag::read_from_path(path) {
        // FLAC uses vorbis comments, which support multiple
        // fields with the same key. For artists/album artists
        // (where this makes sense) we make use of it. All other
        // keys use only the last found (and actually usable, i.e.
        // not empty) field value.

        let album = extract_single("album", &tag);
        let album_artists = extract_multiple_alternatives(&["albumartist", "album artist"], &tag);
        let artists = extract_multiple("artist", &tag);
        let title = extract_single("title", &tag);

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

fn extract_multiple(key: &str, tag: &Tag) -> Vec<String> {
    tag.get_vorbis(key)
        .map(|fields| fields.filter_map(trim_and_reject_empty).collect())
        .unwrap_or_else(Vec::new)
}

fn extract_multiple_alternatives(keys: &[&str], tag: &Tag) -> Vec<String> {
    for key in keys {
        if let Some(fields) = tag.get_vorbis(key) {
            let filtered: Vec<String> = fields.filter_map(trim_and_reject_empty).collect();
            if !filtered.is_empty() {
                return filtered;
            }
        }
    }

    Vec::new()
}

fn extract_single(key: &str, tag: &Tag) -> Option<String> {
    match tag.get_vorbis(key) {
        Some(fields) => fields.fold(None, |result, field| {
            match trim_and_reject_empty(field) {
                Some(field) => Some(field),
                None => result
            }
        }),
        None => None
    }
}
