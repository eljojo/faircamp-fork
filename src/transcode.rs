use std::{
    path::Path,
    process::{Command, Output},
};

pub fn transcode(input_file: &Path, output_file: &Path) -> Result<(), String> {
    let mut command = Command::new("ffmpeg");
    
    // command.env("FOR_REFERENCE_ENV_VAR_SETTING", &self.data_dir);
    
    command.arg("-y");
    command.arg("-i").arg(input_file);
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