pub mod aiff;
pub mod flac;
pub mod mp3;
pub mod ogg_vorbis;
pub mod opus;
pub mod wav;

const I24_MAX: i32 = 8388607;

#[derive(Debug)]
pub struct DecodeResult {
    pub channels: u16,
    pub duration: f32,
    pub sample_count: u32,
    pub sample_rate: u32,
    pub samples: Vec<f32>
}