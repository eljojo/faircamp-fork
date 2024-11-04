// SPDX-FileCopyrightText: 2021-2024 Simon Repp
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

use chrono::NaiveDate;
use sanitize_filename::sanitize;
use serde_derive::{Serialize, Deserialize};
use zip::{CompressionMethod, ZipWriter};
use zip::write::SimpleFileOptions;

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
    DownloadFormat,
    DownloadOption,
    FileMeta,
    HtmlAndStripped,
    Link,
    m3u,
    Permalink,
    render,
    StreamingQuality,
    TagAgenda,
    TagMapping,
    Theme,
    Track,
    TrackNumbering,
    util
};
use crate::util::generic_hash;

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadGranularity {
    AllOptions,
    EntireRelease,
    SingleFiles
}

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
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
    pub download_formats: Vec<DownloadFormat>,
    pub download_granularity: DownloadGranularity,
    pub download_option: DownloadOption,
    pub embedding: bool,
    /// Additional files that are included in the download archive,
    /// such as additional images, liner notes, etc.
    pub extras: Vec<Extra>,
    /// Whether additional files in the release directory (besides audio files,
    /// cover image and manifest(s)) should be included in the archives. 
    pub include_extras: bool,
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
    /// Optional override label for the button that (by default) says "More" on the
    /// release page and points to the long-form text on the release page.
    pub more_label: Option<String>,
    pub permalink: Permalink,
    /// Lazily generated when there is no regular cover
    pub procedural_cover: Option<String>,
    /// Relative path of the release directory in the catalog directory.
    /// This is used to augment permalink conflict errors with additional
    /// info for resolving the conflict.
    pub source_dir: PathBuf,
    pub streaming_quality: StreamingQuality,
    /// Artists that appear on the release as collaborators, features, etc.
    pub support_artists: Vec<ArtistRc>,
    /// See `main_artists_to_map` for what this does
    pub support_artists_to_map: Vec<String>,
    pub synopsis: Option<String>,
    pub tag_agenda: TagAgenda,
    pub text: Option<HtmlAndStripped>,
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
    /// It is critical that every last detail of this hashing implementation
    /// stays the same - unless explicitly needed of course - because this signature
    /// makes or breaks finding cached archives.
    pub fn get_or_create_release_archives(&mut self, cache: &mut Cache) {
        let mut hasher = DefaultHasher::new();

        // TODO: Consider further if there are aspects of the dependency graph missing
        //       that need to be included in the hash signature.
        // TODO: Are the filenames represented at all? Should they? (With which filename
        //       the tracks and extras and cover are written into the zip)

        if let Some(described_image) = &self.cover {
            // The image description is not used for building release archives,
            // so we only hash the image itself
            described_image.image.hash(&mut hasher);
        }

        if self.include_extras {
            // There is no relevant order for extras, they are just included in the zip as
            // files, but for hashing we need to ensure a stable order, as there is no such
            // guarantee coming from where they are initialized - so we sort them here.
            let mut extras_sorted = self.extras.clone();
            extras_sorted.sort_by(|a, b| a.sanitized_filename.cmp(&b.sanitized_filename));
            extras_sorted.hash(&mut hasher);
        }

        self.title.hash(&mut hasher);

        // TODO: TrackNumbering could also be part of signature (how the files are numbered in the filename!)
        for (track_index, track) in self.tracks.iter().enumerate() {
            let tag_mapping = TagMapping::new(self, track, track_index + 1);

            tag_mapping.hash(&mut hasher);
            track.transcodes.borrow().hash.hash(&mut hasher);
        }

        let signature = hasher.finish();

        self.archives = Some(cache.get_or_create_archives(signature));
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
        download_formats: Vec<DownloadFormat>,
        download_granularity: DownloadGranularity,
        download_option: DownloadOption,
        embedding: bool,
        extras: Vec<Extra>,
        include_extras: bool,
        links: Vec<Link>,
        main_artists_to_map: Vec<String>,
        more_label: Option<String>,
        permalink: Option<Permalink>,
        source_dir: PathBuf,
        streaming_quality: StreamingQuality,
        support_artists_to_map: Vec<String>,
        synopsis: Option<String>,
        tag_agenda: TagAgenda,
        text: Option<HtmlAndStripped>,
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
            download_formats,
            download_granularity,
            download_option,
            embedding,
            extras,
            include_extras,
            links,
            main_artists: Vec::new(),
            main_artists_to_map,
            more_label,
            permalink,
            procedural_cover: None,
            source_dir,
            streaming_quality,
            support_artists: Vec::new(),
            support_artists_to_map,
            synopsis,
            tag_agenda,
            text,
            theme,
            title,
            track_numbering,
            tracks,
            unlisted
        }
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

    pub fn write_downloadable_files(&mut self, build: &mut Build) {
        let release_dir = build.build_dir.join(&self.permalink.slug);

        for download_format in &self.download_formats {
            let format_dir = release_dir.join(download_format.as_audio_format().asset_dirname());

            util::ensure_dir(&format_dir);

            let archives_unwrapped = self.archives.as_ref().unwrap(); // at this point guaranteed to be available (delayed initialization)
            let mut archives_mut = archives_unwrapped.borrow_mut();
            let has_cached_archive_asset = archives_mut.has(*download_format);

            let cover_path = self.cover
                .as_ref()
                .map(|described_image| build.catalog_dir.join(&described_image.image.file_meta.path));

            let tag_mappings: Vec<TagMapping> = self.tracks
                .iter()
                .enumerate()
                .map(|(track_index, track)| TagMapping::new(self, track, track_index + 1))
                .collect();

            for (track, tag_mapping) in self.tracks.iter_mut().zip(tag_mappings.iter()) {
                // Transcode track to required format if needed and not yet available
                if !(self.download_granularity == DownloadGranularity::EntireRelease && has_cached_archive_asset) &&
                    !track.transcodes.borrow().has(download_format.as_audio_format(), generic_hash(&tag_mapping)) {
                    if download_format.is_lossless() && !track.transcodes.borrow().source_meta.lossless {
                        warn_discouraged!(
                            "Track {} comes from a lossy format, offering it in a lossless format is wasteful and misleading to those who will download it.",
                            &track.transcodes.file_meta.path.display()
                        );
                    }

                    let asset_intent = if self.download_granularity == DownloadGranularity::EntireRelease {
                        AssetIntent::Intermediate
                    } else {
                        AssetIntent::Deliverable
                    };

                    track.transcode_as(
                        download_format.as_audio_format(),
                        build,
                        asset_intent,
                        tag_mapping,
                        &cover_path
                    );

                    track.transcodes.borrow().persist_to_cache(&build.cache_dir);
                }

                // If single track downloads are enabled copy transcoded track to build
                if self.download_granularity != DownloadGranularity::EntireRelease {
                    let mut transcodes_mut = track.transcodes.borrow_mut();
                    let mut transcode_option = transcodes_mut.get_mut(download_format.as_audio_format(), generic_hash(&tag_mapping));
                    let transcode = transcode_option.as_mut().unwrap();

                    transcode.asset.unmark_stale();

                    let track_filename = format!(
                        "{basename}{extension}",
                        basename = track.asset_basename.as_ref().unwrap(),
                        extension = download_format.as_audio_format().extension()
                    );

                    let hash = build.hash_path_with_salt(
                        &self.permalink.slug,
                        download_format.as_audio_format().asset_dirname(),
                        &track_filename
                    );

                    // TODO: We should calculate this earlier and persist it so we can reuse it for copying
                    // and for rendering the hrefs that point to it, however we need to figure out where 
                    // (or on what) to store it - that's a bit tricky. (applies in a few places)
                    let hash_dir = format_dir.join(hash);

                    util::ensure_dir(&hash_dir);

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
            }

            // If entire release downloads are enabled create the zip (if not available yet) and copy it to build
            if self.download_granularity != DownloadGranularity::SingleFiles {
                // Create zip for required format if not yet available
                if !has_cached_archive_asset {
                    let cached_archive_filename = format!("{}.zip", util::uid());

                    info_zipping!(
                        "Creating download archive for release '{}' ({})",
                        self.title,
                        download_format.as_audio_format()
                    );

                    let zip_file = File::create(
                        build.cache_dir.join(&cached_archive_filename)
                    ).unwrap();
                    let mut zip_writer = ZipWriter::new(zip_file);
                    let options = SimpleFileOptions::default()
                        .compression_method(CompressionMethod::Deflated)
                        .unix_permissions(0o755);

                    let mut buffer = Vec::new();

                    let mut used_filenames = HashSet::new();

                    for (track, tag_mapping) in self.tracks.iter_mut().zip(tag_mappings.iter()) {
                        let transcodes_ref = track.transcodes.borrow();
                        let transcode = transcodes_ref.get_unchecked(download_format.as_audio_format(), generic_hash(&tag_mapping));

                        let filename = format!(
                            "{basename}{extension}",
                            basename = track.asset_basename.as_ref().unwrap(),
                            extension = download_format.as_audio_format().extension()
                        );

                        zip_writer.start_file(&*filename, options).unwrap();
                        used_filenames.insert(filename);

                        let mut zip_inner_file = File::open(
                            &build.cache_dir.join(&transcode.asset.filename)
                        ).unwrap();

                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&buffer).unwrap();
                        buffer.clear();

                        track.transcodes.borrow().persist_to_cache(&build.cache_dir);
                    }

                    if let Some(described_image) = &mut self.cover {
                        let mut image_mut = described_image.image.borrow_mut();
                        let source_path = &described_image.image.file_meta.path;
                        let cover_assets = image_mut.cover_assets(build, AssetIntent::Deliverable, source_path);

                        let cover_filename = String::from("cover.jpg");

                        zip_writer.start_file(&*cover_filename, options).unwrap();
                        used_filenames.insert(cover_filename);

                        let mut zip_inner_file = File::open(
                            &build.cache_dir.join(&cover_assets.largest().filename)
                        ).unwrap();

                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&buffer).unwrap();
                        buffer.clear();

                        image_mut.persist_to_cache(&build.cache_dir);
                    }

                    for extra in &self.extras {
                        let mut extra_filename = extra.sanitized_filename.clone();

                        while used_filenames.contains(&extra_filename) {
                            // TODO: At some point expand so it does a more elegant "foo.jpg" -> "foo(1).jpg" -> "foo(2).jpg" (or similar)
                            extra_filename = match extra_filename.split_once('.') {
                                Some((prefix, postfix)) => format!("{prefix}_duplicate.{postfix}"),
                                None => format!("{extra_filename}_duplicate")
                            };
                        }

                        zip_writer.start_file(&*extra_filename, options).unwrap();
                        used_filenames.insert(extra_filename);

                        let mut zip_inner_file = File::open(
                            &build.catalog_dir.join(&extra.file_meta.path)
                        ).unwrap();

                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&buffer).unwrap();
                        buffer.clear();
                    }

                    match zip_writer.finish() {
                        Ok(_) => {
                            let asset = Asset::new(build, cached_archive_filename, AssetIntent::Deliverable);
                            archives_mut.formats.push(Archive::new(asset, *download_format));
                        }
                        Err(err) => panic!("{}", err)
                    };
                }

                // Now we copy the zip to the build
                let archive_option = archives_mut.get_mut(*download_format);
                let archive_mut = archive_option.unwrap();

                archive_mut.asset.unmark_stale();

                let archive_filename = format!(
                    "{basename}.zip",
                    basename = self.asset_basename.as_ref().unwrap()
                );

                let hash = build.hash_path_with_salt(
                    &self.permalink.slug,
                    download_format.as_audio_format().asset_dirname(),
                    &archive_filename
                );

                let hash_dir = format_dir.join(hash);

                util::ensure_dir(&hash_dir);

                util::hard_link_or_copy(
                    build.cache_dir.join(&archive_mut.asset.filename),
                    hash_dir.join(&archive_filename)
                );

                build.stats.add_archive(archive_mut.asset.filesize_bytes);

                archives_mut.persist_to_cache(&build.cache_dir);
            }
        }

        // Write extras for discrete download access (outside of archives/zips)
        if !self.extras.is_empty() && self.download_granularity != DownloadGranularity::EntireRelease {
            for extra in &self.extras {
                let extras_dir = release_dir.join("extras");

                util::ensure_dir(&extras_dir);

                let hash = build.hash_path_with_salt(
                    &self.permalink.slug,
                    "extras",
                    &extra.sanitized_filename
                );

                // TODO: We should calculate this earlier and persist it so we can reuse it for copying
                // and for rendering the hrefs that point to it, however we need to figure out where 
                // (or on what) to store it - that's a bit tricky. (applies in a few places)
                let hash_dir = extras_dir.join(hash);

                util::ensure_dir(&hash_dir);

                let target_path = hash_dir.join(&extra.sanitized_filename);

                util::hard_link_or_copy(
                    build.catalog_dir.join(&extra.file_meta.path),
                    target_path
                );

                build.stats.add_extra(extra.file_meta.size);
            }
        }
    }

    pub fn write_files(&self, build: &mut Build, catalog: &Catalog) {
        match &self.download_option {
            DownloadOption::Codes { codes, unlock_text } => {
                let t_unlock_permalink = *build.locale.translations.unlock_permalink;
                let page_hash = build.hash_with_salt(&[&self.permalink.slug, t_unlock_permalink]);

                let unlock_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_unlock_permalink)
                    .join(page_hash);

                let unlock_html = render::release::unlock::unlock_html(build, catalog, self, unlock_text);
                util::ensure_dir_and_write_index(&unlock_page_dir, &unlock_html);

                let t_downloads_permalink = *build.locale.translations.downloads_permalink;
                let download_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_downloads_permalink);

                let download_html = render::release::download::download_html(build, catalog, self);

                for code in codes {
                    // TODO: We will need to limit the code character set to url safe characters.
                    //       Needs to be validated when reading the manifests.
                    let code_dir = download_dir.join(code);
                    util::ensure_dir_and_write_index(&code_dir, &download_html);
                }
            }
            DownloadOption::Disabled |
            DownloadOption::External { .. }  => (),
            DownloadOption::Free  => {
                let t_downloads_permalink = *build.locale.translations.downloads_permalink;
                let page_hash = build.hash_with_salt(&[&self.permalink.slug, t_downloads_permalink]);

                let download_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_downloads_permalink)
                    .join(page_hash);

                let download_html = render::release::download::download_html(build, catalog, self);
                util::ensure_dir_and_write_index(&download_page_dir, &download_html);
            }
            DownloadOption::Paid { currency, payment_text, range } => {
                if let Some(payment_text) = payment_text {
                    let t_purchase_permalink = *build.locale.translations.purchase_permalink;
                    let purchase_page_hash = build.hash_with_salt(&[&self.permalink.slug, t_purchase_permalink]);

                    let purchase_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_purchase_permalink)
                        .join(purchase_page_hash);

                    let purchase_html = render::release::purchase::purchase_html(build, catalog, currency, payment_text, range, self);
                    util::ensure_dir_and_write_index(&purchase_page_dir, &purchase_html);

                    let t_downloads_permalink = *build.locale.translations.downloads_permalink;
                    let download_page_hash = build.hash_with_salt(&[&self.permalink.slug, t_downloads_permalink]);

                    let download_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_downloads_permalink)
                        .join(download_page_hash);

                    let download_html = render::release::download::download_html(build, catalog, self);
                    util::ensure_dir_and_write_index(&download_page_dir, &download_html);
                } else {
                    warn!(
                        "No payment text specified for release '{}', no purchase/download option will be displayed for this release.",
                        self.title
                    );
                }
            }
        }
        
        if self.cover.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
            warn_discouraged!("The cover image for release '{}' is missing an image description.", self.title);
            build.missing_image_descriptions = true;
        }

        let release_dir = build.build_dir.join(&self.permalink.slug);
        let release_html = render::release::release_html(build, catalog, self);
        util::ensure_dir_and_write_index(&release_dir, &release_html);

        if let Some(base_url) = &build.base_url {
            // Render m3u playlist
            let r_m3u = m3u::generate(base_url, build, self);
            fs::write(release_dir.join("playlist.m3u"), r_m3u).unwrap();

            // Render embed pages
            if self.embedding  {
                let embed_choices_dir = release_dir.join("embed");
                let embed_choices_html = render::release::embed::embed_choices_html(build, catalog, self, base_url);
                util::ensure_dir_and_write_index(&embed_choices_dir, &embed_choices_html);

                let embed_release_dir = embed_choices_dir.join("all");
                let embed_release_html = render::embed_release::embed_release_html(build, catalog, self);
                util::ensure_dir_and_write_index(&embed_release_dir, &embed_release_html);

                for (index, track) in self.tracks.iter().enumerate() {
                    let track_number = index + 1;
                    let embed_track_dir = embed_choices_dir.join(track_number.to_string());
                    let embed_track_html = render::embed_track::embed_track_html(build, self, track);
                    util::ensure_dir_and_write_index(&embed_track_dir, &embed_track_html);
                }
            }
        }

        // Render page for each track
        for (index, track) in self.tracks.iter().enumerate() {
            let track_number = index + 1;
            let track_dir = release_dir.join(track_number.to_string());
            let track_html = render::track::track_html(build, catalog, self, track, track_number);
            util::ensure_dir_and_write_index(&track_dir, &track_html);
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
