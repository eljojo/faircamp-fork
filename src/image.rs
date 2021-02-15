use std::path::{Path, PathBuf};

use crate::{
    asset_cache::{Asset, CachedImageAssets},
    ffmpeg::{self, MediaFormat},
    image_format::ImageFormat,
    message,
    util
};

#[derive(Debug)]
pub struct Image {
    pub cached_assets: CachedImageAssets,
    pub source_file: PathBuf
}

impl Image {
    pub fn get_as(&self, format: &ImageFormat) -> &Option<Asset> {
        self.cached_assets.get(format)
    }
    
    pub fn get_or_transcode_as(&mut self, format: &ImageFormat, cache_dir: &Path) -> &mut Asset {
        let cached_format = self.cached_assets.get_mut(format);
        
        if cached_format.is_none() {
            let target_filename = format!("{}{}", util::uuid(), format.extension());
        
            message::transcoding(&format!("{:?} to {}", self.source_file, format));
            ffmpeg::transcode(
                &self.source_file,
                &cache_dir.join(&target_filename),
                MediaFormat::Image(format)
            ).unwrap();
        
            cached_format.replace(Asset::init(cache_dir, target_filename));
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