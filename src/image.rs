use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Image {
    pub source_file: PathBuf,
    pub uuid: String
}

impl Image {
    pub fn init(source_file: &Path, uuid: String) -> Image {
        Image {
            source_file: source_file.to_path_buf(),
            uuid
        }
    }
}