use std::path::PathBuf;

#[derive(Debug)]
pub struct Image {
    pub source_file: PathBuf,
    pub uuid: String
}

impl Image {
    pub fn init(source_file: PathBuf, uuid: String) -> Image {
        Image {
            source_file,
            uuid
        }
    }
}