use lewton::header::CommentHeader;
use lewton::inside_ogg::OggStreamReader;
use std::fs::File;
use std::path::Path;

use super::DecodeResult;

pub fn decode(path: &Path) -> Option<(DecodeResult, CommentHeader)> {
    let mut reader = match File::open(path) {
        Ok(file) => match OggStreamReader::new(file) {
            Ok(reader) => reader,
            Err(_) => return None
        },
        Err(_) => return None
    };

    let mut result = DecodeResult {
        channels: reader.ident_hdr.audio_channels as u16,
        duration: 0.0,
        sample_count: 0,
        sample_rate: reader.ident_hdr.audio_sample_rate,
        samples: Vec::new()
    };
    
    while let Ok(Some(packet_samples)) = reader.read_dec_packet_itl() {
        result.sample_count += packet_samples.len() as u32 / result.channels as u32;
        result.samples.reserve(packet_samples.len());
        
        for sample in packet_samples {
            result.samples.push(sample as f32 / std::i16::MAX as f32);
        }
        
        result.duration = result.sample_count as f32 / result.sample_rate as f32;
    }

    Some((result, reader.comment_hdr))
}