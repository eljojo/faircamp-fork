#[derive(Debug)]
pub struct Track {
    pub length: u8,
    pub source_file: String,
    pub title: String,
    pub transcoded_file: String
}

impl Track {
    pub fn init(source_file: String, title: String, transcoded_file: String) -> Track {
        Track {
            length: 0,
            source_file,
            title,
            transcoded_file
        }
    }
}