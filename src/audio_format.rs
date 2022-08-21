#[derive(Clone, Debug, PartialEq)]
pub enum AudioFormat {
    Aac,
    Aiff,
    Flac,
    Mp3VbrV0,
    OggVorbis,
    Opus48Kbps,
    Opus96Kbps,
    Opus128Kbps,
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
    pub const ALL_FORMATS: &'static [AudioFormat] = &[
        AudioFormat::Aac,
        AudioFormat::Aiff,
        AudioFormat::Flac,
        AudioFormat::Mp3VbrV0,
        AudioFormat::OggVorbis,
        AudioFormat::Opus48Kbps,
        AudioFormat::Opus96Kbps,
        AudioFormat::Opus128Kbps,
        AudioFormat::Wav
    ];
    pub const FRUGAL_STREAMING_FORMAT: AudioFormat = AudioFormat::Opus48Kbps;
    pub const STANDARD_STREAMING_FORMAT: AudioFormat = AudioFormat::Opus96Kbps;

    pub fn download_rank(&self) -> u8 {
        match self {
            AudioFormat::Opus128Kbps => 1,
            AudioFormat::Opus96Kbps => 2,
            AudioFormat::Opus48Kbps => 3,
            AudioFormat::Mp3VbrV0 => 4,
            AudioFormat::OggVorbis => 5,
            AudioFormat::Flac => 6,
            AudioFormat::Aac => 7,
            AudioFormat::Wav => 8,
            AudioFormat::Aiff => 9
        }
    }
    
    pub fn extension(&self) -> &str {
        match self {
            AudioFormat::Aac => ".aac",
            AudioFormat::Aiff => ".aiff",
            AudioFormat::Flac => ".flac",
            AudioFormat::Mp3VbrV0 => ".mp3",
            AudioFormat::OggVorbis => ".ogg",
            AudioFormat::Opus48Kbps |
            AudioFormat::Opus96Kbps |
            AudioFormat::Opus128Kbps => ".opus",
            AudioFormat::Wav => ".wav"
        }
    }
    pub fn from_manifest_key(key: &str) -> Option<AudioFormat> {
        match key {
            "aac" => Some(AudioFormat::Aac),
            "aiff" => Some(AudioFormat::Aiff),
            "flac" => Some(AudioFormat::Flac),
            "mp3" => Some(AudioFormat::Mp3VbrV0),
            "ogg_vorbis" => Some(AudioFormat::OggVorbis),
            "opus_48" => Some(AudioFormat::Opus48Kbps),
            "opus_96" => Some(AudioFormat::Opus96Kbps),
            "opus" | "opus_128" => Some(AudioFormat::Opus128Kbps),
            "wav" => Some(AudioFormat::Wav),
            _ =>  None
        }
    }
    
    pub fn lossless(&self) -> bool {
        match self {
            AudioFormat::Aac |
            AudioFormat::Mp3VbrV0 |
            AudioFormat::OggVorbis |
            AudioFormat::Opus48Kbps |
            AudioFormat::Opus96Kbps |
            AudioFormat::Opus128Kbps
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
            AudioFormat::Wav          // wasteful
                => false,
            AudioFormat::Flac |
            AudioFormat::Mp3VbrV0 |
            AudioFormat::OggVorbis |
            AudioFormat::Opus48Kbps |
            AudioFormat::Opus96Kbps |
            AudioFormat::Opus128Kbps
                => true
        }
    }
    
    // A more verbose, user-facing description (e.g. for a download button)
    pub fn user_label(&self) -> &str {
        match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Flac => "FLAC",
            AudioFormat::Mp3VbrV0 => "MP3 (VBR/V0)",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Opus48Kbps => "Opus (48Kbps)",
            AudioFormat::Opus96Kbps => "Opus (96Kbps)",
            AudioFormat::Opus128Kbps => "Opus (128Kbps)",
            AudioFormat::Wav => "WAV"
        }
    }
}

impl std::fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Flac => "FLAC",
            AudioFormat::Mp3VbrV0 => "MP3 V0",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Opus48Kbps => "Opus 48",
            AudioFormat::Opus96Kbps => "Opus 96",
            AudioFormat::Opus128Kbps => "Opus 128",
            AudioFormat::Wav => "WAV"
        };
        
        write!(f, "{}", text)
    }
}