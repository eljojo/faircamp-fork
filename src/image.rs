#[derive(Debug)]
pub struct Image {
    pub source_file: String,
    pub transcoded_file: String
}

impl Image {
    pub fn init(source_file: String, transcoded_file: String) -> Image {
        Image {
            source_file,
            transcoded_file
        }
    }
}