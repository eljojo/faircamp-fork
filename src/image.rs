use chrono::{DateTime, Utc};
use libvips::{ops, VipsImage};
use serde_derive::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    Asset,
    AssetIntent,
    Build,
    CacheManifest,
    ImageFormat,
    SourceFileSignature,
    util
};

const ARTIST_EDGE_SIZE: i32 = 420;
const BACKGROUND_MAX_EDGE_SIZE: i32 = 1280;
const COVER_EDGE_SIZE: i32 = 360;
const FEED_MAX_EDGE_SIZE: i32 = 920;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedImageAssets {
    pub artist: Option<Asset>,
    pub background: Option<Asset>,
    pub cover: Option<Asset>,
    pub feed: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub uid: String
}

#[derive(Debug)]
pub struct Image {
    pub cached_assets: CachedImageAssets,
    pub description: Option<String>,
    pub source_file: PathBuf
}

impl CachedImageAssets {
    pub fn deserialize(path: &Path) -> Option<CachedImageAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<CachedImageAssets>(&bytes).ok(),
            Err(_) => None
        }
    }
    
    pub fn get(&self, format: ImageFormat) -> &Option<Asset> {
        match format {
            ImageFormat::Artist => &self.artist,
            ImageFormat::Background => &self.background,
            ImageFormat::Cover => &self.cover,
            ImageFormat::Feed => &self.feed
        }
    }
    
    pub fn get_mut(&mut self, format: ImageFormat) -> &mut Option<Asset> {
        match format {
            ImageFormat::Artist => &mut self.artist,
            ImageFormat::Background => &mut self.background,
            ImageFormat::Cover => &mut self.cover,
            ImageFormat::Feed => &mut self.feed
        }
    }
    
    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_IMAGES_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for format in ImageFormat::ALL_FORMATS {
            if let Some(asset) = self.get_mut(format) {
                asset.mark_stale(timestamp);
            }
        }
    }
    
    pub fn new(source_file_signature: SourceFileSignature) -> CachedImageAssets {
        CachedImageAssets {
            artist: None,
            background: None,
            cover: None,
            feed: None,
            source_file_signature,
            uid: util::uid()
        }
    }
    
    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), &serialized).unwrap();
    }
}

impl Image {
    pub fn get_as(&self, format: ImageFormat) -> &Option<Asset> {
        self.cached_assets.get(format)
    }
    
    pub fn get_or_transcode_as(&mut self, format: ImageFormat, build: &Build, asset_intent: AssetIntent) -> &mut Asset {
        let cached_format = self.cached_assets.get_mut(format);
        
        match cached_format {
            Some(asset) => if asset_intent == AssetIntent::Deliverable { asset.unmark_stale(); }
            None => {
                let target_filename = format!("{}{}", util::uid(), format.extension());
            
                info_resizing!("{:?} to {}", self.source_file, format);

                let image = VipsImage::new_from_file(&self.source_file.to_string_lossy()).unwrap();

                let height = image.get_height();
                let width = image.get_width();
                let smaller_edge = std::cmp::min(height, width);

                let transformed = match format {
                    ImageFormat::Artist => {
                        let cropped = if height != width {
                            let smaller_edge = std::cmp::min(height, width);
                            ops::smartcrop(&image, smaller_edge, smaller_edge).unwrap()
                        } else {
                            image
                        };

                        if smaller_edge <= ARTIST_EDGE_SIZE {
                            cropped
                        } else {
                            ops::resize(&cropped, ARTIST_EDGE_SIZE as f64 / smaller_edge as f64).unwrap()
                        }
                    }
                    ImageFormat::Background => {
                        let longer_edge = std::cmp::max(height, width);
                        if longer_edge > BACKGROUND_MAX_EDGE_SIZE {
                            ops::resize(&image, BACKGROUND_MAX_EDGE_SIZE as f64 / longer_edge as f64).unwrap()
                        } else {
                            image
                        }
                    }
                    ImageFormat::Cover => {
                        let cropped = if height != width {
                            let smaller_edge = std::cmp::min(height, width);
                            ops::smartcrop(&image, smaller_edge, smaller_edge).unwrap()
                        } else {
                            image
                        };

                        if smaller_edge <= COVER_EDGE_SIZE {
                            cropped
                        } else {
                            ops::resize(&cropped, COVER_EDGE_SIZE as f64 / smaller_edge as f64).unwrap()
                        }
                    }
                    ImageFormat::Feed => {
                        let longer_edge = std::cmp::max(height, width);
                        if longer_edge > FEED_MAX_EDGE_SIZE {
                            ops::resize(&image, FEED_MAX_EDGE_SIZE as f64 / longer_edge as f64).unwrap()
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

                match ops::jpegsave_with_opts(&transformed, &build.cache_dir.join(&target_filename).to_string_lossy(),  &options) {
                    Ok(_) => (),
                    Err(_) => println!("error: {}", build.libvips_app.error_buffer().unwrap())
                }
            
                cached_format.replace(Asset::new(build, target_filename, asset_intent));
            }
        }
        
        cached_format.as_mut().unwrap()
    }
    
    pub fn new(cached_assets: CachedImageAssets, description: Option<String>, source_file: &Path) -> Image {
        Image {
            cached_assets,
            description,
            source_file: source_file.to_path_buf()
        }
    }
}