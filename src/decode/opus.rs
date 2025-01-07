// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::File;
use std::path::Path;

use ogg::PacketReader;
use opus::{Channels, Decoder};

use super::DecodeResult;

// Opus can only encode/decode at specific sample rates (8, 12, 16, 24, or 48 kHz).
// The original input sample rate, which would be available on
// `identification_header.input_sample_rate`, is of no importance to us, as
// we only decode in order to compute a visual representation (waveform).
// In terms of the decoder sample rate we choose, we follow the Opus
// specification, which says: "An Ogg Opus player SHOULD select the playback
// sample rate according to the following procedure: 1. If the hardware
// supports 48 kHz playback, decode at 48 kHz" (For our purposes we can
// consider ourselves a player, and as we merely perform computation on the
// decoded data, any sample rate is acceptable to us)
// See https://wiki.xiph.org/OggOpus#ID_Header
const DECODING_SAMPLE_RATE: u32 = 48000;

pub fn decode(path: &Path) -> Result<DecodeResult, String> {
    let identification_header = match opus_headers::parse_from_path(path) {
        Ok(headers) => headers.id,
        Err(err) => return Err(err.to_string())
    };

    let channels: u16 = identification_header.channel_count as u16;

    let mut reader = match File::open(path) {
        Ok(file) => PacketReader::new(file),
        Err(err) => return Err(err.to_string())
    };

    // Opus only supports mono and stereo, see https://opus-codec.org/
    let channels_enum = if channels == 1 { Channels::Mono } else { Channels::Stereo };

    let mut decoder = match Decoder::new(DECODING_SAMPLE_RATE, channels_enum) {
        Ok(decoder) => decoder,
        Err(err) => return Err(err.to_string())
    };

    let mut result = DecodeResult {
        channels,
        duration: 0.0,
        sample_count: 0,
        sample_rate: DECODING_SAMPLE_RATE,
        samples: Vec::new()
    };

    // Maximum packet duration is 120ms, which equals 5760 samples per channel at 48kHz
    // (and 48kHz is the maxium frame rate Opus supports, see https://opus-codec.org/)
    // https://opus-codec.org/docs/opus_api-1.1.2/group__opus__decoder.html#ga7d1111f64c36027ddcb81799df9b3fc9
    let mut buffer: Vec<f32> = vec![0.0; 5760 * 2];

    while let Ok(Some(packet)) = reader.read_packet() {
        if let Ok(samples_decoded_count) = decoder.decode_float(&packet.data, buffer.as_mut_slice(), false) {
            result.samples.reserve(samples_decoded_count * channels as usize);
            for sample in &buffer[..samples_decoded_count] {
                result.samples.push(*sample);
            }
            result.sample_count += samples_decoded_count as u32;
        }
    }

    if result.sample_count == 0 {
        return Err(DecodeResult::zero_length_message());
    }

    result.duration = result.sample_count as f32 / result.sample_rate as f32;

    Ok(result)
}
