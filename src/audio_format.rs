// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::{Display, Formatter};

use serde_derive::{Serialize, Deserialize};

/// Most generic/low-level audio format representation we use,
/// representing both download and streaming formats at a more
/// technical level.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum AudioFormat {
    Aac,
    Aiff,
    Alac,
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

/// A simplified format description, agnostic of bitrate.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum AudioFormatFamily {
    Aac,
    Aiff,
    Alac,
    Flac,
    Mp3,
    OggVorbis,
    Opus,
    Wav
}

impl AudioFormat {
    /// Assets for each format are rendered into their own directory in order
    /// to avoid filename collisions and this returns the dirname for a format.
    pub fn asset_dirname(&self) -> &str {
        match self {
            AudioFormat::Aac => "aac",
            AudioFormat::Aiff => "aiff",
            AudioFormat::Alac => "alac",
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
            AudioFormat::Alac => ".m4a",
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

    pub fn family(&self) -> AudioFormatFamily {
        match self {
            AudioFormat::Aac => AudioFormatFamily::Aac,
            AudioFormat::Aiff => AudioFormatFamily::Aiff,
            AudioFormat::Alac => AudioFormatFamily::Alac,
            AudioFormat::Flac => AudioFormatFamily::Flac,
            AudioFormat::Mp3VbrV0 |
            AudioFormat::Mp3VbrV5 |
            AudioFormat::Mp3VbrV7 => AudioFormatFamily::Mp3,
            AudioFormat::OggVorbis => AudioFormatFamily::OggVorbis,
            AudioFormat::Opus48Kbps |
            AudioFormat::Opus96Kbps |
            AudioFormat::Opus128Kbps => AudioFormatFamily::Opus,
            AudioFormat::Wav => AudioFormatFamily::Wav
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
            AudioFormat::Alac => unimplemented!(),
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

impl Display for AudioFormat {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let text = match self {
            AudioFormat::Aac => "AAC",
            AudioFormat::Aiff => "AIFF",
            AudioFormat::Alac => "ALAC",
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
        
        write!(formatter, "{}", text)
    }
}
