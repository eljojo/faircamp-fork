pub struct BuildSettings {
    pub host_original_media: bool,
    pub transcode_flac: bool,
    pub transcode_mp3_320cbr: bool,
    pub transcode_mp3_256vbr: bool
}

impl BuildSettings {
    pub fn default() -> BuildSettings {
        BuildSettings {
            host_original_media: false,
            transcode_flac: true,
            transcode_mp3_320cbr: true,
            transcode_mp3_256vbr: false
        }
    }
}