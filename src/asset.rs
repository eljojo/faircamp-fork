// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::fs;

use crate::Build;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub filename: String,
    pub filesize_bytes: u64,
    pub marked_stale: Option<DateTime<Utc>>
}

#[derive(PartialEq)]
pub enum AssetIntent {
    Deliverable,
    Intermediate
}

// TODO: This underlying pattern (marked_stale field and mark_stale/is_stale/obsolete/etc. methods)
//       repeats a few times, at some point consider some reusable code solution for it.
impl Asset {
    pub fn new(build: &Build, filename: String, intent: AssetIntent) -> Asset {
        let metadata = fs::metadata(build.cache_dir.join(&filename)).unwrap();
        
        Asset {
            filename,
            filesize_bytes: metadata.len(),
            marked_stale: match intent {
                AssetIntent::Deliverable => None,
                AssetIntent::Intermediate => Some(build.build_begin)
            }
        }
    }
    
    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }
    
    pub fn is_stale(&self) -> bool {
        self.marked_stale.is_some()
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}
