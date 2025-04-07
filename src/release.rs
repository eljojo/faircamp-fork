// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::ops::RangeFrom;

use chrono::NaiveDate;
use sanitize_filename::sanitize;
use serde_derive::{Serialize, Deserialize};
use zip::{CompressionMethod, ZipWriter};
use zip::write::SimpleFileOptions;

use crate::M3U_PLAYLIST_FILENAME;
use crate::{
    Archive,
    ArchivesRc,
    ArtistRc,
    Asset,
    AssetIntent,
    Build,
    Cache,
    Catalog,
    DescribedImage,
    DownloadAccess,
    DownloadFormat,
    ExtraDownloads,
    FileMeta,
    HtmlAndStripped,
    Link,
    Permalink,
    ProceduralCoverRc,
    TagMapping,
    Theme,
    Track,
    TrackNumbering
};
use crate::{m3u, render, util};
use crate::util::{deduplicate_filename, generic_hash};

/// An unbounded iterator returning track numbers (1, 2, 3, ..) which
/// we generally use with ".zip(TRACK_NUMBERS)" to augment an iteration
/// of tracks with track numbers. We use this instead of a simple ".zip(1..)"
/// statement to ensure that we always iterate over usize rather than some
/// unknown type that rust elides based on local context. We care about type
/// stability here specifically because the track number is often used to
/// compute hashes, and unstable types would mean unstable hashes.
pub const TRACK_NUMBERS: RangeFrom<usize> = RangeFrom { start: 1 };

/// If candidate_filename is not among used_filenames, simply returns candidate_filename.
/// Otherwise makes modifications to candidate_filename until it does not anymore collide
/// with an already used filename.
fn deduplicate_extra_filename(candidate_filename: &str, used_filenames: &HashSet<String>) -> String {
    let mut filename = candidate_filename.to_string();

    while used_filenames.contains(&filename) {
        filename = deduplicate_filename(&filename);
    }

    filename
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Extra {
    pub file_meta: FileMeta,
    pub sanitized_filename: String
}

#[derive(Debug)]
pub struct Release {
    /// This is an option because of delayed initialization - at the point where
    /// we create the [Release] we cannot obtain this yet (we still need to map
    /// the artists and the signature that we need to compute to obtain the right
    /// archives depends on [Release] itself). Eventually this is guaranteed to
    /// exist though, in the later phases of the build process.
    pub archives: Option<ArchivesRc>,
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download asset filenames.
    pub asset_basename: Option<String>,
    pub copy_link: bool,
    pub cover: Option<DescribedImage>,
    pub date: Option<NaiveDate>,
    pub download_access: DownloadAccess,
    pub download_formats: Vec<DownloadFormat>,
    pub embedding: bool,
    pub extra_downloads: ExtraDownloads,
    /// Additional files that are included in the download archive,
    /// such as additional images, liner notes, etc.
    pub extras: Vec<Extra>,
    pub links: Vec<Link>,
    /// The artists that are the principal authors of a release ("Album Artist" in tag lingo)
    pub main_artists: Vec<ArtistRc>,
    /// The order in which we encounter artists and releases when reading the
    /// catalog is arbitrary, hence when we read a release, we might not yet
    /// have read metadata that tells us to which artist(s) it needs to be
    /// mapped. `main_artists_to_map` is an intermediate, name-based mapping
    /// we store until the entire catalog is read. After that point, we
    /// use it to build the final mapping in `main_artists`, then dispose of it.
    pub main_artists_to_map: Vec<String>,
    /// Whether an m3u playlist should be generated and provided on the release page
    pub m3u: bool,
    pub more: Option<HtmlAndStripped>,
    /// Optional custom label for the button that (by default) says "More" on the
    /// release page and points to additional long-form content for the release.
    pub more_label: Option<String>,
    pub permalink: Permalink,
    /// Lazily generated when there is no regular cover
    pub procedural_cover: Option<ProceduralCoverRc>,
    /// Relative path of the release directory in the catalog directory.
    /// This is used to augment permalink conflict errors with additional
    /// info for resolving the conflict.
    pub source_dir: PathBuf,
    /// Whether players should offer speed controls for this release
    pub speed_controls: bool,
    /// Artists that appear on the release as collaborators, features, etc.
    pub support_artists: Vec<ArtistRc>,
    /// See `main_artists_to_map` for what this does
    pub support_artists_to_map: Vec<String>,
    pub synopsis: Option<String>,
    pub theme: Theme,
    pub title: String,
    pub track_numbering: TrackNumbering,
    /// The order of tracks (and derived from this the track numbers) are
    /// authoritative, i.e. when the release is constructed, tracks are
    /// passed in the order that has been determined by track number metadata
    /// and/or alphabetical sorting of filenames as a fallback. When the release
    /// input files include both files with track number metadata and without,
    /// and/or when the track numbers don't start at 1 and/or don't monotonically
    /// increase in steps of 1 some unexpected or random track ordering and numbering
    /// might happen, but this is somewhat impossible to avoid.
    pub tracks: Vec<Track>,
    pub unlisted: bool
}

#[derive(Clone, Debug)]
pub struct ReleaseRc {
    release: Rc<RefCell<Release>>,
}

impl Hash for Extra {
    /// When we compute the hash of an extra we specifically don't factor in
    /// its source path (file_meta.path). If the file has the same size,
    /// modification date and target filename (sanitized_filename, as it is
    /// used for a release archive or separate extra download asset) we
    /// practically consider it the same file in two builds.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_meta.modified.hash(state);
        self.file_meta.size.hash(state);
        self.sanitized_filename.hash(state);
    }
}

