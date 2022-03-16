use std::fmt;

pub const AUDIO_FORMATS: &[AudioFormat] = &[
    AudioFormat::Aac,
    AudioFormat::Aiff,
    AudioFormat::Flac,
    AudioFormat::Mp3Cbr128,
    AudioFormat::Mp3Cbr320,
    AudioFormat::Mp3VbrV0,
    AudioFormat::OggVorbis,
    AudioFormat::Wav
];

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

pub fn sorted_and_annotated_for_download(download_formats: &Vec<AudioFormat>) -> Vec<(&AudioFormat, Option<String>)> {
    download_formats.clone().sort_by(|a, b| a.download_rank().cmp(&b.download_rank()));
    
    let mut recommendation_given = false;
    
    download_formats
        .iter()
        .map(|format| {
            let annotation = if !recommendation_given && format.recommended_download() {
                recommendation_given = true;
                Some(String::from(" - Recommended Format"))
            } else {
                None
            };
            
            (format, annotation)
        })
        .collect()
}

impl AudioFormat {
    pub fn download_rank(&self) -> u8 {
        match self {
            AudioFormat::Mp3VbrV0 => 1,
            AudioFormat::Mp3Cbr320 => 2,
            AudioFormat::Mp3Cbr128 => 3,
            AudioFormat::OggVorbis => 4,
            AudioFormat::Flac => 5,
            AudioFormat::Aac => 6,
            AudioFormat::Wav => 7,
            AudioFormat::Aiff => 8
        }
    }
    
    pub fn extension(&self) -> &str {
        match self {
            AudioFormat::Aac => ".aac",
            AudioFormat::Aiff => ".aiff",
            AudioFormat::Flac => ".flac",
            AudioFormat::Mp3Cbr128 | AudioFormat::Mp3Cbr320 | AudioFormat::Mp3VbrV0 => ".mp3",
            AudioFormat::OggVorbis => ".ogg",
            AudioFormat::Wav => ".wav"
        }
    }
    pub fn from_manifest_key(key: &str) -> Option<AudioFormat> {
        match key {
            "aac" => Some(AudioFormat::Aac),
            "aiff" => Some(AudioFormat::Aiff),
            "flac" => Some(AudioFormat::Flac),
            "mp3_320" => Some(AudioFormat::Mp3Cbr320),
            "mp3_v0" => Some(AudioFormat::Mp3VbrV0),
            "ogg_vorbis" => Some(AudioFormat::OggVorbis),
            "wav" => Some(AudioFormat::Wav),
            _ =>  None
        }
    }
    
    pub fn lossless(&self) -> bool {
        match self {
            AudioFormat::Aac |
            AudioFormat::Mp3Cbr128 |
            AudioFormat::Mp3Cbr320 |
            AudioFormat::Mp3VbrV0 |
            AudioFormat::OggVorbis
                => false,
            AudioFormat::Aiff |
            AudioFormat::Flac |
            AudioFormat::Wav
                => true
        }
    }
    
    pub fn recommended_download(&self) -> bool {
        match self {
            AudioFormat::Aac |        // non-free technology
            AudioFormat::Aiff |       // wasteful
            AudioFormat::Mp3Cbr128 |  // such low quality only makes sense for streaming
            AudioFormat::Wav          // wasteful
                => false,
            AudioFormat::Flac |
            AudioFormat::Mp3Cbr320 |  // technically wasteful but recommendation-worthy in the absence of V0
            AudioFormat::Mp3VbrV0 |
            AudioFormat::OggVorbis
                => true
        }
    }
    
    // A more verbose, user-facing description (e.g. for a download button)
    pub fn user_label(&self) -> &str {
        match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Flac => "FLAC",
            AudioFormat::Mp3Cbr128 => "MP3 (CBR/128kbps)",
            AudioFormat::Mp3Cbr320 => "MP3 (CBR/320kbps)",
            AudioFormat::Mp3VbrV0 => "MP3 (VBR/V0)",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Wav => "WAV"
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