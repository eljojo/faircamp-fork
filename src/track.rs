use chrono::{DateTime, Utc};
use indoc::formatdoc;
use serde_derive::{Serialize, Deserialize};
use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc
};

use crate::{
    artist::Artist,
    asset_cache::{Asset, AssetIntent, CacheManifest, SourceFileSignature},
    audio_format::{AUDIO_FORMATS, AudioFormat},
    audio_meta::AudioMeta,
    build::Build,
    ffmpeg::{self, MediaFormat},
    util
};

const HOUR_SECONDS: u32 = 60 * 60;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedTrackAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub flac: Option<Asset>,
    pub mp3_128: Option<Asset>,
    pub mp3_320: Option<Asset>,
    pub mp3_v0: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    // TODO: There is overlap between this and track.source_file - probably implications for model changes that could/should be made
    pub source_file_signature: SourceFileSignature,
    pub source_meta: AudioMeta,
    pub uid: String,
    pub wav: Option<Asset>
}

#[derive(Debug)]
pub struct Track {
    pub artists: Vec<Rc<Artist>>,
    pub cached_assets: CachedTrackAssets,
    pub source_file: PathBuf,
    pub title: String
}

impl CachedTrackAssets {
    pub fn deserialize(path: &Path) -> Option<CachedTrackAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<CachedTrackAssets>(&bytes).ok(),
            Err(_) => None
        }
    }
    
    pub fn get(&self, format: &AudioFormat) -> &Option<Asset> {
        match format {
            AudioFormat::Aac => &self.aac,
            AudioFormat::Aiff => &self.aiff,
            AudioFormat::Flac => &self.flac,
            AudioFormat::Mp3Cbr128 => &self.mp3_128,
            AudioFormat::Mp3Cbr320 => &self.mp3_320,
            AudioFormat::Mp3VbrV0 => &self.mp3_v0,
            AudioFormat::OggVorbis => &self.ogg_vorbis,
            AudioFormat::Wav => &self.wav
        }
    }
    
    pub fn get_mut(&mut self, format: &AudioFormat) -> &mut Option<Asset> {
        match format {
            AudioFormat::Aac => &mut self.aac,
            AudioFormat::Aiff => &mut self.aiff,
            AudioFormat::Flac => &mut self.flac,
            AudioFormat::Mp3Cbr128 => &mut self.mp3_128,
            AudioFormat::Mp3Cbr320 => &mut self.mp3_320,
            AudioFormat::Mp3VbrV0 => &mut self.mp3_v0,
            AudioFormat::OggVorbis => &mut self.ogg_vorbis,
            AudioFormat::Wav => &mut self.wav
        }
    }
    
    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_TRACKS_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for format in AUDIO_FORMATS {
            if let Some(asset) = self.get_mut(format) {
                asset.mark_stale(timestamp);
            }
        }
    }

    pub fn new(source_file_signature: SourceFileSignature, source_meta: AudioMeta) -> CachedTrackAssets {
        CachedTrackAssets {
            aac: None,
            aiff: None,
            flac: None,
            mp3_128: None,
            mp3_320: None,
            mp3_v0: None,
            ogg_vorbis: None,
            source_file_signature,
            source_meta,
            uid: util::uid(),
            wav: None
        }
    }
    
    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), &serialized).unwrap();
    }
}

impl Track {
    pub fn duration_formatted(&self) -> String {
        match self.cached_assets.source_meta.duration_seconds {
            Some(seconds) => {
                if seconds > HOUR_SECONDS {
                    format!("{}:{}:{:02}", seconds / HOUR_SECONDS, (seconds % HOUR_SECONDS) / 60, seconds % 60)
                } else {
                    format!("{}:{:02}", (seconds % HOUR_SECONDS) / 60, seconds % 60)
                }
            },
            None => String::new()
        }
    }
    
    pub fn get_as(&self, format: &AudioFormat) -> &Option<Asset> {
        self.cached_assets.get(format)
    }
    
    pub fn get_or_transcode_as(&mut self, format: &AudioFormat, build: &Build, asset_intent: AssetIntent) -> &mut Asset {
        let cached_format = self.cached_assets.get_mut(format);
    
        match cached_format {
            Some(asset) => if asset_intent == AssetIntent::Deliverable { asset.unmark_stale(); }
            None => {
                let target_filename = format!("{}{}", util::uid(), format.extension());
            
                info_transcoding!("{:?} to {}", self.source_file, format);
                ffmpeg::transcode(
                    &self.source_file,
                    &build.cache_dir.join(&target_filename),
                    MediaFormat::Audio(format)
                ).unwrap();
            
                cached_format.replace(Asset::init(build, target_filename, asset_intent));
            }
        }
        
        cached_format.as_mut().unwrap()
    }
    
    pub fn init(
        artists: Vec<Rc<Artist>>,
        cached_assets: CachedTrackAssets,
        source_file: PathBuf,
        title: String
    ) -> Track {
        Track {
            artists,
            cached_assets,
            source_file,
            title
        }
    }
    
    pub fn write_peaks(&self, build_dir: &Path) {
        if let Some(peaks) = &self.cached_assets.source_meta.peaks {
            let height = 100;
            let width = peaks.len();
            let center_y = height as f32 / 2.0;
            
            let mut body = String::new();
            
            for (index, peak) in peaks.iter().enumerate() {
                let neg = format!(
                    r#"<rect height="{height}" width="{width}" x="{x}" y="{y}" />"#,
                    height = peak.pos * center_y,
                    width = 1,
                    x = index,
                    y = center_y
                );
                
                let pos = format!(
                    r#"<rect height="{height}" width="{width}" x="{x}" y="{y}" />"#,
                    height = peak.pos * center_y,
                    width = 1,
                    x = index,
                    y = (1.0 - peak.pos) * center_y
                );
                
                body.push_str(&neg);
                body.push_str(&pos);
            }
            
            let svg = formatdoc!(
                r##"
                    <?xml version="1.0" standalone="no"?>
                    <svg viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">
                        <style>
                            circle, rect {{
                                stroke: #3b3b3b;
                                stroke-width: 1;
                            }}
                        </style>
                        {body}
                    </svg>
                "##,
                body=body,
                height=height,
                width=width
            );
            
            fs::write(build_dir.join(format!("peaks_{}.svg", self.cached_assets.uid)), svg).unwrap();
        }
    }
}