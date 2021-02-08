use std::{fs, path::Path};
use uuid::Uuid;

pub fn ensure_dir(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
}

pub fn ensure_empty_dir(dir: &Path) {
    fs::remove_dir_all(dir).ok(); // TODO: Here and elsewhere - catch only certain Err conditions (i.e. exists but undeletable)
    fs::create_dir_all(dir).unwrap();
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}