use chrono::{DateTime, Utc};
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
    
    pub fn new(
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
}