use std::fmt;
use std::path::Path;
use std::process::{Command, Output};

#[cfg(not(target_os = "windows"))]
pub const FFMPEG_BINARY: &str = "ffmpeg";

#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY: &str = "ffmpeg.exe";

#[derive(Clone, Debug, PartialEq)]
pub enum TranscodeFormat {
    Aac,
    Aiff,
    Flac,
    Jpeg,
    Mp3Cbr128,
    Mp3Cbr320,
    Mp3VbrV0,
    OggVorbis,
    Wav
}

impl TranscodeFormat {
    pub fn suffix_and_extension(&self) -> &str {
        match self {
            TranscodeFormat::Aac => ".aac",
            TranscodeFormat::Aiff => ".aiff",
            TranscodeFormat::Flac => ".flac",
            TranscodeFormat::Jpeg => ".jpg", 
            TranscodeFormat::Mp3Cbr128 => "-128.mp3",
            TranscodeFormat::Mp3Cbr320 => "-320.mp3",
            TranscodeFormat::Mp3VbrV0 => "-v0.mp3",
            TranscodeFormat::OggVorbis => ".ogg",
            TranscodeFormat::Wav => ".wav"
        }
    }
}

impl fmt::Display for TranscodeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            TranscodeFormat::Aac => "AAC",
            TranscodeFormat::Aiff => "AIFF",
            TranscodeFormat::Flac => "FLAC",
            TranscodeFormat::Jpeg => "JPEG", 
            TranscodeFormat::Mp3Cbr128 => "MP3 128",
            TranscodeFormat::Mp3Cbr320 => "MP3 320",
            TranscodeFormat::Mp3VbrV0 => "MP3 V0",
            TranscodeFormat::OggVorbis => "Ogg Vorbis",
            TranscodeFormat::Wav => "WAV"
        };
        
        write!(f, "{}", text)
    }
}

pub fn transcode(input_file: &Path, output_file: &Path, target_format: &TranscodeFormat) -> Result<(), String> {
    let mut command = Command::new(FFMPEG_BINARY);
    
    command.arg("-y");
    command.arg("-i").arg(input_file);
    
    match target_format {
        TranscodeFormat::Mp3Cbr128 => {
            command.arg("-codec:a").arg("libmp3lame");
            command.arg("-b:a").arg("128");
        }
        TranscodeFormat::Mp3Cbr320 => {
            command.arg("-codec:a").arg("libmp3lame");
            command.arg("-b:a").arg("320");
        }
        TranscodeFormat::Mp3VbrV0 => {
            command.arg("-codec:a").arg("libmp3lame");
            command.arg("-qscale:a").arg("0");
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