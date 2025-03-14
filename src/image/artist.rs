// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use super::ImgAttributes;

/// A single, resized version of the artist image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAsset {
    /// This is the filename in cache, at build time we derive the filename
    /// using the target_filename() function.
    pub filename: String,
    pub filesize_bytes: u64,
    pub format: String,
    pub height: u32,
    pub width: u32
}

/// Represents multiple, differently sized versions of an artist image, for
/// display on different screen sizes. (Numbers refer to maximum width)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAssets {
    pub fixed_max_320: ArtistAsset,
    pub fixed_max_480: Option<ArtistAsset>,
    pub fixed_max_640: Option<ArtistAsset>,
    pub fluid_max_640: ArtistAsset,
    pub fluid_max_960: Option<ArtistAsset>,
    pub fluid_max_1280: Option<ArtistAsset>,
    pub marked_stale: Option<DateTime<Utc>>
}

impl ArtistAsset {
    /// The filename of this asset as we are writing it to the artist
    /// directory at build time, e.g. something like "fixed_480x240.jpg"
    /// or "fluid_640x240.jpg".
    pub fn target_filename(&self) -> String {
        let format = &self.format;
        let height = self.height;
        let width = self.width;

        format!("image_{format}_{width}x{height}.jpg")
    }
}

impl ArtistAssets {
    pub fn all(&self) -> Vec<&ArtistAsset> {
        let mut result = Vec::with_capacity(4);

        result.push(&self.fixed_max_320);
        if let Some(asset) = &self.fixed_max_480 { result.push(asset); }
        if let Some(asset) = &self.fixed_max_640 { result.push(asset); }
        result.push(&self.fluid_max_640);
        if let Some(asset) = &self.fluid_max_960 { result.push(asset); }
        if let Some(asset) = &self.fluid_max_1280 { result.push(asset); }

        result
    }

    pub fn img_attributes_fixed(
        &self,
        hash: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fixed_max_320);
        if let Some(asset) = &self.fixed_max_480 { assets.push(asset); }
        if let Some(asset) = &self.fixed_max_640 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, hash, prefix)
    }

    pub fn img_attributes_fluid(
        &self,
        hash: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fluid_max_640);
        if let Some(asset) = &self.fluid_max_960 { assets.push(asset); }
        if let Some(asset) = &self.fluid_max_1280 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, hash, prefix)
    }

    pub fn is_stale(&self) -> bool {
        self.marked_stale.is_some()
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    // Returns the best suited asset for usage in opengraph-based link
    // previews.
    pub fn opengraph_asset(&self) -> &ArtistAsset {
        if let Some(fluid_max_960) = &self.fluid_max_960 {
            fluid_max_960
        } else if let Some(fixed_max_640) = &self.fixed_max_640 {
            fixed_max_640
        } else if let Some(fixed_max_480) = &self.fixed_max_480 {
            fixed_max_480
        } else {
            &self.fixed_max_320
        }
    }

    /// Return just the filename of the image asset best suited for use in M3U
    /// playlists.
    pub fn playlist_image(&self) -> String {
        let artist_asset = match &self.fixed_max_480 {
            Some(fixed_max_480) => fixed_max_480,
            None => &self.fixed_max_320
        };

        artist_asset.target_filename()
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}
