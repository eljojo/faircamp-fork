use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::SystemTime
};

use crate::{
    ArchiveAssets,
    Asset,
    AudioFormat,
    AudioMeta,
    Build,
    Catalog,
    DownloadFormat,
    Extra,
    Image,
    ImageAssets,
    Track,
    TrackAssets,
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
const ASSET_CACHE_VERSION_MARKER: &str = "CACHE_VERSION_MARKER_3";

#[derive(Clone, Debug)]
pub struct Cache {
    pub archives: Vec<Rc<RefCell<ArchiveAssets>>>,
    /// We register all files found in the cache root (= actual archive, image
    /// and track files) here before we read the cache manifests. Files
    /// referenced in the manifests that do not appear in the registry mean
    /// that the cache entry is corrupt (we then remove it). The other way
    /// around, every time we find a file in the registry we increase its
    /// usage count (the value in the HashMap). At the end of the cache retrieval
    /// process we know that all files in the registry with a usage count of 0
    /// are orphaned and can therefore be removed.
    artifact_registry: HashMap<String, usize>,
    pub images: Vec<Rc<RefCell<ImageAssets>>>,
    pub tracks: Vec<Rc<RefCell<TrackAssets>>>
}

#[derive(PartialEq)]
pub enum CacheOptimization {
    Default,
    Delayed,
    Immediate,
    Manual,
    Wipe
}

// TODO: PartialEq should be extended to a custom logic probably (first check
//       path + size + modified, alternatively hash, etc.)
/// This stores relevant metadata for checking whether files we are processing
/// in the current build match files we were processing in a previous build.
/// The hash part is not yet implemented at all, so far we only use relative
/// path in catalog directory, size and modification date to determine equality.
/// Eventually if the path does not match we will be able to use hash instead,
/// to detect a file that has just moved.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourceFileSignature {
    pub hash: String,
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
    for archive_assets in cache.archives.iter_mut() {
        optimize_archive_assets(archive_assets, build);
    }

    for image_assets in cache.images.iter_mut() {
        optimize_image_assets(image_assets, build);
    }
    
    for track_assets in cache.tracks.iter_mut() {
        optimize_track_assets(track_assets, build);
    }
    
    for release in &catalog.releases {
        let mut release_mut = release.borrow_mut();

        optimize_archive_assets(&mut release_mut.archive_assets, build);

        if let Some(image) = &mut release_mut.cover {
            optimize_image_assets(&mut image.borrow_mut().assets, build);
        }
        
        for track in release_mut.tracks.iter_mut() {
            optimize_track_assets(&mut track.assets, build);
        }
    }
}

pub fn optimize_archive_assets(archive_assets: &mut Rc<RefCell<ArchiveAssets>>, build: &Build) {
    let mut archive_assets_mut = archive_assets.borrow_mut();
    let mut keep_container = false;

    for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
        let cached_format = archive_assets_mut.get_mut(download_format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached archive asset ({}) for archive with {} tracks and {}.",
                    download_format.as_audio_format(),
                    // TODO: Bit awkward here that we can't easily get a pretty identifying string for the release
                    //       Possibly indication that Release + ArchiveAssets should be merged together (?) (and same story with Image/Track)
                    archive_assets_mut.track_source_file_signatures.len(),
                    if archive_assets_mut.cover_source_file_signature.is_some() { "a cover" } else { "no cover" }
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }

    if keep_container {
        archive_assets_mut.persist_to_cache(&build.cache_dir);
    } else {
        util::remove_file(&archive_assets_mut.manifest_path(&build.cache_dir));
    }
}

