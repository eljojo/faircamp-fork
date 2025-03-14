// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use super::ImgAttributes;

/// A single, resized version of the cover image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverAsset {
    /// Represents both height and width (covers have a square aspect ratio)
    pub edge_size: u32,
    pub filename: String,
    pub filesize_bytes: u64
}

/// Represents multiple, differently sized versions of a cover image, for
/// display on different screen sizes and for inclusion in the release
/// archive. (Numbers refer to the square edge size, both height and width)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverAssets {
    pub marked_stale: Option<DateTime<Utc>>,
    pub max_160: CoverAsset,
    pub max_320: Option<CoverAsset>,
    pub max_480: Option<CoverAsset>,
    pub max_800: Option<CoverAsset>,
    pub max_1280: Option<CoverAsset>
}

impl CoverAsset {
    /// The filename of this asset as we are writing it to the release/track
    /// directory at build time, e.g. something like "cover_480.jpg"
    /// or "cover_240.jpg".
    pub fn target_filename(&self) -> String {
        let edge_size = self.edge_size;

        format!("cover_{edge_size}.jpg")
    }
}

impl CoverAssets {
    pub fn all(&self) -> Vec<&CoverAsset> {
        let mut result = Vec::with_capacity(4);

        result.push(&self.max_160);
        if let Some(asset) = &self.max_320 { result.push(asset); }
        if let Some(asset) = &self.max_480 { result.push(asset); }
        if let Some(asset) = &self.max_800 { result.push(asset); }
        if let Some(asset) = &self.max_1280 { result.push(asset); }

        result
    }

    pub fn img_attributes_up_to_320(&self, hash: &str, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_320 {
            Some(max_320) => vec![&self.max_160, max_320],
            None => vec![&self.max_160]
        };

        ImgAttributes::new_for_cover(assets, hash, prefix)
    }

    pub fn img_attributes_up_to_480(&self, hash: &str, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_320 {
            Some(max_320) => match &self.max_480 {
                Some(max_480) => vec![&self.max_160, max_320, max_480],
                None => vec![&self.max_160, max_320]
            }
            None => vec![&self.max_160]
        };

        ImgAttributes::new_for_cover(assets, hash, prefix)
    }

    pub fn img_attributes_up_to_1280(&self, hash: &str, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.max_160);
        if let Some(asset) = &self.max_320 { assets.push(asset); }
        if let Some(asset) = &self.max_480 { assets.push(asset); }
        if let Some(asset) = &self.max_800 { assets.push(asset); }
        if let Some(asset) = &self.max_1280 { assets.push(asset); }

        ImgAttributes::new_for_cover(assets, hash, prefix)
    }

    pub fn is_stale(&self) -> bool {
        self.marked_stale.is_some()
    }

    pub fn largest(&self) -> &CoverAsset {
        if let Some(max_1280) = &self.max_1280 {
            max_1280
        } else if let Some(max_800) = &self.max_800 {
            max_800
        } else if let Some(max_480) = &self.max_480 {
            max_480
        } else if let Some(max_320) = &self.max_320 {
            max_320
        } else {
            &self.max_160
        }
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    // Returns the best suited asset for usage in opengraph-based link
    // previews.
    pub fn opengraph_asset(&self) -> &CoverAsset {
        if let Some(max_800) = &self.max_800 {
            max_800
        } else if let Some(max_480) = &self.max_480 {
            max_480
        } else if let Some(max_320) = &self.max_320 {
            max_320
        } else {
            &self.max_160
        }
    }

    pub fn playlist_image(&self) -> String {
        let cover_asset = match &self.max_480 {
            Some(max_480) => max_480,
            None => match &self.max_320 {
                Some(max_320) => max_320,
                None => &self.max_160
            }
        };

        cover_asset.target_filename()
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}
