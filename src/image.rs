// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

use crate::{
    Asset,
    AssetIntent,
    Build,
    FileMeta,
    ImageInMemory,
    ResizeMode,
    SourceHash,
    View
};
use crate::util::url_safe_base64;

const BACKGROUND_MAX_EDGE_SIZE: u32 = 1280;
const FEED_MAX_EDGE_SIZE: u32 = 920;

/// Artist/cover images are resized towards certain max widths, e.g. 320, 480, 640.
/// The minimum width version (in the example 320) is always computed.
/// Each other version is only computed if the width of the original image
/// is MIN_OVERSHOOT times larger than the next smaller max width target.
/// I.e. a 460 wide image will be resized to 320 and 460 ("towards" 480) pixels,
/// but a 321 pixels wide image will only be resized to 320 pixels width.
const MIN_OVERSHOOT: f32 = 1.2;

/// A single, resized version of the artist image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAsset {
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

/// Associates an [ImageRcView] with an image description
#[derive(Clone, Debug)]
pub struct DescribedImage {
    pub description: Option<String>,
    pub image: ImageRcView
}

/// Stores the interior (mutable) payload of an image, comprised
/// of compressed/resized assets, the file-content based hash, and
/// views, that is, concrete locations on disk (path) and "in time"
/// (modified time, size) through which the somewhat virtual cache
/// data is concretely requested.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub artist_assets: Option<ArtistAssets>,
    pub background_asset: Option<Asset>,
    pub cover_assets: Option<CoverAssets>,
    pub feed_asset: Option<Asset>,
    /// Hash of the file content of the source image, with this we
    /// can uniquely identify and re-associate the computed cache
    /// data, no matter where the source file moves.
    pub hash: SourceHash,
    pub views: Vec<View>
}

#[derive(Clone, Debug)]
pub struct ImageRc {
    pub image: Rc<RefCell<Image>>,
}

#[derive(Clone, Debug)]
pub struct ImageRcView {
    pub file_meta: FileMeta,
    image: ImageRc
}

pub struct ImgAttributes {
    pub src: String,
    pub srcset: String
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
        permalink: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fixed_max_320);
        if let Some(asset) = &self.fixed_max_480 { assets.push(asset); }
        if let Some(asset) = &self.fixed_max_640 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, hash, permalink, prefix)
    }

    pub fn img_attributes_fluid(
        &self,
        hash: &str,
        permalink: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fluid_max_640);
        if let Some(asset) = &self.fluid_max_960 { assets.push(asset); }
        if let Some(asset) = &self.fluid_max_1280 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, hash, permalink, prefix)
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

    pub fn playlist_image(&self) -> String {
        let cover_asset = match &self.max_480 {
            Some(max_480) => max_480,
            None => match &self.max_320 {
                Some(max_320) => max_320,
                None => &self.max_160
            }
        };
        let edge_size = cover_asset.edge_size;

        format!("cover_{edge_size}.jpg")
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}

impl DescribedImage {
    pub fn new(description: Option<String>, image: ImageRcView) -> DescribedImage {
        DescribedImage {
            description,
            image
        }
    }
}

impl Image {
    /// Increase version on each change to the data layout of [Image].
    /// This automatically informs the cache not to try to deserialize
    /// manifests that hold old, incompatible data.
    pub const CACHE_SERIALIZATION_KEY: &'static str = "image1";

