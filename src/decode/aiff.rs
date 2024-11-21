// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use pacmog::PcmReader;

use super::DecodeResult;

pub fn decode(path: &Path) -> Result<DecodeResult, String> {
    let buffer = match fs::read(path) {
        Ok(buffer) => buffer,
        Err(err) => return Err(err.to_string())
    };

    let reader = PcmReader::new(&buffer);

    let specs = reader.get_pcm_specs();

    if specs.num_samples == 0 {
        return Err(DecodeResult::zero_length_message());
    }

    let mut result = DecodeResult {
        channels: specs.num_channels,
        duration: specs.num_samples as f32 / specs.sample_rate as f32,
        sample_count: specs.num_samples,
        sample_rate: specs.sample_rate,
        samples: Vec::with_capacity(specs.num_samples as usize * specs.num_channels as usize)
    };

    for sample in 0..specs.num_samples {
        for channel in 0..specs.num_channels {
            let sample_value = reader.read_sample(channel as u32, sample).unwrap();
            result.samples.push(sample_value);
        }
    }

    Ok(result)
}