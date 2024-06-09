// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::fs;
use std::mem;
use std::path::Path;

use chrono::{DateTime, Utc};

use crate::{
    Archives,
    ArchivesRc,
    Asset,
    AudioFormat,
    AudioMeta,
    Build,
    DescribedImage,
    DownloadFormat,
    Extra,
    Image,
    ImageRc,
    SourceFileSignature,
    Track,
    Transcodes,
    TranscodesRc,
    util
};

/// This is the name of an empty file created by faircamp in the root of the
/// cache directory. When the entire cache layout (or critical implementation
/// details) change, the cache version can be updated, prompting a complete cache
/// purge and rebuild for site operators picking up the new version of
/// faircamp. More granular cache data invalidation can also be performed at the
/// manifest level, by updating the version included in the `CACHE_SERIALIZATION_KEY`
/// constant of either of [Archives], [Image] and [Transcodes]. This latter
/// mechanism should always be preferred, as cache rebuilds are expensive for users!
const CACHE_VERSION_MARKER: &str = "cache1.marker";

#[derive(Clone, Debug)]
pub struct Cache {
    pub archives: Vec<ArchivesRc>,
    /// We register all assets found in the cache here. During cache retrieval
    /// those assets that are used are tagged as such. After cache retrieval
    /// all assets not tagged as used are considered orphaned and removed.
    assets: HashMap<String, bool>,
    /// We register all manifests found in the cache here. Afterwards we iterate
    /// through all of them, using those with a known manifest extension
    /// (e.g. ".image1.bincode") as entry points for retrieving metadata for
    /// archives, images and transcodes.
    /// Assets referenced in the manifests that do not appear in
    /// `assets` mean that the asset reference is corrupt (we then remove
    /// the reference). The other way around, every time we find an asset
    /// we set its `used` flag (the value in the HashMap) to `true`.
    /// At the end of the cache retrieval process we know that all
    /// files in the registry that haven't been tagged as used are orphaned
    /// and can therefore be removed.
    manifests: Vec<String>,
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

fn optimize_archives(archives: &mut ArchivesRc, build: &Build) {
    let mut archives_mut = archives.borrow_mut();
    let mut keep_container = false;

    for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
        let cached_format = archives_mut.get_mut(download_format);

        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                let _ = fs::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
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
        let _ = fs::remove_file(&archives_mut.manifest_path(&build.cache_dir));
    }
}

