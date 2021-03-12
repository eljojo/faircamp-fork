use id3;
use metaflac;
use rmp3::{Decoder, Frame};
use serde_derive::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioMeta {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub duration_seconds: Option<u32>,
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
        
        if extension == "flac" {
            if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                return AudioMeta {
                    album: tag.get_vorbis("album").map(|iter| iter.collect()),
                    artist: tag.get_vorbis("artist").map(|iter| iter.collect()),
                    duration_seconds: tag.get_streaminfo().map(|streaminfo|
                        (streaminfo.total_samples / streaminfo.sample_rate as u64) as u32
                    ),
                    lossless,
                    peaks: None,
                    title: tag.get_vorbis("title").map(|iter| iter.collect()),
                    track_number: tag.get_vorbis("track") // TODO: Unconfirmed if that key is correct/available ("guessed it" for now :))
                        .map(|iter| iter.collect())
                        .filter(|str: &String| str.parse::<u32>().is_ok())
                        .map(|str: String| str.parse::<u32>().unwrap()) 
                };
            }
        } else if extension == "mp3" {
            let peaks = match read_mp3_experimental(path) {
                Some(decode_result) => Some(compute_peaks(decode_result, 2560)),
                None => None
            };
            
            if let Ok(tag) = id3::Tag::read_from_path(path) {
                return AudioMeta {
                    album: tag.album().map(|str| str.to_string()),
                    artist: tag.artist().map(|str| str.to_string()),
                    duration_seconds: mp3_duration_from_frame_info(path),
                    lossless,
                    peaks,
                    title: tag.title().map(|str| str.to_string()),
                    track_number: tag.track()
                };
            }
        }
        
        AudioMeta {
            album: None,
            artist: None,
            duration_seconds: None,
            lossless,
            peaks: None,
            title: None,
            track_number: None
        }
    }
}

/// Takes the samples of a channel and applies the following processing:
/// - Determine the largest absolute amplitude among all the sample values
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

fn mp3_duration_from_frame_info(path: &Path) -> Option<u32> {
    let buffer = match fs::read(path) {
        Ok(buffer) => buffer,
        Err(_) => return None
    };
    
    let mut decoder = Decoder::new(&buffer);
    let mut duration_seconds: f32 = 0.0;
    
    while let Some(Frame::Audio(audio)) = decoder.peek() {
        let sample_count = audio.sample_count();
        
        if sample_count > 0 {
            duration_seconds += sample_count as f32 / audio.sample_rate() as f32;
        }
        
        decoder.skip();
    }
    
    Some(duration_seconds as u32)
}

fn read_mp3_experimental(path: &Path) -> Option<DecodeResult> {
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
            
            result_unpacked.sample_count += sample_count as u32;
            result_unpacked.samples.reserve(result_unpacked.channels as usize * sample_count);
            
            for sample in audio.samples() {
                result_unpacked.samples.push(*sample as f32 / std::i16::MAX as f32);
            }
            
            if sample_count > 0 {
                result_unpacked.duration = result_unpacked.sample_count as f32 / result_unpacked.sample_rate as f32;
            }
        }
    }
    
    result
}