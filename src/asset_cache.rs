use bincode;
use chrono::{DateTime, Duration, Utc};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime
};

use crate::{
    audio_meta::AudioMeta,
    image::CachedImageAssets,
    message,
    release::{CachedReleaseAssets},
    track::{CachedTrackAssets, Track},
    util
};

const CACHE_MANIFEST_FILENAME: &str = "manifest.bincode";

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

// TODO: PartialEq should be extended to a custom logic probably (first check path + size + modified, alternatively hash, etc.)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub hash: String,
    pub modified: SystemTime,
    pub path: PathBuf,
    pub size: u64
}

pub fn optimize_cache(cache_dir: &Path, cache_manifest: &mut CacheManifest) {
    // TODO: In 2022 check if drain_filter() is available in stable rust, and if it is, rewrite code below.
    //       (see https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter)
    
    let mut i = 0;
    while i != cache_manifest.images.len() {
        if cache_manifest.images[i].jpeg.as_ref().filter(|asset| asset.obsolete()).is_some() {
            message::cache(&format!("Removing cached image asset (JPEG) for {}.", cache_manifest.images[i].source_file_signature.path.display()));
            
            let cached_image_assets = cache_manifest.images.remove(i);
            
            if let Some(asset) = cached_image_assets.jpeg {
                util::remove_file(&cache_dir.join(asset.filename));
            }
        } else {
            i += 1;
        }
    }
    
    i = 0;
    while i != cache_manifest.tracks.len() {
        let mut keep_container = false;
        
        if let Some(asset) = &cache_manifest.tracks[i].aac {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (AAC) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].aiff {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (AIFF) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].flac {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (FLAC) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].mp3_128 {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (MP3 128) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].mp3_320 {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (MP3 320) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].mp3_v0 {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (MP3 V0) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].ogg_vorbis {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (Ogg Vorbis) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].wav {
            if asset.obsolete() {
                message::cache(&format!("Removing cached track asset (WAV) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                util::remove_file(&cache_dir.join(&asset.filename));
            } else {
                keep_container = true;
            }
        }
        
        if !keep_container {
            cache_manifest.tracks.remove(i);
        } else {
            i += 1;
        }
    }
    
    cache_manifest.persist(cache_dir);
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
    
    pub fn obsolete(&self) -> bool {
        match &self.marked_stale {
            Some(datetime_marked_stale) => {
                match Utc::now().checked_sub_signed(Duration::hours(24)) {
                    Some(datetime_24hrs_ago) => datetime_marked_stale < &datetime_24hrs_ago,
                    None => true  // system time probably messed up for good, better to lose usable
                                  // cache data than to leak disk space at potentially every
                                  // following build until this resolves (if it ever does)
                }
            },
            None => false
        }
    }
}

impl CacheManifest {
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
    
    pub fn new() -> CacheManifest {
        CacheManifest {
            images: Vec::new(),
            releases: Vec::new(),
            tracks: Vec::new()
        }
    }
    
    // TODO: Probably can dispose of this
    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(&self).unwrap();
        fs::write(cache_dir.join(CACHE_MANIFEST_FILENAME), &serialized).unwrap();
    }
    
    pub fn report_unused_assets(&self) {
        let mut num_unused = 0;
        let mut unused_bytesize = 0;
        
        for image in &self.images {
            if let Some(filesize_bytes) = image.jpeg.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
        }
        for track in &self.tracks {
            if let Some(filesize_bytes) = track.aac.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.aiff.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.flac.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_128.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_320.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_v0.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.ogg_vorbis.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.wav.as_ref().filter(|asset| asset.obsolete()).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
        }
        
        if num_unused > 0 {
            message::cache(&format!(
                "{num_unused} cached assets were not used for this build - you can run 'faircamp --optimize-cache' to reclaim {unused_bytesize} of disk space by removing unused cache assets.",
                num_unused=num_unused,
                unused_bytesize=util::format_bytes(unused_bytesize)
            ));
        }
    }
    
    fn reset_used_flags(&mut self) {
        let now = Utc::now();
        
        for image in self.images.iter_mut() {
            // Note here and below that we only iterate over the single inner value of the option (if present)
            image.jpeg.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
        }
        for track in self.tracks.iter_mut() {
            track.aac.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.aiff.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.flac.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.mp3_128.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.mp3_320.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.mp3_v0.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.ogg_vorbis.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
            track.wav.iter_mut().for_each(|asset| asset.marked_stale = Some(now));
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