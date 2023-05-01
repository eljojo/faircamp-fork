use crate::Build;

/// Most generic/low-level audio format representation we use,
/// representing both download and streaming formats at a more
/// technical level.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioFormat {
    Aac,
    Aiff,
    Flac,
    /// VBR 220-260 KB/s (see https://trac.ffmpeg.org/wiki/Encode/MP3)
    Mp3VbrV0,
    /// VBR 120-150 KB/s (see https://trac.ffmpeg.org/wiki/Encode/MP3)
    Mp3VbrV5,
    /// VBR 80-120 KB/s (see https://trac.ffmpeg.org/wiki/Encode/MP3)
    Mp3VbrV7,
    OggVorbis,
    Opus48Kbps,
    Opus96Kbps,
    Opus128Kbps,
    Wav
}

/// A higher-level audio format representation that only covers
/// all audio formats that can be enabled for download. Routinely
/// "casted" to the more generic AudioFormat where needed.
#[derive(Clone, Copy, Debug)]
pub enum DownloadFormat {
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

/// Used to store the streaming quality configuration per release.
/// During processing this enum is also called upon to obtain the
/// concrete audio formats needed for a certain streaming quality.
#[derive(Clone, Copy, Debug)]
pub enum StreamingQuality {
    Frugal,
    Standard
}

/// Returns a tuple with ...
/// - .0 => Primary audio format and whether it is recommended
/// - .1 => Audio formats sorted by relevance for a non-expert listener, the recommended one marked as such 
/// Careful this does not support being called with an empty list of formats.
pub fn prioritized_for_download(
    download_formats: &[DownloadFormat]
) -> ((DownloadFormat, bool), Vec<(DownloadFormat, bool)>) {
    let mut sorted_formats = download_formats.to_owned();
    
    sorted_formats.sort_by_key(|format| format.download_rank());
    
    let mut recommended_format = None;
    let mut recommendation_given = false;
    let prioritized_formats: Vec<(DownloadFormat, bool)> = sorted_formats
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
    pub const ALL_AUDIO_FORMATS: [AudioFormat; 11] = [
        AudioFormat::Aac,
        AudioFormat::Aiff,
        AudioFormat::Flac,
        AudioFormat::Mp3VbrV0,
        AudioFormat::Mp3VbrV5,
        AudioFormat::Mp3VbrV7,
        AudioFormat::OggVorbis,
        AudioFormat::Opus48Kbps,
        AudioFormat::Opus96Kbps,
        AudioFormat::Opus128Kbps,
        AudioFormat::Wav
    ];

    /// Assets for each format are rendered into their own directory in order
    /// to avoid filename collisions and this returns the dirname for a format.
    pub fn asset_dirname(&self) -> &str {
        match self {
            AudioFormat::Aac => "aac",
            AudioFormat::Aiff => "aiff",
            AudioFormat::Flac => "flac",
            AudioFormat::Mp3VbrV0 => "mp3-v0",
            AudioFormat::Mp3VbrV5 => "mp3-v5",
            AudioFormat::Mp3VbrV7 => "mp3-v7",
            AudioFormat::OggVorbis => "ogg",
            AudioFormat::Opus48Kbps => "opus-48",
            AudioFormat::Opus96Kbps => "opus-96",
            AudioFormat::Opus128Kbps => "opus-128",
            AudioFormat::Wav => "wav"
        }
    }
    
    pub fn extension(&self) -> &str {
        match self {
            AudioFormat::Aac => ".aac",
            AudioFormat::Aiff => ".aiff",
            AudioFormat::Flac => ".flac",
            AudioFormat::Mp3VbrV0 |
            AudioFormat::Mp3VbrV5 |
            AudioFormat::Mp3VbrV7 => ".mp3",
            AudioFormat::OggVorbis => ".ogg",
            AudioFormat::Opus48Kbps |
            AudioFormat::Opus96Kbps |
            AudioFormat::Opus128Kbps => ".opus",
            AudioFormat::Wav => ".wav"
        }
    }

    /// The mime type that is used for the <source> tag in the streaming player.
    /// This is implemented only for the formats that are currently used for
    /// streaming (which are practically speaking hardcoded). If anybody wants
    /// to research and add mime types for formats currently not used for fun,
    /// please do provide a PR.
    ///
    /// References for opus:
    /// https://datatracker.ietf.org/doc/html/rfc7845#section-9
    /// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#audio_with_multiple_source_elements
    pub fn source_type(&self) -> &str {
        match self {
            AudioFormat::Aac => unimplemented!(),
            AudioFormat::Aiff => unimplemented!(),
            AudioFormat::Flac => unimplemented!(),
            AudioFormat::Mp3VbrV0 |
            AudioFormat::Mp3VbrV5 |
            AudioFormat::Mp3VbrV7 => "audio/mpeg",
            AudioFormat::OggVorbis => unimplemented!(),
            AudioFormat::Opus48Kbps |
            AudioFormat::Opus96Kbps |
            AudioFormat::Opus128Kbps => "audio/ogg; codecs=opus",
            AudioFormat::Wav => unimplemented!()
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
            AudioFormat::Mp3VbrV5 => "MP3 V5",
            AudioFormat::Mp3VbrV7 => "MP3 V7",
            AudioFormat::OggVorbis => "Ogg Vorbis",
            AudioFormat::Opus48Kbps => "Opus 48",
            AudioFormat::Opus96Kbps => "Opus 96",
            AudioFormat::Opus128Kbps => "Opus 128",
            AudioFormat::Wav => "WAV"
        };
        
        write!(f, "{}", text)
    }
}

impl DownloadFormat {
    pub const ALL_DOWNLOAD_FORMATS: [DownloadFormat; 9] = [
        DownloadFormat::Aac,
        DownloadFormat::Aiff,
        DownloadFormat::Flac,
        DownloadFormat::Mp3VbrV0,
        DownloadFormat::OggVorbis,
        DownloadFormat::Opus48Kbps,
        DownloadFormat::Opus96Kbps,
        DownloadFormat::Opus128Kbps,
        DownloadFormat::Wav
    ];

