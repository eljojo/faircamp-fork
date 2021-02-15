use bincode;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    time::SystemTime
};

use crate::{
    audio_format::AudioFormat,
    image_format::ImageFormat,
    message,
    util
};

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
    pub jpeg: Option<Asset>,
    pub source_file_signature: SourceFileSignature
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedTrackAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub flac: Option<Asset>,
    pub mp3_128: Option<Asset>,
    pub mp3_320: Option<Asset>,
    pub mp3_v0: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub wav: Option<Asset>
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
        if cache_manifest.images[i].jpeg.as_ref().filter(|asset| asset.used).is_none() {
            message::cache(&format!("Removing cached image asset (JPEG) for {}.", cache_manifest.images[i].source_file_signature.path.display()));
            
            let cached_image_assets = cache_manifest.images.remove(i);
            
            if let Some(asset) = cached_image_assets.jpeg {
                remove_cached_asset(&cache_dir.join(asset.filename));
            }
        } else {
            i += 1;
        }
    }
    
    i = 0;
    while i != cache_manifest.tracks.len() {
        let mut keep_container = false;
        
        if let Some(asset) = &cache_manifest.tracks[i].aac {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (AAC) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].aiff {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (AIFF) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].flac {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (FLAC) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].mp3_128 {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (MP3 128) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].mp3_320 {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (MP3 320) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].mp3_v0 {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (MP3 V0) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].ogg_vorbis {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (Ogg Vorbis) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
            }
        }
        if let Some(asset) = &cache_manifest.tracks[i].wav {
            if asset.used {
                keep_container = true;
            } else {
                message::cache(&format!("Removing cached track asset (WAV) for {}.", cache_manifest.tracks[i].source_file_signature.path.display()));
                remove_cached_asset(&cache_dir.join(&asset.filename));
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
    // TODO: This is basically identical with get_track_assets(...) below - unify this via generics or enum or something
    pub fn get_image_assets(&self, source_path: &Path) -> CachedImageAssets {
        let source_file_signature = SourceFileSignature::init(source_path);
        
        self.images
            .iter()
            .find(|cached_assets| cached_assets.source_file_signature == source_file_signature)
            .map(|cached_assets| cached_assets.clone())
            .unwrap_or_else(|| CachedImageAssets::new(source_file_signature))
    }
    
    pub fn get_track_assets(&self, source_path: &Path) -> CachedTrackAssets {
        let source_file_signature = SourceFileSignature::init(source_path);
        
        self.tracks
            .iter()
            .find(|cached_assets| cached_assets.source_file_signature == source_file_signature)
            .map(|cached_assets| cached_assets.clone())
            .unwrap_or_else(|| CachedTrackAssets::new(source_file_signature))
    }
    
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
        let mut unused_bytesize = 0;
        
        for image in &self.images {
            if let Some(filesize_bytes) = image.jpeg.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
        }
        for track in &self.tracks {
            if let Some(filesize_bytes) = track.aac.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.aiff.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.flac.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_128.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_320.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.mp3_v0.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.ogg_vorbis.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
                num_unused += 1;
                unused_bytesize += filesize_bytes;
            }
            if let Some(filesize_bytes) = track.wav.as_ref().filter(|asset| !asset.used).map(|asset| asset.filesize_bytes) {
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
        for image in self.images.iter_mut() {
            // Note here and below that we only iterate over the single inner value of the option (if present)
            image.jpeg.iter_mut().for_each(|asset| asset.used = false);
        }
        for track in self.tracks.iter_mut() {
            track.aac.iter_mut().for_each(|asset| asset.used = false);
            track.aiff.iter_mut().for_each(|asset| asset.used = false);
            track.flac.iter_mut().for_each(|asset| asset.used = false);
            track.mp3_128.iter_mut().for_each(|asset| asset.used = false);
            track.mp3_320.iter_mut().for_each(|asset| asset.used = false);
            track.mp3_v0.iter_mut().for_each(|asset| asset.used = false);
            track.ogg_vorbis.iter_mut().for_each(|asset| asset.used = false);
            track.wav.iter_mut().for_each(|asset| asset.used = false);
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
            source_file_signature
        }
    }
}

impl CachedTrackAssets {
    pub fn get(&self, format: &AudioFormat) -> &Option<Asset> {
        match format {
            AudioFormat::Aac => &self.aac,
            AudioFormat::Aiff => &self.aiff,
            AudioFormat::Flac => &self.flac,
            AudioFormat::Mp3Cbr128 => &self.mp3_128,
            AudioFormat::Mp3Cbr320 => &self.mp3_320,
            AudioFormat::Mp3VbrV0 => &self.mp3_v0,
            AudioFormat::OggVorbis => &self.ogg_vorbis,
            AudioFormat::Wav => &self.wav
        }
    }
    
    pub fn get_mut(&mut self, format: &AudioFormat) -> &mut Option<Asset> {
        match format {
            AudioFormat::Aac => &mut self.aac,
            AudioFormat::Aiff => &mut self.aiff,
            AudioFormat::Flac => &mut self.flac,
            AudioFormat::Mp3Cbr128 => &mut self.mp3_128,
            AudioFormat::Mp3Cbr320 => &mut self.mp3_320,
            AudioFormat::Mp3VbrV0 => &mut self.mp3_v0,
            AudioFormat::OggVorbis => &mut self.ogg_vorbis,
            AudioFormat::Wav => &mut self.wav
        }
    }

    pub fn new(source_file_signature: SourceFileSignature) -> CachedTrackAssets {
        CachedTrackAssets {
            aac: None,
            aiff: None,
            flac: None,
            mp3_128: None,
            mp3_320: None,
            mp3_v0: None,
            ogg_vorbis: None,
            source_file_signature,
            wav: None
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