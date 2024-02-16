use alac::Reader;
use mp4parse::MetadataBox;
use std::fs::File;
use std::path::Path;

use super::DecodeResult;

pub fn decode(path: &Path) -> Option<DecodeResult> {
    let reader = match File::open(path) {
        Ok(file) => match Reader::new(file) {
            Ok(reader) => reader,
            Err(_) => return None
        },
        Err(_) => return None
    };

    let stream_info = reader.stream_info();

    let mut result = DecodeResult {
        channels: stream_info.channels() as u16,
        duration: 0.0,
        sample_count: 0,
        sample_rate: stream_info.sample_rate(),
        samples: Vec::new()
    };

    // TODO: Decoding the stream batch-wise in packets could/should be faster.
    //       Do a test implementation to see if it really is (and keep it if so).
    //       See https://docs.rs/alac/latest/alac/struct.Reader.html#method.into_packets
    for sample in reader.into_samples::<i32>() {
        result.sample_count += 1;
        result.samples.push(sample.unwrap() as f32 / std::i32::MAX as f32);
    }

    result.sample_count = result.sample_count / result.channels as u32;
    result.duration = result.sample_count as f32 / result.sample_rate as f32;

    Some(result)
}

pub fn decode_meta(path: &Path) -> Option<MetadataBox> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None
    };

    let media_context = match mp4parse::read_mp4(&mut file){
        Ok(media_context) => media_context,
        Err(_) => return None
    };

    let user_data = match media_context.userdata {
        Some(Ok(user_data)) => user_data,
        _ => return None
    };

    user_data.meta
}