use bincode;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    time::SystemTime
};

use crate::message;

const CACHE_MANIFEST_FILENAME: &str = "manifest.bincode";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub filename: String,
    pub filesize_bytes: u64, 
    pub used: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheManifest {
    pub images: Vec<CachedImageAssets>,
    pub tracks: Vec<CachedTrackAssets>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedImageAssets {
    pub jpg: Option<Asset>,
    pub source_file_signature: SourceFileSignature
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedTrackAssets {
    pub aac: Option<String>,
    pub flac: Option<String>,
    pub mp3_128: Option<String>,
    pub mp3_320: Option<String>,
    pub mp3_v0: Option<String>,
    pub ogg_vorbis: Option<String>,
    pub source_file_signature: SourceFileSignature,
    pub used: bool  // TODO: Track at a more granular level (e.g. identify that FLAC has been used, but AAC hasn't, etc.)
}

// TODO: PartialEq should be extended to a custom logic probably (first check path + size + modified, alternatively hash, etc.)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub hash: String,
    pub modified: SystemTime,
    pub path: PathBuf,
    pub size: u64
}

pub fn optimize_cache(cache_dir: &Path) {
    let mut cache_manifest = CacheManifest::retrieve(cache_dir);
    
    // TODO: In 2022 check if drain_filter() is available in stable rust, and if it is, rewrite code below.
    //       (see https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter)
    
    let mut i = 0;
    while i != cache_manifest.images.len() {
        if cache_manifest.images[i].jpg.as_ref().filter(|asset| asset.used).is_none() {
            message::cache(&format!("Removing cached image asset for {}.", cache_manifest.images[i].source_file_signature.path.display()));
            
            let cached_image_assets = cache_manifest.images.remove(i);
            
            if let Some(asset) = cached_image_assets.jpg {
                remove_cached_asset(&cache_dir.join(asset.filename));
            }
        } else {
            i += 1;
        }
    }
    
    i = 0;
    while i != cache_manifest.tracks.len() {
        if !cache_manifest.tracks[i].used {
            message::cache(&format!("Removing cached assets for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
            
            let cached_track_assets = cache_manifest.tracks.remove(i);
            
            // TODO: That the checks/clean up below can be non-exhaustive (the compiler won't
            //       know when new formats are added) is an(other) indication that the data modeling
            //       for this is not good at all - deal with this at some point.
            
            if let Some(path) = cached_track_assets.aac {
                remove_cached_asset(&cache_dir.join(path));
            }
            if let Some(path) = cached_track_assets.flac {
                remove_cached_asset(&cache_dir.join(path));
            }
            if let Some(path) = cached_track_assets.mp3_128 {
                remove_cached_asset(&cache_dir.join(path));
            }
            if let Some(path) = cached_track_assets.mp3_320 {
                remove_cached_asset(&cache_dir.join(path));
            }
            if let Some(path) = cached_track_assets.mp3_v0 {
                remove_cached_asset(&cache_dir.join(path));
            }
            if let Some(path) = cached_track_assets.ogg_vorbis {
                remove_cached_asset(&cache_dir.join(path));
            }
        } else {
            i += 1;
        }
    }
    
    cache_manifest.persist(cache_dir);
}

fn remove_cached_asset(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => (), // just what we want anyway \o/
        Err(err) => panic!(err)
    }
}

impl Asset {
    pub fn init(cache_dir: &Path, filename: String) -> Asset {
        let metadata = fs::metadata(cache_dir.join(&filename)).expect("Could not access asset");
        
        Asset {
            filename,
            filesize_bytes: metadata.len(),
            used: true
        }
    }
}

impl CacheManifest {
    pub fn new() -> CacheManifest {
        CacheManifest {
            images: Vec::new(),
            tracks: Vec::new()
        }
    }
    
    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(&self).unwrap();
        fs::write(cache_dir.join(CACHE_MANIFEST_FILENAME), &serialized).unwrap();
    }
    
    pub fn report_unused_assets(&self) {
        let mut num_unused = 0;
        
        for image in &self.images {
            if let Some(asset) = &image.jpg {
                if !asset.used { num_unused += 1; }
            }
        }
        for track in &self.tracks {
            if !track.used {
                num_unused += 1;
            }
        }
        
        if num_unused > 0 {
            message::cache(&format!("{} cached assets were not used for this build - you can run 'faircamp --optimize-cache' to reclaim disk space by removing unused cache assets.", num_unused))
        }
    }
    
    fn reset_used_flags(&mut self) {
        for image in self.images.iter_mut() {
            if let Some(ref mut asset) = &mut image.jpg {
                asset.used = false;
            }
        }
        for track in self.tracks.iter_mut() {
            track.used = false;
        }
    }
        
    pub fn retrieve(cache_dir: &Path) -> CacheManifest {
        if let Ok(bytes) = fs::read(cache_dir.join(CACHE_MANIFEST_FILENAME)) {
            if let Ok(mut manifest) = bincode::deserialize::<CacheManifest>(&bytes) {
                manifest.reset_used_flags();
                
                return manifest;
            }
        }
        
        CacheManifest::new()
    }
}

impl CachedImageAssets {
    pub fn new(source_file_signature: SourceFileSignature) -> CachedImageAssets {
        CachedImageAssets {
            jpg: None,
            source_file_signature
        }
    }
}

impl CachedTrackAssets {
    pub fn new(source_file_signature: SourceFileSignature) -> CachedTrackAssets {
        CachedTrackAssets {
            aac: None,
            flac: None,
            mp3_128: None,
            mp3_320: None,
            mp3_v0: None,
            ogg_vorbis: None,
            source_file_signature,
            used: false
        }
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