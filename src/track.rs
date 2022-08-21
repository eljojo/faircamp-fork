use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc
};

use crate::{
    Artist,
    Asset,
    AssetIntent,
    AudioFormat,
    AudioMeta,
    Build,
    CacheManifest,
    ffmpeg,
    MediaFormat,
    SourceFileSignature,
    util
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedTrackAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub flac: Option<Asset>,
    pub mp3: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub opus_48: Option<Asset>,
    pub opus_96: Option<Asset>,
    pub opus_128: Option<Asset>,
    // TODO: There is overlap between this and track.source_file - probably implications for model changes that could/should be made
    pub source_file_signature: SourceFileSignature,
    pub source_meta: AudioMeta,
    pub uid: String,
    pub wav: Option<Asset>
}

#[derive(Debug)]
pub struct Track {
    /// The final mapped artists (including metadata). Used in assembling the final page.
    pub artists: Vec<Rc<RefCell<Artist>>>,
    /// Names/aliases that should be mapped to this track, coming from the audio file metadata or from manifest overrides. Only relevant as an intermediate step before actual mapping.
    pub artists_to_map: Vec<String>,
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
            AudioFormat::Mp3VbrV0 => &self.mp3,
            AudioFormat::OggVorbis => &self.ogg_vorbis,
            AudioFormat::Opus48Kbps => &self.opus_48,
            AudioFormat::Opus96Kbps => &self.opus_96,
            AudioFormat::Opus128Kbps => &self.opus_128,
            AudioFormat::Wav => &self.wav
        }
    }
    
    pub fn get_mut(&mut self, format: &AudioFormat) -> &mut Option<Asset> {
        match format {
            AudioFormat::Aac => &mut self.aac,
            AudioFormat::Aiff => &mut self.aiff,
            AudioFormat::Flac => &mut self.flac,
            AudioFormat::Mp3VbrV0 => &mut self.mp3,
            AudioFormat::OggVorbis => &mut self.ogg_vorbis,
            AudioFormat::Opus48Kbps => &mut self.opus_48,
            AudioFormat::Opus96Kbps => &mut self.opus_96,
            AudioFormat::Opus128Kbps => &mut self.opus_128,
            AudioFormat::Wav => &mut self.wav
        }
    }
    
    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_TRACKS_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for format in AudioFormat::ALL_FORMATS {
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
            mp3: None,
            ogg_vorbis: None,
            opus_48: None,
            opus_96: None,
            opus_128: None,
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
            
                cached_format.replace(Asset::new(build, target_filename, asset_intent));
            }
        }
        
        cached_format.as_mut().unwrap()
    }
    
    pub fn new(
        artists_to_map: Vec<String>,
        cached_assets: CachedTrackAssets,
        source_file: PathBuf,
        title: String
    ) -> Track {
        Track {
            artists: Vec::new(),
            artists_to_map,
            cached_assets,
            source_file,
            title
        }
    }
}