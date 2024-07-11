// SPDX-FileCopyrightText: 2024 Simon Repp
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
}
