// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::{
    Archives,
    ArchivesRc,
    Asset,
    AudioFormat,
    AudioMeta,
    Build,
    Catalog,
    DescribedImage,
    DownloadFormat,
    Extra,
    Image,
    ImageRc,
    Track,
    Transcodes,
    TranscodesRc,
    util
};

/// This is the name of an empty file created by faircamp in the root
/// of the cache directory. When the cache layout (or critical implementation
/// details) change, this name can be updated, prompting cache purge and rebuild
/// for site operators picking up the new version of faircamp.
///
/// History:
/// - 1->2 because AudioMeta.duration_seconds changed from u32 to f32,
///   but bincode would not pick this up and cached former u32 values
///   would just be interpreted as f32 after the change.
/// - 2->3 because we renamed "releases" to "archives" (the directory,
///   but also everywhere in code)
/// - 3->4 because we introduced writing track number metadata during
///   transcoding, but we do not yet have automated cache invalidation
///   markers for tags, so we force a rebuild for 0.9.0
const ASSET_CACHE_VERSION_MARKER: &str = "CACHE_VERSION_MARKER_4";

#[derive(Clone, Debug)]
pub struct Cache {
    pub archives: Vec<ArchivesRc>,
    /// We register all files found in the cache root (= actual archive, image
    /// and track files) here before we read the cache manifests. Files
    /// referenced in the manifests that do not appear in the registry mean
    /// that the cache entry is corrupt (we then remove it). The other way
    /// around, every time we find a file in the registry we increase its
    /// usage count (the value in the HashMap). At the end of the cache retrieval
    /// process we know that all files in the registry with a usage count of 0
    /// are orphaned and can therefore be removed.
    artifact_registry: HashMap<String, usize>,
    pub images: Vec<ImageRc>,
    pub transcodes: Vec<TranscodesRc>
}

#[derive(PartialEq)]
pub enum CacheOptimization {
    Default,
    Delayed,
    Immediate,
    Manual,
    Wipe
}

// TODO: At some point consider implementing a hash property which hashes the
//       file content. With this we can even reidentify files that have changed
//       path within the catalog directory. (Con: Some overhead from computing
//       the hash for the entire file (?))
// TODO: PartialEq should be extended to a custom logic probably (first check
//       path + size + modified, etc.)
/// This stores relevant metadata for checking whether files we are processing
/// in the current build match files we were processing in a previous build.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub modified: SystemTime,
    /// The path is relative to the catalog_dir root. This ensures
    /// that we can correctly re-associate files on each build, even
    /// if the catalog directory moves around on disk. 
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64
}
    
pub fn optimize_cache(
    build: &Build,
    cache: &mut Cache,
    catalog: &mut Catalog
) {
    for archives in cache.archives.iter_mut() {
        optimize_archives(archives, build);
    }

    for image in cache.images.iter_mut() {
        optimize_image(image, build);
    }
    
    for transcodes in cache.transcodes.iter_mut() {
        optimize_transcodes(transcodes, build);
    }
    
    for release in &catalog.releases {
        let mut release_mut = release.borrow_mut();

        optimize_archives(&mut release_mut.archives, build);

        if let Some(described_image) = &mut release_mut.cover {
            optimize_image(&described_image.image, build);
        }
        
        for track in release_mut.tracks.iter_mut() {
            optimize_transcodes(&mut track.transcodes, build);
        }
    }
}

pub fn optimize_archives(archives: &mut ArchivesRc, build: &Build) {
    let mut archives_mut = archives.borrow_mut();
    let mut keep_container = false;

    for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
        let cached_format = archives_mut.get_mut(download_format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached archives ({}) for archive with {} tracks and {}.",
                    download_format.as_audio_format(),
                    // TODO: Bit awkward here that we can't easily get a pretty identifying string for the release
                    //       Possibly indication that Release + ArchivesRc should be merged together (?) (and same story with Image/Track)
                    archives_mut.track_source_file_signatures.len(),
                    if archives_mut.cover_source_file_signature.is_some() { "a cover" } else { "no cover" }
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }

    if keep_container {
        archives_mut.persist_to_cache(&build.cache_dir);
    } else {
        util::remove_file(&archives_mut.manifest_path(&build.cache_dir));
    }
}