impl Extra {
    pub fn new(file_meta: FileMeta) -> Extra {
        let sanitized_filename = sanitize(file_meta.path.file_name().unwrap().to_string_lossy());

        Extra {
            file_meta,
            sanitized_filename
        }
    }
}

impl Release {
    /// Returns - if available - the file name of the release cover,
    /// without any prefixing (i.e. in the context of the release directory)
    pub fn cover_160_filename(&self) -> Option<String> {
        self.cover
            .as_ref()
            .map(|described_image| described_image.borrow().cover_160_filename_unchecked())
    }

    /// It is critical that every last detail of this hashing implementation
    /// stays the same - unless explicitly needed of course - because this signature
    /// makes or breaks finding cached archives.
    pub fn get_or_create_release_archives(&mut self, cache: &mut Cache) {
        match &self.download_access {
            DownloadAccess::Code { .. } |
            DownloadAccess::Free |
            DownloadAccess::Paycurtain { .. } => {
                if self.download_assets_available() {
                    let mut hasher = DefaultHasher::new();

                    // TODO: Consider further if there are aspects of the dependency graph missing
                    //       that need to be included in the hash signature.
                    // TODO: Are the filenames represented at all? Should they? (With which filename
                    //       the tracks and extras and cover are written into the zip)

                    if let Some(described_image) = &self.cover {
                        // The image description is not used for building release archives,
                        // so we only hash the image itself
                        described_image.hash(&mut hasher);
                    }

                    if self.extra_downloads.bundled && !self.extras.is_empty() {
                        // There is no relevant order for extras, they are just included in the zip as
                        // files. For hashing we need to ensure a stable order, and as there is no such
                        // guarantee coming from where they are initialized, we sort them here.
                        let mut extras_sorted = self.extras.clone();
                        extras_sorted.sort_by(|a, b| a.sanitized_filename.cmp(&b.sanitized_filename));
                        extras_sorted.hash(&mut hasher);
                    }

                    self.title.hash(&mut hasher);

                    // TODO: TrackNumbering could also be part of signature (how the files are numbered in the filename!)
                    for (track, track_number) in self.tracks.iter().zip(TRACK_NUMBERS) {
                        let tag_mapping = TagMapping::new(self, track, track_number);

                        tag_mapping.hash(&mut hasher);
                        track.transcodes.borrow().hash.hash(&mut hasher);

                        if let Some(described_image) = &track.cover {
                            // The image description is not used so we only hash the image itself
                            described_image.hash(&mut hasher);
                        }

                        if self.extra_downloads.bundled && track.extra_downloads && !track.extras.is_empty() {
                            // There is no relevant order for extras, they are just included in the zip as
                            // files. For hashing we need to ensure a stable order, and as there is no such
                            // guarantee coming from where they are initialized, we sort them here.
                            let mut extras_sorted = track.extras.clone();
                            extras_sorted.sort_by(|a, b| a.sanitized_filename.cmp(&b.sanitized_filename));
                            extras_sorted.hash(&mut hasher);
                        }
                    }

                    let signature = hasher.finish();

                    self.archives = Some(cache.get_or_create_archives(signature));
                }
            }
            DownloadAccess::Disabled |
            DownloadAccess::External { .. } => ()
        }
    }

    pub fn download_assets_available(&self) -> bool {
        !self.download_formats.is_empty() ||
        ((self.extra_downloads.bundled || self.extra_downloads.separate) && !self.extras.is_empty())
    }

