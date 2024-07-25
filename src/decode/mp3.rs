// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;

use rmp3::{Decoder, Frame};

use super::DecodeResult;

pub fn decode(path: &Path) -> Result<DecodeResult, String> {
    let buffer = match fs::read(path) {
        Ok(buffer) => buffer,
        Err(err) => return Err(err.to_string())
    };
    
    let mut decoder = Decoder::new(&buffer);
    let mut result = None;
    
    while let Some(frame) = decoder.next() {
        if let Frame::Audio(audio) = frame {
            let result_unpacked = result.get_or_insert_with(|| {
                DecodeResult {
                    channels: audio.channels(),
                    duration: 0.0,
                    sample_count: 0,
                    sample_rate: audio.sample_rate(),
                    samples: Vec::new()
                }
            });
            
            let sample_count = audio.sample_count();
            
            if sample_count > 0 {
                result_unpacked.sample_count += sample_count as u32;
                result_unpacked.samples.reserve(result_unpacked.channels as usize * sample_count);
                
                for sample in audio.samples() {
                    // minimp3/rmp3 gives us raw decoded values, which by design can overshoot -1.0/1.0 slightly,
                    // we manually clamp these down to -1.0/1.0 here (see https://github.com/notviri/rmp3/issues/6)
                    result_unpacked.samples.push(sample.clamp(-1.0, 1.0));
                }
                
                result_unpacked.duration = result_unpacked.sample_count as f32 / result_unpacked.sample_rate as f32;
            }
        }
    }

    if let Some(decode_result) = result {
        if decode_result.duration > 0.0 {
            return Ok(decode_result);
        }
    }

    Err(DecodeResult::zero_length_message())
}
