// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::Build;

/// A single, resized version of an image for usage in feeds.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeedImageAsset {
    /// Represents both height and width (feed images have a square aspect ratio)
    pub edge_size: u32,
    /// This is the filename in cache, at build time we derive the filename
    /// using the TARGET_FILENAME constant.
    pub filename: String,
    pub filesize_bytes: u64,
    pub marked_stale: Option<DateTime<Utc>>
}

impl FeedImageAsset {
    pub const TARGET_FILENAME: &str = "feed.jpg";

    pub fn is_stale(&self) -> bool {
        self.marked_stale.is_some()
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    pub fn new(
        build: &Build,
        edge_size: u32,
        filename: String
    ) -> FeedImageAsset {
        let metadata = fs::metadata(build.cache_dir.join(&filename)).unwrap();

        FeedImageAsset {
            edge_size,
            filename,
            filesize_bytes: metadata.len(),
            marked_stale: None
        }
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}
