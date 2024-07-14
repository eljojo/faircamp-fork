// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use crate::decode::ogg_vorbis;

use super::{
    AudioMeta,
    compute_peaks,
    parse_track_number_ignoring_total_tracks,
    trim_and_reject_empty
};

pub fn extract(path: &Path) -> AudioMeta {
    let (duration_seconds, peaks, comment_header) = match ogg_vorbis::decode(path) {
        Some((decode_result, comment_header)) => (
            decode_result.duration,
            Some(compute_peaks(decode_result, 320)),
            Some(comment_header)
        ),
        // TODO: Shall we make it a hard error when we can't determine duration?
        //       It creates strange states e.g. in the audio player rendering when
        //       we don't actually know the duration. (here and elsewhere)
        None => (0.0, None, None)
    };

    let mut album = None;
    let mut album_artists = Vec::new();
    let mut artists = Vec::new();
    let mut title = None;
    let mut track_number = None;

    if let Some(comment_header) = comment_header {
        for (key, value) in comment_header.comment_list {
            match key.as_str() {
                "album" => if let Some(trimmed) = trim_and_reject_empty(&value) {
                    album = Some(trimmed);
                }
                "albumartist" |
                "album artist" => if let Some(trimmed) = trim_and_reject_empty(&value) {
                    album_artists.push(trimmed);
                }
                "artist" => if let Some(trimmed) = trim_and_reject_empty(&value) {
                    artists.push(trimmed);
                }
                "title" => if let Some(trimmed) = trim_and_reject_empty(&value) {
                    title = Some(trimmed);
                }
                "track_number" => if let Some(number) = parse_track_number_ignoring_total_tracks(&value) {
                    track_number = Some(number);
                }
                _ => ()
            }
        }

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
            album,
            album_artists,
            artists,
            duration_seconds,
            lossless: false,
            peaks,
            title,
            track_number
        }
    }
}
