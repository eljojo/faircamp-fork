// SPDX-FileCopyrightText: 2021-2023 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    Cache,
    ffmpeg,
    SourceFileSignature,
    TagMapping,
    util
};

#[derive(Debug)]
pub struct Track {
    /// The final mapped artists (including metadata). Used in assembling the final page.
    pub artists: Vec<Rc<RefCell<Artist>>>,
    /// Names/aliases that should be mapped to this track, coming from the
    /// audio file metadata or from manifest overrides. Only relevant as an
    /// intermediate step before actual mapping.
    pub artists_to_map: Vec<String>,
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download/stream asset filenames.
    pub asset_basename: Option<String>,
    pub assets: Rc<RefCell<TrackAssets>>,
    pub title: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrackAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub alac: Option<Asset>,
    pub flac: Option<Asset>,
    pub mp3_v0: Option<Asset>,
    pub mp3_v5: Option<Asset>,
    pub mp3_v7: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub opus_48: Option<Asset>,
    pub opus_96: Option<Asset>,
    pub opus_128: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub source_meta: AudioMeta,
    pub uid: String,
    pub wav: Option<Asset>
}

impl Track {
    pub fn transcode_as(
        &mut self,
        format: AudioFormat,
        build: &Build,
        asset_intent: AssetIntent,
        tag_mapping_option: &Option<TagMapping>
    ) {
        let mut assets_mut = self.assets.borrow_mut();

        if let Some(asset) = assets_mut.get_mut(format) {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let target_filename = format!("{}{}", util::uid(), format.extension());

            info_transcoding!("{:?} to {}", assets_mut.source_file_signature.path, format);
            ffmpeg::transcode(
                &build.catalog_dir.join(&assets_mut.source_file_signature.path),
                &build.cache_dir.join(&target_filename),
                format,
                tag_mapping_option
            ).unwrap();

            assets_mut.get_mut(format).replace(Asset::new(build, target_filename, asset_intent));
        }
    }
    
    pub fn new(
        artists_to_map: Vec<String>,
        assets: Rc<RefCell<TrackAssets>>,
        title: String
    ) -> Track {
        Track {
            artists: Vec::new(),
            artists_to_map,
            asset_basename: None,
            assets,
            title
        }
    }
}

impl TrackAssets {
    pub fn deserialize_cached(path: &Path) -> Option<TrackAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<TrackAssets>(&bytes).ok(),
            Err(_) => None
        }
    }
    
    pub fn get(&self, format: AudioFormat) -> &Option<Asset> {
        match format {
            AudioFormat::Aac => &self.aac,
            AudioFormat::Aiff => &self.aiff,
            AudioFormat::Alac => &self.alac,
            AudioFormat::Flac => &self.flac,
            AudioFormat::Mp3VbrV0 => &self.mp3_v0,
            AudioFormat::Mp3VbrV5 => &self.mp3_v5,
            AudioFormat::Mp3VbrV7 => &self.mp3_v7,
            AudioFormat::OggVorbis => &self.ogg_vorbis,
            AudioFormat::Opus48Kbps => &self.opus_48,
            AudioFormat::Opus96Kbps => &self.opus_96,
            AudioFormat::Opus128Kbps => &self.opus_128,
            AudioFormat::Wav => &self.wav
        }
    }
    
    pub fn get_mut(&mut self, format: AudioFormat) -> &mut Option<Asset> {
        match format {
            AudioFormat::Aac => &mut self.aac,
            AudioFormat::Aiff => &mut self.aiff,
            AudioFormat::Alac => &mut self.alac,
            AudioFormat::Flac => &mut self.flac,
            AudioFormat::Mp3VbrV0 => &mut self.mp3_v0,
            AudioFormat::Mp3VbrV5 => &mut self.mp3_v5,
            AudioFormat::Mp3VbrV7 => &mut self.mp3_v7,
            AudioFormat::OggVorbis => &mut self.ogg_vorbis,
            AudioFormat::Opus48Kbps => &mut self.opus_48,
            AudioFormat::Opus96Kbps => &mut self.opus_96,
            AudioFormat::Opus128Kbps => &mut self.opus_128,
            AudioFormat::Wav => &mut self.wav
        }
    }
    
    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(Cache::TRACK_MANIFESTS_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
            if let Some(track_asset) = self.get_mut(audio_format) {
                track_asset.mark_stale(timestamp);
            }
        }
    }

    pub fn new(
        source_file_signature: SourceFileSignature,
        source_meta: AudioMeta
    ) -> TrackAssets {
        TrackAssets {
            aac: None,
            aiff: None,
            alac: None,
            flac: None,
            mp3_v0: None,
            mp3_v5: None,
            mp3_v7: None,
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
    
    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}
