// SPDX-FileCopyrightText: 2021 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;
use std::process::{Command, Output};

pub fn sync(source_dir: &Path, destination: &str) -> Result<(), String> {
    let mut command = Command::new("rsync");
    
    command.arg("-avz");
    command.arg("--delete");
    command.arg(format!("{}/", source_dir.display()));
    command.arg(destination);
    
    info!("Deploying with command {:?}", command);
    
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let rsync_output = sync_debug_output(output);
                Err(format!("The rsync child process returned an error exit code.\n\n{}", rsync_output))
            }
        }
        Err(_) => Err("The rsync child process could not be executed.".to_string())
    }
}

fn sync_debug_output(output: Output) -> String {
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    format!("stderr: {}\n\nstdout: {}", stderr, stdout)
}