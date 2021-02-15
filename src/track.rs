use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::{
    artist::Artist,
    asset_cache::{Asset, CachedTrackAssets},
    audio_format::AudioFormat,
    ffmpeg::{self, MediaFormat},
    message,
    util
};

const HOUR_SECONDS: u32 = 60 * 60;

#[derive(Debug)]
pub struct Track {
    pub artists: Vec<Rc<Artist>>,
    pub cached_assets: CachedTrackAssets,
    pub duration_seconds: Option<u32>,
    pub lossless_source: bool, 
    pub number: Option<u32>,
    pub source_file: PathBuf,
    pub title: String,
    pub uuid: String
}

impl Track {
    pub fn duration_formatted(&self) -> String {
        match self.duration_seconds {
            Some(seconds) => {
                if seconds > HOUR_SECONDS {
                    format!("{}:{}:{:02}", seconds / HOUR_SECONDS, (seconds % HOUR_SECONDS) / 60, seconds % 60)
                } else {
                    format!("{}:{:02}", (seconds % HOUR_SECONDS) / 60, seconds % 60)
                }
            },
            None => String::new()
        }
    }
    
    pub fn get_as(&self, format: &AudioFormat) -> &Option<Asset> {
        self.cached_assets.get(format)
    }
    
    pub fn get_or_transcode_as(&mut self, format: &AudioFormat, cache_dir: &Path) -> &mut Asset {
        let cached_format = self.cached_assets.get_mut(format);
        
        if cached_format.is_none() {
            let target_filename = format!("{}{}", util::uuid(), format.extension());
            
            message::transcoding(&format!("{:?} to {}", self.source_file, format));
            ffmpeg::transcode(
                &self.source_file,
                &cache_dir.join(&target_filename),
                MediaFormat::Audio(format)
            ).unwrap();
            
            cached_format.replace(Asset::init(cache_dir, target_filename));
        }
        
        cached_format.as_mut().unwrap()
    }
    
    pub fn init(
        artists: Vec<Rc<Artist>>,
        cached_assets: CachedTrackAssets,
        duration_seconds: Option<u32>,
        lossless_source: bool,
        number: Option<u32>,
        source_file: PathBuf,
        title: String,
        uuid: String
    ) -> Track {
        Track {
            artists,
            cached_assets,
            duration_seconds,
            lossless_source,
            number,
            source_file,
            title,
            uuid
        }
    }
}