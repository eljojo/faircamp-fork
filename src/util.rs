use std::{fs, io, path::Path};
use uuid::Uuid;

const BYTES_KB: u64 = 1024; 
const BYTES_MB: u64 = 1024 * BYTES_KB; 
const BYTES_GB: u64 = 1024 * BYTES_MB; 

pub fn ensure_dir(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
}

pub fn ensure_empty_dir(dir: &Path) {
    remove_dir(dir);
    fs::create_dir_all(dir).unwrap();
}

pub fn format_bytes(size: u64) -> String {
    if size >= 512 * BYTES_MB {
        format!("{:.1}GB", size as f64 / BYTES_GB as f64) // e.g. "0.5GB", "1.3GB", "13.8GB"
    } else if size >= 100 * BYTES_MB {
        format!("{}MB", size / BYTES_MB) // e.g. "64MB", "267MB", "510MB"
    } else if size >= 512 * BYTES_KB {
        format!("{:.1}MB", size as f64 / BYTES_MB as f64) // e.g. "0.5MB", "1.3MB", "62.4MB"
    } else {
        format!("{}KB", size / BYTES_KB) // e.g. "3KB", "267KB", "510KB"
    }
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