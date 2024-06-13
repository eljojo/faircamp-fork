// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;
use std::process::{Command, Output};

use crate::{AudioFormat, TagMapping};

#[cfg(not(target_os = "windows"))]
pub const FFMPEG_BINARY: &str = "ffmpeg";

#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY: &str = "ffmpeg.exe";

pub fn transcode(
    input_file: &Path,
    output_file: &Path,
    target_format: AudioFormat,
    tag_mapping: &TagMapping
) -> Result<(), String> {
    let mut command = Command::new(FFMPEG_BINARY);
    
    command.arg("-y");
    command.arg("-i").arg(input_file);

    match tag_mapping {
        TagMapping::Copy => {
            // Copy metadata from first audio stream to output.
            // Necessary because some conversions do not automatically carry
            // over the metadata (e.g. observed for opus to mp3).
            command.arg("-map_metadata").arg("0:s:a:0");
        }
        TagMapping::Custom { album, album_artist, artist, image, title, track } => {
            command.arg("-map_metadata").arg("-1");

            if let Some(album) = album {
                command.arg("-metadata").arg(format!("album={}", album));
            }

            if let Some(album_artist) = album_artist {
                command.arg("-metadata").arg(format!("album_artist={}", album_artist));
            }

            if let Some(artist) = artist {
                command.arg("-metadata").arg(format!("artist={}", artist));
            }

            if *image {
                // TODO: If we have a cover image (from folder) map it here with -i xxx and -map xxx
                // TODO: For this one we might need to differentiate between copy and rewrite up until here
            } else {
                command.arg("-vn");
            }

            if let Some(title) = title {
                command.arg("-metadata").arg(format!("title={}", title));
            }

            if let Some(track) = track {
                command.arg("-metadata").arg(format!("track={}", track));
            }
        }
        TagMapping::Remove => {
            command.arg("-map_metadata").arg("-1");
            command.arg("-vn");
        }
    }

    match target_format {
        AudioFormat::Alac => {
            command.arg("-vn");
            command.arg("-codec:a").arg("alac");
        }
        AudioFormat::Mp3VbrV0 => {
            command.arg("-codec:a").arg("libmp3lame");
            command.arg("-qscale:a").arg("0");
        }
        AudioFormat::Opus48Kbps => {
            command.arg("-codec:a").arg("libopus");
            command.arg("-b:a").arg("48k");
        }
        AudioFormat::Opus96Kbps => {
            command.arg("-codec:a").arg("libopus");
            command.arg("-b:a").arg("96k");
        }
        AudioFormat::Opus128Kbps => {
            command.arg("-codec:a").arg("libopus");
            command.arg("-b:a").arg("128k");
        }
        _ => ()
    }
    
    command.arg(output_file);

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let ffmpeg_output = transcode_debug_output(output);
                Err(format!("The ffmpeg child process returned an error exit code.\n\n{}", ffmpeg_output))
            }
        }
        Err(err) => Err(format!("The ffmpeg child process could not be executed.\n\n{err}"))
    }
}

fn transcode_debug_output(output: Output) -> String {
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    format!("stderr: {}\n\nstdout: {}", stderr, stdout)
}
