// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AudioFormat;

/// Used to store the streaming quality configuration per release.
/// During processing this enum is also called upon to obtain the
/// concrete audio formats needed for a certain streaming quality.
#[derive(Clone, Copy, Debug)]
pub enum StreamingQuality {
    Frugal,
    Standard
}

impl StreamingQuality {
    /// Returns both streaming formats (we always render two) for iteration.
    /// [0] is the primary format (opus) which we preferentially offer for
    /// streaming through the website. [1] is the secondary format(mp3) which
    /// serves as a compatibility fallback for streaming through the website,
    /// but is used as the (only) format for podcast rss provision, as opus
    /// is not all supported in that context.
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

    pub fn from_key(key: &str) -> Result<StreamingQuality, String> {
        match key {
            "frugal" => Ok(StreamingQuality::Frugal),
            "standard" => Ok(StreamingQuality::Standard),
            _ => {
                let message = format!("Unknown key '{key}' (available keys: standard, frugal)");
                Err(message)
            }
        }
    }

    /// Returns just the secondary mp3 format
    pub fn mp3_format(&self) -> AudioFormat {
        match self {
            StreamingQuality::Frugal => AudioFormat::Mp3VbrV7,
            StreamingQuality::Standard => AudioFormat::Mp3VbrV5
        }
    }
}
