// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::File;
use std::path::Path;

use ogg::PacketReader;
use opus::{Channels, Decoder};

use super::DecodeResult;

pub fn decode(path: &Path) -> Option<DecodeResult> {
    let identification_header = match opus_headers::parse_from_path(path) {
        Ok(headers) => headers.id,
        Err(_) => return None
    };

    let channels: u16 = identification_header.channel_count as u16;
    let sample_rate: u32 = identification_header.input_sample_rate;

    let mut reader = match File::open(path) {
        Ok(file) => PacketReader::new(file),
        Err(_) => return None
    };

    // TODO: Rather an edge-case, but we should sometime check what happens
    //       when channels > 2 (and/or if that is even possible in opus).
    let channels_enum = if channels == 1 { Channels::Mono } else { Channels::Stereo };

    let mut decoder = match Decoder::new(sample_rate, channels_enum) {
        Ok(decoder) => decoder,
        Err(_) => return None
    };

    let mut result = DecodeResult {
        channels,
        duration: 0.0,
        sample_count: 0,
        sample_rate,
        samples: Vec::new()
    };

    // Maximum packet duration is 120ms, which equals 5760 samples per channel at 48kHz
    // https://opus-codec.org/docs/opus_api-1.1.2/group__opus__decoder.html#ga7d1111f64c36027ddcb81799df9b3fc9
    let mut buffer: Vec<f32> = vec![0.0; 5760 * 2];

    while let Ok(Some(packet)) = reader.read_packet() {
        if let Ok(samples_decoded_count) = decoder.decode_float(&packet.data, buffer.as_mut_slice(), false) {
            result.samples.reserve(samples_decoded_count);
            for sample in &buffer[..samples_decoded_count] {
                result.samples.push(*sample);
            }
            result.sample_count += samples_decoded_count as u32 / channels as u32;
        }
    }

    result.duration = result.sample_count as f32 / result.sample_rate as f32;

    Some(result)
}