    pub fn longest_track_duration(&self) -> f32 {
        let mut longest_track_duration = 0.0;
        for track in &self.tracks {
            let duration_seconds = &track.transcodes.borrow().source_meta.duration_seconds;
            if *duration_seconds > longest_track_duration {
                longest_track_duration = *duration_seconds;
            }
        }
        longest_track_duration
    }

    pub fn new(
        copy_link: bool,
        cover: Option<DescribedImage>,
        date: Option<NaiveDate>,
        download_access: DownloadAccess,
        download_formats: Vec<DownloadFormat>,
        embedding: bool,
        extra_downloads: ExtraDownloads,
        extras: Vec<Extra>,
        links: Vec<Link>,
        m3u: bool,
        main_artists_to_map: Vec<String>,
        more: Option<HtmlAndStripped>,
        more_label: Option<String>,
        permalink: Option<Permalink>,
        source_dir: PathBuf,
        speed_controls: bool,
        support_artists_to_map: Vec<String>,
        synopsis: Option<String>,
        theme: Theme,
        title: String,
        track_numbering: TrackNumbering,
        tracks: Vec<Track>,
        unlisted: bool
    ) -> Release {
        let permalink = permalink.unwrap_or_else(|| Permalink::generate(&title));

        Release {
            archives: None,
            asset_basename: None,
            copy_link,
            cover,
            date,
            download_access,
            download_formats,
            embedding,
            extra_downloads,
            extras,
            links,
            m3u,
            main_artists: Vec::new(),
            main_artists_to_map,
            more,
            more_label,
            permalink,
            procedural_cover: None,
            source_dir,
            speed_controls,
            support_artists: Vec::new(),
            support_artists_to_map,
            synopsis,
            theme,
            title,
            track_numbering,
            tracks,
            unlisted
        }
    }

    /// Returns the file name of the procedural release cover without any
    /// prefixing (i.e. in the context of the release directory). Only call if
    /// you know there is one present, otherwise will panic.
    pub fn procedural_cover_120_filename_unchecked(&self) -> String {
        self.procedural_cover_unchecked()
            .borrow()
            .filename_120()
    }

    /// Returns the file name of the procedural release cover without any
    /// prefixing (i.e. in the context of the release directory). Only call if
    /// you know there is one present, otherwise will panic.
    pub fn procedural_cover_480_filename_unchecked(&self) -> String {
        self.procedural_cover_unchecked()
            .borrow()
            .filename_480()
    }

    /// Returns the file name of the procedural release cover without any
    /// prefixing (i.e. in the context of the release directory). Only call if
    /// you know there is one present, otherwise will panic.
    pub fn procedural_cover_720_filename_unchecked(&self) -> String {
        self.procedural_cover_unchecked()
            .borrow()
            .filename_720()
    }

    pub fn procedural_cover_unchecked(&self) -> &ProceduralCoverRc {
        self.procedural_cover.as_ref().unwrap()
    }

    pub fn shortest_track_duration(&self) -> f32 {
        let mut shortest_track_duration = f32::INFINITY;
        for track in &self.tracks {
            let duration_seconds = &track.transcodes.borrow().source_meta.duration_seconds;
            if *duration_seconds < shortest_track_duration {
                shortest_track_duration = *duration_seconds;
            }
        }
        shortest_track_duration
    }

    /// Returns true if there is at least one track on this release on
    /// which the artist(s) differ from the other tracks.
    pub fn varying_track_artists(&self) -> bool {
        let mut track_iterator = self.tracks.iter().peekable();
        while let Some(track) = track_iterator.next() {
            if let Some(next_track) = track_iterator.peek() {
                if track.artists
                    .iter()
                    .zip(next_track.artists.iter())
                    .any(|(track_artist, next_track_artist)| !ArtistRc::ptr_eq(track_artist, next_track_artist)) {
                    return true;
                }
            }
        }

        false
    }

