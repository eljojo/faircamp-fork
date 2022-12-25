#[derive(Clone, Copy, Debug, PartialEq)]
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

/// Returns a tuple with ...
/// - .0 => Primary audio format and whether it is recommended
/// - .1 => Audio formats sorted by relevance for a non-expert listener, the recommended one marked as such 
/// Careful this does not support being called with an empty list of formats.
pub fn prioritized_for_download(
    download_formats: &Vec<AudioFormat>
) -> ((AudioFormat, bool), Vec<(AudioFormat, bool)>) {
    let mut sorted_formats = download_formats.clone();
    
    sorted_formats.sort_by(|a, b| a.download_rank().cmp(&b.download_rank()));
    
    let mut recommended_format = None;
    let mut recommendation_given = false;
    let prioritized_formats: Vec<(AudioFormat, bool)> = sorted_formats
        .iter()
        .map(|format| {
            if !recommendation_given && format.recommended_download() {
                recommended_format = Some(*format);
                recommendation_given = true;
                (*format, true)
            } else {
                (*format, false)
            }
        })
        .collect();

    let primary_format = match recommended_format {
        Some(format) => (format, true),
        None => *prioritized_formats.first().unwrap()
    };

    (
        primary_format,
        prioritized_formats
    )
}

impl AudioFormat {
    pub const ALL_FORMATS: [AudioFormat; 9] = [
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
    pub const DEFAULT_DOWNLOAD_FORMAT: AudioFormat = AudioFormat::Opus128Kbps;
    pub const FRUGAL_STREAMING_FORMAT: AudioFormat = AudioFormat::Opus48Kbps;
    pub const STANDARD_STREAMING_FORMAT: AudioFormat = AudioFormat::Opus96Kbps;

    /// Assets for each format are rendered into their own directory in order
    /// to avoid filename collisions and this returns the dirname for a format.
    pub fn asset_dirname(&self) -> &str {
        match self {
            AudioFormat::Aac => "aac",
            AudioFormat::Aiff => "aiff",
            AudioFormat::Flac => "flac",
            AudioFormat::Mp3VbrV0 => "mp3",
            AudioFormat::OggVorbis => "ogg",
            AudioFormat::Opus48Kbps => "opus-48",
            AudioFormat::Opus96Kbps => "opus-96",
            AudioFormat::Opus128Kbps => "opus-128",
            AudioFormat::Wav => "wav"
        }
    }

    /// A one-liner describing the format for someone unfamiliar with audio formats
    pub fn description(&self) -> &str {
        match self {
            AudioFormat::Aac => "Average encoding quality – appropriate if your player does not support better formats",
            AudioFormat::Aiff => "Uncompressed large files – appropriate only for audio production",
            AudioFormat::Flac => "Lossless and compressed – best choice for archival",
            AudioFormat::Mp3VbrV0 => "Inferior encoding quality – appropriate if compatibility with older players is needed",
            AudioFormat::OggVorbis => "Average encoding quality – appropriate if your player does not support better formats",
            AudioFormat::Opus48Kbps => "State-of-the-art encoding quality at 48Kbps – best choice for high-demand streaming",
            AudioFormat::Opus96Kbps => "State-of-the-art encoding quality at 96Kbps – best choice for streaming",
            AudioFormat::Opus128Kbps => "State-of-the-art encoding quality at 128Kbps – best choice for offline listening",
            AudioFormat::Wav => "Uncompressed large files – appropriate only for audio production"
        }
    }

    /// Abbreviated labels for use as clickable download links on the Downloads page
    pub fn download_label(&self) -> &str {
        match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Flac => "FLAC",
            AudioFormat::Mp3VbrV0 => "MP3",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Opus48Kbps => "Opus 48",
            AudioFormat::Opus96Kbps => "Opus 96",
            AudioFormat::Opus128Kbps => "Opus 128",
            AudioFormat::Wav => "WAV"
        }
    }

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
    
    /// A more verbose, user-facing description (e.g. for a download button)
    pub fn user_label(&self) -> &str {
        match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Flac => "FLAC",
            AudioFormat::Mp3VbrV0 => "MP3",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Opus48Kbps => "Opus 48Kbps",
            AudioFormat::Opus96Kbps => "Opus 96Kbps",
            AudioFormat::Opus128Kbps => "Opus 128Kbps",
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