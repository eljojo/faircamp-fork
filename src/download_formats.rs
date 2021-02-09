#[derive(Clone, Debug)]
pub struct DownloadFormats {
    pub flac: bool,
    pub mp3_320: bool,
    pub mp3_v0: bool
}

impl DownloadFormats {
    pub fn none() -> DownloadFormats {
        DownloadFormats {
            flac: false,
            mp3_320: false,
            mp3_v0: false
        }
    }
}