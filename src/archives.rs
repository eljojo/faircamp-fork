// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::{Asset, DownloadFormat};
use crate::util::url_safe_base64;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archive {
    pub asset: Asset,
    pub format: DownloadFormat
}

/// Downloadable zip archives for a release, including cover, tracks
/// and extras such as liner notes, graphics, etc.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Archives {
    pub formats: Vec<Archive>,
    /// This is a hash computed from the entire dependency graph for downloads
    /// of this release, allowing us to retrieve the right archives with only
    /// 64 bits of space used.
    pub signature: u64
}

#[derive(Clone, Debug)]
pub struct ArchivesRc {
    archives: Rc<RefCell<Archives>>,
}

impl Archive {
    pub fn new(
        asset: Asset,
        format: DownloadFormat
    ) -> Archive {
        Archive {
            asset,
            format
        }
    }
}

impl Archives {
    /// Increase version on each change to the data layout of [Archive].
    /// This automatically informs the cache not to try to deserialize
    /// manifests that hold old, incompatible data.
    pub const CACHE_SERIALIZATION_KEY: &'static str = "archives1";

    pub fn deserialize_cached(path: &Path) -> Option<Archives> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<Archives>(&bytes).ok(),
            Err(_) => None
        }
    }

    /// Only call this if you know the format must exist (e.g. right after requesting
    /// it through [transcode_as]), because it will panic if it doesn't.
    pub fn get_unchecked(&self, format: DownloadFormat) -> &Archive {
        self.formats
            .iter()
            .find(|archive| archive.format == format)
            .unwrap()
    }

    pub fn get_mut(&mut self, format: DownloadFormat) -> Option<&mut Archive> {
        self.formats
            .iter_mut()
            .find(|archive| archive.format == format)
    }

    pub fn has(&self, format: DownloadFormat) -> bool {
        self.formats
            .iter()
            .any(|archive| archive.format == format)
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let manifest_filename = format!("{}.{}.bincode", url_safe_base64(self.signature), Archives::CACHE_SERIALIZATION_KEY);
        cache_dir.join(manifest_filename)
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for archive in self.formats.iter_mut() {
            archive.asset.mark_stale(timestamp);
        }
    }

    pub fn new(signature: u64) -> Archives {
        Archives {
            formats: Vec::new(),
            signature
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