    pub const DEFAULT: DownloadFormat = DownloadFormat::Opus128Kbps;

    /// DownloadFormat is a more user-facing abstraction over AudioFormat,
    /// but when we go towards transcoding etc. we "downcast" it into the
    /// more generic, internal AudioFormat representation.
    pub fn as_audio_format(&self) -> AudioFormat {
        match self {
            DownloadFormat::Aac => AudioFormat::Aac,
            DownloadFormat::Aiff => AudioFormat::Aiff,
            DownloadFormat::Flac => AudioFormat::Flac,
            DownloadFormat::Mp3VbrV0 => AudioFormat::Mp3VbrV0,
            DownloadFormat::OggVorbis => AudioFormat::OggVorbis,
            DownloadFormat::Opus48Kbps => AudioFormat::Opus48Kbps,
            DownloadFormat::Opus96Kbps => AudioFormat::Opus96Kbps,
            DownloadFormat::Opus128Kbps => AudioFormat::Opus128Kbps,
            DownloadFormat::Wav => AudioFormat::Wav
        }
    }

    /// A one-liner describing the format for someone unfamiliar with audio formats.
    pub fn description(&self, build: &Build) -> String {
        match self {
            DownloadFormat::Aac => build.locale.translations.audio_format_description_aac.clone(),
            DownloadFormat::Aiff => build.locale.translations.audio_format_description_aiff.clone(),
            DownloadFormat::Flac => build.locale.translations.audio_format_description_flac.clone(),
            DownloadFormat::Mp3VbrV0 => build.locale.translations.audio_format_description_mp3_vbr.clone(),
            DownloadFormat::OggVorbis => build.locale.translations.audio_format_description_ogg_vorbis.clone(),
            DownloadFormat::Opus48Kbps => build.locale.translations.audio_format_description_opus_48.clone(),
            DownloadFormat::Opus96Kbps => build.locale.translations.audio_format_description_opus_96.clone(),
            DownloadFormat::Opus128Kbps => build.locale.translations.audio_format_description_opus_128.clone(),
            DownloadFormat::Wav => build.locale.translations.audio_format_description_wav.clone(),
        }
    }