fn optimize_image(image: &ImageRc, build: &Build) {
    let mut image_mut = image.borrow_mut();
    let mut keep_container = false;

    let path = image_mut.source_file_signature.path.display().to_string();

    {
        let mut optimize = |asset_option: &mut Option<Asset>, format: &str, path: &str| {
            match asset_option.as_ref().map(|asset| asset.obsolete(build)) {
                Some(true) => {
                    let _ = fs::remove_file(&build.cache_dir.join(asset_option.take().unwrap().filename));
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
                    let _ = fs::remove_file(&build.cache_dir.join(&asset.filename));
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
                    let _ = fs::remove_file(&build.cache_dir.join(&asset.filename));
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
        let _ = fs::remove_file(&image_mut.manifest_path(&build.cache_dir));
    }
}

fn optimize_transcodes(transcodes: &mut TranscodesRc, build: &Build) {
    let mut transcodes_mut = transcodes.borrow_mut();
    let mut keep_container = false;

    for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
        let cached_format = transcodes_mut.get_mut(audio_format);

        match cached_format.as_ref().map(|asset| asset.obsolete(build)) {
            Some(true) => {
                let _ = fs::remove_file(&build.cache_dir.join(cached_format.take().unwrap().filename));
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
        let _ = fs::remove_file(&transcodes_mut.manifest_path(&build.cache_dir));
    }
}

fn report_stale_archives(
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

fn report_stale_images(
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

fn report_stale_transcodes(
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
            assets: HashMap::new(),
            images: Vec::new(),
            manifests: Vec::new(),
            transcodes: Vec::new()
        }
    }

    pub fn optimize_cache(&mut self, build: &Build) {
        for archives in self.archives.iter_mut() {
            optimize_archives(archives, build);
        }

        for image in self.images.iter_mut() {
            optimize_image(image, build);
        }
        
        for transcodes in self.transcodes.iter_mut() {
            optimize_transcodes(transcodes, build);
        }
    }

    fn process_manifests(&mut self, cache_dir: &Path) {
        for file_name in mem::take(&mut self.manifests) {
            if file_name.ends_with(&format!(".{}.bincode", Archives::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_archives(cache_dir, &file_name);
            } else if file_name.ends_with(&format!(".{}.bincode", Image::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_image(cache_dir, &file_name);
            } else if file_name.ends_with(&format!(".{}.bincode", Transcodes::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_transcodes(cache_dir, &file_name);
            } else {
                info!(
                    "Removing incompatible cache manifest {} - it was probably created with a different version of faircamp.",
                    file_name
                );
                let _ = fs::remove_file(cache_dir.join(&file_name));
            }
        }
    }

    fn register_files(&mut self, cache_dir: &Path) {
        let dir_entries = match cache_dir.read_dir() {
            Ok(dir_entries) => dir_entries,
            Err(err) => panic!("Could not read cache_dir ({err})")
        };

        for dir_entry_result in dir_entries {
            if let Ok(dir_entry) = dir_entry_result {
                if let Ok(file_type) = dir_entry.file_type() {
                    let file_name = dir_entry.file_name().to_str().unwrap().to_string();

                    if file_type.is_dir() {
                        info!(
                            "Removing incompatible cache directory {} - it was probably created with a different version of faircamp.",
                            file_name
                        );
                        let _ = fs::remove_dir_all(dir_entry.path());
                    } else if file_type.is_file() {

                        if file_name.ends_with(".bincode") {
                            self.manifests.push(file_name);
                        } else if file_name != CACHE_VERSION_MARKER {
                            self.assets.insert(file_name, false);
                        }
                    } else {
                        info!("Ignoring unsupported cache file {} of type {:?}", file_name, file_type);
                    }
                }
            }
        }
    }

    fn remove_orphaned_assets(&mut self, cache_dir: &Path) {
        for (file_name, used) in self.assets.drain() {
            if !used {
                info!(
                    "Removing orphaned cache asset ({}) - it was probably created with a different version of faircamp.",
                    file_name
                );
                let _ = fs::remove_file(&cache_dir.join(file_name));
            }
        }
    }

    pub fn report_stale(&self) {
        let mut num_unused = 0;
        let mut unused_bytesize = 0;

        for archives in &self.archives {
            report_stale_archives(archives, &mut num_unused, &mut unused_bytesize);
        }

        for image in &self.images {
            report_stale_images(image, &mut num_unused, &mut unused_bytesize);
        }

        for transcodes in &self.transcodes {
            report_stale_transcodes(transcodes, &mut num_unused, &mut unused_bytesize);
        }

        if num_unused > 0 {
            info_cache!(
                "{} cached assets were identified as obsolete - you can run 'faircamp --optimize-cache' to to remove them and reclaim {} of disk space.",
                num_unused,
                util::format_bytes(unused_bytesize)
            );
        } else {
            info_cache!("No cached assets identified as obsolete.");
        }
    }

    pub fn retrieve(cache_dir: &Path) -> Cache {
        let mut cache = Cache::new();

        let version_marker_file = cache_dir.join(CACHE_VERSION_MARKER);

        if !version_marker_file.exists() {
            if cache_dir.exists() {
                info!("Existing cache data is in an incompatible format (from a different faircamp version), the cache will be purged and regenerated.");
                util::ensure_empty_dir(cache_dir);
            } else {
                util::ensure_dir(cache_dir);
            }
            fs::write(version_marker_file, "").unwrap();
        }

        cache.register_files(cache_dir);
        cache.process_manifests(cache_dir);
        cache.remove_orphaned_assets(cache_dir);

        cache
    }

    fn retrieve_archives(&mut self, cache_dir: &Path, file_name: &str) {
        let manifest_path = cache_dir.join(file_name);

        if let Some(mut archives) = Archives::deserialize_cached(&manifest_path) {
            let mut assets_present = false;
            let mut dead_references_removed = false;

            for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
                let asset_option = archives.get_mut(download_format);
                if let Some(asset) = &asset_option {
                    if let Some(used) = self.assets.get_mut(&asset.filename) {
                        assets_present = true;
                        *used = true;
                    } else {
                        asset_option.take();
                        dead_references_removed = true;
                    }
                }
            }

            if assets_present {
                if dead_references_removed {
                    // Persist corrections so we don't have to re-apply them next time around
                    archives.persist_to_cache(cache_dir);
                }

                self.archives.push(ArchivesRc::new(archives));
            } else {
                // No single cached asset present, we throw away the manifest
                let _ = fs::remove_file(&manifest_path);
            }
        } else {
            info!(
                "Removing incompatible archives cache manifest ({}) - it was probably created with a different version of faircamp.",
                &manifest_path.display()
            );
            let _ = fs::remove_file(&manifest_path);
        }
    }

    fn retrieve_image(&mut self, cache_dir: &Path, file_name: &str) {
        let manifest_path = cache_dir.join(file_name);

        if let Some(mut image_mut) = Image::deserialize_cached(&manifest_path) {
            let mut dead_references_removed = false;

            if let Some(artist_assets) = image_mut.artist_assets.as_mut() {
                let all_assets = artist_assets.all();

                if all_assets.iter().all(|asset| self.assets.contains_key(&asset.filename)) {
                    // All asset references have been verified, mark all as used
                    for asset in all_assets.iter() {
                        *self.assets.get_mut(&asset.filename).unwrap() = true;
                    }
                } else {
                    // If a single artist asset is in a corrupt state (cached file missing)
                    // we drop all artist assets, letting them become orphaned so the cache
                    // removes them afterwards.
                    image_mut.artist_assets = None;
                    dead_references_removed = true;
                }
            }

            if let Some(background_asset) = &image_mut.background_asset {
                if let Some(used) = self.assets.get_mut(&background_asset.filename) {
                    *used = true;
                } else {
                    image_mut.background_asset = None;
                    dead_references_removed = true;
                }
            }

            if let Some(cover_assets) = image_mut.cover_assets.as_mut() {
                let all_assets = cover_assets.all();

                if all_assets.iter().all(|asset| self.assets.contains_key(&asset.filename)) {
                    // All asset references have been verified, mark all as used
                    for asset in all_assets.iter() {
                        *self.assets.get_mut(&asset.filename).unwrap() = true;
                    }
                } else {
                    // If a single cover asset is in a corrupt state (cached file missing)
                    // we drop all cover assets, letting them become orphaned so the cache
                    // removes them afterwards.
                    image_mut.artist_assets = None;
                    dead_references_removed = true;
                }
            }

            if let Some(feed_asset) = &image_mut.feed_asset {
                if let Some(used) = self.assets.get_mut(&feed_asset.filename) {
                    *used = true;
                } else {
                    image_mut.feed_asset = None;
                    dead_references_removed = true;
                }
            }

            if image_mut.artist_assets.is_some() ||
                image_mut.background_asset.is_some() ||
                image_mut.cover_assets.is_some() ||
                image_mut.feed_asset.is_some() {
                if dead_references_removed {
                    // Persist corrections so we don't have to re-apply them next time around
                    image_mut.persist_to_cache(cache_dir);
                }

                self.images.push(ImageRc::new(image_mut));
            } else {
                // No single cached asset present, we throw away the manifest
                let _ = fs::remove_file(&manifest_path);
            }
        } else {
            info!(
                "Removing incompatible image cache manifest ({}) - it was probably created with a different version of faircamp.",
                &manifest_path.display()
            );
            let _ = fs::remove_file(&manifest_path);
        }
    }
    
    fn retrieve_transcodes(&mut self, cache_dir: &Path, file_name: &str) {
        let manifest_path = cache_dir.join(file_name);

        if let Some(mut transcodes) = Transcodes::deserialize_cached(&manifest_path) {
            let mut dead_references_removed = false;

            for audio_format in AudioFormat::ALL_AUDIO_FORMATS {
                let asset_option = transcodes.get_mut(audio_format);
                if let Some(asset) = &asset_option {
                    if let Some(used) = self.assets.get_mut(&asset.filename) {
                        *used = true;
                    } else {
                        asset_option.take();
                        dead_references_removed = true;
                    }
                }
            }

            if dead_references_removed {
                // Persist corrections so we don't have to re-apply them next time around
                transcodes.persist_to_cache(cache_dir);
            }

            // With archives and images we would throw away
            // the manifest here if no actual cached assets are
            // present. However for a track the cached metadata
            // contains AudioMeta, which is expensively computed,
            // therefore we always retain the manifest and only
            // remove it if cache optimization calls for it.

            self.transcodes.push(TranscodesRc::new(transcodes));
        } else {
            info!(
                "Removing incompatible transcodes cache manifest ({}) - it was probably created with a different version of faircamp.",
                &manifest_path.display()
            );
            let _ = fs::remove_file(&manifest_path);
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
                let image = Image::new(source_file_signature);
                let image_rc = ImageRc::new(image);
                self.images.push(image_rc.clone());
                image_rc
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
