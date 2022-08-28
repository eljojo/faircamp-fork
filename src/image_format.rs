#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageFormat {
    Artist,
    Background,
    Cover,
    Feed
}

impl ImageFormat {
    pub const ALL_FORMATS: [ImageFormat; 4] = [
        ImageFormat::Artist,
        ImageFormat::Background,
        ImageFormat::Cover,
        ImageFormat::Feed
    ];

    /// Leaving this for now in case we want to introduce different formats
    /// for different usages in the future (e.g. encode to .png for the
    /// covers that we include with the release download zips)
    pub fn extension(&self) -> &str {
        ".jpg"
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            ImageFormat::Artist => "JPEG (Artist image usage)",
            ImageFormat::Background => "JPEG (Background image usage)",
            ImageFormat::Cover => "JPEG (Cover image usage)",
            ImageFormat::Feed => "JPEG (Feed image usage)"
        };
        
        write!(f, "{}", text)
    }
}