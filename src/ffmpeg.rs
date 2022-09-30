use std::path::Path;
use std::process::{Command, Output};

use crate::AudioFormat;

#[cfg(not(target_os = "windows"))]
pub const FFMPEG_BINARY: &str = "ffmpeg";

#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY: &str = "ffmpeg.exe";

pub struct TagMapping {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>
}

pub fn transcode(
    input_file: &Path,
    output_file: &Path,
    target_format: AudioFormat,
    tag_mapping_option: &Option<TagMapping>
) -> Result<(), String> {
    let mut command = Command::new(FFMPEG_BINARY);
    
    command.arg("-y");
    command.arg("-i").arg(input_file);

    if let Some(tag_mapping) = tag_mapping_option {
        command.arg("-map_metadata").arg("-1");

        if let Some(album) = &tag_mapping.album {
            command.arg("-metadata").arg(format!("album={}", album));
        }

        if let Some(album_artist) = &tag_mapping.album_artist {
            command.arg("-metadata").arg(format!("album_artist={}", album_artist));
        }

        if let Some(artist) = &tag_mapping.artist {
            command.arg("-metadata").arg(format!("artist={}", artist));
        }

        if let Some(title) = &tag_mapping.title {
            command.arg("-metadata").arg(format!("title={}", title));
        }
    }

    match target_format {
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
        Err(_) => Err("The ffmpeg child process could not be executed.".to_string())
    }
}

fn transcode_debug_output(output: Output) -> String {
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    format!("stderr: {}\n\nstdout: {}", stderr, stdout)
}