// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use hound::{SampleFormat, WavReader};

use super::{DecodeResult, I24_MAX};

pub fn decode(path: &Path) -> Result<DecodeResult, String> {
    let mut reader = match WavReader::open(path) {
        Ok(reader) => reader,
        Err(err) => return Err(err.to_string())
    };
    
    let sample_count = reader.duration();

    if sample_count == 0 {
        return Err(DecodeResult::zero_length_message());
    }

    let spec = reader.spec();
    
    let mut result = DecodeResult {
        channels: spec.channels,
        duration: sample_count as f32 / spec.sample_rate as f32,
        sample_count,
        sample_rate: spec.sample_rate,
        samples: Vec::with_capacity(sample_count as usize * spec.channels as usize)
    };
    
    match (spec.sample_format, spec.bits_per_sample) {
        (SampleFormat::Float, _) => for sample in reader.samples::<f32>() {
            result.samples.push(sample.unwrap());
        }
        (SampleFormat::Int, 8) => for sample in reader.samples::<i8>() {
            result.samples.push(sample.unwrap() as f32 / i8::MAX as f32);
        }
        (SampleFormat::Int, 16) => for sample in reader.samples::<i16>() {
            result.samples.push(sample.unwrap() as f32 / i16::MAX as f32);
        }
        (SampleFormat::Int, 24) => for sample in reader.samples::<i32>() {
            result.samples.push(sample.unwrap() as f32 / I24_MAX as f32);
        }
        (SampleFormat::Int, 32) => for sample in reader.samples::<i32>() {
            result.samples.push(sample.unwrap() as f32 / i32::MAX as f32);
        }
        _ => unimplemented!()
    }
    
    Ok(result)
}