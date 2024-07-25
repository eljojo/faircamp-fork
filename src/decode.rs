// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod aiff;
pub mod alac;
pub mod flac;
pub mod mp3;
pub mod ogg_vorbis;
pub mod opus;
pub mod wav;

const I24_MAX: i32 = 8388607;

#[derive(Debug)]
pub struct DecodeResult {
    pub channels: u16,
    pub duration: f32,
    /// This sample count is decoupled from channel count, i.e. a mono file with
    /// 48kHz sample rate will have the same sample count as a stereo file
    /// with 48kHz sample rate (that is, 48000 in both cases).
    pub sample_count: u32,
    pub sample_rate: u32,
    /// Samples are stored interleaved
    pub samples: Vec<f32>
}

impl DecodeResult {
    pub fn zero_length_message() -> String {
        "Audio files without samples (zero length) are not supported".to_string()
    }
}