    pub fn artist_assets(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent,
        source_path: &Path
    ) -> &mut ArtistAssets {
        if let Some(assets) = self.artist_assets.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                assets.unmark_stale();
            }
        } else {
            info_resizing!("{:?} for usage as an artist image", &source_path);

            let image_in_memory = build.image_processor.open(build, source_path);
            let source_width = image_in_memory.width() as f32;

            // Compute fixed sizes.
            // Viewport width < 30rem (480px at 16px font-size) = 100vw/40vw = 2.5
            // Viewport width > 60rem (960px at 16px font-size) = 27rem/12rem = 2.25
            // We therefore approximate it for both by limiting the aspect to 2.25.-2.5

            let resize_mode_fixed_320 = ResizeMode::CoverRectangle {
                max_aspect: 2.5,
                max_width: 320,
                min_aspect: 2.25
            };
            let fixed_max_320 = Image::compute_artist_asset(build, "fixed", &image_in_memory, resize_mode_fixed_320);

            let fixed_max_480 = if source_width > 320.0 * MIN_OVERSHOOT {
                let resize_mode_fixed_480 = ResizeMode::CoverRectangle {
                    max_aspect: 2.5,
                    max_width: 480,
                    min_aspect: 2.25
                };
                Some(Image::compute_artist_asset(build, "fixed", &image_in_memory, resize_mode_fixed_480))
            } else {
                None
            };

            let fixed_max_640 = if source_width > 480.0 * MIN_OVERSHOOT {
                let resize_mode_fixed_640 = ResizeMode::CoverRectangle {
                    max_aspect: 2.5,
                    max_width: 640,
                    min_aspect: 2.25
                };
                Some(Image::compute_artist_asset(build, "fixed", &image_in_memory, resize_mode_fixed_640))
            } else {
                None
            };

            // Compute fluid sizes
            // Viewport width @ 30rem (480px at 16px font-size) = 100vw=30rem/12rem = 2.5
            // Viewport width @ 60rem (960px at 16px font-size) = 100vw=960px/12rem = 5
            // We therefore approximate it for both by limiting the aspect to 2.5-5

            let resize_mode_fluid_640 = ResizeMode::CoverRectangle {
                max_aspect: 5.0,
                max_width: 640,
                min_aspect: 2.5
            };
            let fluid_max_640 = Image::compute_artist_asset(build, "fluid", &image_in_memory, resize_mode_fluid_640);

            let fluid_max_960 = if source_width > 640.0 * MIN_OVERSHOOT {
                let resize_mode_fluid_960 = ResizeMode::CoverRectangle {
                    max_aspect: 5.0,
                    max_width: 960,
                    min_aspect: 2.5
                };
                Some(Image::compute_artist_asset(build, "fluid", &image_in_memory, resize_mode_fluid_960))
            } else {
                None
            };

            let fluid_max_1280 = if source_width > 960.0 * MIN_OVERSHOOT {
                let resize_mode_fluid_1280 = ResizeMode::CoverRectangle {
                    max_aspect: 5.0,
                    max_width: 1280,
                    min_aspect: 2.5
                };
                Some(Image::compute_artist_asset(build, "fluid", &image_in_memory, resize_mode_fluid_1280))
            } else {
                None
            };

            let artist_assets = ArtistAssets {
                fixed_max_320,
                fixed_max_480,
                fixed_max_640,
                fluid_max_640,
                fluid_max_960,
                fluid_max_1280,
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                }
            };

            self.artist_assets.replace(artist_assets);
        }

        self.artist_assets.as_mut().unwrap()
    }

    pub fn background_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent,
        source_path: &Path
    ) -> &mut Asset {
        if let Some(asset) = self.background_asset.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            info_resizing!("{:?} for usage as a background image", &source_path);

            let image_in_memory = build.image_processor.open(build, source_path);

            let resize_mode = ResizeMode::ContainInSquare { max_edge_size: BACKGROUND_MAX_EDGE_SIZE };
            let (filename, _dimensions) = build.image_processor.resize(build, &image_in_memory, resize_mode);

            self.background_asset.replace(Asset::new(build, filename, asset_intent));
        }

        self.background_asset.as_mut().unwrap()
    }

    fn compute_artist_asset(
        build: &Build,
        format: &str,
        image_in_memory: &ImageInMemory,
        resize_mode: ResizeMode
    ) -> ArtistAsset {
        let (filename, dimensions) = build.image_processor.resize(
            build,
            image_in_memory,
            resize_mode
        );

        let metadata = fs::metadata(build.cache_dir.join(&filename)).unwrap();

        ArtistAsset {
            filename,
            filesize_bytes: metadata.len(),
            format: format.to_string(),
            height: dimensions.1,
            width: dimensions.0
        }
    }

    fn compute_cover_asset(
        build: &Build,
        image_in_memory: &ImageInMemory,
        resize_mode: ResizeMode
    ) -> CoverAsset {
        let (filename, dimensions) = build.image_processor.resize(
            build,
            image_in_memory,
            resize_mode
        );

        let metadata = fs::metadata(build.cache_dir.join(&filename)).unwrap();

        CoverAsset {
            edge_size: dimensions.0,
            filename,
            filesize_bytes: metadata.len()
        }
    }

    pub fn cover_assets(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent,
        source_path: &Path
    ) -> &mut CoverAssets {
        if let Some(assets) = self.cover_assets.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                assets.unmark_stale();
            }
        } else {
            info_resizing!("{:?} for usage as a cover image", source_path);

            let image_in_memory = build.image_processor.open(build, source_path);
            let source_width = image_in_memory.width() as f32;

            let resize_mode_max_160 = ResizeMode::CoverSquare { edge_size: 160 };
            let max_160 = Image::compute_cover_asset(build, &image_in_memory, resize_mode_max_160);

            let max_320 = if source_width > 160.0 * MIN_OVERSHOOT {
                let resize_mode_max_320 = ResizeMode::CoverSquare { edge_size: 320 };
                Some(Image::compute_cover_asset(build, &image_in_memory, resize_mode_max_320))
            } else {
                None
            };

            let max_480 = if source_width > 320.0 * MIN_OVERSHOOT {
                let resize_mode_max_480 = ResizeMode::CoverSquare { edge_size: 480 };
                Some(Image::compute_cover_asset(build, &image_in_memory, resize_mode_max_480))
            } else {
                None
            };

            let max_800 = if source_width > 480.0 * MIN_OVERSHOOT {
                let resize_mode_max_800 = ResizeMode::CoverSquare { edge_size: 800 };
                Some(Image::compute_cover_asset(build, &image_in_memory, resize_mode_max_800))
            } else {
                None
            };

            let max_1280 = if source_width > 800.0 * MIN_OVERSHOOT {
                let resize_mode_max_1280 = ResizeMode::CoverSquare { edge_size: 1280 };
                Some(Image::compute_cover_asset(build, &image_in_memory, resize_mode_max_1280))
            } else {
                None
            };

            let cover_assets = CoverAssets {
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                },
                max_160,
                max_320,
                max_480,
                max_800,
                max_1280
            };

            self.cover_assets.replace(cover_assets);
        }

        self.cover_assets.as_mut().unwrap()
    }

    pub fn deserialize_cached(path: &Path) -> Option<Image> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<Image>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn feed_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent,
        source_path: &Path
    ) -> &mut Asset {
        if let Some(asset) = self.feed_asset.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            info_resizing!("{:?} for usage as a feed image", &source_path);

            let image_in_memory = build.image_processor.open(build, source_path);

            let (filename, _dimensions) = build.image_processor.resize(
                build,
                &image_in_memory,
                ResizeMode::ContainInSquare { max_edge_size: FEED_MAX_EDGE_SIZE }
            );

            self.feed_asset.replace(Asset::new(build, filename, asset_intent));
        }

        self.feed_asset.as_mut().unwrap()
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let manifest_filename = format!("{}.{}.bincode", url_safe_base64(self.hash.value), Image::CACHE_SERIALIZATION_KEY);
        cache_dir.join(manifest_filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        if let Some(asset) = self.artist_assets.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.background_asset.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.cover_assets.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.feed_asset.as_mut() { asset.mark_stale(timestamp); }

        for view in self.views.iter_mut() {
            view.mark_stale(timestamp);
        }
    }

    pub fn new(file_meta: FileMeta, hash: SourceHash) -> Image {
        Image {
            artist_assets: None,
            background_asset: None,
            cover_assets: None,
            feed_asset: None,
            hash,
            views: vec![View::new(file_meta)]
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let manifest_path = self.manifest_path(cache_dir);
        let serialized = bincode::serialize(self).unwrap();
        fs::write(manifest_path, serialized).unwrap();
    }
}

