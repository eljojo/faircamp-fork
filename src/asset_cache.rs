use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::{
    AudioFormat,
    AudioMeta,
    Build,
    CachedImageAssets,
    CachedReleaseAssets,
    CachedTrackAssets,
    Catalog,
    ImageFormat,
    Track,
    util
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheManifest {
    pub images: Vec<CachedImageAssets>,
    pub releases: Vec<CachedReleaseAssets>,
    pub tracks: Vec<CachedTrackAssets>
}

#[derive(PartialEq)]
pub enum CacheOptimization {
    Default,
    Delayed,
    Immediate,
    Manual,
    Wipe
}

// TODO: PartialEq should be extended to a custom logic probably (first check path + size + modified, alternatively hash, etc.)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub hash: String,
    pub modified: SystemTime,
    pub path: PathBuf,
    pub size: u64
}
    
pub fn optimize_cache(
    build: &Build,
    cache_manifest: &mut CacheManifest,
    catalog: &mut Catalog
) {
    for cached_assets in cache_manifest.images.iter_mut() {
        optimize_image_assets(cached_assets, build);
    }
    
    for cached_assets in cache_manifest.releases.iter_mut() {
        optimize_release_assets(cached_assets, build);
    }
    
    for cached_assets in cache_manifest.tracks.iter_mut() {
        optimize_track_assets(cached_assets, build);
    }
    
    for release in catalog.releases.iter_mut() {
        if let Some(image) = &mut release.cover {
            optimize_image_assets(&mut image.borrow_mut().cached_assets, build);
        }
        
        for track in release.tracks.iter_mut() {
            optimize_track_assets(&mut track.cached_assets, build);
        }
        
        optimize_release_assets(&mut release.cached_assets, build);
    }
}

