#[derive(Clone, Debug)]
pub struct DownloadFormats {
    pub aac: bool,
    pub flac: bool,
    pub mp3_320: bool,
    pub mp3_v0: bool,
    pub ogg_vorbis: bool
}

impl DownloadFormats {
    pub fn none() -> DownloadFormats {
        DownloadFormats {
            aac: false,
            flac: false,
            mp3_320: false,
            mp3_v0: false,
            ogg_vorbis: false
        }
    }
}