    /// Abbreviated labels for use as clickable download links on the Downloads page
    pub fn download_label(&self) -> &str {
        match self {
            DownloadFormat::Aac => "AAC",
            DownloadFormat::Aiff => "AIFF",
            DownloadFormat::Flac => "FLAC",
            DownloadFormat::Mp3VbrV0 => "MP3",
            DownloadFormat::OggVorbis => "Ogg Vorbis",
            DownloadFormat::Opus48Kbps => "Opus 48",
            DownloadFormat::Opus96Kbps => "Opus 96",
            DownloadFormat::Opus128Kbps => "Opus 128",
            DownloadFormat::Wav => "WAV"
        }
    }

    pub fn download_rank(&self) -> u8 {
        match self {
            DownloadFormat::Opus128Kbps => 1,
            DownloadFormat::Opus96Kbps => 2,
            DownloadFormat::Opus48Kbps => 3,
            DownloadFormat::Mp3VbrV0 => 4,
            DownloadFormat::OggVorbis => 5,
            DownloadFormat::Flac => 6,
            DownloadFormat::Aac => 7,
            DownloadFormat::Wav => 8,
            DownloadFormat::Aiff => 9
        }
    }

    pub fn from_manifest_key(key: &str) -> Option<DownloadFormat> {
        match key {
            "aac" => Some(DownloadFormat::Aac),
            "aiff" => Some(DownloadFormat::Aiff),
            "flac" => Some(DownloadFormat::Flac),
            "mp3" => Some(DownloadFormat::Mp3VbrV0),
            "ogg_vorbis" => Some(DownloadFormat::OggVorbis),
            "opus_48" => Some(DownloadFormat::Opus48Kbps),
            "opus_96" => Some(DownloadFormat::Opus96Kbps),
            "opus" | "opus_128" => Some(DownloadFormat::Opus128Kbps),
            "wav" => Some(DownloadFormat::Wav),
            _ =>  None
        }
    }

    pub fn is_lossless(&self) -> bool {
        match self {
            DownloadFormat::Aac |
            DownloadFormat::Mp3VbrV0 |
            DownloadFormat::OggVorbis |
            DownloadFormat::Opus48Kbps |
            DownloadFormat::Opus96Kbps |
            DownloadFormat::Opus128Kbps
                => false,
            DownloadFormat::Aiff |
            DownloadFormat::Flac |
            DownloadFormat::Wav
                => true
        }
    }

    pub fn recommended_download(&self) -> bool {
        match self {
            DownloadFormat::Aac |        // non-free technology
            DownloadFormat::Aiff |       // wasteful
            DownloadFormat::Wav          // wasteful
                => false,
            DownloadFormat::Flac |
            DownloadFormat::Mp3VbrV0 |
            DownloadFormat::OggVorbis |
            DownloadFormat::Opus48Kbps |
            DownloadFormat::Opus96Kbps |
            DownloadFormat::Opus128Kbps
                => true
        }
    }

    /// A more verbose, user-facing description (e.g. for a download button)
    pub fn user_label(&self) -> &str {
        match self {
            DownloadFormat::Aac => "AAC",
            DownloadFormat::Aiff => "AIFF",
            DownloadFormat::Flac => "FLAC",
            DownloadFormat::Mp3VbrV0 => "MP3",
            DownloadFormat::OggVorbis => "Ogg Vorbis",
            DownloadFormat::Opus48Kbps => "Opus 48Kbps",
            DownloadFormat::Opus96Kbps => "Opus 96Kbps",
            DownloadFormat::Opus128Kbps => "Opus 128Kbps",
            DownloadFormat::Wav => "WAV"
        }
    }
}

impl StreamingQuality {
    /// [0] is primary (opus), [1] is fallback (mp3)
    pub fn formats(&self) -> [AudioFormat; 2] {
        match self {
            StreamingQuality::Frugal => [
                AudioFormat::Opus48Kbps,
                AudioFormat::Mp3VbrV7
            ],
            StreamingQuality::Standard => [
                AudioFormat::Opus96Kbps,
                AudioFormat::Mp3VbrV5
            ]
        }
    }
}