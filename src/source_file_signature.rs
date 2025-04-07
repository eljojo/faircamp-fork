// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde_derive::{Deserialize, Serialize};

use crate::Build;
use crate::util::url_safe_hash_base64;

/// This stores relevant metadata for checking whether files we are processing
/// in the current build match files we were processing in a previous build.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct FileMeta {
    pub modified: SystemTime,
    /// The path is relative to the catalog_dir root. This ensures
    /// that we can correctly re-associate files on each build, even
    /// if the catalog directory moves around on disk.
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64
}

/// The hash of the file content to be able to look up files
/// that have moved location.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct SourceHash {
    pub value: u64,
    version: usize
}

impl FileMeta {
    pub fn new(build: &Build, path: &Path) -> FileMeta {
        let metadata = fs::metadata(build.catalog_dir.join(path))
            .expect("Could not access source file");

        FileMeta {
            // Fallback to UNIX_EPOCH happens on platforms that don't support this field.
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            path: path.to_path_buf(),
            size: metadata.len()
        }
    }
}

impl SourceHash {
    /// Increment when our hashing algorithm changes, that way the hashes
    /// can be invalidated and recomputed for the cache.
    const HASHING_ALGORITHM_VERSION: usize = 1;

    /// Takes the wrapped `value` (the computed hash itself) and encodes
    /// and returns it as a url-safe base64 string.
    pub fn as_url_safe_base64(&self) -> String {
        url_safe_hash_base64(&self.value)
    }

    pub fn incompatible_version(&self) -> bool {
        self.version != SourceHash::HASHING_ALGORITHM_VERSION
    }

    pub fn new(path: &Path) -> SourceHash {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();

        let _ = file.read_to_end(&mut buffer);

        let value = seahash::hash(&buffer);

        SourceHash {
            value,
            version: SourceHash::HASHING_ALGORITHM_VERSION
        }
    }
}
