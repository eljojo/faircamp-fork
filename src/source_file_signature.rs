// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde_derive::{Deserialize, Serialize};

use crate::Build;

// TODO: At some point consider implementing a hash property which hashes the
//       file content. With this we can even reidentify files that have changed
//       path within the catalog directory. (Con: Some overhead from computing
//       the hash for the entire file (?))
// TODO: PartialEq should be extended to a custom logic probably (first check
//       path + size + modified, (then hash if we have implemented it))
/// This stores relevant metadata for checking whether files we are processing
/// in the current build match files we were processing in a previous build.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub modified: SystemTime,
    /// The path is relative to the catalog_dir root. This ensures
    /// that we can correctly re-associate files on each build, even
    /// if the catalog directory moves around on disk.
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64
}

impl SourceFileSignature {
    pub fn new(build: &Build, path: &Path) -> SourceFileSignature {
        let metadata = fs::metadata(build.catalog_dir.join(path))
            .expect("Could not access source file");

        SourceFileSignature {
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            path: path.to_path_buf(),
            size: metadata.len()
        }
    }
}