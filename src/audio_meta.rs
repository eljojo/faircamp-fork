// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Vorbis comment reference:
/// - https://www.xiph.org/vorbis/doc/v-comment.html
/// - https://datatracker.ietf.org/doc/html/draft-ietf-cellar-flac-04#name-standard-field-names
/// - https://picard-docs.musicbrainz.org/en/variables/variables.html
///
/// ID3 reference:
/// - http://www.unixgods.org/Ruby/ID3/docs/ID3_comparison.html

use std::path::Path;

use serde_derive::{Serialize, Deserialize};

use crate::{AudioFormatFamily, Build};
use crate::decode::DecodeResult;

mod aiff;
mod alac;
mod flac;
mod id3_util;
mod mp3;
mod ogg_vorbis;
mod opus;
mod wav;

use id3_util::Id3Util;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioMeta {
    /// The album name as provided by tags
    pub album: Option<String>,
    /// The album artists as provided by tags
    /// (Vec because some tag standards support multiple artists)
    pub album_artists: Vec<String>,
    /// The track artists as provided by tags
    /// (Vec because some tag standards support multiple artists)
    pub artists: Vec<String>,
    pub duration_seconds: f32,
    pub format_family: AudioFormatFamily,
    pub lossless: bool,
    /// A simplified, compressed sequence of peaks in the audio,
    /// which are used to later compute the waveform visualization
    pub peaks: Vec<f32>,
    /// The track title as provided by tags
    pub title: Option<String>,
    /// The track number as provided by tags
    pub track_number: Option<u32>
}

impl AudioMeta {
    pub fn extract(build: &Build, extension: &str, relative_path: &Path) -> Result<AudioMeta, String> {
        info_decoding!("{:?} (Generating waveform/reading metadata)", relative_path);

        let absolute_path = build.catalog_dir.join(relative_path);

        match extension {
            "aac" => {
                // TODO: AAC is not yet supported as an input file - there was
                //       previously no library available for this. We could use
                //       ffmpeg and ffprobe to work around this but given aac
                //       is not a good input format either, and proprietary,
                //       this has low priority.

                // AudioMeta {
                //     album: None,
                //     album_artists: Vec::new(),
                //     artists: Vec::new(),
                //     comment: None,
                //     duration_seconds: 0.0,
                //     format_family: AudioFormatFamily::Aac,
                //     license: None,
                //     lossless: false,
                //     peaks: None,
                //     title: None,
                //     track_number: None
                // }

                unreachable!()
            }
            "aif" |
            "aifc" |
            "aiff" => aiff::extract(&absolute_path),
            "alac" => alac::extract(&absolute_path),
            "flac" => flac::extract(&absolute_path),
            "mp3" => mp3::extract(&absolute_path),
            "ogg" => ogg_vorbis::extract(&absolute_path),
            "opus" => opus::extract(&absolute_path),
            "wav" => wav::extract(&absolute_path),
            _ => unreachable!()
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
