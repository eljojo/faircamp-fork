// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use sanitize_filename::sanitize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;
use std::path::{Path, PathBuf};

use crate::{
    Artist,
    ArtistRc,
    AssetIntent,
    Build,
    Cache,
    DescribedImage,
    DownloadOption,
    Extra,
    Favicon,
    FileMeta,
    HeuristicAudioMeta,
    HtmlAndStripped,
    ImageRcView,
    Link,
    PermalinkUsage,
    Release,
    ReleaseRc,
    TagMapping,
    Theme,
    Track,
    TranscodesRcView,
    util
};
use crate::manifest::{self, LocalOptions, Overrides};
use crate::util::{generic_hash, url_safe_hash_base64};

const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["aif", "aifc", "aiff", "alac", "flac", "mp3", "ogg", "opus", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["gif", "heif", "jpeg", "jpg", "png", "webp"];

const UNSUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["aac", "m4a"];

const PERMALINK_CONFLICT_RESOLUTION_HINT: &str = "Hint: In order to resolve the conflict, explicitly specify non-conflicting permalinks for all involved artists/releases through manifests (see faircamp's README.md)";

#[derive(Debug)]
pub struct Catalog {
    /// Stores the primary artist for "single artist" catalogs
    pub artist: Option<ArtistRc>,
    /// All artists (main_artists + support_artists)
    pub artists: Vec<ArtistRc>,
    pub copy_link: bool,
    pub favicon: Favicon,
    /// Whether support artists should get their own
    /// pages and be linked to them
    pub feature_support_artists: bool,
    /// Those artists that get their own page
    pub featured_artists: Vec<ArtistRc>,
    pub feed_enabled: bool,
    pub home_image: Option<DescribedImage>,
    pub label_mode: bool,
    pub links: Vec<Link>,
    /// Whether an m3u playlist should be generated and provided for the entire catalog
    pub m3u: bool,
    pub main_artists: Vec<ArtistRc>,
    /// Optional override label for the button that (by default) says "More" on the
    /// catalog homepage and points to the long-form catalog text on the homepage.
    pub more_label: Option<String>,
    pub releases: Vec<ReleaseRc>,
    pub show_support_artists: bool,
    pub support_artists: Vec<ArtistRc>,
    pub synopsis: Option<String>,
    pub text: Option<HtmlAndStripped>,
    pub theme: Theme,
    title: Option<String>
}

/// Gets passed the images found in a release directory. Checks against a few
/// hardcoded filenames (the usual suspects) to determine which image is most
/// likely to be the intended release cover image.
fn pick_best_cover_image(images: &[ImageRcView]) -> Option<DescribedImage> {
    let mut cover_candidate_option: Option<(usize, &ImageRcView)> = None;

    for image in images {
        let priority = match image
            .file_meta
            .path.file_stem().unwrap().to_str().unwrap().to_lowercase().as_str() {
            "cover" => 1,
            "front" => 2,
            "album" => 3,
            _ => 4
        };

        if let Some(cover_candidate) = &cover_candidate_option {
            if priority < cover_candidate.0 {
                cover_candidate_option = Some((priority, image));
            }
        } else {
            cover_candidate_option = Some((priority, image));
        }
    }

    cover_candidate_option
        .map(|cover_candidate| DescribedImage::new(None, cover_candidate.1.clone()))
}

// TODO: Optimize this (and also the related mechanism in styles.rs).
//       Right now we see if we already generated the file (in build) to decide
//       whether to go forward, but it would be more elegant/efficient another
//       way, because like this we do more processing than is necessary.
pub fn write_background_image(build: &mut Build, image: &ImageRcView) {
    let mut image_mut = image.borrow_mut();
    let source_path = &image.file_meta.path;
    let background_asset = image_mut.background_asset(build, AssetIntent::Deliverable, source_path);

    let hashed_filename = format!("background-{}.jpg", url_safe_hash_base64(&background_asset.filename));
    let hashed_path = build.build_dir.join(hashed_filename);

    if !hashed_path.exists() {
        util::hard_link_or_copy(
            build.cache_dir.join(&background_asset.filename),
            hashed_path
        );

        build.stats.add_image(background_asset.filesize_bytes);

        image_mut.persist_to_cache(&build.cache_dir);
    }
}

impl Catalog {
    /// Use the metadata we gathered for tracks and releases to compute
    /// the folder and file names we are going to create in our build
    /// directory.
    pub fn compute_asset_basenames(&mut self) {
        for release in &self.releases {
            let mut release_mut = release.borrow_mut();

            let main_artists = if release_mut.main_artists.is_empty() {
                String::new()
            } else {
                let list = release_mut.main_artists
                    .iter()
                    .map(|artist| sanitize(&artist.borrow().name))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("{list} - ")
            };
            let release_title = sanitize(&release_mut.title);

            let release_basename = format!("{main_artists}{release_title}");

            release_mut.asset_basename = Some(release_basename);

            for (index, track) in release_mut.tracks.iter_mut().enumerate() {
                let track_artists = if track.artists.is_empty() {
                    String::new()
                } else {
                    let list = track.artists
                        .iter()
                        .map(|artist| sanitize(&artist.borrow().name))
                        .collect::<Vec<String>>()
                        .join(", ");

                    format!("{list} - ")
                };
                let track_number = index + 1;
                let track_title = sanitize(&track.title());

                let track_basename = format!("{track_number:02} {track_artists}{track_title}");

                track.asset_basename = Some(track_basename);
            }
        }
    }

    pub fn create_artist(&mut self, copy_link: bool, name: &str, theme: Theme) -> ArtistRc {
        let artist = ArtistRc::new(Artist::new(copy_link, name, theme));
        self.artists.push(artist.clone());
        artist
    }

    pub fn get_or_create_release_archives(&mut self, cache: &mut Cache) {
        for release in self.releases.iter_mut() {
            release.borrow_mut().get_or_create_release_archives(cache);
        }
    }

    /// For each release goes through the following mappings:
    /// - main_artists_to_map
    /// - support_artists_to_map
    /// - artists_to_map (for each track of a release)
    ///
    /// For each of these mappings (wich are just lists of strings - artist names),
    /// it tries to find an artist in catalog.artists that either has that name,
    /// or an alias associating it to the name. If found, the artist is associated
    /// with the release (either as main or support artist) or track. If not found,
    /// an artist of that name is created and added to catalog.artists and then
    /// associated as described before. Main and support artists are also registered
    /// in a catalog-wide listing of main and support artists, which is then used
    /// to determine pages and links on the site that need to be generated.
    fn map_artists(&mut self) {
        for release in &self.releases {
            let mut release_mut = release.borrow_mut();

            let main_artists_to_map: Vec<String> = release_mut.main_artists_to_map
                .drain(..) // move out of release
                .collect();

            for main_artist_to_map in main_artists_to_map {
                let main_artist_to_map_lowercase = main_artist_to_map.to_lowercase();
                let mut any_artist_found = false;
                for artist in &self.artists {
                    let mut artist_mut = artist.borrow_mut();
                    if artist_mut.name.to_lowercase() == main_artist_to_map_lowercase ||
                        artist_mut.aliases.iter().any(|alias| alias.to_lowercase() == main_artist_to_map_lowercase) {
                        any_artist_found = true;

                        // Only assign artist to release's main artists if it hasn't already been assigned to them
                        if !release_mut.main_artists.iter().any(|main_artist| ArtistRc::ptr_eq(main_artist, artist)) {
                            artist_mut.releases.push(release.clone());
                            release_mut.main_artists.push(artist.clone());
                        }

                        // Only assign artist to catalog's main artists if it hasn't already been assigned to them
                        if !self.main_artists.iter().any(|main_artist| ArtistRc::ptr_eq(main_artist, artist)) {
                            self.main_artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = ArtistRc::new(Artist::new(self.copy_link, &main_artist_to_map, self.theme.clone()));
                    new_artist.borrow_mut().releases.push(release.clone());
                    self.artists.push(new_artist.clone());
                    self.main_artists.push(new_artist.clone());
                    release_mut.main_artists.push(new_artist);
                }
            }

            let support_artists_to_map: Vec<String> = release_mut.support_artists_to_map
                .drain(..) // move out of release
                .collect();

            for support_artist_to_map in support_artists_to_map {
                let support_artist_to_map_lowercase = support_artist_to_map.to_lowercase();
                let mut any_artist_found = false;
                for artist in &self.artists {
                    let mut artist_mut = artist.borrow_mut();
                    if artist_mut.name.to_lowercase() == support_artist_to_map_lowercase ||
                        artist_mut.aliases.iter().any(|alias| alias.to_lowercase() == support_artist_to_map_lowercase) {
                        any_artist_found = true;

                        // Only assign artist to release's support artists if it hasn't already been assigned to them
                        if !release_mut.support_artists.iter().any(|support_artist| ArtistRc::ptr_eq(support_artist, artist)) {
                            artist_mut.releases.push(release.clone());
                            release_mut.support_artists.push(artist.clone());
                        }

                        // Only assign artist to catalog's support artists if it hasn't already been assigned to them
                        if !self.support_artists.iter().any(|support_artist| ArtistRc::ptr_eq(support_artist, artist)) {
                            self.support_artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = ArtistRc::new(Artist::new(self.copy_link, &support_artist_to_map, self.theme.clone()));
                    new_artist.borrow_mut().releases.push(release.clone());
                    self.artists.push(new_artist.clone());
                    self.support_artists.push(new_artist.clone());
                    release_mut.support_artists.push(new_artist);
                }
            }

            for track in release_mut.tracks.iter_mut() {
                for track_artist_to_map in track.artists_to_map.drain(..) {
                    let track_artist_to_map_lowercase = track_artist_to_map.to_lowercase();
                    let mut any_artist_found = false;
                    for artist in &self.artists {
                        let artist_ref = artist.borrow();
                        if artist_ref.name.to_lowercase() == track_artist_to_map_lowercase ||
                            artist_ref.aliases.iter().any(|alias| alias.to_lowercase() == track_artist_to_map_lowercase) {
                            any_artist_found = true;

                            // Only assign artist to track if it hasn't already been assigned to it
                            if !track.artists.iter().any(|track_artist| ArtistRc::ptr_eq(track_artist, artist)) {
                                track.artists.push(artist.clone());
                            }
                        }
                    }

                    if !any_artist_found {
                        // TODO: An artist created here curiously belongs neither to catalog.main_artists,
                        //       nor catalog.support_artists. This might indicate that in fact we never
                        //       enter into this branch at all?
                        let new_artist = ArtistRc::new(Artist::new(self.copy_link, &track_artist_to_map, self.theme.clone()));
                        self.artists.push(new_artist.clone());
                        track.artists.push(new_artist);
                    }
                }
            }
        }
    }
    
    pub fn new() -> Catalog {
        Catalog {
            artist: None,
            artists: Vec::new(),
            copy_link: true,
            favicon: Favicon::Default,
            feature_support_artists: false,
            featured_artists: Vec::new(),
            feed_enabled: true,
            home_image: None,
            label_mode: false,
            links: Vec::new(),
            m3u: true,
            main_artists: Vec::new(),
            more_label: None,
            releases: Vec::new(),
            show_support_artists: false,
            support_artists: Vec::new(),
            synopsis: None,
            text: None,
            theme: Theme::new(),
            title: None
        }
    }

    pub fn public_releases(&self) -> Vec<ReleaseRc> {
        self.releases
            .iter()
            .filter_map(|release| {
                match release.borrow().unlisted {
                    true => None,
                    false => Some(release.clone())
                }
            })
            .collect()
    }
    
    pub fn read(build: &mut Build, cache: &mut Cache) -> Result<Catalog, ()> {
        let mut catalog = Catalog::new();
        
        catalog.read_dir(&build.catalog_dir.clone(), build, cache, &Overrides::default()).unwrap();

        if catalog.home_image.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
            warn_discouraged!("The catalog home image is missing an image description.");
            build.missing_image_descriptions = true;
        }

        catalog.map_artists();

        if catalog.label_mode {
            catalog.featured_artists.extend(catalog.main_artists.iter().cloned());

            if catalog.feature_support_artists {
                for support_artist in &catalog.support_artists {
                    // Only assign support artist to catalog's featured artists if
                    // it hasn't already been assigned to them as a main artist
                    if !catalog.featured_artists.iter().any(|featured_artist| ArtistRc::ptr_eq(featured_artist, support_artist)) {
                        catalog.featured_artists.push(support_artist.clone());
                    }
                }
            }

            catalog.featured_artists.sort_unstable_by_key(|artist| artist.borrow().name.to_lowercase());

            for artist in &catalog.featured_artists {
                let artist_ref = artist.borrow();
                if artist_ref.image.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
                    warn_discouraged!("The image for artist '{}' is missing an image description.", artist_ref.name);
                    build.missing_image_descriptions = true;
                }
            }
        } else {
            catalog.set_artist();
        }

        catalog.get_or_create_release_archives(cache);

        if !catalog.validate_permalinks() { return Err(()); }

        catalog.compute_asset_basenames();

        catalog.unlist_artists();

        Ok(catalog)
    }
    
    fn read_dir(
        &mut self,
        dir: &Path,
        build: &mut Build,
        cache: &mut Cache,
        parent_overrides: &Overrides
    ) -> Result<(), String> {
        let dir_canonicalized = dir.canonicalize().unwrap();
        for special_dir in &[&build.build_dir, &build.cache_dir] {
            if let Ok(special_dir_canonicalized) = special_dir.canonicalize() {
                if dir_canonicalized == special_dir_canonicalized {
                    if build.verbose {
                        info!("Ignoring special directory {}", special_dir.display());
                    }
                    return Ok(())
                }
            }
        }

        for exclude_pattern in &build.exclude_patterns {
            if let Some(dir_str) = dir.to_str() {
                if dir_str.contains(exclude_pattern) {
                    if build.verbose {
                        info!("Ignoring directory {} and all below (excluded by pattern '{}')", dir.display(), exclude_pattern);
                    }
                    return Ok(())
                }
            }
        }
        
        if build.verbose {
            info!("Reading directory {}", dir.display());
        }
        
        let mut local_options = LocalOptions::new();
        let mut local_overrides = None;
        
        // We get the 'album' metadata from each track in a release. As each track in a
        // release could have a different 'album' specified, we count how often each
        // distinct 'album' tag is present on a track in the release, and then when we
        // create the release struct, we assign the 'album' title we've encountered most.
        // (this is what release_title_metrics is for => Vec<count, title>)
        let mut release_title_metrics: Vec<(u32, String)> = Vec::new();
        let mut release_tracks: Vec<Track> = Vec::new();
        
        let mut dir_paths: Vec<PathBuf> = Vec::new();
        let mut extra_paths: Vec<PathBuf> = Vec::new();
        let mut image_paths: Vec<PathBuf> = Vec::new();
        let mut meta_paths: Vec<PathBuf> = Vec::new();
        let mut track_paths: Vec<(PathBuf, String)> = Vec::new();

        let mut is_artist_dir = false;

        match dir.read_dir() {
            Ok(dir_entries) => {
                'dir_entry_iter: for dir_entry_result in dir_entries {
                    if let Ok(dir_entry) = dir_entry_result {
                        if let Some(filename) = dir_entry.file_name().to_str() {
                            if filename.starts_with('.') {
                                if build.verbose {
                                    info!("Ignoring hidden file '{}'", filename);
                                }
                                continue
                            }
                        }
                        
                        if let Ok(file_type) = dir_entry.file_type() {
                            let path = dir_entry.path();
                            
                            if file_type.is_dir() {
                                dir_paths.push(path);
                            } else if file_type.is_file() {
                                for exclude_pattern in &build.exclude_patterns {
                                    if let Some(dir_entry_str) = dir_entry.path().to_str() {
                                        if dir_entry_str.contains(exclude_pattern) {
                                            if build.verbose {
                                                info!("Ignoring file {} (excluded by pattern '{}')", dir_entry.path().display(), exclude_pattern);
                                            }
                                            continue 'dir_entry_iter
                                        }
                                    }
                                }

                                if !build.include_patterns.is_empty() {
                                    let mut include = false;

                                    for include_pattern in &build.include_patterns {
                                        if let Some(dir_entry_str) = dir_entry.path().to_str() {
                                            if dir_entry_str.contains(include_pattern) {
                                                include = true;
                                                break
                                            }
                                        }
                                    }

                                    if !include {
                                        if build.verbose {
                                            info!("Ignoring file {} (matches no include pattern)", dir_entry.path().display());
                                        }
                                        continue 'dir_entry_iter
                                    }
                                }

                                if path.ends_with("_artist.eno") {
                                    is_artist_dir = true;
                                } else if let Some(extension) = path
                                    .extension()
                                    .and_then(|osstr|
                                        osstr.to_str().map(|str|
                                            str.to_lowercase().as_str().to_string()
                                        )
                                    ) {
                                    if extension == "eno" {
                                        meta_paths.push(path);
                                    } else if SUPPORTED_AUDIO_EXTENSIONS.contains(&&extension[..]) {
                                        track_paths.push((path, extension));
                                    } else if SUPPORTED_IMAGE_EXTENSIONS.contains(&&extension[..]) {
                                        image_paths.push(path);
                                    } else if UNSUPPORTED_AUDIO_EXTENSIONS.contains(&&extension[..]) {
                                        error!("Support for reading audio files with the extension '{}' from the catalog is not yet supported - please get in touch or open an issue if you need this", extension);
                                    } else {
                                        extra_paths.push(path);
                                    }
                                } else {
                                    extra_paths.push(path);
                                }
                            } else if file_type.is_symlink() {
                                warn!("Ignoring symlink '{}'", path.display());
                            } else {
                                warn!("Ignoring unsupported file '{}'", path.display());
                            }
                        }
                    }
                }
            }
            Err(err) => error!("Cannot read directory '{}' ({})", dir.display(), err)
        }

        if is_artist_dir {
            Artist::read_manifest(build, cache, self, dir, parent_overrides);
        }

        for meta_path in &meta_paths {
            if build.verbose {
                info!("Reading meta {}", meta_path.display());
            }
            
            manifest::apply_options(
                meta_path,
                build,
                cache,
                self,
                &mut local_options,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }

        // At this point all overrides have been read and we can consolidate things.
        let merged_overrides = local_overrides.as_ref().unwrap_or(parent_overrides);

        if !is_artist_dir {
            for (track_path, extension) in &track_paths {
                let path_relative_to_catalog = track_path.strip_prefix(&build.catalog_dir).unwrap();

                if build.verbose {
                    info!("Reading track {}", path_relative_to_catalog.display());
                }

                let transcodes = match cache.get_or_create_transcodes(build, path_relative_to_catalog, extension) {
                    Ok(transcodes) => transcodes,
                    Err(err) => {
                        error!("Skipping track {} due to decoding error ({})", path_relative_to_catalog.display(), err);
                        continue;
                    }
                };

                if let Some(release_title) = &transcodes.borrow().source_meta.album {
                    if let Some(metric) = &mut release_title_metrics
                        .iter_mut()
                        .find(|(_count, title)| title == release_title) {
                        metric.0 += 1;
                    } else {
                        release_title_metrics.push((1, release_title.to_string()));
                    }
                }

                let track = self.read_track(merged_overrides, transcodes);

                release_tracks.push(track);
            }
            
            if !release_tracks.is_empty() {
                // Process bare image paths into ImageRc representations
                let images: Vec<ImageRcView> = image_paths
                    .into_iter()
                    .map(|image_path| {
                        let path_relative_to_catalog = image_path.strip_prefix(&build.catalog_dir).unwrap();

                        if build.verbose {
                            info!("Reading image {}", path_relative_to_catalog.display());
                        }

                        cache.get_or_create_image(build, path_relative_to_catalog)
                    })
                    .collect();

                HeuristicAudioMeta::compute(&mut release_tracks);

                // TODO: Print warning if all tracks have track numbers as tags but they don't start a 0/1 and don't increase monotonically
                // TODO: Print warning if only some tracks have track numbers as tags

                release_tracks.sort_by(|track_a, track_b| {
                    let transcodes_ref_a = track_a.transcodes.borrow();
                    let transcodes_ref_b = track_b.transcodes.borrow();

                    let track_numbers = (
                        transcodes_ref_a.source_meta.track_number.or(track_a.heuristic_audio_meta.as_ref().map(|meta| meta.track_number)),
                        transcodes_ref_b.source_meta.track_number.or(track_b.heuristic_audio_meta.as_ref().map(|meta| meta.track_number))
                    );

                    match track_numbers {
                        (Some(a_track_number), Some(b_track_number)) => a_track_number.cmp(&b_track_number),
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        // If both tracks have no track number, sort by original source file name instead
                        (None, None) => track_a.transcodes.file_meta.path.cmp(
                            &track_b.transcodes.file_meta.path
                        )
                    }
                });

                // Sort most often occuring title to the end of the Vec
                release_title_metrics.sort_by(|a, b| a.0.cmp(&b.0));

                let mut main_artists_to_map: Vec<String> = Vec::new();
                let mut support_artists_to_map: Vec<String> = Vec::new();

                // This sets main_artists_to_map in one of three ways, see comments in branches
                if let Some(artist_names) = &merged_overrides.release_artists {
                    // Here, main_artists_to_map is set manually through manifest metadata
                    for artist_name in artist_names {
                        main_artists_to_map.push(artist_name.to_string());
                    }
                } else if release_tracks
                    .iter()
                    .any(|track| !track.transcodes.borrow().source_meta.album_artists.is_empty()) {
                    // Here, main_artists_to_map is set through "album artist" tags found on at least one track
                    for release_track in &release_tracks {
                        let album_artists = &release_track.transcodes.borrow().source_meta.album_artists;

                        for artist in album_artists {
                            if !main_artists_to_map.contains(artist) {
                                main_artists_to_map.push(artist.clone());
                            }
                        }
                    }
                } else {
                    // Here, main_artists_to_map is set through finding the artist(s)
                    // that appear in the "artist" tag on the highest number of tracks.
                    let mut track_artist_metrics = Vec::new();

                    for release_track in &release_tracks {
                        for track_artist_to_map in &release_track.artists_to_map {
                            if let Some((count, _artist)) = &mut track_artist_metrics
                                .iter_mut()
                                .find(|(_count, artist)| artist == track_artist_to_map) {
                                *count += 1;
                            } else {
                                track_artist_metrics.push((1, track_artist_to_map.to_string()));
                            }
                        }
                    }

                    // Sort most often occuring artist(s) to the start of the Vec
                    track_artist_metrics.sort_by(|a, b| b.0.cmp(&a.0));

                    let max_count = track_artist_metrics
                        .first()
                        .map(|(count, _artist)| count.to_owned())
                        .unwrap_or(0);
                    for (count, artist) in track_artist_metrics {
                        if count == max_count {
                            main_artists_to_map.push(artist);
                        } else {
                            support_artists_to_map.push(artist);
                        }
                    }
                }

                let title = &local_options
                    .release_title
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(||
                        release_title_metrics
                            .pop()
                            .map(|(_count, title)| title)
                            .unwrap_or_else(||
                                dir
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string()
                            )
                    );

                if merged_overrides.embedding {
                    build.embeds_requested = true;
                }

                let cover = match &merged_overrides.release_cover {
                    Some(described_image) => Some(described_image.clone()),
                    None => pick_best_cover_image(&images)
                };

                if cover.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
                    warn_discouraged!("The cover image for release '{}' is missing an image description.", title);
                    build.missing_image_descriptions = true;
                }

                let mut extras = Vec::new();
                if merged_overrides.include_extras {
                    for image in images {
                        if let Some(ref described_image) = cover {
                            // If the image we're iterating is the cover image for this release
                            // we don't include it as an extra (that would be redundant).
                            if image.file_meta.path ==
                                described_image.image.file_meta.path {
                                continue
                            }
                        }

                        let extra = Extra::new(image.file_meta.clone());
                        extras.push(extra);
                    }

                    for extra_path in extra_paths {
                        let path_relative_to_catalog = extra_path.strip_prefix(&build.catalog_dir).unwrap();
                        let file_meta = FileMeta::new(build, path_relative_to_catalog);
                        extras.push(Extra::new(file_meta));
                    }
                }

                let mut download_option = merged_overrides.download_option.clone();
                match &mut download_option {
                    DownloadOption::Codes { unlock_text, .. } => {
                        if let Some(custom_unlock_text) = &merged_overrides.unlock_text {
                            unlock_text.replace(custom_unlock_text.clone());
                        }
                    }
                    DownloadOption::Disabled |
                    DownloadOption::External { .. } |
                    DownloadOption::Free => (),
                    DownloadOption::Paid { payment_text, .. } => {
                        if let Some(custom_payment_text) = &merged_overrides.payment_text {
                            payment_text.replace(custom_payment_text.clone());
                        }
                    }
                }

                let release_dir_relative_to_catalog = dir.strip_prefix(&build.catalog_dir).unwrap().to_path_buf();

                let release = Release::new(
                    merged_overrides.copy_link,
                    cover,
                    local_options.release_date.take(),
                    merged_overrides.download_formats.clone(),
                    merged_overrides.download_granularity.clone(),
                    download_option,
                    merged_overrides.embedding,
                    extras,
                    merged_overrides.include_extras,
                    mem::take(&mut local_options.links),
                    merged_overrides.m3u_enabled,
                    main_artists_to_map,
                    merged_overrides.more_label.clone(),
                    local_options.release_permalink.take(),
                    release_dir_relative_to_catalog,
                    merged_overrides.streaming_quality,
                    support_artists_to_map,
                    merged_overrides.release_synopsis.clone(),
                    merged_overrides.tag_agenda.clone(),
                    merged_overrides.release_text.clone(),
                    merged_overrides.theme.clone(),
                    title.to_string(),
                    merged_overrides.release_track_numbering.clone(),
                    release_tracks,
                    local_options.unlisted_release
                );

                self.releases.push(ReleaseRc::new(release));
            }
        }

        if dir == build.catalog_dir {
            if !local_options.links.is_empty() {
                self.links = local_options.links;
            }

            self.theme = merged_overrides.theme.clone();
        }
        
        for dir_path in &dir_paths {
            self.read_dir(dir_path, build, cache, merged_overrides).unwrap();
        }

        Ok(())
    }

    pub fn read_track(
        &mut self,
        overrides: &Overrides,
        transcodes: TranscodesRcView
    ) -> Track {
        let artists_to_map = if let Some(artist_names) = &overrides.track_artists {
            artist_names.to_vec()
        } else {
            transcodes.borrow().source_meta.artists.to_vec()
        };

        let theme = overrides.theme.clone();
        
        Track::new(
            artists_to_map,
            overrides.copy_link,
            theme,
            transcodes
        )
    }

    // TODO: Should we have a manifest option for setting the catalog.artist manually in edge cases?
    /// Uses a heuristic to determine the main artist of the faircamp site (used only
    /// when the site is in artist mode)
    fn set_artist(&mut self) {
        let mut releases_and_tracks_per_artist = self.artists
            .iter()
            .map(|artist| {
                let mut num_releases = 0;
                let mut num_tracks = 0;
                for release in &self.releases {
                    let release_ref = release.borrow();
                    if release_ref.main_artists
                        .iter()
                        .any(|release_main_artist| ArtistRc::ptr_eq(release_main_artist, artist)) {
                        num_releases += 1;
                    }
                    for track in &release_ref.tracks {
                        if track.artists
                            .iter()
                            .any(|track_artist| ArtistRc::ptr_eq(track_artist, artist)) {
                            num_tracks += 1;
                        }
                    }
                }
                (artist.clone(), num_releases, num_tracks)
            })
            .collect::<Vec<(ArtistRc, usize, usize)>>();

        releases_and_tracks_per_artist.sort_by(|a, b|
            match a.1.cmp(&b.1) {
                Ordering::Equal => a.2.cmp(&b.2).reverse(),
                ordering => ordering.reverse()
            }
        );

        if let Some(most_featured_artist) = releases_and_tracks_per_artist.first() {
            self.artist = Some(most_featured_artist.0.clone());
        }
    }
    
    pub fn set_title(&mut self, title: String) -> Option<String> {
        self.title.replace(title)
    }
    
    pub fn title(&self) -> String {
        if let Some(catalog_title) = &self.title {
            return catalog_title.to_string()
        }

        if !self.label_mode {
            if let Some(artist) = &self.artist {
                return artist.borrow().name.clone()
            }
        }

        String::from("Faircamp")
    }

    /// Artists are implicitly unlisted when they have releases and all of these
    /// releases are unlisted. This is determined and set here.
    fn unlist_artists(&self) {
        for artist in &self.artists {
            let mut artist_mut = artist.borrow_mut();
            artist_mut.unlisted =
                !artist_mut.releases.is_empty() &&
                artist_mut.releases.iter().all(|release| release.borrow().unlisted);
        }
    }

    /// Checks the (either auto-generated or user-assigned) permalinks of all
    /// artists and releases in the catalog, printing errors when any two
    /// conflict with each other. Also prints warnings if there are
    /// auto-generated permalinks, as these are not truly permanent and
    /// should be replaced with manually specified ones. Returns whether any
    /// conflicts were found.
    fn validate_permalinks(&self) -> bool {
        let mut generated_permalinks = (None, None, None, 0);
        let mut used_permalinks: HashMap<String, PermalinkUsage> = HashMap::new();

        let mut add_generated_usage = |usage: &PermalinkUsage| {
            if generated_permalinks.2.is_some() {
                generated_permalinks.3 += 1;
            } else {
                let label = match usage {
                    PermalinkUsage::Artist(artist) => format!("artist '{}'", artist.borrow().name),
                    PermalinkUsage::Release(release) => format!("release '{}'", release.borrow().title)
                };

                if generated_permalinks.1.is_some() {
                    generated_permalinks.2 = Some(label);
                } else if generated_permalinks.0.is_some() {
                    generated_permalinks.1 = Some(label);
                } else {
                    generated_permalinks.0 = Some(label);
                }
            }
        };

        for release in &self.releases {
            let release_ref = release.borrow();

            if let Some(previous_usage) = used_permalinks.get(&release_ref.permalink.slug) {
                let generated_or_assigned = &release_ref.permalink.generated_or_assigned_str();
                let slug = &release_ref.permalink.slug;
                let title = &release_ref.title;
                let previous_usage_formatted = previous_usage.as_string();
                let release_dir = release_ref.source_dir.display();
                let message = format!("The {generated_or_assigned} permalink '{slug}' of the release '{title}' from directory '{release_dir}' conflicts with the {previous_usage_formatted}");
                error!("{}\n{}", message, PERMALINK_CONFLICT_RESOLUTION_HINT);
                return false;
            } else {
                let usage = PermalinkUsage::Release(release);
                if release_ref.permalink.generated { add_generated_usage(&usage); }
                used_permalinks.insert(release_ref.permalink.slug.to_string(), usage);
            }
        }
        
        // TODO: We could think about validating this even for non-featured
        // artists already (especially, or maybe only if their permalinks were
        // user-assigned). This way the behavior would be a bit more stable
        // when someone suddenly "flips the switch" on label_mode and/or
        // feature_supported_artists.
        for artist in &self.featured_artists {
            let artist_ref = artist.borrow();
            if let Some(previous_usage) = used_permalinks.get(&artist_ref.permalink.slug) {
                let generated_or_assigned = &artist_ref.permalink.generated_or_assigned_str();
                let slug = &artist_ref.permalink.slug;
                let name = &artist_ref.name;
                let previous_usage_formatted = previous_usage.as_string();
                let message = format!("The {generated_or_assigned} permalink '{slug}' of the artist '{name}' conflicts with the {previous_usage_formatted}");
                error!("{}\n{}", message, PERMALINK_CONFLICT_RESOLUTION_HINT);
                return false;
            } else {
                let usage = PermalinkUsage::Artist(artist);
                if artist_ref.permalink.generated { add_generated_usage(&usage); }
                used_permalinks.insert(artist_ref.permalink.slug.to_string(), usage);
            }
        }

        match generated_permalinks {
            (None, None, None, 0) => (),
            (Some(first), None, None, 0) => warn!("The {} has no user-assigned permalink, it is recommended to assign one.", first),
            (Some(first), Some(second), None, 0) => warn!("The {} and the {} have no user-assigned permalinks, it is recommended to assign some.", first, second),
            (Some(first), Some(second), Some(third), 0) => warn!("The {}, the {} and the {} have no user-assigned permalinks, it is recommended to assign some.", first, second, third),
            (Some(first), Some(second), Some(third), further) => warn!("The {}, the {}, the {} and {} other things have no user-assigned permalinks, it is recommended to assign some.", first, second, third, further),
            _ => unreachable!()
        }

        true
    }
    
    pub fn write_assets(&mut self, build: &mut Build) {
        if let Some(image) = &self.theme.background_image {
            write_background_image(build, image);
        }

        if let Some(described_image) = &self.home_image {
            let mut image_mut = described_image.image.borrow_mut();
            let source_path = &described_image.image.file_meta.path;
            // Write home image as poster image for homepage
            let poster_assets = image_mut.artist_assets(build, AssetIntent::Deliverable, source_path);

            for asset in &poster_assets.all() {
                util::hard_link_or_copy(
                    build.cache_dir.join(&asset.filename),
                    // TODO: Address the ugly __home__ hack soon (maybe hashes are again a solution for these naming questions?)
                    build.build_dir.join(format!("{}_{}_{}x{}.jpg", "__home__", asset.format, asset.width, asset.height))
                );

                build.stats.add_image(asset.filesize_bytes);
            }

            // Write home image as feed image
            if build.base_url.is_some() && self.feed_enabled {
                let source_path = &described_image.image.file_meta.path;
                let feed_image_asset = image_mut.feed_asset(build, AssetIntent::Deliverable, source_path);

                util::hard_link_or_copy(
                    build.cache_dir.join(&feed_image_asset.filename),
                    build.build_dir.join("feed.jpg")
                );

                build.stats.add_image(feed_image_asset.filesize_bytes);
            }

            image_mut.persist_to_cache(&build.cache_dir);
        }

        for artist in self.featured_artists.iter_mut() {
            let artist_ref = artist.borrow();

            let permalink = artist_ref.permalink.slug.to_string();
            if let Some(described_image) = &artist_ref.image {
                let mut image_mut = described_image.image.borrow_mut();
                let source_path = &described_image.image.file_meta.path;
                let poster_assets = image_mut.artist_assets(build, AssetIntent::Deliverable, source_path);

                for asset in &poster_assets.all() {
                    util::hard_link_or_copy(
                        build.cache_dir.join(&asset.filename),
                        build.build_dir.join(format!("{}_{}_{}x{}.jpg", &permalink, asset.format, asset.width, asset.height))
                    );

                    build.stats.add_image(asset.filesize_bytes);
                }

                image_mut.persist_to_cache(&build.cache_dir);
            }

            if let Some(image) = &artist_ref.theme.background_image {
                write_background_image(build, image);
            }
        }

        let max_tracks_in_release = self.releases
            .iter()
            .map(|release| release.borrow().tracks.len())
            .max()
            .unwrap_or(0);

        for release in &self.releases {
            let mut release_mut = release.borrow_mut();

            let release_dir = build.build_dir.join(&release_mut.permalink.slug);

            util::ensure_dir(&release_dir);

            if let Some(image) = &release_mut.theme.background_image {
                write_background_image(build, image);
            }

            if let Some(described_image) = &release_mut.cover {
                let mut image_mut = described_image.image.borrow_mut();
                let source_path = &described_image.image.file_meta.path;
                let cover_assets = image_mut.cover_assets(build, AssetIntent::Deliverable, source_path);

                for asset in &cover_assets.all() {
                    util::hard_link_or_copy(
                        build.cache_dir.join(&asset.filename),
                        release_dir.join(format!("cover_{}.jpg", asset.edge_size))
                    );

                    build.stats.add_image(asset.filesize_bytes);
                }

                image_mut.persist_to_cache(&build.cache_dir);
            } else {
                let t_auto_generated_cover = &build.locale.translations.auto_generated_cover;
                let procedural_cover = self.theme.cover_generator.generate(t_auto_generated_cover, &release_mut, max_tracks_in_release);
                release_mut.procedural_cover = Some(procedural_cover);
            }

            for streaming_format in release_mut.streaming_quality.formats() {
                let streaming_format_dir = build.build_dir
                    .join(&release_mut.permalink.slug)
                    .join(streaming_format.asset_dirname());

                util::ensure_dir(&streaming_format_dir);

                let release_slug = release_mut.permalink.slug.clone();

                let cover_path = release_mut.cover
                    .as_ref()
                    .map(|described_image| build.catalog_dir.join(&described_image.image.file_meta.path));

                let tag_mappings: Vec<TagMapping> = release_mut.tracks
                    .iter()
                    .enumerate()
                    .map(|(track_index, track)| TagMapping::new(&release_mut, track, track_index + 1))
                    .collect();

                for (track, tag_mapping) in release_mut.tracks.iter_mut().zip(tag_mappings.iter()) {
                    track.transcode_as(
                        streaming_format,
                        build,
                        AssetIntent::Deliverable,
                        tag_mapping,
                        &cover_path
                    );

                    let track_filename = format!(
                        "{basename}{extension}",
                        basename = track.asset_basename.as_ref().unwrap(),
                        extension = streaming_format.extension()
                    );

                    let hash = build.hash_path_with_salt(
                        &release_slug,
                        streaming_format.asset_dirname(),
                        &track_filename
                    );

                    let hash_dir = streaming_format_dir.join(hash);

                    util::ensure_dir(&hash_dir);

                    let transcodes_ref = track.transcodes.borrow();
                    let streaming_transcode = transcodes_ref.get_unchecked(streaming_format, generic_hash(&tag_mapping));

                    util::hard_link_or_copy(
                        build.cache_dir.join(&streaming_transcode.asset.filename),
                        hash_dir.join(track_filename)
                    );

                    build.stats.add_track(streaming_transcode.asset.filesize_bytes);

                    track.transcodes.borrow().persist_to_cache(&build.cache_dir);
                }
            }

            if release_mut.download_option.requires_writing_files() {
                release_mut.write_downloadable_files(build);
            }
        }
    }
}