    /// Writes release downloads (zip archives including track audio files,
    /// covers and extras as well as release cover and extras) and track
    /// downloads (separate track audio files, covers and extras) to the build
    /// directory.
    pub fn write_downloadable_files(&mut self, build: &mut Build) {
        let tag_mappings: Vec<TagMapping> = self.tracks
            .iter()
            .zip(TRACK_NUMBERS)
            .map(|(track, track_number)| TagMapping::new(self, track, track_number))
            .collect();

        // Transcode and copy track downloads
        for ((track, tag_mapping), track_number) in self.tracks.iter_mut().zip(tag_mappings.iter()).zip(TRACK_NUMBERS) {
            match track.download_access {
                DownloadAccess::Code { .. } |
                DownloadAccess::Free |
                DownloadAccess::Paycurtain { .. } => {
                    // Transcode and copy track audio files
                    let track_download_formats = track.download_formats.clone();
                    for download_format in track_download_formats {
                        // Transcode track to download format (to cache) if not yet available
                        if !track.transcodes.borrow().has(download_format.as_audio_format(), generic_hash(&tag_mapping)) {
                            if download_format.is_lossless() && !track.transcodes.borrow().source_meta.lossless {
                                warn_discouraged!(
                                    "Track {} comes from a lossy source format, offering it in a lossless download format is somewhat wasteful and misleading to those who will download it.",
                                    &track.transcodes.file_meta.path.display()
                                );
                            }

                            let cover_path = track.cover.as_ref().or(self.cover.as_ref())
                                .as_ref()
                                .map(|described_image| build.catalog_dir.join(&described_image.file_meta.path));

                            track.transcode_as(
                                download_format.as_audio_format(),
                                build,
                                AssetIntent::Deliverable,
                                tag_mapping,
                                &cover_path
                            );

                            track.transcodes.borrow().persist_to_cache(&build.cache_dir);
                        }

                        // Copy transcoded track (from cache) to build
                        let mut transcodes_mut = track.transcodes.borrow_mut();
                        let mut transcode_option = transcodes_mut.get_mut(download_format.as_audio_format(), generic_hash(&tag_mapping));
                        let transcode = transcode_option.as_mut().unwrap();

                        transcode.asset.unmark_stale();

                        let track_filename = format!(
                            "{basename}{extension}",
                            basename = track.asset_basename.as_ref().unwrap(),
                            extension = download_format.as_audio_format().extension()
                        );

                        let hash = build.hash_with_salt(|hasher| {
                            self.permalink.slug.hash(hasher);
                            track_number.hash(hasher);
                            download_format.as_audio_format().asset_dirname().hash(hasher);
                            track_filename.hash(hasher);
                        });

                        let hash_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join(track_number.to_string())
                            .join(download_format.as_audio_format().asset_dirname())
                            .join(hash);

                        util::ensure_dir_all(&hash_dir);

                        let target_path = hash_dir.join(&track_filename);

                        // The track asset might already have been copied to the build directory
                        // if the download format is identical to one of the streaming formats.
                        // So we only copy and add it to the stats if that hasn't yet happened.
                        if !target_path.exists() {
                            util::hard_link_or_copy(
                                build.cache_dir.join(&transcode.asset.filename),
                                target_path
                            );

                            build.stats.add_track(transcode.asset.filesize_bytes);
                        }
                    }

                    // Copy track extras
                    if track.extra_downloads {
                        for extra in &track.extras {
                            let hash = build.hash_with_salt(|hasher| {
                                self.permalink.slug.hash(hasher);
                                track_number.hash(hasher);
                                "extras".hash(hasher);
                                extra.sanitized_filename.hash(hasher);
                            });

                            let hash_dir = build.build_dir
                                .join(&self.permalink.slug)
                                .join(track_number.to_string())
                                .join("extras")
                                .join(hash);

                            util::ensure_dir_all(&hash_dir);

                            let target_path = hash_dir.join(&extra.sanitized_filename);

                            util::hard_link_or_copy(
                                build.catalog_dir.join(&extra.file_meta.path),
                                target_path
                            );

                            build.stats.add_extra(extra.file_meta.size);
                        }
                    }
                }
                DownloadAccess::Disabled |
                DownloadAccess::External { .. } => ()
            }
        }

        // Write and copy release archives (includes transcoding of tracks where required)
        match self.download_access {
            DownloadAccess::Code { .. } |
            DownloadAccess::Free |
            DownloadAccess::Paycurtain { .. } => {
                for download_format in &self.download_formats {
                    let archives_ref = self.archives.as_ref().unwrap();
                    let mut archives_mut = archives_ref.borrow_mut();

                    // Write zip archive for required format (to cache) if not yet available
                    if !archives_mut.has(*download_format) {
                        let cached_archive_filename = format!("{}.zip", util::uid());

                        info_zipping!(
                            "Creating download archive for release '{}' ({})",
                            self.title,
                            download_format.as_audio_format()
                        );

                        let zip_file = File::create(build.cache_dir.join(&cached_archive_filename)).unwrap();
                        let mut zip_writer = ZipWriter::new(zip_file);
                        let options = SimpleFileOptions::default()
                            .compression_method(CompressionMethod::Deflated)
                            .unix_permissions(0o755);

                        let mut buffer = Vec::new();

                        let mut used_filenames_release_level = HashSet::new();

                        for (track, tag_mapping) in self.tracks.iter_mut().zip(tag_mappings.iter()) {
                            // Transcode track to download format (to cache) if not yet available
                            if !track.transcodes.borrow().has(download_format.as_audio_format(), generic_hash(&tag_mapping)) {
                                if download_format.is_lossless() && !track.transcodes.borrow().source_meta.lossless {
                                    warn_discouraged!(
                                        "Track {} comes from a lossy source format, offering it in a lossless download format is somewhat wasteful and misleading to those who will download it.",
                                        &track.transcodes.file_meta.path.display()
                                    );
                                }

                                let cover_path = track.cover.as_ref().or(self.cover.as_ref())
                                    .map(|described_image| build.catalog_dir.join(&described_image.file_meta.path));

                                track.transcode_as(
                                    download_format.as_audio_format(),
                                    build,
                                    AssetIntent::Intermediate,
                                    tag_mapping,
                                    &cover_path
                                );

                                track.transcodes.borrow().persist_to_cache(&build.cache_dir);
                            }

                            let transcodes_ref = track.transcodes.borrow();
                            let transcode = transcodes_ref.get_unchecked(download_format.as_audio_format(), generic_hash(&tag_mapping));

                            let filename = format!(
                                "{basename}{extension}",
                                basename = track.asset_basename.as_ref().unwrap(),
                                extension = download_format.as_audio_format().extension()
                            );

                            zip_writer.start_file(&*filename, options).unwrap();
                            used_filenames_release_level.insert(filename);

                            let mut zip_inner_file = File::open(
                                build.cache_dir.join(&transcode.asset.filename)
                            ).unwrap();

                            zip_inner_file.read_to_end(&mut buffer).unwrap();
                            zip_writer.write_all(&buffer).unwrap();
                            buffer.clear();

                            track.transcodes.borrow().persist_to_cache(&build.cache_dir);

                            // Write track cover and/or extras to a subdirectory named like the track
                            if track.cover.is_some() ||
                                (self.extra_downloads.bundled && track.extra_downloads && !track.extras.is_empty()) {
                                let mut used_filenames_track_level = HashSet::new();

                                let t_extras = &build.locale.translations.extras;
                                let extra_dirname = format!(
                                    "{basename} ({t_extras})",
                                    basename = track.asset_basename.as_ref().unwrap()
                                );

                                zip_writer.add_directory(&extra_dirname, options).unwrap();

                                // Write track cover
                                if let Some(described_image) = &mut track.cover {
                                    let mut image_mut = described_image.borrow_mut();
                                    let source_path = &described_image.file_meta.path;

                                    // Technically we should only request/compute a
                                    // single asset specifically suitable for
                                    // inclusion in the download here, not all of
                                    // them as we use them for display on the
                                    // website. That should go hand in hand with
                                    // marking this single asset with
                                    // AssetIntent::Intermediate, i.e. immediately
                                    // beginning its decay in the cache for future
                                    // removal.
                                    let cover_assets = image_mut.cover_assets(build, source_path);

                                    let cover_filename = String::from("cover.jpg");
                                    let cover_path = format!("{extra_dirname}/{cover_filename}");

                                    zip_writer.start_file(cover_path, options).unwrap();
                                    used_filenames_track_level.insert(cover_filename);

                                    let mut zip_inner_file = File::open(
                                        build.cache_dir.join(&cover_assets.largest().filename)
                                    ).unwrap();

                                    zip_inner_file.read_to_end(&mut buffer).unwrap();
                                    zip_writer.write_all(&buffer).unwrap();
                                    buffer.clear();

                                    image_mut.persist_to_cache(&build.cache_dir);
                                }

                                // Write track extras
                                if self.extra_downloads.bundled && track.extra_downloads {
                                    for extra in &track.extras {
                                        let extra_filename = deduplicate_extra_filename(
                                            &extra.sanitized_filename,
                                            &used_filenames_track_level
                                        );

                                        let extra_path = format!("{extra_dirname}/{extra_filename}");

                                        zip_writer.start_file(extra_path, options).unwrap();
                                        used_filenames_track_level.insert(extra_filename);

                                        let mut zip_inner_file = File::open(
                                            build.catalog_dir.join(&extra.file_meta.path)
                                        ).unwrap();

                                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                                        zip_writer.write_all(&buffer).unwrap();
                                        buffer.clear();
                                    }
                                }
                            }
                        }

                        // Write release cover
                        if let Some(described_image) = &mut self.cover {
                            let mut image_mut = described_image.borrow_mut();
                            let source_path = &described_image.file_meta.path;

                            // Technically we should only request/compute a
                            // single asset specifically suitable for
                            // inclusion in the download here, not all of
                            // them as we use them for display on the
                            // website. That should go hand in hand with
                            // marking this single asset with
                            // AssetIntent::Intermediate, i.e. immediately
                            // beginning its decay in the cache for future
                            // removal.
                            let cover_assets = image_mut.cover_assets(build, source_path);

                            let cover_filename = String::from("cover.jpg");

                            zip_writer.start_file(&*cover_filename, options).unwrap();
                            used_filenames_release_level.insert(cover_filename);

                            let mut zip_inner_file = File::open(
                                build.cache_dir.join(&cover_assets.largest().filename)
                            ).unwrap();

                            zip_inner_file.read_to_end(&mut buffer).unwrap();
                            zip_writer.write_all(&buffer).unwrap();
                            buffer.clear();

                            image_mut.persist_to_cache(&build.cache_dir);
                        }

                        if self.extra_downloads.bundled {
                            for extra in &self.extras {
                                let extra_filename = deduplicate_extra_filename(
                                    &extra.sanitized_filename,
                                    &used_filenames_release_level
                                );

                                zip_writer.start_file(&*extra_filename, options).unwrap();
                                used_filenames_release_level.insert(extra_filename);

                                let mut zip_inner_file = File::open(
                                    build.catalog_dir.join(&extra.file_meta.path)
                                ).unwrap();

                                zip_inner_file.read_to_end(&mut buffer).unwrap();
                                zip_writer.write_all(&buffer).unwrap();
                                buffer.clear();
                            }
                        }

                        match zip_writer.finish() {
                            Ok(_) => {
                                let asset = Asset::new(build, cached_archive_filename, AssetIntent::Deliverable);
                                archives_mut.formats.push(Archive::new(asset, *download_format));
                            }
                            Err(err) => panic!("{}", err)
                        };
                    }

                    // Copy the zip archive (from cache) to the build
                    let archive_option = archives_mut.get_mut(*download_format);
                    let archive_mut = archive_option.unwrap();

                    archive_mut.asset.unmark_stale();

                    let archive_filename = format!(
                        "{basename}.zip",
                        basename = self.asset_basename.as_ref().unwrap()
                    );

                    let hash = build.hash_with_salt(|hasher| {
                        self.permalink.slug.hash(hasher);
                        download_format.as_audio_format().asset_dirname().hash(hasher);
                        archive_filename.hash(hasher);
                    });

                    let hash_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(download_format.as_audio_format().asset_dirname())
                        .join(hash);

                    util::ensure_dir_all(&hash_dir);

                    util::hard_link_or_copy(
                        build.cache_dir.join(&archive_mut.asset.filename),
                        hash_dir.join(&archive_filename)
                    );

                    build.stats.add_archive(archive_mut.asset.filesize_bytes);

                    archives_mut.persist_to_cache(&build.cache_dir);
                }

                // Write extras for discrete download access (outside of archives/zips)
                if self.extra_downloads.separate {
                    for extra in &self.extras {
                        let hash = build.hash_with_salt(|hasher| {
                            self.permalink.slug.hash(hasher);
                            "extras".hash(hasher);
                            extra.sanitized_filename.hash(hasher);
                        });

                        let hash_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join("extras")
                            .join(hash);

                        util::ensure_dir_all(&hash_dir);

                        let target_path = hash_dir.join(&extra.sanitized_filename);

                        util::hard_link_or_copy(
                            build.catalog_dir.join(&extra.file_meta.path),
                            target_path
                        );

                        build.stats.add_extra(extra.file_meta.size);
                    }
                }
            }
            DownloadAccess::Disabled |
            DownloadAccess::External { .. } => ()
        }
    }

