use chrono::{DateTime, Duration, Utc};
use std::{
    fmt,
    fs,
    path::{Path, PathBuf},
    time::SystemTime
};

use crate::{
    audio_format::AudioFormat,
    audio_meta::AudioMeta,
    image::CachedImageAssets,
    message,
    release::{CachedReleaseAssets},
    track::{CachedTrackAssets, Track},
    util
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub filename: String,
    pub filesize_bytes: u64, 
    pub marked_stale: Option<DateTime<Utc>>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheManifest {
    pub images: Vec<CachedImageAssets>,
    pub releases: Vec<CachedReleaseAssets>,
    pub tracks: Vec<CachedTrackAssets>
}

pub enum CacheOptimization {
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

pub fn optimize_cache(cache_dir: &Path, cache_manifest: &mut CacheManifest, cache_optimization: &CacheOptimization) {
    for cached_assets in cache_manifest.images.iter_mut() {
        if cached_assets.jpeg.as_ref().filter(|asset| asset.obsolete(cache_optimization)).is_some() {
            if let Some(asset) = cached_assets.jpeg.take() {
                message::cache(&format!("Removing cached image asset (JPEG) for {}.", cached_assets.source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(asset.filename));
            }
            
            util::remove_file(&cached_assets.manifest_path(cache_dir));
        }    
    }
    
    for cached_assets in cache_manifest.releases.iter_mut() {
        let mut keep_container = false;
        
        for format in &[
            AudioFormat::Aac,
            AudioFormat::Aiff,
            AudioFormat::Flac,
            AudioFormat::Mp3Cbr128,
            AudioFormat::Mp3Cbr320,
            AudioFormat::Mp3VbrV0,
            AudioFormat::OggVorbis,
            AudioFormat::Wav
        ] {
            let cached_format = cached_assets.get_mut(&format);
            
            match cached_format.as_ref().map(|asset| asset.obsolete(cache_optimization)) {
                Some(true) => {
                    util::remove_file(&cache_dir.join(cached_format.take().unwrap().filename));
                    message::cache(&format!(
                        "Removed cached release asset ({}) for archive with {} tracks.",
                        format,
                        cached_assets.source_file_signatures.len()  // TODO: Bit awkward here that we can't easily get a pretty identifying string for the release
                                                                    //       Possibly indication that Release + CachedReleaseAssets should be merged together (?) (and same story with Image/Track)
                    ));
                }
                Some(false) => keep_container = true,
                None => ()
            }
        }
        
        if keep_container {
            cached_assets.persist(cache_dir);
        } else {
            util::remove_file(&cached_assets.manifest_path(cache_dir));
        }
    }
    
    for cached_assets in cache_manifest.tracks.iter_mut() {
        let mut keep_container = false;
        
        for format in &[
            AudioFormat::Aac,
            AudioFormat::Aiff,
            AudioFormat::Flac,
            AudioFormat::Mp3Cbr128,
            AudioFormat::Mp3Cbr320,
            AudioFormat::Mp3VbrV0,
            AudioFormat::OggVorbis,
            AudioFormat::Wav
        ] {
            let cached_format = cached_assets.get_mut(&format);
            
            match cached_format.as_ref().map(|asset| asset.obsolete(cache_optimization)) {
                Some(true) => {
                    util::remove_file(&cache_dir.join(cached_format.take().unwrap().filename));
                    message::cache(&format!(
                        "Removed cached track asset ({}) for {}.",
                        format,
                        cached_assets.source_file_signature.path.display()
                    ));
                }
                Some(false) => keep_container = true,
                None => ()
            }
        }
        
        if keep_container {
            cached_assets.persist(cache_dir);
        } else {
            util::remove_file(&cached_assets.manifest_path(cache_dir));
        }
    }
}

impl Asset {
    pub fn init(cache_dir: &Path, filename: String) -> Asset {
        let metadata = fs::metadata(cache_dir.join(&filename)).expect("Could not access asset");
        
        Asset {
            filename,
            filesize_bytes: metadata.len(),
            marked_stale: None
        }
    }
    
    pub fn mark_stale(&mut self) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(Utc::now());
        }
    }
    
    pub fn obsolete(&self, cache_optimization: &CacheOptimization) -> bool {
        match &self.marked_stale {
            Some(datetime_marked_stale) => {
                match cache_optimization {
                    CacheOptimization::Delayed => match Utc::now().checked_sub_signed(Duration::hours(24)) {
                        Some(datetime_24hrs_ago) => datetime_marked_stale < &datetime_24hrs_ago,
                        None => true  // system time probably messed up for good, better to lose usable
                                      // cache data than to leak disk space at potentially every
                                      // following build until this resolves (if it ever does)
                    }
                    CacheOptimization::Immediate |
                    CacheOptimization::Manual |
                    CacheOptimization::Wipe => true
                }
            },
            None => false
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
    
    // TODO: This is basically identical with get_track_assets(...) below - unify this via generics or enum or something
    pub fn get_image_assets(&self, source_path: &Path) -> CachedImageAssets {
        let source_file_signature = SourceFileSignature::init(source_path);
        
        self.images
            .iter()
            .find(|cached_assets| cached_assets.source_file_signature == source_file_signature)
            .map(|cached_assets| cached_assets.clone())
            .unwrap_or_else(|| CachedImageAssets::new(source_file_signature))
    }
    
    pub fn get_release_assets(&self, tracks: &Vec<Track>) -> CachedReleaseAssets {
        self.releases
            .iter()
            .find(|cached_assets| {
                tracks
                    .iter()
                    .zip(cached_assets.source_file_signatures.iter())
                    .all(|(track, source_file_signature)| {
                        &track.cached_assets.source_file_signature == source_file_signature
                    })
            })
            .map(|cached_assets| cached_assets.clone())
            .unwrap_or_else(|| {
                CachedReleaseAssets::new(
                    tracks
                        .iter()
                        .map(|track| track.cached_assets.source_file_signature.clone())
                        .collect()
                )
            })
    }
    
    pub fn get_track_assets(&self, source_path: &Path, extension: &str) -> CachedTrackAssets {
        let source_file_signature = SourceFileSignature::init(source_path);
        
        self.tracks
            .iter()
            .find(|cached_assets| cached_assets.source_file_signature == source_file_signature)
            .map(|cached_assets| cached_assets.clone())
            .unwrap_or_else(|| {
                let source_meta = AudioMeta::extract(source_path, extension);
                
                CachedTrackAssets::new(source_file_signature, source_meta)
            })
    }
    
    pub fn report_unused_assets(&self) {
        let mut num_unused = 0;
        let mut unused_bytesize = 0;
        
        for image in &self.images {
            if let Some(filesize_bytes) = image.jpeg.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
        }
        for track in &self.tracks {
            if let Some(filesize_bytes) = track.aac.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.aiff.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.flac.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_128.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_320.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_v0.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.ogg_vorbis.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.wav.as_ref().filter(|asset| asset.obsolete(&CacheOptimization::Manual)).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
        }
        
        if num_unused > 0 {
            message::cache(&format!(
                "{num_unused} cached assets were identified as obsolete - you can run 'faircamp --optimize-cache' to to remove them and reclaim {unused_bytesize} of disk space.",
                num_unused=num_unused,
                unused_bytesize=util::format_bytes(unused_bytesize)
            ));
        } else {
            message::cache(&format!("No cached assets were identied as obsolete."));
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

impl fmt::Display for CacheOptimization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
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