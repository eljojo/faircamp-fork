use chrono::{DateTime, Duration, Utc};
use libvips::{ops, VipsImage};
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
    SourceFileSignature,
    util
};

const BACKGROUND_MAX_EDGE_SIZE: i32 = 1280;
const FEED_MAX_EDGE_SIZE: i32 = 920;

/// A single, resized version of the artist image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAsset {
    pub filename: String,
    pub filesize_bytes: u64,
    pub height: i32,
    pub width: i32
}

/// Represents multiple, differently sized versions of an artist image, for
/// display on different screen sizes. (Numbers refer to maximum width)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAssets {
    pub marked_stale: Option<DateTime<Utc>>,
    pub max_360: ArtistAsset,
    pub max_560: Option<ArtistAsset>,
    pub max_800: Option<ArtistAsset>,
    pub max_1120: Option<ArtistAsset>
}

/// A single, resized version of the cover image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverAsset {
    /// Represents both height and width (covers have a square aspect ratio)
    pub edge_size: i32,
    pub filename: String,
    pub filesize_bytes: u64
}

/// Represents multiple, differently sized versions of a cover image, for
/// display on different screen sizes and for inclusion in the release
/// archive. (Numbers refer to the square edge size, both height and width)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverAssets {
    pub marked_stale: Option<DateTime<Utc>>,
    pub max_180: CoverAsset,
    pub max_360: Option<CoverAsset>,
    pub max_720: Option<CoverAsset>,
    pub max_1080: Option<CoverAsset>
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

enum ResizeMode {
    /// Resize such that the longer edge of the image does not exceed the maximum edge size.
    ContainInSquare { max_edge_size: i32 },
    /// Perform a square crop, then resize to a maximum edge size.
    CoverSquare { edge_size: i32 },
}

fn resize(
    build: &Build,
    path: &PathBuf,
    resize_mode: ResizeMode
) -> (String, (i32, i32)) {
    let image = VipsImage::new_from_file(&build.catalog_dir.join(path).to_string_lossy()).unwrap();

    let height = image.get_height();
    let width = image.get_width();

    let transformed = match resize_mode {
        ResizeMode::CoverSquare { edge_size } => {
            info_resizing!("{:?} to cover a square <= {}px", path, edge_size);

            let smaller_edge = std::cmp::min(height, width);

            let cropped_square = if height != width {
                ops::smartcrop(&image, smaller_edge, smaller_edge).unwrap()
            } else {
                image
            };

            if smaller_edge <= edge_size {
                cropped_square
            } else {
                ops::resize(&cropped_square, edge_size as f64 / smaller_edge as f64).unwrap()
            }
        }
        ResizeMode::ContainInSquare { max_edge_size } => {
            info_resizing!("{:?} to contain within a square <= {}px", path, max_edge_size);

            let longer_edge = std::cmp::max(height, width);

            if longer_edge > max_edge_size {
                ops::resize(&image, max_edge_size as f64 / longer_edge as f64).unwrap()
            } else {
                image
            }
        }
    };

    let options = ops::JpegsaveOptions {
        interlace: true,
        optimize_coding: true,
        q: 80,
        strip: true,
        ..ops::JpegsaveOptions::default()
    };

    let target_filename = format!("{}.jpg", util::uid());

    let result_dimensions = (transformed.get_width(), transformed.get_height());

    match ops::jpegsave_with_opts(
        &transformed,
        &build.cache_dir.join(&target_filename).to_string_lossy(),
        &options
    ) {
        Ok(_) => (),
        Err(_) => println!("error: {}", build.libvips_app.error_buffer().unwrap())
    }

    (target_filename, result_dimensions)
}

impl ArtistAssets {
    pub fn all(&self) -> Vec<&ArtistAsset> {
        let mut result = Vec::with_capacity(4);

        result.push(&self.max_360);
        if let Some(asset) = &self.max_560 { result.push(asset); }
        if let Some(asset) = &self.max_800 { result.push(asset); }
        if let Some(asset) = &self.max_1120 { result.push(asset); }

        result
    }

    pub fn img_attributes_up_to_1120(&self, permalink: &str, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.max_360);
        if let Some(asset) = &self.max_560 { assets.push(asset); }
        if let Some(asset) = &self.max_800 { assets.push(asset); }
        if let Some(asset) = &self.max_1120 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, permalink, prefix)
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

impl CoverAssets {
    pub fn all(&self) -> Vec<&CoverAsset> {
        let mut result = Vec::with_capacity(4);

        result.push(&self.max_180);
        if let Some(asset) = &self.max_360 { result.push(asset); }
        if let Some(asset) = &self.max_720 { result.push(asset); }
        if let Some(asset) = &self.max_1080 { result.push(asset); }

        result
    }