impl ImageRc {
    pub fn add_view(&self, file_meta: &FileMeta) {
        self.image.borrow_mut().views.push(View::new(file_meta.clone()));
    }

    pub fn borrow(&self) -> Ref<'_, Image> {
        self.image.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Image> {
        self.image.borrow_mut()
    }

    pub fn matches_hash(&self, hash: &SourceHash) -> bool {
        self.image.borrow().hash == *hash
    }

    pub fn new(file_meta: FileMeta, hash: SourceHash) -> ImageRc {
        let image = Image::new(file_meta, hash);

        ImageRc {
            image: Rc::new(RefCell::new(image))
        }
    }

    pub fn retrieved(image: Image) -> ImageRc {
        ImageRc {
            image: Rc::new(RefCell::new(image))
        }
    }

    pub fn revive_view(&self, file_meta: &FileMeta) -> bool {
        for view_mut in self.image.borrow_mut().views.iter_mut() {
            if view_mut.file_meta == *file_meta {
                view_mut.unmark_stale();
                return true;
            }
        }

        false
    }
}

impl ImageRcView {
    pub fn borrow(&self) -> Ref<'_, Image> {
        self.image.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Image> {
        self.image.borrow_mut()
    }

    pub fn new(file_meta: FileMeta, image: ImageRc) -> ImageRcView {
        ImageRcView {
            file_meta,
            image
        }
    }
}

impl Hash for ImageRcView {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.image.borrow().hash.hash(state);
    }
}

impl ImgAttributes {
    /// Assets MUST be passed in ascending size
    pub fn new_for_artist(
        assets_ascending_by_size: Vec<&ArtistAsset>,
        hash: &str,
        permalink: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut src = String::new();
        let mut srcset = Vec::new();

        let mut asset_peek_iter = assets_ascending_by_size.iter().peekable();

        while let Some(asset) = asset_peek_iter.next() {
            let format = &asset.format;
            let height = asset.height;
            let width = asset.width;
            srcset.push(format!("{prefix}{permalink}_{format}_{width}x{height}.jpg?{hash} {width}w"));

            if asset_peek_iter.peek().is_none() {
                src = format!("{prefix}{permalink}_{format}_{width}x{height}.jpg?{hash}");
            }
        }

        ImgAttributes {
            src,
            srcset: srcset.join(",")
        }
    }

    /// Assets MUST be passed in ascending size
    pub fn new_for_cover(
        assets_ascending_by_size: Vec<&CoverAsset>,
        hash: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut src = String::new();
        let mut srcset = Vec::new();

        let mut asset_peek_iter = assets_ascending_by_size.iter().peekable();

        while let Some(asset) = asset_peek_iter.next() {
            let edge_size = asset.edge_size;
            srcset.push(format!("{prefix}cover_{edge_size}.jpg?{hash} {edge_size}w"));

            if asset_peek_iter.peek().is_none() {
                src = format!("{prefix}cover_{edge_size}.jpg?{hash}");
            }
        }

        ImgAttributes {
            src,
            srcset: srcset.join(",")
        }
    }
}
