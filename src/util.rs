use std::{fs, io, path::Path};
use uuid::Uuid;

pub fn ensure_dir(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
}

pub fn ensure_empty_dir(dir: &Path) {
    remove_dir(dir);
    fs::create_dir_all(dir).unwrap();
}

pub fn is_lossless(extension: &str) -> bool {
    match extension {
        "aiff" | "alac" | "flac" | "wav" => true,
        "aac" | "mp3" | "ogg" => false,
        _ => unimplemented!("foo")
    }
}

pub fn remove_dir(dir: &Path) {
    match fs::remove_dir_all(dir) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => (), // just what we want anyway \o/
        Err(err) => panic!(err)
    };
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}