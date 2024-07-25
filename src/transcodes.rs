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
    FileMeta,
    SourceHash,
    View
};
use crate::util::url_safe_base64;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transcode {
    pub asset: Asset,
    pub format: AudioFormat,
    /// This is a hash computed from TagMapping, allowing us to retrieve
    /// the right transcode without cloning an entire tag mapping struct
    /// to each transcode in the cache.
    pub tag_signature: u64
}

/// Holds the retrieved audio metadata (source_meta) of a uniquely
/// identified (hash) audio source file and all its available
/// transcoded versions (formats).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transcodes {
    pub formats: Vec<Transcode>,
    pub hash: SourceHash,
    pub source_meta: AudioMeta,
    pub views: Vec<View>
}

#[derive(Clone, Debug)]
pub struct TranscodesRc {
    transcodes: Rc<RefCell<Transcodes>>,
}

#[derive(Clone, Debug)]
pub struct TranscodesRcView {
    pub file_meta: FileMeta,
    transcodes: TranscodesRc
}

impl Transcode {
    pub fn new(
        asset: Asset,
        format: AudioFormat,
        tag_signature: u64
    ) -> Transcode {
        Transcode {
            asset,
            format,
            tag_signature
        }
    }
}

impl Transcodes {
    /// Increase version on each change to the data layout of [Transcodes]
    /// (or underlying structs that are contained within). This automatically
    /// informs the cache not to try to deserialize manifests that hold old,
    /// incompatible data.
    pub const CACHE_SERIALIZATION_KEY: &'static str = "transcodes3";

    pub fn deserialize_cached(path: &Path) -> Option<Transcodes> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<Transcodes>(&bytes).ok(),
            Err(_) => None
        }
    }

    /// Only call this if you know the format must exist (e.g. right after requesting
    /// it through [transcode_as]), because it will panic if it doesn't.
    pub fn get_unchecked(&self, format: AudioFormat, tag_signature: u64) -> &Transcode {
        self.formats
            .iter()
            .find(|transcode| transcode.format == format && transcode.tag_signature == tag_signature)
            .unwrap()
    }

    // TODO: Introduce a SomethingHash struct that holds an additional version identifer
    //       just like we do with SourceHash? (needs to happen all over the place)
    pub fn get_mut(&mut self, format: AudioFormat, tag_signature: u64) -> Option<&mut Transcode> {
        self.formats
            .iter_mut()
            .find(|transcode| transcode.format == format && transcode.tag_signature == tag_signature)
    }

    pub fn has(&self, format: AudioFormat, tag_signature: u64) -> bool {
        self.formats
            .iter()
            .any(|transcode| transcode.format == format && transcode.tag_signature == tag_signature)
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let manifest_filename = format!("{}.{}.bincode", url_safe_base64(self.hash.value), Transcodes::CACHE_SERIALIZATION_KEY);
        cache_dir.join(manifest_filename)
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for transcode in self.formats.iter_mut() {
            transcode.asset.mark_stale(timestamp);
        }

        for view in self.views.iter_mut() {
            view.mark_stale(timestamp);
        }
    }

    pub fn new(
        file_meta: FileMeta,
        hash: SourceHash,
        source_meta: AudioMeta
    ) -> Transcodes {
        Transcodes {
            formats: Vec::new(),
            hash,
            source_meta,
            views: vec![View::new(file_meta)]
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}

impl TranscodesRc {
    pub fn add_view(&self, file_meta: &FileMeta) {
        self.transcodes.borrow_mut().views.push(View::new(file_meta.clone()));
    }

    pub fn borrow(&self) -> Ref<'_, Transcodes> {
        self.transcodes.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Transcodes> {
        self.transcodes.borrow_mut()
    }

    pub fn matches_hash(&self, hash: &SourceHash) -> bool {
        self.transcodes.borrow().hash == *hash
    }

    pub fn new(file_meta: FileMeta, hash: SourceHash, source_meta: AudioMeta) -> TranscodesRc {
        let transcodes = Transcodes::new(file_meta, hash, source_meta);

        TranscodesRc {
            transcodes: Rc::new(RefCell::new(transcodes))
        }
    }

    pub fn retrieved(transcodes: Transcodes) -> TranscodesRc {
        TranscodesRc {
            transcodes: Rc::new(RefCell::new(transcodes))
        }
    }

    pub fn revive_view(&self, file_meta: &FileMeta) -> bool {
        for view_mut in self.transcodes.borrow_mut().views.iter_mut() {
            if view_mut.file_meta == *file_meta {
                view_mut.unmark_stale();
                return true;
            }
        }

        false
    }
}

impl TranscodesRcView {
    pub fn borrow(&self) -> Ref<'_, Transcodes> {
        self.transcodes.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Transcodes> {
        self.transcodes.borrow_mut()
    }

    pub fn new(file_meta: FileMeta, transcodes: TranscodesRc) -> TranscodesRcView {
        TranscodesRcView {
            file_meta,
            transcodes
        }
    }
}
