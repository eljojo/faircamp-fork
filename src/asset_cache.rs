use bincode;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const CACHE_MANIFEST_FILENAME: &str = "manifest.bincode";

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheManifest {
    pub entries: Vec<CachedTrackAssets>,
}

impl CacheManifest {
    pub fn new() -> CacheManifest {
        CacheManifest {
            entries: Vec::new(),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CachedTrackAssets {
    pub assets: Vec<String>,
    pub source_file_signature: SourceFileSignature,
}

// TODO: PartialEq should be extended to a custom logic probably (first check path + size + modified, alternatively hash, etc.)
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub hash: String,
    pub modified: SystemTime,
    pub path: PathBuf,
    pub size: u64
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