pub fn optimize_image(image: &ImageRc, build: &Build) {
    let mut image_mut = image.borrow_mut();
    let mut keep_container = false;

    let path = image_mut.source_file_signature.path.display().to_string();

    {
        let mut optimize = |asset_option: &mut Option<Asset>, format: &str, path: &str| {
            match asset_option.as_ref().map(|asset| asset.obsolete(build)) {
                Some(true) => {
                    util::remove_file(&build.cache_dir.join(asset_option.take().unwrap().filename));
                    info_cache!(
                        "Removed cached image asset ({}) for {}.",
                        format,
                        path
                    );
                }
                Some(false) => keep_container = true,
                None => ()
            }
        };

        optimize(&mut image_mut.background_asset, "background", &path);
        optimize(&mut image_mut.feed_asset, "feed", &path);
    }

    {
        match image_mut.artist_assets.as_ref().map(|assets| assets.obsolete(build)) {
            Some(true) => {
                for asset in image_mut.artist_assets.take().unwrap().all() {
                    util::remove_file(&build.cache_dir.join(&asset.filename));
                    info_cache!(
                        "Removed cached image asset ({}) for {} {}x{}.",
                        "artist",
                        &path,
                        asset.height,
                        asset.width
                    );
                }
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }

    {
        match image_mut.cover_assets.as_ref().map(|assets| assets.obsolete(build)) {
            Some(true) => {
                for asset in image_mut.cover_assets.take().unwrap().all() {
                    util::remove_file(&build.cache_dir.join(&asset.filename));
                    info_cache!(
                        "Removed cached image asset ({}) for {} {}x{}.",
                        "cover",
                        &path,
                        asset.edge_size,
                        asset.edge_size
                    );
                }
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }

    if keep_container {
        image_mut.persist_to_cache(&build.cache_dir);
    } else {
        util::remove_file(&image_mut.manifest_path(&build.cache_dir));
    }
}

pub fn optimize_transcodes(transcodes: &mut TranscodesRc, build: &Build) {
    let mut transcodes_mut = transcodes.borrow_mut();
    let mut keep_container = false;
    
    for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
        let cached_format = transcodes_mut.get_mut(audio_format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached transcode ({}) for {}.",
                    audio_format,
                    transcodes_mut.source_file_signature.path.display()
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }
    
    if keep_container {
        transcodes_mut.persist_to_cache(&build.cache_dir);
    } else {
        util::remove_file(&transcodes_mut.manifest_path(&build.cache_dir));
    }
}

pub fn report_stale(cache: &Cache, catalog: &Catalog) {
    let mut num_unused = 0;
    let mut unused_bytesize = 0;
    
    for archives in &cache.archives {
        report_stale_archives(archives, &mut num_unused, &mut unused_bytesize);
    }

    for image in &cache.images {
        report_stale_images(image, &mut num_unused, &mut unused_bytesize);
    }
    
    for transcodes in &cache.transcodes {
        report_stale_transcodes(transcodes, &mut num_unused, &mut unused_bytesize);
    }
    
    for release in &catalog.releases {
        let release_ref = release.borrow();

        report_stale_archives(&release_ref.archives, &mut num_unused, &mut unused_bytesize);

        if let Some(described_image) = &release_ref.cover {
            report_stale_images(&described_image.image, &mut num_unused, &mut unused_bytesize);
        }
        
        for track in &release_ref.tracks {
            report_stale_transcodes(&track.transcodes, &mut num_unused, &mut unused_bytesize);
        }
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

pub fn report_stale_archives(
    archives: &ArchivesRc,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
        if let Some(filesize_bytes) = archives
            .borrow()
            .get(download_format)
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    }
}

pub fn report_stale_images(
    image: &ImageRc,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    let image_ref = image.borrow();

    let mut report = |asset_option: &Option<Asset>| {
        if let Some(filesize_bytes) = asset_option
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    };

    report(&image_ref.background_asset);
    report(&image_ref.feed_asset);

    if let Some(assets) = image_ref.artist_assets
        .as_ref()
        .filter(|assets| assets.marked_stale.is_some()) {
        for asset in &assets.all() {
            *num_unused += 1;
            *unused_bytesize += asset.filesize_bytes;
        }
    }

    if let Some(assets) = image_ref.cover_assets
        .as_ref()
        .filter(|assets| assets.marked_stale.is_some()) {
        for asset in &assets.all() {
            *num_unused += 1;
            *unused_bytesize += asset.filesize_bytes;
        }
    }
}

pub fn report_stale_transcodes(
    transcodes: &TranscodesRc,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
        if let Some(filesize_bytes) = transcodes
            .borrow()
            .get(audio_format)
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    }
}

impl Cache {
    pub const ARCHIVE_MANIFESTS_DIR: &'static str = "archives";
    pub const IMAGE_MANIFESTS_DIR: &'static str = "images";
    pub const TRACK_MANIFESTS_DIR: &'static str = "tracks";
    
    fn ensure_manifest_dirs(cache_dir: &Path) {
        util::ensure_dir(&cache_dir.join(Cache::ARCHIVE_MANIFESTS_DIR));
        util::ensure_dir(&cache_dir.join(Cache::IMAGE_MANIFESTS_DIR));
        util::ensure_dir(&cache_dir.join(Cache::TRACK_MANIFESTS_DIR));
    }

    /// Scans the cache root dir and initializes a HashMap that tracks
    /// all filenames, mapping them to a usage count (initialized to 0).
    /// The asset cache version marker is explicitly excluded from this.
    fn fill_registry(&mut self, cache_dir: &Path) {
        if let Ok(dir_entries) = cache_dir.read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if dir_entry
                        .file_type()
                        .map(|file_type| file_type.is_file())
                        .unwrap_or(false) {
                        let filename = dir_entry.file_name().to_str().unwrap().to_string();

                        if filename != ASSET_CACHE_VERSION_MARKER {
                            self.artifact_registry.insert(filename, 0);
                        }
                    }
                }
            }
        }
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for archives in self.archives.iter_mut() {
            archives.borrow_mut().mark_all_stale(timestamp);
        }

        for image in self.images.iter_mut() {
            image.borrow_mut().mark_all_stale(timestamp);
        }
        
        for transcodes in self.transcodes.iter_mut() {
            transcodes.borrow_mut().mark_all_stale(timestamp);
        }
    }

    fn new() -> Cache {
        Cache {
            archives: Vec::new(),
            artifact_registry: HashMap::new(),
            images: Vec::new(),
            transcodes: Vec::new()
        }
    }

    pub fn retrieve(cache_dir: &Path) -> Cache {
        let mut cache = Cache::new();

        let version_marker_file = cache_dir.join(ASSET_CACHE_VERSION_MARKER);

        if !version_marker_file.exists() {
            if cache_dir.exists() {
                warn!("Existing cache data is in an incompatible format (= from an older faircamp version), the cache will be purged and regenerated.");
                util::ensure_empty_dir(cache_dir);
            } else {
                util::ensure_dir(cache_dir);
            }
            fs::write(version_marker_file, "").unwrap();
        }

        Cache::ensure_manifest_dirs(cache_dir);

        cache.fill_registry(cache_dir);

        cache.retrieve_archives(cache_dir);
        cache.retrieve_images(cache_dir);
        cache.retrieve_transcodes(cache_dir);

        cache.remove_orphaned(cache_dir);

        cache
    }

    fn remove_orphaned(&mut self, cache_dir: &Path) {
        for (filename, usage_counter) in self.artifact_registry.drain() {
            if usage_counter == 0 {
                warn!(
                    "Removing orphaned cache artifact ({}) - it was probably created with a different version of faircamp.",
                    filename
                );
                util::remove_file(&cache_dir.join(filename));
            }
        }
    }

    fn retrieve_archives(&mut self, cache_dir: &Path) {
        if let Ok(dir_entries) = cache_dir.join(Cache::ARCHIVE_MANIFESTS_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(mut archives) = Archives::deserialize_cached(&dir_entry.path()) {
                        let mut assets_present = false;

                        for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
                            let asset_option = archives.get_mut(download_format);
                            if let Some(asset) = &asset_option {
                                if let Some(usage_counter) = self.artifact_registry.get_mut(&asset.filename) {
                                    assets_present = true;
                                    *usage_counter += 1;
                                } else {
                                    asset_option.take();
                                }
                            }
                        }

                        if assets_present {
                            self.archives.push(ArchivesRc::new(archives));
                        } else {
                            // No actual cached files present, can throw away serialized metadata too
                            util::remove_file(&dir_entry.path());
                        }
                    } else {
                        warn!(
                            "Removing unsupported archive asset manifest in cache ({}) - it was probably created with a different version of faircamp.",
                            &dir_entry.path().display()
                        );
                        util::remove_file(&dir_entry.path());
                    }
                }
            }
        }
    }

    // TODO: Should probably not quietly ignore everything that can go wrong here (here and elsewhere)
    fn retrieve_images(&mut self, cache_dir: &Path) {
        if let Ok(dir_entries) = cache_dir.join(Cache::IMAGE_MANIFESTS_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(mut image_mut) = Image::deserialize_cached(&dir_entry.path()) {
                        if let Some(artist_assets) = image_mut.artist_assets.as_mut() {
                            let all_assets = artist_assets.all();

                            for asset in all_assets.iter() {
                                if let Some(usage_counter) = self.artifact_registry.get_mut(&asset.filename) {
                                    *usage_counter += 1;
                                } else {
                                    // If a single asset is in a corrupt state (cached file missing)
                                    // we delete all other assets and remove the cache entry altogether.

                                    for asset_to_delete in all_assets.iter() {
                                        if cache_dir.join(&asset_to_delete.filename).exists() {
                                            util::remove_file(&cache_dir.join(&asset_to_delete.filename));
                                        }
                                    }

                                    image_mut.artist_assets = None;
                                    break;
                                }
                            }
                        }


                        if let Some(background_asset) = &image_mut.background_asset {
                            if let Some(usage_counter) = self.artifact_registry.get_mut(&background_asset.filename) {
                                *usage_counter += 1;
                            } else {
                                image_mut.background_asset = None;
                            }
                        }

                        if let Some(cover_assets) = image_mut.cover_assets.as_mut() {
                            let all_assets = cover_assets.all();

                            for asset in all_assets.iter() {
                                if let Some(usage_counter) = self.artifact_registry.get_mut(&asset.filename) {
                                    *usage_counter += 1;
                                } else {
                                    // If a single asset is in a corrupt state (cached file missing)
                                    // we delete all other assets and remove the cache entry altogether.

                                    for asset_to_delete in all_assets.iter() {
                                        if cache_dir.join(&asset_to_delete.filename).exists() {
                                            util::remove_file(&cache_dir.join(&asset_to_delete.filename));
                                        }
                                    }

                                    image_mut.cover_assets = None;
                                    break;
                                }
                            }
                        }

                        if let Some(feed_asset) = &image_mut.feed_asset {
                            if let Some(usage_counter) = self.artifact_registry.get_mut(&feed_asset.filename) {
                                *usage_counter += 1;
                            } else {
                                image_mut.feed_asset = None;
                            }
                        }

                        if image_mut.artist_assets.is_some() ||
                            image_mut.background_asset.is_some() ||
                            image_mut.cover_assets.is_some() ||
                            image_mut.feed_asset.is_some() {
                            self.images.push(ImageRc::retrieved(image_mut));
                        } else {
                            // No actual cached files present, can throw away serialized metadata too
                            util::remove_file(&dir_entry.path());
                        }
                    } else {
                        warn!(
                            "Removing unsupported image asset manifest in cache ({}) - it was probably created with a different version of faircamp.",
                            &dir_entry.path().display()
                        );
                        util::remove_file(&dir_entry.path());
                    }
                }
            }
        }
    }
    
    fn retrieve_transcodes(&mut self, cache_dir: &Path) {  
        if let Ok(dir_entries) = cache_dir.join(Cache::TRACK_MANIFESTS_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(mut transcodes) = Transcodes::deserialize_cached(&dir_entry.path()) {
                        let mut dead_references_removed = false;

                        for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
                            let asset_option = transcodes.get_mut(audio_format);
                            if let Some(asset) = &asset_option {
                                if let Some(usage_counter) = self.artifact_registry.get_mut(&asset.filename) {
                                    *usage_counter += 1;
                                } else {
                                    dead_references_removed = true;
                                    asset_option.take();
                                }
                            }
                        }

                        if dead_references_removed {
                            // Persist corrections so we don't have to re-apply them next time around
                            transcodes.persist_to_cache(cache_dir);
                        }

                        // With archives and images we would throw away
                        // serialized metadata here if no actual cached files are
                        // present. However for a track the cached metadata
                        // contains AudioMeta, which is expensively computed,
                        // therefore we always retain the serialized metadata
                        // and only remove it if cache optimization calls for
                        // it.

                        self.transcodes.push(TranscodesRc::new(transcodes));
                    } else {
                        warn!(
                            "Removing unsupported track asset manifest in cache ({}) - it was probably created with a different version of faircamp.",
                            &dir_entry.path().display()
                        );
                        util::remove_file(&dir_entry.path());
                    }
                }
            }
        }
    }

    /// This basically checks "Do we have cached download archives which
    /// include the given cover image, all tracks in the right order, and all extras?"
    /// (whether we have the image and transcodes in all required formats is not
    /// yet relevant at this point). If yes they are returned, otherwise
    /// created (but not yet computed).
    pub fn get_or_create_archives(
        &mut self,
        cover: &Option<DescribedImage>,
        tracks: &[Track],
        extras: &[Extra]
    ) -> ArchivesRc {
        match self.archives
            .iter()
            .find(|archives| {
                let archives_ref = archives.borrow();

                if let Some(described_image) = cover {
                    if archives_ref.cover_source_file_signature.as_ref() !=
                       Some(&described_image.image.borrow().source_file_signature) {
                        return false;
                    }
                } else if archives_ref.cover_source_file_signature.is_some() {
                    return false;
                }

                if extras.len() != archives_ref.extra_source_file_signatures.len() {
                    return false;
                }

                for extra in extras {
                    if !archives_ref.extra_source_file_signatures.contains(&extra.source_file_signature) {
                        return false;
                    }
                }

                tracks
                    .iter()
                    .zip(archives_ref.track_source_file_signatures.iter())
                    .all(|(track, source_file_signature)| {
                        &track.transcodes.borrow().source_file_signature == source_file_signature
                    })
            }) {
            Some(archives) => archives.clone(),
            None => {
                let track_source_file_signatures = tracks
                    .iter()
                    .map(|track| track.transcodes.borrow().source_file_signature.clone())
                    .collect();

                let archives = ArchivesRc::new(Archives::new(
                    cover.as_ref().map(|described_image| described_image.image.borrow().source_file_signature.clone()),
                    track_source_file_signatures,
                    extras.iter().map(|extra| extra.source_file_signature.clone()).collect()
                ));
                self.archives.push(archives.clone());
                archives
            }
        }
    }

    pub fn get_or_create_image(
        &mut self,
        build: &Build,
        source_path: &Path
    ) -> ImageRc {
        let source_file_signature = SourceFileSignature::new(build, source_path);

        match self.images.iter().find(|image|
            image.borrow().source_file_signature == source_file_signature
        ) {
            Some(image) => image.clone(),
            None => {
                let image = ImageRc::new(source_file_signature);
                self.images.push(image.clone());
                image
            }
        }
    }
    
    pub fn get_or_create_transcodes(
        &mut self,
        build: &Build,
        source_path:&Path,
        extension: &str
    ) -> TranscodesRc {
        let source_file_signature = SourceFileSignature::new(build, source_path);
        
        match self.transcodes.iter().find(|transcodes|
            transcodes.borrow().source_file_signature == source_file_signature
        ) {
            Some(transcodes) => transcodes.clone(),
            None => {
                let source_meta = AudioMeta::extract(&build.catalog_dir.join(source_path), extension);
                let transcodes = Transcodes::new(source_file_signature, source_meta);

                // We already extracted the AudioMeta for this track - which is costly
                // to compute - therefore we already persist the transcodes to the 
                // cache, even if there are no transcoded artifacts yet.
                transcodes.persist_to_cache(&build.cache_dir);

                let transcodes_rc = TranscodesRc::new(transcodes);
                self.transcodes.push(transcodes_rc.clone());
                transcodes_rc
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
    pub fn new(build: &Build, path: &Path) -> SourceFileSignature {
        let metadata = fs::metadata(build.catalog_dir.join(path))
            .expect("Could not access source file");
        
        SourceFileSignature {
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            path: path.to_path_buf(),
            size: metadata.len()
        }
    }
}