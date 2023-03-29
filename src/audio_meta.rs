use id3::{self, TagLike};
use serde_derive::{Serialize, Deserialize};
use std::path::Path;

use crate::decode::{
    DecodeResult,
    aiff,
    flac,
    mp3,
    ogg_vorbis,
    opus,
    wav
};

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
    pub artist: Vec<String>,
    pub duration_seconds: u32,
    pub lossless: bool,
    pub peaks: Option<Vec<f32>>,
    pub title: Option<String>,
    pub track_number: Option<u32>
}

impl AudioMeta {
    pub fn extract(path: &Path, extension: &str) -> AudioMeta {
        info_decoding!("{:?} (Generating waveform/reading metadata)", path);

        let lossless = match extension {
            "aiff" | "alac" | "flac" | "wav" => true,
            "aac" | "mp3" | "ogg" | "opus" => false,
            _ => unimplemented!("Determination whether extension {} indicates lossless audio in the file not yet implemented.", extension)
        };

        match extension {
            "aiff" => {
                let (duration_seconds, peaks) = match aiff::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };

                if let Ok(tag) = id3::Tag::read_from_path(path) {
                    let album = match tag.album() {
                        Some(album) => trim_and_reject_empty(album),
                        None => None
                    };

                    let artist = match tag.artist() {
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
                        artist,
                        duration_seconds,
                        lossless,
                        peaks,
                        title,
                        track_number: tag.track()
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: Vec::new(),
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
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };

                if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                    // FLAC uses vorbis comments, which support multiple
                    // fields with the same key. For the artist key
                    // (where this makes sense) we make use of it. All other
                    // keys use one the last found (and actually usable, i.e.
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

                    let artist = tag
                        .get_vorbis("artist")
                        .map(|fields|
                            fields.map(|field| field.to_string()).collect()
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
                            match field.trim().parse::<u32>() {
                                Ok(number) => Some(number),
                                Err(_) => result
                            }
                        }),
                        None => None
                    };
                    
                    AudioMeta {
                        album,
                        artist,
                        duration_seconds,
                        lossless,
                        peaks,
                        title,
                        track_number
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: Vec::new(),
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
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                if let Ok(tag) = id3::Tag::read_from_path(path) {
                    let album = match tag.album() {
                        Some(album) => trim_and_reject_empty(album),
                        None => None
                    };

                    let artist = match tag.artist() {
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
                        artist,
                        duration_seconds,
                        lossless,
                        peaks,
                        title,
                        track_number: tag.track()
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: Vec::new(),
                        duration_seconds,
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "ogg" => {
                let (duration_seconds, peaks) = match ogg_vorbis::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                AudioMeta {
                    album: None,
                    artist: Vec::new(),
                    duration_seconds,
                    lossless,
                    peaks,
                    title: None,
                    track_number: None
                }
            }
            "opus" => {
                let (duration_seconds, peaks) = match opus::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                AudioMeta {
                    album: None,
                    artist: Vec::new(),
                    duration_seconds,
                    lossless,
                    peaks,
                    title: None,
                    track_number: None
                }
            }
            "wav" => {
                let (duration_seconds, peaks) = match wav::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                AudioMeta {
                    album: None,
                    artist: Vec::new(),
                    duration_seconds,
                    lossless,
                    peaks,
                    title: None,
                    track_number: None
                }
            }
            _ => {
                    AudioMeta {
                    album: None,
                    artist: Vec::new(),
                    duration_seconds: 0,
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