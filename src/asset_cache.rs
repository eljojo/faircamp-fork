use bincode;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const CACHE_MANIFEST_FILENAME: &str = "manifest.bincode";

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheManifest {
    pub images: Vec<CachedImageAssets>,
    pub tracks: Vec<CachedTrackAssets>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CachedImageAssets {
    pub image: Option<String>,
    pub source_file_signature: SourceFileSignature
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CachedTrackAssets {
    pub aac: Option<String>,
    pub flac: Option<String>,
    pub mp3_128: Option<String>,
    pub mp3_320: Option<String>,
    pub mp3_v0: Option<String>,
    pub ogg_vorbis: Option<String>,
    pub source_file_signature: SourceFileSignature
}

// TODO: PartialEq should be extended to a custom logic probably (first check path + size + modified, alternatively hash, etc.)
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub hash: String,
    pub modified: SystemTime,
    pub path: PathBuf,
    pub size: u64
}

impl CacheManifest {
    pub fn new() -> CacheManifest {
        CacheManifest {
            images: Vec::new(),
            tracks: Vec::new()
        }
    }
    
    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(cache_dir.join(CACHE_MANIFEST_FILENAME), &serialized).unwrap();
    }
        
    pub fn retrieve(cache_dir: &Path) -> CacheManifest {
        if let Ok(bytes) = fs::read(cache_dir.join(CACHE_MANIFEST_FILENAME)) {
            if let Ok(manifest) = bincode::deserialize(&bytes) {
                return manifest;
            }
        }
        
        CacheManifest::new()
    }
}

impl CachedImageAssets {
    pub fn new(source_file_signature: SourceFileSignature) -> CachedImageAssets {
        CachedImageAssets {
            image: None,
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
            source_file_signature
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