    pub fn write_pages_and_playlist_files(&self, build: &mut Build, catalog: &Catalog) {
        // Render release page
        let release_dir = build.build_dir.join(&self.permalink.slug);
        let release_html = render::release::release_html(build, catalog, self);
        util::ensure_dir_all_and_write_index(&release_dir, &release_html);

        // Render release download/purchase/unlock page
        if !self.download_formats.is_empty() ||
           ((self.extra_downloads.bundled || self.extra_downloads.separate) && !self.extras.is_empty()) {
            match &self.download_access {
                DownloadAccess::Code { download_codes, unlock_info } => {
                    let t_unlock_permalink = *build.locale.translations.unlock_permalink;
                    let unlock_page_hash = build.hash_with_salt(|hasher| {
                        self.permalink.slug.hash(hasher);
                        t_unlock_permalink.hash(hasher);
                    });

                    let unlock_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_unlock_permalink)
                        .join(unlock_page_hash);

                    let unlock_html = render::release_unlock::release_unlock_html(build, catalog, self, unlock_info);
                    util::ensure_dir_all_and_write_index(&unlock_page_dir, &unlock_html);

                    let download_html = render::release_download::release_download_html(build, catalog, self);
                    let t_downloads_permalink = *build.locale.translations.downloads_permalink;

                    let download_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_downloads_permalink);

                    for code in download_codes {
                        let code_dir = download_dir.join(code);
                        util::ensure_dir_all_and_write_index(&code_dir, &download_html);
                    }
                }
                DownloadAccess::Disabled => (),
                DownloadAccess::External { .. } => (),
                DownloadAccess::Free  => {
                    let download_html = render::release_download::release_download_html(build, catalog, self);
                    let t_downloads_permalink = *build.locale.translations.downloads_permalink;

                    let download_page_hash = build.hash_with_salt(|hasher| {
                        self.permalink.slug.hash(hasher);
                        t_downloads_permalink.hash(hasher);
                    });

                    let download_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_downloads_permalink)
                        .join(download_page_hash);

                    util::ensure_dir_all_and_write_index(&download_page_dir, &download_html);
                }
                DownloadAccess::Paycurtain { payment_info, price } => {
                    if let Some(payment_info) = payment_info {
                        let t_purchase_permalink = *build.locale.translations.purchase_permalink;
                        let purchase_page_hash = build.hash_with_salt(|hasher| {
                            self.permalink.slug.hash(hasher);
                            t_purchase_permalink.hash(hasher);
                        });

                        let purchase_page_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join(t_purchase_permalink)
                            .join(purchase_page_hash);

                        let purchase_html = render::release_purchase::release_purchase_html(build, catalog, payment_info, price, self);
                        util::ensure_dir_all_and_write_index(&purchase_page_dir, &purchase_html);

                        let download_html = render::release_download::release_download_html(build, catalog, self);
                        let t_downloads_permalink = *build.locale.translations.downloads_permalink;

                        let download_page_hash = build.hash_with_salt(|hasher| {
                            self.permalink.slug.hash(hasher);
                            t_downloads_permalink.hash(hasher);
                        });

                        let download_page_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join(t_downloads_permalink)
                            .join(download_page_hash);

                        util::ensure_dir_all_and_write_index(&download_page_dir, &download_html);
                    } else {
                        warn!(
                            "No payment info specified for release '{}', no purchase/download option will be displayed for this release.",
                            self.title
                        );
                    }
                }
            }
        }

        if let Some(base_url) = &build.base_url {
            // Render m3u playlist
            if self.m3u {
                let r_m3u = m3u::generate_for_release(base_url, build, self);
                fs::write(release_dir.join(M3U_PLAYLIST_FILENAME), r_m3u).unwrap();
            }

            // Render release embed pages
            if self.embedding {
                let release_embed_codes_dir = release_dir.join("embed");
                let release_embed_codes_html = render::release_embed_codes::release_embed_codes_html(base_url, build, catalog, self);
                util::ensure_dir_all_and_write_index(&release_embed_codes_dir, &release_embed_codes_html);

                if self.embedding {
                    let release_embed_dir = release_embed_codes_dir.join("all");
                    let release_embed_html = render::release_embed::release_embed_html(base_url, build, catalog, self);
                    util::ensure_dir_all_and_write_index(&release_embed_dir, &release_embed_html);
                }
            }

            // Render track embed pages
            for (track, track_number) in self.tracks.iter().zip(TRACK_NUMBERS) {
                if track.embedding {
                    let track_embed_codes_dir = release_dir.join(track_number.to_string()).join("embed");
                    let track_embed_codes_html = render::track_embed_codes::track_embed_codes_html(base_url, build, catalog, self, track, track_number);
                    util::ensure_dir_all_and_write_index(&track_embed_codes_dir, &track_embed_codes_html);

                    let track_embed_dir = release_dir.join("embed").join(track_number.to_string());
                    let track_embed_html = render::track_embed::track_embed_html(base_url, build, self, track, track_number);
                    util::ensure_dir_all_and_write_index(&track_embed_dir, &track_embed_html);
                }
            }
        }

        // Render pages for each track
        for (track, track_number) in self.tracks.iter().zip(TRACK_NUMBERS) {
            // Render track page
            let track_dir = release_dir.join(track_number.to_string());
            let track_html = render::track::track_html(build, catalog, self, track, track_number);
            util::ensure_dir_all_and_write_index(&track_dir, &track_html);

            // Render track download/purchase/unlock page
            if !track.download_formats.is_empty() ||
               (track.extra_downloads && !track.extras.is_empty()) {
                match &track.download_access {
                    DownloadAccess::Code { download_codes, unlock_info } => {
                        let t_unlock_permalink = *build.locale.translations.unlock_permalink;

                        let unlock_page_hash = build.hash_with_salt(|hasher| {
                            self.permalink.slug.hash(hasher);
                            track_number.hash(hasher);
                            t_unlock_permalink.hash(hasher);
                        });

                        let unlock_page_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join(track_number.to_string())
                            .join(t_unlock_permalink)
                            .join(unlock_page_hash);

                        let unlock_html = render::track_unlock::track_unlock_html(
                            build,
                            catalog,
                            self,
                            track,
                            track_number,
                            unlock_info
                        );
                        util::ensure_dir_all_and_write_index(&unlock_page_dir, &unlock_html);

                        let download_html = render::track_download::track_download_html(build, catalog, self, track, track_number);
                        let t_downloads_permalink = *build.locale.translations.downloads_permalink;

                        let download_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join(track_number.to_string())
                            .join(t_downloads_permalink);

                        for code in download_codes {
                            let code_dir = download_dir.join(code);
                            util::ensure_dir_all_and_write_index(&code_dir, &download_html);
                        }
                    }
                    DownloadAccess::Disabled => (),
                    DownloadAccess::External { .. } => (),
                    DownloadAccess::Free  => {
                        let download_html = render::track_download::track_download_html(build, catalog, self, track, track_number);
                        let t_downloads_permalink = *build.locale.translations.downloads_permalink;

                        let download_page_hash = build.hash_with_salt(|hasher| {
                            self.permalink.slug.hash(hasher);
                            track_number.hash(hasher);
                            t_downloads_permalink.hash(hasher);
                        });

                        let download_page_dir = build.build_dir
                            .join(&self.permalink.slug)
                            .join(track_number.to_string())
                            .join(t_downloads_permalink)
                            .join(download_page_hash);

                        util::ensure_dir_all_and_write_index(&download_page_dir, &download_html);
                    }
                    DownloadAccess::Paycurtain { payment_info, price } => {
                        if let Some(payment_info) = payment_info {
                            let t_purchase_permalink = *build.locale.translations.purchase_permalink;
                            let purchase_page_hash = build.hash_with_salt(|hasher| {
                                self.permalink.slug.hash(hasher);
                                track_number.hash(hasher);
                                t_purchase_permalink.hash(hasher);
                            });

                            let purchase_page_dir = build.build_dir
                                .join(&self.permalink.slug)
                                .join(track_number.to_string())
                                .join(t_purchase_permalink)
                                .join(purchase_page_hash);

                            let purchase_html = render::track_purchase::track_purchase_html(
                                build,
                                catalog,
                                payment_info,
                                price,
                                self,
                                track,
                                track_number
                            );
                            util::ensure_dir_all_and_write_index(&purchase_page_dir, &purchase_html);

                            let download_html = render::track_download::track_download_html(build, catalog, self, track, track_number);
                            let t_downloads_permalink = *build.locale.translations.downloads_permalink;

                            let download_page_hash = build.hash_with_salt(|hasher| {
                                self.permalink.slug.hash(hasher);
                                track_number.hash(hasher);
                                t_downloads_permalink.hash(hasher);
                            });

                            let download_page_dir = build.build_dir
                                .join(&self.permalink.slug)
                                .join(track_number.to_string())
                                .join(t_downloads_permalink)
                                .join(download_page_hash);

                            util::ensure_dir_all_and_write_index(&download_page_dir, &download_html);
                        } else {
                            warn!(
                                "No payment info specified for track '{}', no purchase/download option will be displayed for this track.",
                                self.title
                            );
                        }
                    }
                }
            }
        }
    }
}

impl ReleaseRc {
    pub fn borrow(&self) -> Ref<'_, Release> {
        self.release.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Release> {
        self.release.borrow_mut()
    }

    pub fn new(release: Release) -> ReleaseRc {
        ReleaseRc {
            release: Rc::new(RefCell::new(release))
        }
    }
}
