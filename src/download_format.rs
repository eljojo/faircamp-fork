// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::{Display, Formatter};

use serde_derive::{Serialize, Deserialize};

use crate::{AudioFormat, Build};

/// A higher-level audio format representation that only covers
/// all audio formats that can be enabled for download. Routinely
/// "casted" to the more generic AudioFormat where needed.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum DownloadFormat {
    Aac,
    Aiff,
    Alac,
    Flac,
    Mp3VbrV0,
    OggVorbis,
    Opus48Kbps,
    Opus96Kbps,
    Opus128Kbps,
    Wav
}

impl DownloadFormat {
    pub const DEFAULT: DownloadFormat = DownloadFormat::Opus128Kbps;

    /// DownloadFormat is a more user-facing abstraction over AudioFormat,
    /// but when we go towards transcoding etc. we "downcast" it into the
    /// more generic, internal AudioFormat representation.
    pub fn as_audio_format(&self) -> AudioFormat {
        match self {
            DownloadFormat::Aac => AudioFormat::Aac,
            DownloadFormat::Aiff => AudioFormat::Aiff,
            DownloadFormat::Alac => AudioFormat::Alac,
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
            DownloadFormat::Aac |
            DownloadFormat::OggVorbis => build.locale.translations.audio_format_average.clone(),
            DownloadFormat::Aiff |
            DownloadFormat::Wav => build.locale.translations.audio_format_uncompressed.clone(),
            DownloadFormat::Alac => build.locale.translations.audio_format_alac.clone(),
            DownloadFormat::Flac => build.locale.translations.audio_format_flac.clone(),
            DownloadFormat::Mp3VbrV0 => build.locale.translations.audio_format_mp3.clone(),
            DownloadFormat::Opus48Kbps => build.locale.translations.audio_format_opus_48.clone(),
            DownloadFormat::Opus96Kbps => build.locale.translations.audio_format_opus_96.clone(),
            DownloadFormat::Opus128Kbps => build.locale.translations.audio_format_opus_128.clone(),
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
            DownloadFormat::Alac => 7,
            DownloadFormat::Aac => 8,
            DownloadFormat::Wav => 9,
            DownloadFormat::Aiff => 10
        }
    }

    pub fn from_manifest_key(key: &str) -> Option<DownloadFormat> {
        match key {
            "aac" => Some(DownloadFormat::Aac),
            "aiff" => Some(DownloadFormat::Aiff),
            "alac" => Some(DownloadFormat::Alac),
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
            DownloadFormat::Alac |
            DownloadFormat::Flac |
            DownloadFormat::Wav
                => true
        }
    }

    /// Returns the download formats sorted by relevance for a non-expert listener, as a tuple ...
    /// - .0 => the respective format
    /// - .1 => boolean saying whether the format is recommended
    /// Careful this does not support being called with an empty list of formats.
    pub fn prioritized_for_download(download_formats: &[DownloadFormat]) -> Vec<(DownloadFormat, bool)> {
        let mut sorted_formats = download_formats.to_owned();

        sorted_formats.sort_by_key(|format| format.download_rank());

        let mut recommendation_given = false;

        sorted_formats
            .iter()
            .map(|format| {
                if !recommendation_given && format.recommended_download() {
                    recommendation_given = true;
                    (*format, true)
                } else {
                    (*format, false)
                }
            })
            .collect()
    }

    pub fn recommended_download(&self) -> bool {
        match self {
            DownloadFormat::Aac |        // non-free technology
            DownloadFormat::Aiff |       // wasteful
            DownloadFormat::Wav          // wasteful
                => false,
            DownloadFormat::Alac |
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
            DownloadFormat::Alac => "ALAC",
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

impl Display for DownloadFormat {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let text = match self {
            DownloadFormat::Aac => "AAC",
            DownloadFormat::Aiff => "AIFF",
            DownloadFormat::Alac => "ALAC",
            DownloadFormat::Flac => "FLAC",
            DownloadFormat::Mp3VbrV0 => "MP3",
            DownloadFormat::OggVorbis => "Ogg Vorbis",
            DownloadFormat::Opus48Kbps => "Opus 48Kbps",
            DownloadFormat::Opus96Kbps => "Opus 96Kbps",
            DownloadFormat::Opus128Kbps => "Opus 128Kbps",
            DownloadFormat::Wav => "WAV"
        };

        write!(formatter, "{}", text)
    }
}
