// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::AudioFormat;

/// Used to store the streaming quality configuration per release.
/// During processing this enum is also called upon to obtain the
/// concrete audio formats needed for a certain streaming quality.
#[derive(Clone, Copy, Debug)]
pub enum StreamingQuality {
    Frugal,
    Standard,
    Original,
    Hybrid
}

impl StreamingQuality {
    /// [0] is primary (opus), [1] is fallback (mp3)
    pub fn formats(&self) -> Vec<AudioFormat> {
        match self {
            StreamingQuality::Frugal => [
                AudioFormat::Opus48Kbps,
                AudioFormat::Mp3VbrV7
            ].to_vec(),
            StreamingQuality::Standard => [
                AudioFormat::Opus96Kbps,
                AudioFormat::Mp3VbrV5
            ].to_vec(),
            StreamingQuality::Hybrid => [
                AudioFormat::Opus128Kbps,
                AudioFormat::Mp3Orig
            ].to_vec(),
            StreamingQuality::Original => [
                AudioFormat::Mp3Orig
            ].to_vec()
        }
    }

    pub fn from_key(key: &str) -> Result<StreamingQuality, String> {
        match key {
            "frugal" => Ok(StreamingQuality::Frugal),
            "standard" => Ok(StreamingQuality::Standard),
            "original" => Ok(StreamingQuality::Original),
            "hybrid" => Ok(StreamingQuality::Hybrid),
            _ => {
                let message = format!("Unknown key '{key}' (available keys: standard, frugal, original, hybrid)");
                Err(message)
            }
        }
    }
}
