// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use claxon::{Block, FlacReader};

use super::{DecodeResult, I24_MAX};

pub fn decode(path: &Path) -> Result<DecodeResult, String> {
    let mut reader = match FlacReader::open(path) {
        Ok(reader) => reader,
        Err(err) => return Err(err.to_string())
    };
    
    let streaminfo = reader.streaminfo();
    let mut frame_reader = reader.blocks();
    
    let mut result = DecodeResult {
        channels: streaminfo.channels as u16,
        duration: 0.0,
        sample_count: 0,
        sample_rate: streaminfo.sample_rate,
        samples: Vec::new()
    };
    
    let mut block = Block::empty();
    
    loop {
        match frame_reader.read_next_or_eof(block.into_buffer()) {
            Ok(Some(next_block)) => block = next_block,
            Ok(None) => break,
            Err(err) => return Err(err.to_string())
        }
        
        let sample_count = block.duration();
        
        result.sample_count += sample_count;
        result.samples.reserve(sample_count as usize * result.channels as usize);
        
        for sample in 0..sample_count {
            for channel in 0..result.channels {
                let raw_sample = block.sample(channel as u32, sample);
                let normalized_sample = match streaminfo.bits_per_sample {
                    8 => raw_sample as f32 / i8::MAX as f32,
                    16 => raw_sample as f32 / i16::MAX as f32,
                    24 => raw_sample as f32 / I24_MAX as f32,
                    _ => unimplemented!()
                };

                result.samples.push(normalized_sample);
            }
        }
    }

    if result.sample_count == 0 {
        return Err(DecodeResult::zero_length_message());
    }

    result.duration = result.sample_count as f32 / result.sample_rate as f32;
    
    Ok(result)
}