use claxon::{Block, FlacReader};
use hound::{SampleFormat, WavReader};
use id3;
use metaflac;
use rmp3::{Decoder, Frame};
use serde_derive::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const I24_MAX: i32 = 8388607;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioMeta {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub duration_seconds: u32,
    pub lossless: bool,
    pub peaks: Option<Vec<f32>>,
    pub title: Option<String>,
    pub track_number: Option<u32>
}

#[derive(Debug)]
struct DecodeResult {
    pub channels: u16,
    pub duration: f32,
    pub sample_count: u32,
    pub sample_rate: u32,
    pub samples: Vec<f32>
}

impl AudioMeta {
    pub fn extract(path: &Path, extension: &str) -> AudioMeta {
        let lossless = match extension {
            "aiff" | "alac" | "flac" | "wav" => true,
            "aac" | "mp3" | "ogg" => false,
            _ => unimplemented!("Determination whether extension {} indicates lossless audio in the file not yet implemented.", extension)
        };
        
        match extension {
            "flac" => {
                let (duration_seconds, peaks) = match decode_flac(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                    let track_number = tag
                        .get_vorbis("track") // TODO: Unconfirmed if that key is correct/available ("guessed it" for now :))
                        .map(|iter| iter.collect())
                        .filter(|str: &String| str.parse::<u32>().is_ok())
                        .map(|str: String| str.parse::<u32>().unwrap());
                    
                    AudioMeta {
                        album: tag.get_vorbis("album").map(|iter| iter.collect()),
                        artist: tag.get_vorbis("artist").map(|iter| iter.collect()),
                        duration_seconds,
                        lossless,
                        peaks,
                        title: tag.get_vorbis("title").map(|iter| iter.collect()),
                        track_number
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: None,
                        duration_seconds,
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "mp3" => {
                let (duration_seconds, peaks) = match decode_mp3(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                if let Ok(tag) = id3::Tag::read_from_path(path) {
                    AudioMeta {
                        album: tag.album().map(|str| str.to_string()),
                        artist: tag.artist().map(|str| str.to_string()),
                        duration_seconds,
                        lossless,
                        peaks,
                        title: tag.title().map(|str| str.to_string()),
                        track_number: tag.track()
                    }
                } else {
                    AudioMeta {
                        album: None,
                        artist: None,
                        duration_seconds,
                        lossless,
                        peaks,
                        title: None,
                        track_number: None
                    }
                }
            }
            "wav" => {
                let (duration_seconds, peaks) = match decode_wav(path) {
                    Some(decode_result) => (
                        decode_result.duration as u32,
                        Some(compute_peaks(decode_result, 320))
                    ),
                    None => (0, None)
                };
                
                AudioMeta {
                    album: None,
                    artist: None,
                    duration_seconds,
                    lossless,
                    peaks,
                    title: None,
                    track_number: None
                }
            }
            _ => {
                    AudioMeta {
                    album: None,
                    artist: None,
                    duration_seconds: 0,
                    lossless,
                    peaks: None,
                    title: None,
                    track_number: None
                }
            }
        }
    }
}

/// Takes interleaved samples and applies the following processing:
/// - Determine the largest absolute amplitude among all samples, throughout all channels
/// - Group every [n] samples into a window, for which the average positive and negative amplitude is stored
/// - Determine the largest absolute average amplitude among all calculated windows
/// - For all windows the averaged amplitudes are now upscaled again so that the maximum absolute window amplitude
///   is identical to the largest absolute amplitude found in all discrete samples
fn compute_peaks(decode_result: DecodeResult, points: u32) -> Vec<f32> {
    let window_size = (decode_result.channels as u32 * decode_result.sample_count) / points;

    let mut peaks = Vec::with_capacity(points as usize);

    let mut window_samples = 0;
    let mut window_accumulated = 0.0;

    let mut sample_abs_max: f32 = 0.0;
    let mut window_abs_max: f32 = 0.0;

    for amplitude in decode_result.samples {
        sample_abs_max = sample_abs_max.max(amplitude.abs());

        if window_samples > window_size {
            let peak = window_accumulated / window_samples as f32;

            window_abs_max = window_abs_max.max(peak);

            peaks.push(peak);

            window_samples = 0;
            window_accumulated = 0.0;
        }

        if amplitude.is_sign_positive() {
            window_accumulated += amplitude;
        } else {
            window_accumulated -= amplitude;
        }

        window_samples += 1;
    }

    let upscale = sample_abs_max / window_abs_max;
    
    peaks
        .iter()
        .map(|peak| {
            match "verbatim" {
               "verbatim" => peak * upscale,
               "log2" => (peak * 2.0 + 1.0).log2() * upscale,
               "log10" => (peak * 10.0 + 1.0).log10() * upscale,
               _ => unreachable!()
           }
    
        })
        .collect()
}

fn decode_flac(path: &Path) -> Option<DecodeResult> {
    let mut reader = match FlacReader::open(path) {
        Ok(reader) => reader,
        Err(_) => return None
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
            Err(error) => return None
        }
        
        let sample_count = block.duration();
        
        result.sample_count += sample_count;
        result.samples.reserve(sample_count as usize * result.channels as usize);
        
        for sample in 0..sample_count {
            for channel in 0..result.channels {
                let raw_sample = block.sample(channel as u32, sample);
                let normalized_sample = match streaminfo.bits_per_sample {
                    8 => raw_sample as f32 / std::i8::MAX as f32,
                    16 => raw_sample as f32 / std::i16::MAX as f32,
                    24 => raw_sample as f32 / I24_MAX as f32,
                    _ => unimplemented!()
                };

                result.samples.push(normalized_sample);
            }
        }
    }

    result.duration = result.sample_count as f32 / result.sample_rate as f32;
    
    Some(result)
}

fn decode_mp3(path: &Path) -> Option<DecodeResult> {
    let buffer = match fs::read(path) {
        Ok(buffer) => buffer,
        Err(_) => return None
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
    
    result
}

fn decode_wav(path: &Path) -> Option<DecodeResult> {
    let mut reader = match WavReader::open(path) {
        Ok(reader) => reader,
        Err(_) => return None
    };
    
    let sample_count = reader.duration();
    let spec = reader.spec();
    
    let mut result = DecodeResult {
        channels: spec.channels,
        duration: sample_count as f32 / spec.sample_rate as f32,
        sample_count: sample_count,
        sample_rate: spec.sample_rate,
        samples: Vec::with_capacity(sample_count as usize * spec.channels as usize)
    };
    
    match (spec.sample_format, spec.bits_per_sample) {
        (SampleFormat::Float, _) => for sample in reader.samples::<f32>() {
            result.samples.push(sample.unwrap());
        }
        (SampleFormat::Int, 8) => for sample in reader.samples::<i8>() {
            result.samples.push(sample.unwrap() as f32 / std::i8::MAX as f32);
        }
        (SampleFormat::Int, 16) => for sample in reader.samples::<i16>() {
            result.samples.push(sample.unwrap() as f32 / std::i16::MAX as f32);
        }
        (SampleFormat::Int, 24) => for sample in reader.samples::<i32>() {
            result.samples.push(sample.unwrap() as f32 / I24_MAX as f32);
        }
        (SampleFormat::Int, 32) => for sample in reader.samples::<i32>() {
            result.samples.push(sample.unwrap() as f32 / std::i32::MAX as f32);
        }
        _ => unimplemented!()
    }
    
    Some(result)
}