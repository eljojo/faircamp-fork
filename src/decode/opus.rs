use ogg::PacketReader;
use opus::{Channels, Decoder};
use std::fs::File;
use std::path::Path;

use super::DecodeResult;

pub fn decode(path: &Path) -> Option<DecodeResult> {
    let mut reader = match File::open(path) {
        Ok(file) => PacketReader::new(file),
        Err(_) => return None
    };

    // TODO: Should actually be based on what the ogg container says
    let channel_count_hardcoded: u16 = 2; 
    let sample_rate_hardcoded: u32 = 48_000;

    let mut decoder = match Decoder::new(sample_rate_hardcoded, Channels::Stereo) {
        Ok(decoder) => decoder,
        Err(_) => return None
    };

    let mut result = DecodeResult {
        channels: channel_count_hardcoded, 
        duration: 0.0,
        sample_count: 0,
        sample_rate: sample_rate_hardcoded,
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
            result.sample_count += samples_decoded_count as u32 / channel_count_hardcoded as u32;
        }
    }

    result.duration = result.sample_count as f32 / result.sample_rate as f32;

    Some(result)
}