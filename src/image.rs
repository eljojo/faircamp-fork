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
    CacheManifest,
    CacheOptimization,
    SourceFileSignature,
    util
};

const ARTIST_EDGE_SIZE: i32 = 420;
const BACKGROUND_MAX_EDGE_SIZE: i32 = 1280;
const COVER_EDGE_SIZE: i32 = 360;
const DOWNLOAD_COVER_EDGE_SIZE: i32 = 1080;
const FEED_MAX_EDGE_SIZE: i32 = 920;

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
    pub artist: Option<Asset>,
    pub background: Option<Asset>,
    pub cover: Option<CoverImageVersions>,
    pub download_cover: Option<Asset>,
    pub feed: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub uid: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageVersion {
    pub filename: String,
    pub filesize_bytes: u64,
    pub height: i32,
    pub width: i32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverImageVersions {
    pub marked_stale: Option<DateTime<Utc>>,
    pub versions: Vec<ImageVersion>
}

enum ResizeMode {
    CropSquare(i32),
    CropWithin(i32)
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
        ResizeMode::CropSquare(edge_size) => {
            info_resizing!("{:?} to square cropped <= {}px", path, edge_size);

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
        ResizeMode::CropWithin(longer_edge_size) => {
            info_resizing!("{:?} to cropped <= {}px", path, longer_edge_size);

            let longer_edge = std::cmp::max(height, width);

            if longer_edge > longer_edge_size {
                ops::resize(&image, longer_edge_size as f64 / longer_edge as f64).unwrap()
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

impl CoverImageVersions {
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
    ) -> &mut Asset {
        if let Some(asset) = self.artist.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let (filename, _dimensions) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CropSquare(ARTIST_EDGE_SIZE)
            );

            self.artist.replace(Asset::new(build, filename, asset_intent));
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
                ResizeMode::CropWithin(BACKGROUND_MAX_EDGE_SIZE)
            );

            self.background.replace(Asset::new(build, filename, asset_intent));
        }

        self.background.as_mut().unwrap()
    }

    pub fn cover_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut CoverImageVersions {
        if let Some(asset) = self.cover.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let mut versions = Vec::new();

            let (filename_hires, dimensions_hires) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CropSquare(COVER_EDGE_SIZE)
            );

            let metadata_hires = fs::metadata(&build.cache_dir.join(&filename_hires)).unwrap();

            versions.push(ImageVersion {
                filename: filename_hires,
                filesize_bytes: metadata_hires.len(),
                height: dimensions_hires.1,
                width: dimensions_hires.0
            });

            if dimensions_hires.0 as f32 > (COVER_EDGE_SIZE as f32 * 0.75) {
                let (filename_lores, dimensions_lores) = resize(
                    build,
                    &self.source_file_signature.path,
                    ResizeMode::CropSquare(COVER_EDGE_SIZE / 2)
                );

                let metadata_lores = fs::metadata(&build.cache_dir.join(&filename_lores)).unwrap();

                versions.push(ImageVersion {
                    filename: filename_lores,
                    filesize_bytes: metadata_lores.len(),
                    height: dimensions_lores.1,
                    width: dimensions_lores.0
                });
            }

            let cover_image_versions = CoverImageVersions {
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                },
                versions
            };

            self.cover.replace(cover_image_versions);
        }

        self.cover.as_mut().unwrap()
    }

    pub fn deserialize_cached(path: &Path) -> Option<ImageAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<ImageAssets>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn download_cover_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut Asset {
        if let Some(asset) = self.download_cover.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let (filename, _dimensions) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CropSquare(DOWNLOAD_COVER_EDGE_SIZE)
            );

            self.download_cover.replace(Asset::new(build, filename, asset_intent));
        }

        self.download_cover.as_mut().unwrap()
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
                ResizeMode::CropWithin(FEED_MAX_EDGE_SIZE)
            );

            self.feed.replace(Asset::new(build, filename, asset_intent));
        }

        self.feed.as_mut().unwrap()
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_IMAGES_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        self.artist.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.background.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.cover.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.download_cover.as_mut().map(|asset| asset.mark_stale(timestamp));
        self.feed.as_mut().map(|asset| asset.mark_stale(timestamp));
    }
    
    pub fn new(source_file_signature: SourceFileSignature) -> ImageAssets {
        ImageAssets {
            artist: None,
            background: None,
            cover: None,
            download_cover: None,
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