    pub fn img_attributes_up_to_360(&self, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_360 {
            Some(max_360) => vec![&self.max_180, max_360],
            None => vec![&self.max_180]
        };

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn img_attributes_up_to_1080(&self, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.max_180);
        if let Some(asset) = &self.max_360 { assets.push(asset); }
        if let Some(asset) = &self.max_720 { assets.push(asset); }
        if let Some(asset) = &self.max_1080 { assets.push(asset); }

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn largest(&self) -> &CoverAsset {
        if let Some(max_1080) = &self.max_1080 {
            max_1080
        } else if let Some(max_720) = &self.max_720 {
            max_720
        } else if let Some(max_360) = &self.max_360 {
            max_360
        } else {
            &self.max_180
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
            let (filename_360, dimensions_360) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::ContainInSquare { max_edge_size: 360 }
            );

            let metadata_360 = fs::metadata(&build.cache_dir.join(&filename_360)).unwrap();

            let max_360 = ArtistAsset {
                filename: filename_360,
                filesize_bytes: metadata_360.len(),
                height: dimensions_360.1,
                width: dimensions_360.0
            };

            let mut max_560 = None;
            let mut max_800 = None;
            let mut max_1120 = None;

            if dimensions_360.0 == 360 {
                let (filename_560, dimensions_560) = resize(
                    build,
                    &self.source_file_signature.path,
                    ResizeMode::ContainInSquare { max_edge_size: 560 }
                );

                let metadata_560 = fs::metadata(&build.cache_dir.join(&filename_560)).unwrap();

                max_560 = Some(ArtistAsset {
                    filename: filename_560,
                    filesize_bytes: metadata_560.len(),
                    height: dimensions_560.1,
                    width: dimensions_560.0
                });

                if dimensions_560.0 == 560 {
                    let (filename_800, dimensions_800) = resize(
                        build,
                        &self.source_file_signature.path,
                        ResizeMode::ContainInSquare { max_edge_size: 800 }
                    );

                    let metadata_800 = fs::metadata(&build.cache_dir.join(&filename_800)).unwrap();

                    max_800 = Some(ArtistAsset {
                        filename: filename_800,
                        filesize_bytes: metadata_800.len(),
                        height: dimensions_800.1,
                        width: dimensions_800.0
                    });

                    if dimensions_800.0 == 800 {
                        let (filename_1120, dimensions_1120) = resize(
                            build,
                            &self.source_file_signature.path,
                            ResizeMode::ContainInSquare { max_edge_size: 1120 }
                        );

                        let metadata_1120 = fs::metadata(&build.cache_dir.join(&filename_1120)).unwrap();

                        max_1120 = Some(ArtistAsset {
                            filename: filename_1120,
                            filesize_bytes: metadata_1120.len(),
                            height: dimensions_1120.1,
                            width: dimensions_1120.0
                        });
                    }
                }
            }

            let artist_assets = ArtistAssets {
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                },
                max_360,
                max_560,
                max_800,
                max_1120
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
            let (filename, _dimensions) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::ContainInSquare { max_edge_size: BACKGROUND_MAX_EDGE_SIZE }
            );

            self.background.replace(Asset::new(build, filename, asset_intent));
        }

        self.background.as_mut().unwrap()
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
            let (filename_180, dimensions_180) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CoverSquare { edge_size: 180 }
            );

            let metadata_180 = fs::metadata(&build.cache_dir.join(&filename_180)).unwrap();

            let max_180 = CoverAsset {
                edge_size: dimensions_180.0,
                filename: filename_180,
                filesize_bytes: metadata_180.len()
            };

            let mut max_360 = None;
            let mut max_720 = None;
            let mut max_1080 = None;

            if dimensions_180.0 == 180 {
                let (filename_360, dimensions_360) = resize(
                    build,
                    &self.source_file_signature.path,
                    ResizeMode::CoverSquare { edge_size: 360 }
                );

                let metadata_360 = fs::metadata(&build.cache_dir.join(&filename_360)).unwrap();

                max_360 = Some(CoverAsset {
                    edge_size: dimensions_360.0,
                    filename: filename_360,
                    filesize_bytes: metadata_360.len()
                });

                if dimensions_360.0 == 360 {
                    let (filename_720, dimensions_720) = resize(
                        build,
                        &self.source_file_signature.path,
                        ResizeMode::CoverSquare { edge_size: 720 }
                    );

                    let metadata_720 = fs::metadata(&build.cache_dir.join(&filename_720)).unwrap();

                    max_720 = Some(CoverAsset {
                        edge_size: dimensions_720.0,
                        filename: filename_720,
                        filesize_bytes: metadata_720.len()
                    });

                    if dimensions_720.0 == 720 {
                        let (filename_1080, dimensions_1080) = resize(
                            build,
                            &self.source_file_signature.path,
                            ResizeMode::CoverSquare { edge_size: 1080 }
                        );

                        let metadata_1080 = fs::metadata(&build.cache_dir.join(&filename_1080)).unwrap();

                        max_1080 = Some(CoverAsset {
                            edge_size: dimensions_1080.0,
                            filename: filename_1080,
                            filesize_bytes: metadata_1080.len()
                        });
                    }
                }
            }

            let cover_assets = CoverAssets {
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                },
                max_180,
                max_360,
                max_720,
                max_1080
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
            let (filename, _dimensions) = resize(
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
        cache_dir.join(Cache::MANIFEST_IMAGES_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        self.artist.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.background.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.cover.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.feed.as_mut().map(|asset| asset.mark_stale(timestamp));
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
        fs::write(self.manifest_path(cache_dir), &serialized).unwrap();
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
            srcset.push(format!("{}{}_{}x{}.jpg {}w", prefix, permalink, asset.width, asset.height, asset.width));

            if asset_peek_iter.peek().is_none() {
                src = format!("{}{}_{}x{}.jpg", prefix, permalink, asset.width, asset.height);
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