// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;
use std::process::{Command, Output};

use crate::{AudioFormat, AudioFormatFamily, TagMapping};

#[cfg(not(target_os = "windows"))]
pub const FFMPEG_BINARY: &str = "ffmpeg";

#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY: &str = "ffmpeg.exe";

/// FFmpeg does not always copy tags, this depends on the combination of
/// source and target format for a specific transcode. This function applies
/// extra flags to ensure tag copying in as many format combinations as
/// possible (it might not always be possible).
fn apply_tag_copy_flags(
    command: &mut Command,
    source_format_family: AudioFormatFamily,
    target_format_family: AudioFormatFamily
) {
    // With certain source/target format combinations we need to
    // explicitly map metadata from source to target. We can not
    // apply this explicit mapping by default, because for other
    // format combinations it has the opposite effect - no tags
    // would be copied at all there.

    // If we copy from Ogg Vorbis to anything but Ogg Vorbis or
    // Opus, expliclity map metadata.
    if source_format_family == AudioFormatFamily::OggVorbis {
        match target_format_family {
            AudioFormatFamily::Aac |
            AudioFormatFamily::Aiff |
            AudioFormatFamily::Alac |
            AudioFormatFamily::Flac |
            AudioFormatFamily::Mp3 |
            AudioFormatFamily::Wav => {
                command.arg("-map_metadata").arg("0:s:a:0");
            }
            AudioFormatFamily::OggVorbis |
            AudioFormatFamily::Opus => ()
        }
    }

    // If we copy from Opus to anything but Ogg Vorbis or
    // Opus, expliclity map metadata.
    if source_format_family == AudioFormatFamily::Opus {
        match target_format_family {
            AudioFormatFamily::Aac |
            AudioFormatFamily::Aiff |
            AudioFormatFamily::Alac |
            AudioFormatFamily::Flac |
            AudioFormatFamily::Mp3 |
            AudioFormatFamily::Wav => {
                command.arg("-map_metadata").arg("0:s:a:0");
            }
            AudioFormatFamily::OggVorbis |
            AudioFormatFamily::Opus => ()
        }
    }
}

/// FFmpeg does not always write tags, this depends on the muxer used for
// a specific format. This function applies extra flags to enable tag
// writing for all formats.
fn apply_tag_write_flags(
    command: &mut Command,
    target_format_family: AudioFormatFamily
) {
    // FFmpeg's adts (aac) muxer does not write (ID3v2.4) tags by default,
    // hence we manually enable it whenever we encode an AAC file.
    // (see https://ffmpeg.org/ffmpeg-formats.html#adts-1)
    if target_format_family == AudioFormatFamily::Aac {
        command.arg("-write_id3v2").arg("1");
    }

    // FFmpeg's aiff muxer does not write (ID3v2) tags by default,
    // hence we manually enable it whenever we encode an AIFF file.
    // With this enabled, ID3v2.4 tags are the default to be written.
    // (see https://ffmpeg.org/ffmpeg-formats.html#aiff-1)
    if target_format_family == AudioFormatFamily::Aiff {
        command.arg("-write_id3v2").arg("1");
    }
}

pub fn transcode(
    input_file: &Path,
    output_file: &Path,
    source_format_family: AudioFormatFamily,
    target_format: AudioFormat,
    tag_mapping: &TagMapping
) -> Result<(), String> {
    let mut command = Command::new(FFMPEG_BINARY);
    
    command.arg("-y");
    command.arg("-i").arg(input_file);

    match tag_mapping {
        TagMapping::Copy => {
            let target_format_family = target_format.family();

            apply_tag_copy_flags(&mut command, source_format_family, target_format_family);
            apply_tag_write_flags(&mut command, target_format_family);
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

            apply_tag_write_flags(&mut command, target_format.family());
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
