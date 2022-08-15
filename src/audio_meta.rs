use id3::{self, TagLike};
use metaflac;
use serde_derive::{Serialize, Deserialize};
use std::path::Path;

use crate::decode::{
    DecodeResult,
    flac,
    mp3,
    ogg_vorbis,
    opus,
    wav
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioMeta {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub duration_seconds: u32,
    pub lossless: bool,
    pub peaks: Option<Vec<f32>>,
    pub title: Option<String>,
    pub track_number: Option<u32>
}

impl AudioMeta {
    pub fn extract(path: &Path, extension: &str) -> AudioMeta {
        let lossless = match extension {
            "aiff" | "alac" | "flac" | "wav" => true,
            "aac" | "mp3" | "ogg" | "opus" => false,
            _ => unimplemented!("Determination whether extension {} indicates lossless audio in the file not yet implemented.", extension)
        };

        match extension {
            "flac" => {
                let (duration_seconds, peaks) = match flac::decode(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                    let track_number = tag
                        .get_vorbis("track") // TODO: Unconfirmed if that key is correct/available ("guessed it" for now :))
                        .map(|iter| iter.collect())
                        .filter(|str: &String| str.parse::<u32>().is_ok())
                        .map(|str: String| str.parse::<u32>().unwrap());
                    
                    AudioMeta {
                        album: tag.get_vorbis("album").map(|iter| iter.collect()),
                        artist: tag.get_vorbis("artist").map(|iter| iter.collect()),
                        duration_seconds,
                        lossless,
                        peaks,
                        title: tag.get_vorbis("title").map(|iter| iter.collect()),
                        track_number
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: None,
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
                    AudioMeta {
                        album: tag.album().map(|str| str.to_string()),
                        artist: tag.artist().map(|str| str.to_string()),
                        duration_seconds,
                        lossless,
                        peaks,
                        title: tag.title().map(|str| str.to_string()),
                        track_number: tag.track()
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: None,
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
                    artist: None,
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
                    artist: None,
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
                    artist: None,
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
                    artist: None,
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