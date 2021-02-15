use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum AudioFormat {
    Aac,
    Aiff,
    Flac,
    Mp3Cbr128,
    Mp3Cbr320,
    Mp3VbrV0,
    OggVorbis,
    Wav
}

impl AudioFormat {
    pub fn suffix_and_extension(&self) -> &str {
        match self {
            AudioFormat::Aac => ".aac",
            AudioFormat::Aiff => ".aiff",
            AudioFormat::Flac => ".flac",
            AudioFormat::Mp3Cbr128 => "-128.mp3",
            AudioFormat::Mp3Cbr320 => "-320.mp3",
            AudioFormat::Mp3VbrV0 => "-v0.mp3",
            AudioFormat::OggVorbis => ".ogg",
            AudioFormat::Wav => ".wav"
        }
    }
}

impl fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Flac => "FLAC",
            AudioFormat::Mp3Cbr128 => "MP3 128",
            AudioFormat::Mp3Cbr320 => "MP3 320",
            AudioFormat::Mp3VbrV0 => "MP3 V0",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Wav => "WAV"
        };
        
        write!(f, "{}", text)
    }
}