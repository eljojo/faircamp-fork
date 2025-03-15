// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;
use std::path::Path;

use chrono::{DateTime, Duration, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::{
    Archives,
    ArchivesRc,
    Asset,
    AudioMeta,
    Build,
    CoverGenerator,
    FileMeta,
    Image,
    ImageRc,
    ImageRcView,
    ProceduralCover,
    ProceduralCoverRc,
    Release,
    SourceHash,
    Transcodes,
    TranscodesRc,
    TranscodesRcView,
    util
};
use crate::util::string_from_os;

/// This is the name of an empty file created by faircamp in the root of the
/// cache directory. When the entire cache layout (or critical implementation
/// details) change, the cache version can be updated, prompting a complete cache
/// purge and rebuild for site operators picking up the new version of
/// faircamp. More granular cache data invalidation can also be performed at the
/// manifest level, by updating the version included in the `CACHE_SERIALIZATION_KEY`
/// constant of either of [Archives], [Image] and [Transcodes]. This latter
/// mechanism should always be preferred, as cache rebuilds are expensive for users!
const CACHE_VERSION_MARKER: &str = "cache1.marker";

#[derive(Debug)]
pub struct Cache {
    pub archives: Vec<ArchivesRc>,
    /// We register all assets found in the cache here. During cache retrieval
    /// those assets that are used are tagged as such. After cache retrieval
    /// all assets not tagged as used are considered orphaned and removed.
    assets: HashMap<String, bool>,
    pub images: Vec<ImageRc>,
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
    pub optimization: CacheOptimization,
    pub procedural_covers: Vec<ProceduralCoverRc>,
    pub transcodes: Vec<TranscodesRc>
}

#[derive(Debug, PartialEq)]
pub enum CacheOptimization {
    Default,
    Delayed,
    Immediate,
    Manual,
    Wipe
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct View {
    pub file_meta: FileMeta,
    marked_stale: Option<DateTime<Utc>>
}

fn recompute_hash(build: &Build, views: &[View]) -> Option<SourceHash> {
    for view in views {
        if view.exists(build) {
            info!(
                "Recomputing hash for {} with new algorithm.",
                &view.file_meta.path.display()
            );
            return Some(SourceHash::new(&build.catalog_dir.join(&view.file_meta.path)));
        }
    }

    None
}

fn report_stale_archives(
    archives: &ArchivesRc,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    for archive in &archives.borrow().formats {
        if archive.asset.is_stale() {
            *num_unused += 1;
            *unused_bytesize += archive.asset.filesize_bytes;
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
            .filter(|asset| asset.is_stale())
            .map(|asset| asset.filesize_bytes) {
            *num_unused += 1;
            *unused_bytesize += filesize_bytes;
        }
    };

    report(&image_ref.background_asset);

    if let Some(filesize_bytes) = &image_ref.feed_asset
        .as_ref()
        .filter(|asset| asset.is_stale())
        .map(|asset| asset.filesize_bytes) {
        *num_unused += 1;
        *unused_bytesize += filesize_bytes;
    }

    if let Some(assets) = image_ref.artist_assets
        .as_ref()
        .filter(|assets| assets.is_stale()) {
        for asset in &assets.all() {
            *num_unused += 1;
            *unused_bytesize += asset.filesize_bytes;
        }
    }

    if let Some(assets) = image_ref.cover_assets
        .as_ref()
        .filter(|assets| assets.is_stale()) {
        for asset in &assets.all() {
            *num_unused += 1;
            *unused_bytesize += asset.filesize_bytes;
        }
    }
}

fn report_stale_procedural_cover(
    procedural_cover: &ProceduralCoverRc,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    let procedural_cover_ref = procedural_cover.borrow();

    if procedural_cover_ref.is_stale() {
        *num_unused += 1;
        *unused_bytesize += procedural_cover_ref.asset_120.filesize_bytes;
        *unused_bytesize += procedural_cover_ref.asset_240.filesize_bytes;
        *unused_bytesize += procedural_cover_ref.asset_480.filesize_bytes;
        *unused_bytesize += procedural_cover_ref.asset_720.filesize_bytes;
    }
}

fn report_stale_transcodes(
    transcodes: &TranscodesRc,
    num_unused: &mut u32,
    unused_bytesize: &mut u64
) {
    for transcode in &transcodes.borrow().formats {
        if transcode.asset.is_stale() {
            *num_unused += 1;
            *unused_bytesize += transcode.asset.filesize_bytes;
        }
    }
}

impl Cache {
    /// Based on optimization strategy this does varying things:
    /// - Either it completely wipes the cache after a build
    /// - In any other case it always goes through all cached data and
    ///   repersists the manifests to the cache (note here that this happens
    ///   after the build) to ensure that the `marked_stale` fields get updated
    ///   on disk, as those might change during each build when assets are not
    ///   used.
    /// - For some cache strategies and depending on the time since an asset
    ///   was marked stale, assets are removed in this pass when they are
    ///   unused and considered obsolete
    /// - Lastly, with manual cache optimization a report is printed about all
    ///   currently obsolete assets, so manual action can be taken if desired
    pub fn maintain(&mut self, build: &Build) {
        if self.optimization == CacheOptimization::Wipe {
            let _ = fs::remove_dir_all(&build.cache_dir);
            info_cache!("Wiped cache");
            return;
        }

        for archives in &self.archives {
            self.maintain_archives(archives, build);
        }

        for image in &self.images {
            self.maintain_image(image, build);
        }

        for procedural_cover in &self.procedural_covers {
            self.maintain_procedural_cover(procedural_cover, build);
        }

        for transcodes in &self.transcodes {
            self.maintain_transcodes(transcodes, build);
        }

        if self.optimization == CacheOptimization::Manual {
            self.report_stale();
        }
    }

    fn maintain_archives(&self, archives: &ArchivesRc, build: &Build) {
        let mut archives_mut = archives.borrow_mut();

        if archives_mut.formats.iter().any(|archive| self.obsolete(build, &archive.asset.marked_stale)) {
            let signature = archives_mut.signature;

            archives_mut.formats.retain_mut(|archive| {
                if self.obsolete(build, &archive.asset.marked_stale) {
                    let _ = fs::remove_file(build.cache_dir.join(&archive.asset.filename));
                    info_cache!(
                        "Removed cached archive ({}) with signature {}.",
                        archive.format,
                        signature
                    );

                    false
                } else {
                    true
                }
            });
        }

        if archives_mut.formats.is_empty() {
            let _ = fs::remove_file(archives_mut.manifest_path(&build.cache_dir));
        } else {
            archives_mut.persist_to_cache(&build.cache_dir);
        }
    }

    fn maintain_image(&self, image: &ImageRc, build: &Build) {
        let mut image_mut = image.borrow_mut();
        let mut keep_container = false;

        image_mut.views.retain(|view| {
            if self.obsolete(build, &view.marked_stale) {
                info_cache!(
                    "Removed expired cache view for {}.",
                    view.file_meta.path.display()
                );
                false
            } else {
                true
            }
        });

        let views_context = if image_mut.views.is_empty() {
            "without views".to_string()
        } else {
            let paths = image_mut.views
                .iter()
                .map(|view| view.file_meta.path.display().to_string())
                .collect::<Vec<String>>()
                .join(", ");

            format!("for {paths}")
        };

        match image_mut.background_asset
            .as_ref()
            .map(|asset| self.obsolete(build, &asset.marked_stale)) {
            Some(true) => {
                let _ = fs::remove_file(build.cache_dir.join(image_mut.background_asset.take().unwrap().filename));
                info_cache!("Removed cached background image asset {}.", views_context);
            }
            Some(false) => keep_container = true,
            None => ()
        }

        match image_mut.feed_asset
            .as_ref()
            .map(|asset| self.obsolete(build, &asset.marked_stale)) {
            Some(true) => {
                let _ = fs::remove_file(build.cache_dir.join(image_mut.feed_asset.take().unwrap().filename));
                info_cache!("Removed cached feed image asset (feed) {}.", views_context);
            }
            Some(false) => keep_container = true,
            None => ()
        }

        match image_mut.artist_assets
            .as_ref()
            .map(|assets| self.obsolete(build, &assets.marked_stale)) {
            Some(true) => {
                for asset in image_mut.artist_assets.take().unwrap().all() {
                    let _ = fs::remove_file(build.cache_dir.join(&asset.filename));
                    info_cache!(
                        "Removed cached image asset ({}) {} {}x{}.",
                        "artist",
                        &views_context,
                        asset.height,
                        asset.width
                    );
                }
            }
            Some(false) => keep_container = true,
            None => ()
        }

        match image_mut.cover_assets
            .as_ref()
            .map(|assets| self.obsolete(build, &assets.marked_stale)) {
            Some(true) => {
                for asset in image_mut.cover_assets.take().unwrap().all() {
                    let _ = fs::remove_file(build.cache_dir.join(&asset.filename));
                    info_cache!(
                        "Removed cached image asset ({}) {} {}x{}.",
                        "cover",
                        &views_context,
                        asset.edge_size,
                        asset.edge_size
                    );
                }
            }
            Some(false) => keep_container = true,
            None => ()
        }

        if keep_container {
            image_mut.persist_to_cache(&build.cache_dir);
        } else {
            let _ = fs::remove_file(image_mut.manifest_path(&build.cache_dir));
        }
    }

    fn maintain_procedural_cover(&self, procedural_cover: &ProceduralCoverRc, build: &Build) {
        let procedural_cover_ref = procedural_cover.borrow();

        if self.obsolete(build, &procedural_cover_ref.marked_stale) {
            let signature = procedural_cover_ref.signature;

            info_cache!("Removed cached procedural cover assets with signature {}.", signature);

            let _ = fs::remove_file(build.cache_dir.join(&procedural_cover_ref.asset_120.filename));
            let _ = fs::remove_file(build.cache_dir.join(&procedural_cover_ref.asset_240.filename));
            let _ = fs::remove_file(build.cache_dir.join(&procedural_cover_ref.asset_480.filename));
            let _ = fs::remove_file(build.cache_dir.join(&procedural_cover_ref.asset_720.filename));
            let _ = fs::remove_file(procedural_cover_ref.manifest_path(&build.cache_dir));
        } else {
            procedural_cover_ref.persist_to_cache(&build.cache_dir);
        }
    }

    fn maintain_transcodes(&self, transcodes: &TranscodesRc, build: &Build) {
        let mut transcodes_mut = transcodes.borrow_mut();

        transcodes_mut.views.retain(|view| {
            if self.obsolete(build, &view.marked_stale) {
                info_cache!(
                    "Removed expired cache view for {}.",
                    view.file_meta.path.display()
                );
                false
            } else {
                true
            }
        });

        if transcodes_mut.formats.iter().any(|transcode| self.obsolete(build, &transcode.asset.marked_stale)) {
             let views_context = if transcodes_mut.views.is_empty() {
                "without views".to_string()
            } else {
                let paths = transcodes_mut.views
                    .iter()
                    .map(|view| view.file_meta.path.display().to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("for {paths}")
            };

            transcodes_mut.formats.retain_mut(|transcode| {
                if self.obsolete(build, &transcode.asset.marked_stale) {

                    let _ = fs::remove_file(build.cache_dir.join(&transcode.asset.filename));
                    info_cache!(
                        "Removed cached transcode ({}) {}.",
                        transcode.format,
                        views_context
                    );

                    false
                } else {
                    true
                }
            });
        }

        if transcodes_mut.formats.is_empty() {
            let _ = fs::remove_file(transcodes_mut.manifest_path(&build.cache_dir));
        } else {
            transcodes_mut.persist_to_cache(&build.cache_dir);
        }
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for archives in self.archives.iter_mut() {
            archives.borrow_mut().mark_all_stale(timestamp);
        }

        for image in self.images.iter_mut() {
            image.borrow_mut().mark_all_stale(timestamp);
        }

        for procedural_cover in self.procedural_covers.iter_mut() {
            procedural_cover.borrow_mut().mark_stale(timestamp);
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
            optimization: CacheOptimization::Default,
            procedural_covers: Vec::new(),
            transcodes: Vec::new()
        }
    }

    /// Gets passed the `marked_stale` option of some asset-like entity ([Asset],
    /// [ArtistAssets], [CoverAssets], [ProceduralCoverAssets]) and based
    /// on cache_optimization and build begin time decides whether that
    /// entity can be considered obsolete (i.e.: removable).
    pub fn obsolete(&self, build: &Build, marked_stale: &Option<DateTime<Utc>>) -> bool {
        match marked_stale {
            Some(date_time) => {
                match &self.optimization {
                    CacheOptimization::Default |
                    CacheOptimization::Delayed =>
                        build.build_begin.signed_duration_since(*date_time) > Duration::hours(24),
                    CacheOptimization::Immediate |
                    CacheOptimization::Wipe => true,
                    CacheOptimization::Manual => false
                }
            },
            None => false
        }
    }

    fn process_manifests(&mut self, build: &Build) {
        for file_name in mem::take(&mut self.manifests) {
            if file_name.ends_with(&format!(".{}.bincode", Archives::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_archives(build, &file_name);
            } else if file_name.ends_with(&format!(".{}.bincode", Image::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_image(build, &file_name);
            } else if file_name.ends_with(&format!(".{}.bincode", ProceduralCover::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_procedural_cover(build, &file_name);
            } else if file_name.ends_with(&format!(".{}.bincode", Transcodes::CACHE_SERIALIZATION_KEY)) {
                self.retrieve_transcodes(build, &file_name);
            } else {
                info!(
                    "Removing incompatible cache manifest {} - it was probably created with a different version of faircamp.",
                    file_name
                );
                let _ = fs::remove_file(build.cache_dir.join(&file_name));
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
                    let file_name = string_from_os(dir_entry.file_name());

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
                    "Removing orphaned cache asset ({}).",
                    file_name
                );
                let _ = fs::remove_file(cache_dir.join(file_name));
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

        for procedural_cover in &self.procedural_covers {
            report_stale_procedural_cover(procedural_cover, &mut num_unused, &mut unused_bytesize);
        }

        for transcodes in &self.transcodes {
            report_stale_transcodes(transcodes, &mut num_unused, &mut unused_bytesize);
        }

        if num_unused > 0 {
            info_cache!(
                "{} cached assets were identified as obsolete - you can run 'faircamp --optimize-cache' to remove them and reclaim {} of disk space.",
                num_unused,
                util::format_bytes(unused_bytesize)
            );
        } else {
            info_cache!("No cached assets identified as obsolete.");
        }
    }

    pub fn retrieve(build: &Build) -> Cache {
        let mut cache = Cache::new();

        let version_marker_file = build.cache_dir.join(CACHE_VERSION_MARKER);

        if !version_marker_file.exists() {
            if build.cache_dir.exists() {
                info!("Existing cache data is in an incompatible format (from a different faircamp version), the cache will be purged and regenerated.");
                util::ensure_empty_dir(&build.cache_dir);
            } else {
                util::ensure_dir_all(&build.cache_dir);
            }
            fs::write(version_marker_file, "").unwrap();
        }

        cache.register_files(&build.cache_dir);
        cache.process_manifests(build);
        cache.remove_orphaned_assets(&build.cache_dir);

        cache
    }

    fn retrieve_archives(&mut self, build: &Build, file_name: &str) {
        let manifest_path = build.cache_dir.join(file_name);

        if let Some(mut archives_mut) = Archives::deserialize_cached(&manifest_path) {
            let mut dead_references_removed = false;

            archives_mut.formats.retain(|archive| {
                if let Some(used) = self.assets.get_mut(&archive.asset.filename) {
                    *used = true;
                    true
                } else {
                    dead_references_removed = true;
                    false
                }
            });

            if !archives_mut.formats.is_empty() {
                if dead_references_removed {
                    // Persist corrections so we don't have to re-apply them next time around
                    archives_mut.persist_to_cache(&build.cache_dir);
                }

                self.archives.push(ArchivesRc::new(archives_mut));
            } else {
                // No single cached asset present, we throw away the manifest
                let _ = fs::remove_file(&manifest_path);
            }
        } else {
            info!(
                "Removing incompatible archives cache manifest ({}) - it was probably created with a different version of faircamp.",
                file_name
            );
            let _ = fs::remove_file(&manifest_path);
        }
    }

    fn retrieve_image(&mut self, build: &Build, file_name: &str) {
        let manifest_path = &build.cache_dir.join(file_name);

        if let Some(mut image_mut) = Image::deserialize_cached(manifest_path) {
            if image_mut.hash.incompatible_version() {
                match recompute_hash(build, &image_mut.views) {
                    Some(hash) => {
                        image_mut.hash = hash;
                    }
                    None => {
                        info!(
                            "Removing cache manifest {} because its hash was incompatible (from a different version of faircamp) and no files were available to recompute it.",
                            file_name
                        );
                        let _ = fs::remove_file(manifest_path);
                        return;
                    }
                }
            }

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
                    image_mut.persist_to_cache(&build.cache_dir);
                }

                self.images.push(ImageRc::retrieved(image_mut));
            } else {
                // No single cached asset present, we throw away the manifest
                let _ = fs::remove_file(manifest_path);
            }
        } else {
            info!(
                "Removing incompatible image cache manifest ({}) - it was probably created with a different version of faircamp.",
                file_name
            );
            let _ = fs::remove_file(manifest_path);
        }
    }

    fn retrieve_procedural_cover(&mut self, build: &Build, file_name: &str) {
        let manifest_path = build.cache_dir.join(file_name);

        if let Some(procedural_cover) = ProceduralCover::deserialize_cached(&manifest_path) {
            if self.assets.contains_key(&procedural_cover.asset_120.filename) &&
                self.assets.contains_key(&procedural_cover.asset_240.filename) &&
                self.assets.contains_key(&procedural_cover.asset_480.filename) &&
                self.assets.contains_key(&procedural_cover.asset_720.filename) {

                // All asset references have been verified, mark all as used
                *self.assets.get_mut(&procedural_cover.asset_120.filename).unwrap() = true;
                *self.assets.get_mut(&procedural_cover.asset_240.filename).unwrap() = true;
                *self.assets.get_mut(&procedural_cover.asset_480.filename).unwrap() = true;
                *self.assets.get_mut(&procedural_cover.asset_720.filename).unwrap() = true;

                self.procedural_covers.push(ProceduralCoverRc::new(procedural_cover));
            } else {
                // If a single procedural cover asset is in a corrupt state
                // (cached file missing) we drop the entire procedural cover
                // by letting all assets become orphaned so the cache removes
                // them afterwards.

                // Throw away the manifest
                let _ = fs::remove_file(&manifest_path);
            }
        } else {
            info!(
                "Removing incompatible procedural cover cache manifest ({}) - it was probably created with a different version of faircamp.",
                file_name
            );
            let _ = fs::remove_file(&manifest_path);
        }
    }

    fn retrieve_transcodes(&mut self, build: &Build, file_name: &str) {
        let manifest_path = build.cache_dir.join(file_name);

        if let Some(mut transcodes_mut) = Transcodes::deserialize_cached(&manifest_path) {
            if transcodes_mut.hash.incompatible_version() {
                match recompute_hash(build, &transcodes_mut.views) {
                    Some(hash) => {
                        transcodes_mut.hash = hash;
                    }
                    None => {
                        info!(
                            "Removing cache manifest {} because its hash was incompatible (from a different version of faircamp) and no files were available to recompute it.",
                            file_name
                        );
                        let _ = fs::remove_file(manifest_path);
                        return;
                    }
                }
            }

            let mut dead_references_removed = false;

            transcodes_mut.formats.retain(|transcode| {
                if let Some(used) = self.assets.get_mut(&transcode.asset.filename) {
                    *used = true;
                    true
                } else {
                    dead_references_removed = true;
                    false
                }
            });

            if dead_references_removed {
                // Persist corrections so we don't have to re-apply them next time around
                transcodes_mut.persist_to_cache(&build.cache_dir);
            }

            // With archives and images we would throw away
            // the manifest here if no actual cached assets are
            // present. However for a track the cached metadata
            // contains AudioMeta, which is expensively computed,
            // therefore we always retain the manifest and only
            // remove it if cache optimization calls for it.

            self.transcodes.push(TranscodesRc::retrieved(transcodes_mut));
        } else {
            info!(
                "Removing incompatible transcodes cache manifest ({}) - it was probably created with a different version of faircamp.",
                file_name
            );
            let _ = fs::remove_file(&manifest_path);
        }
    }

    /// This basically checks "Do we have cached download archives with the
    /// hash signature that uniquely identifies the entire dependency graph
    /// of of the release?" (whether we have the image and transcodes in all
    /// required formats is not yet relevant at this point). If yes they are
    /// returned, otherwise created (but not yet computed).
    pub fn get_or_create_archives(&mut self, signature: u64) -> ArchivesRc {
        for archive in &self.archives {
            if archive.borrow().signature == signature {
                return archive.clone();
            }
        }

        let archive = ArchivesRc::new(Archives::new(signature));
        self.archives.push(archive.clone());
        archive
    }

    pub fn get_or_create_image(
        &mut self,
        build: &Build,
        source_path: &Path
    ) -> ImageRcView {
        let file_meta = FileMeta::new(build, source_path);

        for image in &self.images {
            if image.revive_view(&file_meta) {
                return ImageRcView::new(file_meta, image.clone());
            }
        }

        let hash = SourceHash::new(&build.catalog_dir.join(source_path));

        for image in &self.images {
            if image.matches_hash(&hash) {
                image.add_view(&file_meta);
                return ImageRcView::new(file_meta, image.clone());
            }
        }

        let image = ImageRc::new(file_meta.clone(), hash);
        self.images.push(image.clone());
        ImageRcView::new(file_meta, image)
    }

    /// This basically checks "Do we have a cached procedural cover with the
    /// hash signature that uniquely identifies the procedural cover?"
    pub fn get_or_create_procedural_cover(
        &mut self,
        build: &Build,
        cover_generator: &CoverGenerator,
        max_tracks_in_release: usize,
        release: &Release
    ) -> ProceduralCoverRc {
        // Compute a signature for the needed cover, factoring in all data
        // that is used in the generation of the cover (cover generator,
        // maximum number of tracks on any release in the catalog, peaks of
        // the tracks).
        let mut hasher = DefaultHasher::new();
        cover_generator.hash(&mut hasher);
        max_tracks_in_release.hash(&mut hasher);
        release.theme.base.hash(&mut hasher);
        for track in &release.tracks {
            track.transcodes.borrow().hash.hash(&mut hasher);
        }
        let signature = hasher.finish();

        // If we already have a cached procedural cover matching the signature, return it
        for procedural_cover in &self.procedural_covers {
            if procedural_cover.borrow().signature == signature {
                return procedural_cover.clone();
            }
        }

        // Otherwise generate the cover, persist it to cache and return it
        let procedural_cover = cover_generator.generate(
            build,
            max_tracks_in_release,
            release,
            signature
        );

        let procedural_cover_rc = ProceduralCoverRc::new(procedural_cover);

        procedural_cover_rc.borrow().persist_to_cache(&build.cache_dir);

        self.procedural_covers.push(procedural_cover_rc.clone());
        procedural_cover_rc
    }

    /// Obtain transcodes by either reviving a view or computing a new
    /// transcodes instance from scratch. This may fail when we create
    /// a new instance and the decoding fails somehow.
    pub fn get_or_create_transcodes(
        &mut self,
        build: &Build,
        source_path: &Path,
        extension: &str
    ) -> Result<TranscodesRcView, String> {
        let file_meta = FileMeta::new(build, source_path);

        for transcodes in &self.transcodes {
            if transcodes.revive_view(&file_meta) {
                return Ok(TranscodesRcView::new(file_meta, transcodes.clone()));
            }
        }

        let hash = SourceHash::new(&build.catalog_dir.join(source_path));

        for transcodes in &self.transcodes {
            if transcodes.matches_hash(&hash) {
                transcodes.add_view(&file_meta);
                return Ok(TranscodesRcView::new(file_meta, transcodes.clone()));
            }
        }

        let source_meta = match AudioMeta::extract(build, extension, source_path) {
            Ok(audio_meta) => audio_meta,
            Err(err) => return Err(err)
        };

        let transcodes = TranscodesRc::new(file_meta.clone(), hash, source_meta);

        transcodes.borrow().persist_to_cache(&build.cache_dir);

        self.transcodes.push(transcodes.clone());

        Ok(TranscodesRcView::new(file_meta, transcodes))
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

impl View {
    /// Check whether the file path in the view still exists and the file's
    /// metadata still 1:1 matches what is stored in the view.
    pub fn exists(&self, build: &Build) -> bool {
        if build.catalog_dir.join(&self.file_meta.path).exists() {
            let file_meta_now = FileMeta::new(build, &self.file_meta.path);

            if file_meta_now == self.file_meta {
                return true;
            }
        }

        false
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    pub fn new(file_meta: FileMeta) -> View {
        View {
            file_meta,
            marked_stale: None
        }
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}
