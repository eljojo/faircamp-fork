use std::{
    fs,
    path::{Path, PathBuf}
};

use crate::{
    asset_cache::{Asset, SourceFileSignature},
    ffmpeg::{self, MediaFormat},
    image_format::ImageFormat,
    message,
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
    
    pub fn new(source_file_signature: SourceFileSignature) -> CachedImageAssets {
        CachedImageAssets {
            jpeg: None,
            source_file_signature,
            uid: util::uid()
        }
    }
    
    pub fn persist(&self, cache_dir: &Path) {
        let filename = format!("cached_image_assets_{}.bincode", self.uid); // TODO: Remove verbose prefix after testing (?)
        let serialized = bincode::serialize(self).unwrap();
        fs::write(cache_dir.join(filename), &serialized).unwrap();
    }
}

impl Image {
    pub fn get_as(&self, format: &ImageFormat) -> &Option<Asset> {
        self.cached_assets.get(format)
    }
    
    pub fn get_or_transcode_as(&mut self, format: &ImageFormat, cache_dir: &Path) -> &mut Asset {
        let cached_format = self.cached_assets.get_mut(format);
        
        if cached_format.is_none() {
            let target_filename = format!("{}{}", util::uid(), format.extension());
        
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