pub fn optimize_image_assets(cached_assets: &mut CachedImageAssets, build: &Build) {
    let mut keep_container = false;

    for format in ImageFormat::ALL_FORMATS {
        let cached_format = cached_assets.get_mut(&format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached image asset ({}) for {}.",
                    format,
                    cached_assets.source_file_signature.path.display()
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }

    if keep_container {
        cached_assets.persist(&build.cache_dir);
    } else {
        util::remove_file(&cached_assets.manifest_path(&build.cache_dir));
    }
}

pub fn optimize_release_assets(cached_assets: &mut CachedReleaseAssets, build: &Build) {
    let mut keep_container = false;
    
    for format in AudioFormat::ALL_FORMATS {
        let cached_format = cached_assets.get_mut(&format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached release asset ({}) for archive with {} tracks.",
                    format,
                    cached_assets.source_file_signatures.len()  // TODO: Bit awkward here that we can't easily get a pretty identifying string for the release
                                                                //       Possibly indication that Release + CachedReleaseAssets should be merged together (?) (and same story with Image/Track)
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }
    
    if keep_container {
        cached_assets.persist(&build.cache_dir);
    } else {
        util::remove_file(&cached_assets.manifest_path(&build.cache_dir));
    }
}

pub fn optimize_track_assets(cached_assets: &mut CachedTrackAssets, build: &Build) {
    let mut keep_container = false;
    
    for format in AudioFormat::ALL_FORMATS {
        let cached_format = cached_assets.get_mut(&format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached track asset ({}) for {}.",
                    format,
                    cached_assets.source_file_signature.path.display()
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }
    
    if keep_container {
        cached_assets.persist(&build.cache_dir);
    } else {
        util::remove_file(&cached_assets.manifest_path(&build.cache_dir));
    }
}

pub fn report_stale(cache_manifest: &CacheManifest, catalog: &Catalog) {
    let mut num_unused = 0;
    let mut unused_bytesize = 0;
    
    for cached_assets in &cache_manifest.images {
        report_stale_image_assets(cached_assets, &mut num_unused, &mut unused_bytesize);
    }
    
    for cached_assets in &cache_manifest.releases {
        report_stale_release_assets(cached_assets, &mut num_unused, &mut unused_bytesize);
    }
    
    for cached_assets in &cache_manifest.tracks {
        report_stale_track_assets(cached_assets, &mut num_unused, &mut unused_bytesize);
    }
    
    for release in &catalog.releases {
        if let Some(image) = &release.cover {
            report_stale_image_assets(&image.borrow().cached_assets, &mut num_unused, &mut unused_bytesize);
        }
        
        for track in &release.tracks {
            report_stale_track_assets(&track.cached_assets, &mut num_unused, &mut unused_bytesize);
        }
        
        report_stale_release_assets(&release.cached_assets, &mut num_unused, &mut unused_bytesize);
    }
    
    if num_unused > 0 {
        info_cache!(
            "{} cached assets were identified as obsolete - you can run 'faircamp --optimize-cache' to to remove them and reclaim {} of disk space.",
            num_unused,
            util::format_bytes(unused_bytesize)
        );
    } else {
        info_cache!("No cached assets identied as obsolete.");
    }
}

pub fn report_stale_image_assets(cached_assets: &CachedImageAssets, num_unused: &mut u32, unused_bytesize: &mut u64) {
    for format in ImageFormat::ALL_FORMATS {
        if let Some(filesize_bytes) = cached_assets
            .get(format)
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    }
}

pub fn report_stale_release_assets(cached_assets: &CachedReleaseAssets, num_unused: &mut u32, unused_bytesize: &mut u64) {
    for format in AudioFormat::ALL_FORMATS {
        if let Some(filesize_bytes) = cached_assets
            .get(format)
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    }
}

pub fn report_stale_track_assets(cached_assets: &CachedTrackAssets, num_unused: &mut u32, unused_bytesize: &mut u64) {
    for format in AudioFormat::ALL_FORMATS {
        if let Some(filesize_bytes) = cached_assets
            .get(format)
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    }
}

impl CacheManifest {
    pub const MANIFEST_IMAGES_DIR: &'static str = "manifest/images";
    pub const MANIFEST_RELEASES_DIR: &'static str = "manifest/releases";
    pub const MANIFEST_TRACKS_DIR: &'static str = "manifest/tracks";
    
    pub fn ensure_dirs(cache_dir: &Path) {
        util::ensure_dir(&cache_dir.join(CacheManifest::MANIFEST_IMAGES_DIR));
        util::ensure_dir(&cache_dir.join(CacheManifest::MANIFEST_RELEASES_DIR));
        util::ensure_dir(&cache_dir.join(CacheManifest::MANIFEST_TRACKS_DIR));
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for cached_assets in self.images.iter_mut() {
            cached_assets.mark_all_stale(timestamp);
        }
        
        for cached_assets in self.releases.iter_mut() {
            cached_assets.mark_all_stale(timestamp);
        }
        
        for cached_assets in self.tracks.iter_mut() {
            cached_assets.mark_all_stale(timestamp);
        }
    }
        
    pub fn retrieve(cache_dir: &Path) -> CacheManifest {
        CacheManifest {
            images: CacheManifest::retrieve_images(cache_dir),
            releases: CacheManifest::retrieve_releases(cache_dir),
            tracks: CacheManifest::retrieve_tracks(cache_dir)
        }
    }
    
    // TODO: Should probably not quietly ignore everything that can go wrong here (here and elsewhere)
    // TODO: Also very boilerplatey (up your generics game here?)
    pub fn retrieve_images(cache_dir: &Path) -> Vec<CachedImageAssets> {      
        let mut images = Vec::new();
          
        if let Ok(dir_entries) = cache_dir.join(CacheManifest::MANIFEST_IMAGES_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(cached_assets) = CachedImageAssets::deserialize(&dir_entry.path()) {
                        images.push(cached_assets);
                    }
                }
            }
        }
        
        images
    }
    
    pub fn retrieve_releases(cache_dir: &Path) -> Vec<CachedReleaseAssets> {
        let mut releases = Vec::new();
             
        if let Ok(dir_entries) = cache_dir.join(CacheManifest::MANIFEST_RELEASES_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(cached_assets) = CachedReleaseAssets::deserialize(&dir_entry.path()) {
                        releases.push(cached_assets);
                    }
                }
            }
        }
        
        releases
    }
    
    pub fn retrieve_tracks(cache_dir: &Path) -> Vec<CachedTrackAssets> {  
        let mut tracks = Vec::new();
           
        if let Ok(dir_entries) = cache_dir.join(CacheManifest::MANIFEST_TRACKS_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(cached_assets) = CachedTrackAssets::deserialize(&dir_entry.path()) {
                        tracks.push(cached_assets);
                    }
                }
            }
        }
        
        tracks
    }
    
    pub fn take_or_create_image_assets(&mut self, source_path: &Path) -> CachedImageAssets {
        let source_file_signature = SourceFileSignature::init(source_path);
        
        match self.images.iter().position(|cached_assets| cached_assets.source_file_signature == source_file_signature) {
            Some(index) => self.images.swap_remove(index),
            None => CachedImageAssets::new(source_file_signature)
        }
    }
    
    pub fn take_or_create_release_assets(&mut self, tracks: &Vec<Track>) -> CachedReleaseAssets {
        match self.releases
            .iter()
            .position(|cached_assets| {
                tracks
                    .iter()
                    .zip(cached_assets.source_file_signatures.iter())
                    .all(|(track, source_file_signature)| {
                        &track.cached_assets.source_file_signature == source_file_signature
                    })
            }) {
            Some(index) => self.releases.swap_remove(index),
            None => {
                CachedReleaseAssets::new(
                    tracks
                        .iter()
                        .map(|track| track.cached_assets.source_file_signature.clone())
                        .collect()
                )
            }
        }
    }
    
    pub fn take_or_create_track_assets(&mut self, source_path: &Path, extension: &str) -> CachedTrackAssets {
        let source_file_signature = SourceFileSignature::init(source_path);
        
        match self.tracks.iter().position(|cached_assets| cached_assets.source_file_signature == source_file_signature) {
            Some(index) => self.tracks.swap_remove(index),
            None => {
                let source_meta = AudioMeta::extract(source_path, extension);
                CachedTrackAssets::new(source_file_signature, source_meta)
            }
        }
    }
}

impl CacheOptimization {
    pub fn from_manifest_key(key: &str) -> Option<CacheOptimization> {        
        match key {
            "delayed" => Some(CacheOptimization::Delayed),
            "immediate" => Some(CacheOptimization::Immediate),
            "manual" => Some(CacheOptimization::Manual),
            "wipe" => Some(CacheOptimization::Wipe),
            _ => None
        }
    }
}

impl std::fmt::Display for CacheOptimization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            CacheOptimization::Default => "Default",
            CacheOptimization::Delayed => "Delayed",
            CacheOptimization::Immediate => "Immediate",
            CacheOptimization::Manual => "Manual",
            CacheOptimization::Wipe => "Wipe"
        };
        
        write!(f, "{}", text)
    }
}

impl SourceFileSignature {
    pub fn init(file: &Path) -> SourceFileSignature {
        let metadata = fs::metadata(file).expect("Could not access source file");
        
        SourceFileSignature {
            hash: String::new(),
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            path: file.to_path_buf(),
            size: metadata.len()
        }
    }
}