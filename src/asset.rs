use chrono::{DateTime, Duration, Utc};
use serde_derive::{Serialize, Deserialize};
use std::fs;

use crate::{Build, CacheOptimization};

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

impl Asset {
    pub fn new(build: &Build, filename: String, intent: AssetIntent) -> Asset {
        let metadata = fs::metadata(build.cache_dir.join(&filename)).expect("Could not access asset");
        
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
            self.marked_stale = Some(timestamp.clone());
        }
    }
    
    pub fn obsolete(&self, build: &Build) -> bool {
        match &self.marked_stale {
            Some(marked_stale) => {
                match &build.cache_optimization {
                    CacheOptimization::Default | 
                    CacheOptimization::Delayed => 
                        build.build_begin.signed_duration_since(marked_stale.clone()) > Duration::hours(24),
                    CacheOptimization::Immediate |
                    CacheOptimization::Manual |
                    CacheOptimization::Wipe => true
                }
            },
            None => false
        }
    }
    
    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}