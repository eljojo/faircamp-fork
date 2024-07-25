// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use crate::AudioFormatFamily;
use crate::decode::ogg_vorbis;

use super::{
    AudioMeta,
    compute_peaks,
    parse_track_number_ignoring_total_tracks,
    trim_and_reject_empty
};

pub fn extract(path: &Path) -> Result<AudioMeta, String> {
    let format_family = AudioFormatFamily::OggVorbis;
    let lossless = false;

    let (duration_seconds, peaks, comment_header) = match ogg_vorbis::decode(path) {
        Ok((decode_result, comment_header)) => (
            decode_result.duration,
            compute_peaks(decode_result, 320),
            Some(comment_header)
        ),
        Err(err) => return Err(err)
    };

    let mut album = None;
    let mut album_artists = Vec::new();
    let mut artists = Vec::new();
    let mut title = None;
    let mut track_number = None;

    let audio_meta = if let Some(comment_header) = comment_header {
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
            format_family,
            lossless,
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
            format_family,
            lossless,
            peaks,
            title,
            track_number
        }
    };

    Ok(audio_meta)
}
