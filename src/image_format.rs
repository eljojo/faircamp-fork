#[derive(Clone, Debug, PartialEq)]
pub enum ImageFormat {
    Jpeg
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Jpeg => ".jpg"
        }
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            ImageFormat::Jpeg => "JPEG"
        };
        
        write!(f, "{}", text)
    }
}