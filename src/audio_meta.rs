// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Vorbis comment reference:
/// - https://www.xiph.org/vorbis/doc/v-comment.html
/// - https://datatracker.ietf.org/doc/html/draft-ietf-cellar-flac-04#name-standard-field-names
/// - https://picard-docs.musicbrainz.org/en/variables/variables.html
///
/// ID3 reference:
/// - http://www.unixgods.org/Ruby/ID3/docs/ID3_comparison.html

use std::path::Path;

use id3::TagLike;
use serde_derive::{Serialize, Deserialize};

use crate::decode::{
    DecodeResult,
    aiff,
    alac,
    flac,
    mp3,
    ogg_vorbis,
    opus,
    wav
};

/// Sometimes a tag storing the track number might contain either only
/// the track number ("01") or also the total track count ("01/07").
/// We don't ever need the total track count so this is a parsing routine
/// that extracts only the track number. This function practically also
/// accepts nonsense like "01/boom", happily returning 1, as there's
/// not really any harm coming from that.
pub fn parse_track_number_ignoring_total_tracks(string: &str) -> Option<u32> {
    let mut split_by_slash = string.trim().split('/');

    if let Some(first_token) = split_by_slash.next() {
        match first_token.trim_end().parse::<u32>() {
            Ok(number) => Some(number),
            Err(_) => None
        }
    } else {
        None
    }
}

/// Return None if the passed string is empty or all whitespace,
/// otherwise pass Some(String) containing the trimmed input string. 
fn trim_and_reject_empty(string: &str) -> Option<String> {
    match string.trim() {
        "" => None,
        trimmed => Some(trimmed.to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioMeta {
    pub album: Option<String>,
    /// Vec because vorbis comments support multiple artists
    pub album_artists: Vec<String>,
    /// Vec because vorbis comments support multiple artists
    pub artists: Vec<String>,
    pub duration_seconds: f32,
    pub lossless: bool,
    pub peaks: Option<Vec<f32>>,
    pub title: Option<String>,
    pub track_number: Option<u32>
}

impl AudioMeta {
    pub fn extract(path: &Path, extension: &str) -> AudioMeta {
        info_decoding!("{:?} (Generating waveform/reading metadata)", path);

        let lossless = match extension {
            "aif" |
            "aifc" |
            "aiff" |
            "alac" |
            "flac" |
            "wav" => true,
            "aac" |
            "mp3" |
            "ogg" |
            "opus" => false,
            _ => unimplemented!("Determination whether extension {} indicates lossless audio in the file not yet implemented.", extension)
        };

        match extension {
            "aif" | "aifc" | "aiff" => {
                let (duration_seconds, peaks) = match aiff::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0.0, None)
                };

                if let Ok(tag) = id3::Tag::read_from_path(path) {
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
                        lossless,
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "alac" => {
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "flac" => {
                let (duration_seconds, peaks) = match flac::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0.0, None)
                };

                if let Ok(tag) = metaflac::Tag::read_from_path(path) {
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "mp3" => {
                let (duration_seconds, peaks) = match mp3::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0.0, None)
                };
                
                if let Ok(tag) = id3::Tag::read_from_path(path) {
                    // Due to a bug in the id3 crate, in ID3v2.2 and ID3v2.3 tags
                    // the character '/' (slash) is replaced with '\0' (null byte).
                    // The issue is a bit more complex than that, hence unresolved,
                    // but as a practical workaround we are for the time being re-
                    // replacing '\0' with '\' when we encounter it. A bugreport
                    // for the underlying issue is found at the following url:
                    // https://github.com/polyfloyd/rust-id3/issues/103
                    let trim_and_reject_empty_override = match tag.version() {
                        id3::Version::Id3v22 |
                        id3::Version::Id3v23 => |string: &str| -> Option<String> {
                            let repaired_string = string.replace('\0', "/");
                            trim_and_reject_empty(&repaired_string)
                        },
                        id3::Version::Id3v24 => trim_and_reject_empty
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
                        lossless,
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "ogg" => {
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "opus" => {
                let (duration_seconds, peaks) = match opus::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0.0, None)
                };

                if let Ok(headers) = opus_headers::parse_from_path(path) {
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "wav" => {
                let (duration_seconds, peaks) = match wav::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0.0, None)
                };

                if let Ok(tag) = id3::Tag::read_from_path(path) {
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
                        lossless,
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
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            _ => {
                AudioMeta {
                    album: None,
                    album_artists: Vec::new(),
                    artists: Vec::new(),
                    duration_seconds: 0.0,
                    lossless,
                    peaks: None,
                    title: None,
                    track_number: None
                }
            }
        }
    }
}

/// Takes interleaved samples and applies the following processing:
/// - Determine the largest absolute amplitude among all samples, throughout all channels
/// - Group every [n] samples into a window, for which the average positive and negative amplitude is stored
/// - Determine the largest absolute average amplitude among all calculated windows
/// - For all windows the averaged amplitudes are now upscaled again so that the maximum absolute window amplitude
///   is identical to the largest absolute amplitude found in all discrete samples
fn compute_peaks(decode_result: DecodeResult, points: u32) -> Vec<f32> {
    let window_size = (decode_result.channels as u32 * decode_result.sample_count) / points;

    let mut peaks = Vec::with_capacity(points as usize);

    let mut window_samples = 0;
    let mut window_accumulated = 0.0;

    let mut sample_abs_max: f32 = 0.0;
    let mut window_abs_max: f32 = 0.0;

    for amplitude in decode_result.samples {
        sample_abs_max = sample_abs_max.max(amplitude.abs());

        if window_samples > window_size {
            let peak = window_accumulated / window_samples as f32;

            window_abs_max = window_abs_max.max(peak);

            peaks.push(peak);

            window_samples = 0;
            window_accumulated = 0.0;
        }

        if amplitude.is_sign_positive() {
            window_accumulated += amplitude;
        } else {
            window_accumulated -= amplitude;
        }

        window_samples += 1;
    }

    let upscale = sample_abs_max / window_abs_max;
    
    peaks
        .iter()
        .map(|peak| {
            match "verbatim" {
               "verbatim" => peak * upscale,
               "log2" => (peak * 2.0 + 1.0).log2() * upscale,
               "log10" => (peak * 10.0 + 1.0).log10() * upscale,
               _ => unreachable!()
           }
    
        })
        .collect()
}