pub fn optimize_image_assets(assets: &mut Rc<RefCell<ImageAssets>>, build: &Build) {
    let mut assets_mut = assets.borrow_mut();
    let mut keep_container = false;

    let path = assets_mut.source_file_signature.path.display().to_string();

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

        optimize(&mut assets_mut.background, "background", &path);
        optimize(&mut assets_mut.feed, "feed", &path);
    }

    {
        match assets_mut.artist.as_ref().map(|assets| assets.obsolete(build)) {
            Some(true) => {
                for asset in assets_mut.artist.take().unwrap().all() {
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
        match assets_mut.cover.as_ref().map(|assets| assets.obsolete(build)) {
            Some(true) => {
                for asset in assets_mut.cover.take().unwrap().all() {
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
        assets_mut.persist_to_cache(&build.cache_dir);
    } else {
        util::remove_file(&assets_mut.manifest_path(&build.cache_dir));
    }
}

pub fn optimize_track_assets(track_assets: &mut Rc<RefCell<TrackAssets>>, build: &Build) {
    let mut track_assets_mut = track_assets.borrow_mut();
    let mut keep_container = false;
    
    for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
        let cached_format = track_assets_mut.get_mut(audio_format);
        
        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                util::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
                info_cache!(
                    "Removed cached track asset ({}) for {}.",
                    audio_format,
                    track_assets_mut.source_file_signature.path.display()
                );
            }
            Some(false) => keep_container = true,
            None => ()
        }
    }
    
    if keep_container {
        track_assets_mut.persist_to_cache(&build.cache_dir);
    } else {
        util::remove_file(&track_assets_mut.manifest_path(&build.cache_dir));
    }
}

pub fn report_stale(cache: &Cache, catalog: &Catalog) {
    let mut num_unused = 0;
    let mut unused_bytesize = 0;
    
    for assets in &cache.archives {
        report_stale_archive_assets(assets, &mut num_unused, &mut unused_bytesize);
    }

    for assets in &cache.images {
        report_stale_image_assets(assets, &mut num_unused, &mut unused_bytesize);
    }
    
    for assets in &cache.tracks {
        report_stale_track_assets(assets, &mut num_unused, &mut unused_bytesize);
    }
    
    for release in &catalog.releases {
        let release_ref = release.borrow();

        report_stale_archive_assets(&release_ref.archive_assets, &mut num_unused, &mut unused_bytesize);

        if let Some(image) = &release_ref.cover {
            report_stale_image_assets(&image.borrow().assets, &mut num_unused, &mut unused_bytesize);
        }
        
        for track in &release_ref.tracks {
            report_stale_track_assets(&track.assets, &mut num_unused, &mut unused_bytesize);
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

pub fn report_stale_archive_assets(
    archive_assets: &Rc<RefCell<ArchiveAssets>>,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
        if let Some(filesize_bytes) = archive_assets
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

pub fn report_stale_image_assets(
    assets: &Rc<RefCell<ImageAssets>>,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    let assets_ref = assets.borrow();

    let mut report = |asset_option: &Option<Asset>| {
        if let Some(filesize_bytes) = asset_option
            .as_ref()
            .filter(|asset| asset.marked_stale.is_some())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    };

    report(&assets_ref.background);
    report(&assets_ref.feed);

    if let Some(assets) = assets_ref.artist
        .as_ref()
        .filter(|assets| assets.marked_stale.is_some()) {
        for asset in &assets.all() {
            *num_unused += 1;
            *unused_bytesize += asset.filesize_bytes;
        }
    }

    if let Some(assets) = assets_ref.cover
        .as_ref()
        .filter(|assets| assets.marked_stale.is_some()) {
        for asset in &assets.all() {
            *num_unused += 1;
            *unused_bytesize += asset.filesize_bytes;
        }
    }
}

pub fn report_stale_track_assets(
    assets: &Rc<RefCell<TrackAssets>>,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
        if let Some(filesize_bytes) = assets
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
        for assets in self.archives.iter_mut() {
            assets.borrow_mut().mark_all_stale(timestamp);
        }

        for assets in self.images.iter_mut() {
            assets.borrow_mut().mark_all_stale(timestamp);
        }
        
        for assets in self.tracks.iter_mut() {
            assets.borrow_mut().mark_all_stale(timestamp);
        }
    }

    fn new() -> Cache {
        Cache {
            archives: Vec::new(),
            artifact_registry: HashMap::new(),
            images: Vec::new(),
            tracks: Vec::new()
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
        cache.retrieve_tracks(cache_dir);

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
                    if let Some(mut archive_assets) = ArchiveAssets::deserialize_cached(&dir_entry.path()) {
                        let mut assets_present = false;

                        for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
                            let asset_option = archive_assets.get_mut(download_format);
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
                            self.archives.push(Rc::new(RefCell::new(archive_assets)));
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
                    if let Some(mut assets) = ImageAssets::deserialize_cached(&dir_entry.path()) {
                        if let Some(artist_assets) = assets.artist.as_mut() {
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

                                    assets.artist = None;
                                    break;
                                }
                            }
                        }


                        if let Some(background_image) = &assets.background {
                            if let Some(usage_counter) = self.artifact_registry.get_mut(&background_image.filename) {
                                *usage_counter += 1;
                            } else {
                                assets.background = None;
                            }
                        }

                        if let Some(cover_assets) = assets.cover.as_mut() {
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

                                    assets.cover = None;
                                    break;
                                }
                            }
                        }

                        if let Some(feed_image) = &assets.feed {
                            if let Some(usage_counter) = self.artifact_registry.get_mut(&feed_image.filename) {
                                *usage_counter += 1;
                            } else {
                                assets.feed = None;
                            }
                        }

                        if assets.artist.is_some() ||
                            assets.background.is_some() ||
                            assets.cover.is_some() ||
                            assets.feed.is_some() {
                            self.images.push(Rc::new(RefCell::new(assets)));
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
    
    fn retrieve_tracks(&mut self, cache_dir: &Path) {  
        if let Ok(dir_entries) = cache_dir.join(Cache::TRACK_MANIFESTS_DIR).read_dir() {
            for dir_entry_result in dir_entries {
                if let Ok(dir_entry) = dir_entry_result {
                    if let Some(mut track_assets) = TrackAssets::deserialize_cached(&dir_entry.path()) {
                        let mut dead_references_removed = false;

                        for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
                            let asset_option = track_assets.get_mut(audio_format);
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
                            track_assets.persist_to_cache(cache_dir);
                        }

                        // With archives and images we would throw away
                        // serialized metadata here if no actual cached files are
                        // present. However for a track the cached metadata
                        // contains AudioMeta, which is expensively computed,
                        // therefore we always retain the serialized metadata
                        // and only remove it if cache optimization calls for
                        // it.

                        self.tracks.push(Rc::new(RefCell::new(track_assets)));
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
    /// (whether we have the image and tracks in all required formats is not
    /// yet relevant at this point). If yes they are returned, otherwise
    /// created (but not yet computed).
    pub fn get_or_create_archive_assets(
        &mut self,
        cover: &Option<Rc<RefCell<Image>>>,
        tracks: &[Track],
        extras: &[Extra]
    ) -> Rc<RefCell<ArchiveAssets>> {
        match self.archives
            .iter()
            .find(|assets| {
                let assets_ref = assets.borrow();

                if let Some(cover) = cover {
                    if assets_ref.cover_source_file_signature.as_ref() !=
                       Some(&cover.borrow().assets.borrow().source_file_signature) {
                        return false;
                    }
                } else if assets_ref.cover_source_file_signature.is_some() {
                    return false;
                }

                if extras.len() != assets_ref.extra_source_file_signatures.len() {
                    return false;
                }

                for extra in extras {
                    if !assets_ref.extra_source_file_signatures.contains(&extra.source_file_signature) {
                        return false;
                    }
                }

                tracks
                    .iter()
                    .zip(assets_ref.track_source_file_signatures.iter())
                    .all(|(track, source_file_signature)| {
                        &track.assets.borrow().source_file_signature == source_file_signature
                    })
            }) {
            Some(assets) => assets.clone(),
            None => {
                let track_source_file_signatures = tracks
                    .iter()
                    .map(|track| track.assets.borrow().source_file_signature.clone())
                    .collect();

                let assets = Rc::new(RefCell::new(ArchiveAssets::new(
                    cover.as_ref().map(|cover| cover.borrow().assets.borrow().source_file_signature.clone()),
                    track_source_file_signatures,
                    extras.iter().map(|extra| extra.source_file_signature.clone()).collect()
                )));
                self.archives.push(assets.clone());
                assets
            }
        }
    }

    pub fn get_or_create_image_assets(
        &mut self,
        build: &Build,
        source_path: &Path
    ) -> Rc<RefCell<ImageAssets>> {
        let source_file_signature = SourceFileSignature::new(build, source_path);

        match self.images.iter().find(|assets|
            assets.borrow().source_file_signature == source_file_signature
        ) {
            Some(assets) => assets.clone(),
            None => {
                let assets = Rc::new(RefCell::new(ImageAssets::new(source_file_signature)));
                self.images.push(assets.clone());
                assets
            }
        }
    }
    
    pub fn get_or_create_track_assets(
        &mut self,
        build: &Build,
        source_path:&Path,
        extension: &str
    ) -> Rc<RefCell<TrackAssets>> {
        let source_file_signature = SourceFileSignature::new(build, source_path);
        
        match self.tracks.iter().find(|assets|
            assets.borrow().source_file_signature == source_file_signature
        ) {
            Some(assets) => assets.clone(),
            None => {
                let source_meta = AudioMeta::extract(&build.catalog_dir.join(source_path), extension);
                let track_assets = TrackAssets::new(source_file_signature, source_meta);

                // We already extracted the AudioMeta for this track - which is costly
                // to compute - therefore we already persist the track assets to the 
                // cache, even if there are no transcoded artifacts yet.
                track_assets.persist_to_cache(&build.cache_dir);

                let track_assets_rc = Rc::new(RefCell::new(track_assets));
                self.tracks.push(track_assets_rc.clone());
                track_assets_rc
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
            hash: String::new(), // TODO: Implement somewhere, somehow (maybe on demand rather?)
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            path: path.to_path_buf(),
            size: metadata.len()
        }
    }
}