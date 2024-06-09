// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::{
    Asset,
    AudioFormat,
    AudioMeta,
    SourceFileSignature
};
use crate::util::url_safe_hash;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transcodes {
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
    // TODO: Revisit why source_meta is on Transcodes (rather than ... Track maybe?)
    pub source_meta: AudioMeta,
    pub wav: Option<Asset>
}

#[derive(Clone, Debug)]
pub struct TranscodesRc {
    transcodes: Rc<RefCell<Transcodes>>,
}

impl Transcodes {
    /// Increase version on each change to the data layout of [Transcodes].
    /// This automatically informs the cache not to try to deserialize
    /// manifests that hold old, incompatible data.
    pub const CACHE_SERIALIZATION_KEY: &'static str = "transcodes1";

    pub fn deserialize_cached(path: &Path) -> Option<Transcodes> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<Transcodes>(&bytes).ok(),
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
        let source_file_signature_hash = url_safe_hash(&self.source_file_signature);
        let manifest_filename = format!("{source_file_signature_hash}.{}.bincode", Transcodes::CACHE_SERIALIZATION_KEY);
        cache_dir.join(manifest_filename)
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
    ) -> Transcodes {
        Transcodes {
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
            wav: None
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}

impl TranscodesRc {
    pub fn borrow(&self) -> Ref<'_, Transcodes> {
        self.transcodes.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Transcodes> {
        self.transcodes.borrow_mut()
    }

    pub fn new(transcodes: Transcodes) -> TranscodesRc {
        TranscodesRc {
            transcodes: Rc::new(RefCell::new(transcodes))
        }
    }
}
