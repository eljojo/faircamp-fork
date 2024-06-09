// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::{Asset, DownloadFormat, SourceFileSignature};

// TODO: Here we need to insert more factors of the dependency graph,
//       in addition to cover/extra/track_source_file_signature(s).
//       In concrete terms, especially: Tag overrides from manifests
//       are not considered at all.
/// Downloadable zip archives for a release, including cover and tracks
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archives {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub alac: Option<Asset>,
    pub cover_source_file_signature: Option<SourceFileSignature>,
    pub extra_source_file_signatures: Vec<SourceFileSignature>,
    pub flac: Option<Asset>,
    pub mp3_v0: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub opus_48: Option<Asset>,
    pub opus_96: Option<Asset>,
    pub opus_128: Option<Asset>,
    pub track_source_file_signatures: Vec<SourceFileSignature>,
    pub wav: Option<Asset>
}

#[derive(Clone, Debug)]
pub struct ArchivesRc {
    archives: Rc<RefCell<Archives>>,
}

impl Archives {
    /// Increase version on each change to the data layout of [Archives].
    /// This automatically informs the cache not to try to deserialize
    /// manifests that hold old, incompatible data.
    pub const CACHE_SERIALIZATION_KEY: &'static str = "archives1";

    pub fn deserialize_cached(path: &Path) -> Option<Archives> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<Archives>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn get(&self, download_format: DownloadFormat) -> &Option<Asset> {
        match download_format {
            DownloadFormat::Aac => &self.aac,
            DownloadFormat::Aiff => &self.aiff,
            DownloadFormat::Alac => &self.alac,
            DownloadFormat::Flac => &self.flac,
            DownloadFormat::Mp3VbrV0 => &self.mp3_v0,
            DownloadFormat::OggVorbis => &self.ogg_vorbis,
            DownloadFormat::Opus48Kbps => &self.opus_48,
            DownloadFormat::Opus96Kbps => &self.opus_96,
            DownloadFormat::Opus128Kbps => &self.opus_128,
            DownloadFormat::Wav => &self.wav
        }
    }

    pub fn get_mut(&mut self, download_format: DownloadFormat) -> &mut Option<Asset> {
        match download_format {
            DownloadFormat::Aac => &mut self.aac,
            DownloadFormat::Aiff => &mut self.aiff,
            DownloadFormat::Alac => &mut self.alac,
            DownloadFormat::Flac => &mut self.flac,
            DownloadFormat::Mp3VbrV0 => &mut self.mp3_v0,
            DownloadFormat::OggVorbis => &mut self.ogg_vorbis,
            DownloadFormat::Opus48Kbps => &mut self.opus_48,
            DownloadFormat::Opus96Kbps => &mut self.opus_96,
            DownloadFormat::Opus128Kbps => &mut self.opus_128,
            DownloadFormat::Wav => &mut self.wav
        }
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let mut hasher = DefaultHasher::new();

        self.cover_source_file_signature.hash(&mut hasher);
        self.extra_source_file_signatures.hash(&mut hasher);
        self.track_source_file_signatures.hash(&mut hasher);

        let source_file_signatures_hash = URL_SAFE_NO_PAD.encode(hasher.finish().to_le_bytes());
        let manifest_filename = format!("{source_file_signatures_hash}.{}.bincode", Archives::CACHE_SERIALIZATION_KEY);
        cache_dir.join(manifest_filename)
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
            if let Some(asset) = self.get_mut(download_format) {
                asset.mark_stale(timestamp);
            }
        }
    }

    pub fn new(
        cover_source_file_signature: Option<SourceFileSignature>,
        track_source_file_signatures: Vec<SourceFileSignature>,
        extra_source_file_signatures: Vec<SourceFileSignature>
    ) -> Archives {
        Archives {
            aac: None,
            aiff: None,
            alac: None,
            cover_source_file_signature,
            extra_source_file_signatures,
            flac: None,
            mp3_v0: None,
            ogg_vorbis: None,
            opus_48: None,
            opus_96: None,
            opus_128: None,
            track_source_file_signatures,
            wav: None
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}

impl ArchivesRc {
    pub fn borrow(&self) -> Ref<'_, Archives> {
        self.archives.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Archives> {
        self.archives.borrow_mut()
    }

    pub fn new(archives: Archives) -> ArchivesRc {
        ArchivesRc {
            archives: Rc::new(RefCell::new(archives))
        }
    }
}