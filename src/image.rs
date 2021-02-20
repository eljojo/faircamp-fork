use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::{
    fs,
    path::{Path, PathBuf}
};

use crate::{
    asset_cache::{Asset, AssetIntent, CacheManifest, SourceFileSignature},
    build::Build,
    ffmpeg::{self, MediaFormat},
    image_format::ImageFormat,
    util
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedImageAssets {
    pub jpeg: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub uid: String
}

#[derive(Debug)]
pub struct Image {
    pub cached_assets: CachedImageAssets,
    pub source_file: PathBuf
}

impl CachedImageAssets {
    pub fn deserialize(path: &Path) -> Option<CachedImageAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<CachedImageAssets>(&bytes).ok(),
            Err(_) => None
        }
    }
    
    pub fn get(&self, format: &ImageFormat) -> &Option<Asset> {
        match format {
            ImageFormat::Jpeg => &self.jpeg
        }
    }
    
    pub fn get_mut(&mut self, format: &ImageFormat) -> &mut Option<Asset> {
        match format {
            ImageFormat::Jpeg => &mut self.jpeg
        }
    }
    
    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_IMAGES_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        if let Some(asset) = self.jpeg.as_mut() {
            asset.mark_stale(timestamp);
        }
    }
    
    pub fn new(source_file_signature: SourceFileSignature) -> CachedImageAssets {
        CachedImageAssets {
            jpeg: None,
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
    pub fn get_as(&self, format: &ImageFormat) -> &Option<Asset> {
        self.cached_assets.get(format)
    }
    
    pub fn get_or_transcode_as(&mut self, format: &ImageFormat, build: &Build, asset_intent: AssetIntent) -> &mut Asset {
        let cached_format = self.cached_assets.get_mut(format);
        
        match cached_format {
            Some(asset) => if asset_intent == AssetIntent::Deliverable { asset.unmark_stale(); }
            None => {
                let target_filename = format!("{}{}", util::uid(), format.extension());
            
                info_transcoding!("{:?} to {}", self.source_file, format);
                ffmpeg::transcode(
                    &self.source_file,
                    &build.cache_dir.join(&target_filename),
                    MediaFormat::Image(format)
                ).unwrap();
            
                cached_format.replace(Asset::init(build, target_filename, asset_intent));
            }
        }
        
        cached_format.as_mut().unwrap()
    }
    
    pub fn init(cached_assets: CachedImageAssets, source_file: &Path) -> Image {
        Image {
            cached_assets,
            source_file: source_file.to_path_buf()
        }
    }
}