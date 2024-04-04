use chrono::{DateTime, Duration, Utc};
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc
};

use crate::{
    Asset,
    AssetIntent,
    Build,
    Cache,
    CacheOptimization,
    ResizeMode,
    SourceFileSignature,
    util
};

const BACKGROUND_MAX_EDGE_SIZE: u32 = 1280;
const FEED_MAX_EDGE_SIZE: u32 = 920;

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

/// Associates image assets with an image description
#[derive(Debug)]
pub struct Image {
    pub assets: Rc<RefCell<ImageAssets>>,
    pub description: Option<String>
}

/// Represents a source image file in the catalog and all its generated
/// (compressed/resized) derived versions.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageAssets {
    pub artist: Option<ArtistAssets>,
    pub background: Option<Asset>,
    pub cover: Option<CoverAssets>,
    pub feed: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub uid: String
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

    pub fn img_attributes_fixed(&self, permalink: &str, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fixed_max_320);
        if let Some(asset) = &self.fixed_max_480 { assets.push(asset); }
        if let Some(asset) = &self.fixed_max_640 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, permalink, prefix)
    }

    pub fn img_attributes_fluid(&self, permalink: &str, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fluid_max_640);
        if let Some(asset) = &self.fluid_max_960 { assets.push(asset); }
        if let Some(asset) = &self.fluid_max_1280 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, permalink, prefix)
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    pub fn obsolete(&self, build: &Build) -> bool {
        match &self.marked_stale {
            Some(marked_stale) => {
                match &build.cache_optimization {
                    CacheOptimization::Default | 
                    CacheOptimization::Delayed => 
                        build.build_begin.signed_duration_since(*marked_stale) > Duration::hours(24),
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

    pub fn img_attributes_up_to_320(&self, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_320 {
            Some(max_320) => vec![&self.max_160, max_320],
            None => vec![&self.max_160]
        };

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn img_attributes_up_to_480(&self, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_320 {
            Some(max_320) => match &self.max_480 {
                Some(max_480) => vec![&self.max_160, max_320, max_480],
                None => vec![&self.max_160, max_320]
            }
            None => vec![&self.max_160]
        };

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn img_attributes_up_to_1280(&self, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.max_160);
        if let Some(asset) = &self.max_320 { assets.push(asset); }
        if let Some(asset) = &self.max_480 { assets.push(asset); }
        if let Some(asset) = &self.max_800 { assets.push(asset); }
        if let Some(asset) = &self.max_1280 { assets.push(asset); }

        ImgAttributes::new_for_cover(assets, prefix)
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

    pub fn obsolete(&self, build: &Build) -> bool {
        match &self.marked_stale {
            Some(marked_stale) => {
                match &build.cache_optimization {
                    CacheOptimization::Default | 
                    CacheOptimization::Delayed => 
                        build.build_begin.signed_duration_since(*marked_stale) > Duration::hours(24),
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

impl Image {
    pub fn new(assets: Rc<RefCell<ImageAssets>>, description: Option<String>) -> Image {
        Image {
            assets,
            description
        }
    }
}

impl ImageAssets {
    pub fn artist_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut ArtistAssets {
        if let Some(assets) = self.artist.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                assets.unmark_stale();
            }
        } else {
            // Compute fixed sizes.
            // Viewport width < 30rem (480px at 16px font-size) = 100vw/40vw = 2.5
            // Viewport width > 60rem (960px at 16px font-size) = 27rem/12rem = 2.25
            // We therefore approximate it for both by limiting the aspect to 2.25.-2.5

            let resize_mode_fixed_320 = ResizeMode::CoverRectangle {
                max_aspect: 2.5,
                max_width: 320,
                min_aspect: 2.25
            };
            let fixed_max_320 = self.compute_artist_asset(build, "fixed", resize_mode_fixed_320);

            let fixed_max_480 = if fixed_max_320.width == 320 {
                let resize_mode_fixed_480 = ResizeMode::CoverRectangle {
                    max_aspect: 2.5,
                    max_width: 480,
                    min_aspect: 2.25
                };
                Some(self.compute_artist_asset(build, "fixed", resize_mode_fixed_480))
            } else {
                None
            };

            let fixed_max_640 = if fixed_max_480.as_ref().is_some_and(|asset| asset.width == 480) {
                let resize_mode_fixed_640 = ResizeMode::CoverRectangle {
                    max_aspect: 2.5,
                    max_width: 640,
                    min_aspect: 2.25
                };
                Some(self.compute_artist_asset(build, "fixed", resize_mode_fixed_640))
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
            let fluid_max_640 = self.compute_artist_asset(build, "fluid", resize_mode_fluid_640);

            let fluid_max_960 = if fluid_max_640.width == 640 {
                let resize_mode_fluid_960 = ResizeMode::CoverRectangle {
                    max_aspect: 5.0,
                    max_width: 960,
                    min_aspect: 2.5
                };
                Some(self.compute_artist_asset(build, "fluid", resize_mode_fluid_960))
            } else {
                None
            };

            let fluid_max_1280 = if fluid_max_960.as_ref().is_some_and(|asset| asset.width == 960) {
                let resize_mode_fluid_1280 = ResizeMode::CoverRectangle {
                    max_aspect: 5.0,
                    max_width: 1280,
                    min_aspect: 2.5
                };
                Some(self.compute_artist_asset(build, "fluid", resize_mode_fluid_1280))
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

            self.artist.replace(artist_assets);
        }

        self.artist.as_mut().unwrap()
    }

    pub fn background_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut Asset {
        if let Some(asset) = self.background.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let (filename, _dimensions) = build.image_processor.resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::ContainInSquare { max_edge_size: BACKGROUND_MAX_EDGE_SIZE }
            );

            self.background.replace(Asset::new(build, filename, asset_intent));
        }

        self.background.as_mut().unwrap()
    }

    fn compute_artist_asset(
        &self,
        build: &Build,
        format: &str,
        resize_mode: ResizeMode
    ) -> ArtistAsset {
        let (filename, dimensions) = build.image_processor.resize(
            build,
            &self.source_file_signature.path,
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

    fn compute_cover_asset(&self, build: &Build, resize_mode: ResizeMode) -> CoverAsset {
        let (filename, dimensions) = build.image_processor.resize(
            build,
            &self.source_file_signature.path,
            resize_mode
        );

        let metadata = fs::metadata(build.cache_dir.join(&filename)).unwrap();

        CoverAsset {
            edge_size: dimensions.0,
            filename,
            filesize_bytes: metadata.len()
        }
    }

    pub fn cover_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut CoverAssets {
        if let Some(assets) = self.cover.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                assets.unmark_stale();
            }
        } else {
            let resize_mode_max_160 = ResizeMode::CoverSquare { edge_size: 160 };
            let max_160 = self.compute_cover_asset(build, resize_mode_max_160);

            let max_320 = if max_160.edge_size == 160 {
                let resize_mode_max_320 = ResizeMode::CoverSquare { edge_size: 320 };
                Some(self.compute_cover_asset(build, resize_mode_max_320))
            } else {
                None
            };

            let max_480 = if max_320.as_ref().is_some_and(|asset| asset.edge_size == 320) {
                let resize_mode_max_480 = ResizeMode::CoverSquare { edge_size: 480 };
                Some(self.compute_cover_asset(build, resize_mode_max_480))
            } else {
                None
            };

            let max_800 = if max_480.as_ref().is_some_and(|asset| asset.edge_size == 480) {
                let resize_mode_max_800 = ResizeMode::CoverSquare { edge_size: 800 };
                Some(self.compute_cover_asset(build, resize_mode_max_800))
            } else {
                None
            };

            let max_1280 = if max_800.as_ref().is_some_and(|asset| asset.edge_size == 800) {
                let resize_mode_max_1280 = ResizeMode::CoverSquare { edge_size: 1280 };
                Some(self.compute_cover_asset(build, resize_mode_max_1280))
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

            self.cover.replace(cover_assets);
        }

        self.cover.as_mut().unwrap()
    }

    pub fn deserialize_cached(path: &Path) -> Option<ImageAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<ImageAssets>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn feed_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut Asset {
        if let Some(asset) = self.feed.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let (filename, _dimensions) = build.image_processor.resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::ContainInSquare { max_edge_size: FEED_MAX_EDGE_SIZE }
            );

            self.feed.replace(Asset::new(build, filename, asset_intent));
        }

        self.feed.as_mut().unwrap()
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(Cache::IMAGE_MANIFESTS_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        if let Some(asset) = self.artist.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.background.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.cover.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.feed.as_mut() { asset.mark_stale(timestamp); }
    }
    
    pub fn new(source_file_signature: SourceFileSignature) -> ImageAssets {
        ImageAssets {
            artist: None,
            background: None,
            cover: None,
            feed: None,
            source_file_signature,
            uid: util::uid()
        }
    }
    
    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}

impl ImgAttributes {
    /// Assets MUST be passed in ascending size
    pub fn new_for_artist(
        assets_ascending_by_size: Vec<&ArtistAsset>,
        permalink: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut src = String::new();
        let mut srcset = Vec::new();

        let mut asset_peek_iter = assets_ascending_by_size.iter().peekable();

        while let Some(asset) = asset_peek_iter.next() {
            srcset.push(format!("{}{}_{}_{}x{}.jpg {}w", prefix, permalink, asset.format, asset.width, asset.height, asset.width));

            if asset_peek_iter.peek().is_none() {
                src = format!("{}{}_{}_{}x{}.jpg", prefix, permalink, asset.format, asset.width, asset.height);
            }
        }

        ImgAttributes {
            src,
            srcset: srcset.join(",")
        }
    }

    /// Assets MUST be passed in ascending size
    pub fn new_for_cover(assets_ascending_by_size: Vec<&CoverAsset>, prefix: &str) -> ImgAttributes {
        let mut src = String::new();
        let mut srcset = Vec::new();

        let mut asset_peek_iter = assets_ascending_by_size.iter().peekable();

        while let Some(asset) = asset_peek_iter.next() {
            srcset.push(format!("{}cover_{}.jpg {}w", prefix, asset.edge_size, asset.edge_size));

            if asset_peek_iter.peek().is_none() {
                src = format!("{prefix}cover_{edge_size}.jpg", edge_size = asset.edge_size);
            }
        }

        ImgAttributes {
            src,
            srcset: srcset.join(",")
        }
    }
}