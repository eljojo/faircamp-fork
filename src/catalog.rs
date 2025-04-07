// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
use std::path::Path;

use indoc::{formatdoc, indoc};
use sanitize_filename::sanitize;

use crate::{
    Artist,
    ArtistRc,
    AssetIntent,
    Build,
    Cache,
    DescribedImage,
    Extra,
    FairDir,
    Favicon,
    FeedImageAsset,
    Feeds,
    FileMeta,
    HeuristicAudioMeta,
    HtmlAndStripped,
    ImageRcView,
    Link,
    PermalinkUsage,
    ProceduralCover,
    ProceduralCoverAsset,
    Release,
    ReleaseRc,
    SiteAsset,
    SiteMetadata,
    TagMapping,
    Theme,
    Track,
    TRACK_NUMBERS,
    TranscodesRcView,
    util
};
use crate::manifest::{self, LocalOptions, Overrides};
use crate::util::{generic_hash, url_safe_hash_base64};

const PERMALINK_CONFLICT_RESOLUTION_HINT: &str = "In order to resolve the conflict, explicitly specify non-conflicting permalinks for all involved artists/releases through manifests using the 'permalink: example' option.";

#[derive(Debug)]
pub struct Catalog {
    /// Stores the primary artist for "single artist" catalogs
    pub artist: Option<ArtistRc>,
    /// All artists (main_artists + support_artists)
    pub artists: Vec<ArtistRc>,
    pub copy_link: bool,
    pub faircamp_signature: bool,
    pub favicon: Favicon,
    /// Whether support artists should get their own
    /// pages and be linked to them
    pub feature_support_artists: bool,
    /// Those artists that get their own page
    pub featured_artists: Vec<ArtistRc>,
    pub feeds: Feeds,
    pub home_image: Option<DescribedImage>,
    pub label_mode: bool,
    pub links: Vec<Link>,
    /// Whether an m3u playlist should be generated and provided for the entire catalog
    pub m3u: bool,
    pub main_artists: Vec<ArtistRc>,
    pub more: Option<HtmlAndStripped>,
    /// Optional custom label for the button that (by default) says "More" on the
    /// catalog homepage and points to additional long-form content for the catalog.
    pub more_label: Option<String>,
    /// Whether to include Open Graph metadata tags on all major pages (pages not intended
    /// for sharing generally don't render Open graph tags)
    pub opengraph: bool,
    pub releases: Vec<ReleaseRc>,
    pub show_support_artists: bool,
    /// Files specified through the site_assets option that are meant to be
    /// included in the build, e.g. to reference/include them from custom
    /// site_metadata.
    pub site_assets: Vec<SiteAsset>,
    /// Arbitrary html content with interpolated filenames like {{example.css}}
    /// specified through the site_metadata option that is injected into the
    /// <head>…</head> section on all rendered pages.
    pub site_metadata: Option<SiteMetadata>,
    /// The page presenting subscription choices for the catalog competes with all
    /// artist+release permalinks, therefore we do a run-time computation to
    /// determine a conflict-free permalink for it (which starts with our
    /// desired translation and adds (an) additional character(s) if
    /// needed).
    pub subscribe_permalink: Option<String>,
    pub support_artists: Vec<ArtistRc>,
    pub synopsis: Option<String>,
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
    let background_asset = image_mut.background_asset(build, source_path);

    let hashed_filename = format!("background-{}.jpg", url_safe_hash_base64(&background_asset.filename));
    let hashed_path = build.build_dir.join(&hashed_filename);

    build.reserve_filename(hashed_filename);

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

            for (track, track_number) in release_mut.tracks.iter_mut().zip(TRACK_NUMBERS) {
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
                let track_title = sanitize(track.title());

                let track_basename = format!("{track_number:02} {track_artists}{track_title}");

                track.asset_basename = Some(track_basename);
            }
        }
    }

    /// If the subscribe page permalink we have in our translations collides with
    /// any of the artist or release permalinks, we prepend underscores to it
    /// until there is no collision anymore.
    fn compute_subscribe_permalink(&mut self, build: &Build) {
        let mut subscribe_slug = build.locale.translations.subscribe_permalink.to_string();

        while self.featured_artists.iter().any(|artist| artist.borrow().permalink.slug == subscribe_slug) ||
            self.releases.iter().any(|release| release.borrow().permalink.slug == subscribe_slug) {
            subscribe_slug = format!("_{subscribe_slug}");
        }

        self.subscribe_permalink = Some(subscribe_slug);
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
                let mut any_artist_found = false;
                for artist in &self.artists {
                    let mut artist_mut = artist.borrow_mut();
                    if artist_mut.name == main_artist_to_map ||
                        artist_mut.aliases.iter().any(|alias| *alias == main_artist_to_map) {
                        any_artist_found = true;

                        // Only assign artist to release's main artists if it
                        // hasn't already been assigned to the release as
                        // main artist before.
                        if !release_mut.main_artists.iter().any(|main_artist| ArtistRc::ptr_eq(main_artist, artist)) {
                            artist_mut.releases.push(release.clone());
                            release_mut.main_artists.push(artist.clone());
                        }

                        // Only assign artist to catalog's main artists if it
                        // hasn't already been assigned to the catalog as
                        // main artist before.
                        if !self.main_artists.iter().any(|main_artist| ArtistRc::ptr_eq(main_artist, artist)) {
                            self.main_artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = ArtistRc::new(Artist::new_automatic(self, &main_artist_to_map));
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
                let mut any_artist_found = false;
                for artist in &self.artists {
                    let mut artist_mut = artist.borrow_mut();
                    if artist_mut.name == support_artist_to_map ||
                        artist_mut.aliases.iter().any(|alias| *alias == support_artist_to_map) {
                        any_artist_found = true;

                        // Only assign artist to release's supports artists if it
                        // hasn't already been assigned to the release as
                        // main or support artist before.
                        if !release_mut.main_artists.iter().any(|main_artist| ArtistRc::ptr_eq(main_artist, artist)) &&
                           !release_mut.support_artists.iter().any(|support_artist| ArtistRc::ptr_eq(support_artist, artist)) {
                            artist_mut.releases.push(release.clone());
                            release_mut.support_artists.push(artist.clone());
                        }

                        // Only assign artist to catalog's support artists if
                        // it hasn't already been assigned to the catalog as
                        // main or support artist before.
                        if !self.main_artists.iter().any(|main_artist| ArtistRc::ptr_eq(main_artist, artist)) &&
                           !self.support_artists.iter().any(|support_artist| ArtistRc::ptr_eq(support_artist, artist)) {
                            self.support_artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = ArtistRc::new(Artist::new_automatic(self, &support_artist_to_map));
                    new_artist.borrow_mut().releases.push(release.clone());
                    self.artists.push(new_artist.clone());
                    self.support_artists.push(new_artist.clone());
                    release_mut.support_artists.push(new_artist);
                }
            }

            for track in release_mut.tracks.iter_mut() {
                for track_artist_to_map in track.artists_to_map.drain(..) {
                    let mut any_artist_found = false;
                    for artist in &self.artists {
                        let artist_ref = artist.borrow();
                        if artist_ref.name == track_artist_to_map ||
                            artist_ref.aliases.iter().any(|alias| *alias == track_artist_to_map) {
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
                        let new_artist = ArtistRc::new(Artist::new_automatic(self, &track_artist_to_map));
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
            faircamp_signature: true,
            favicon: Favicon::Default,
            feature_support_artists: false,
            featured_artists: Vec::new(),
            feeds: Feeds::DEFAULT,
            home_image: None,
            label_mode: false,
            links: Vec::new(),
            m3u: false,
            main_artists: Vec::new(),
            more: None,
            more_label: None,
            opengraph: false,
            releases: Vec::new(),
            show_support_artists: false,
            site_assets: Vec::new(),
            site_metadata: None,
            subscribe_permalink: None,
            support_artists: Vec::new(),
            synopsis: None,
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

        catalog.read_catalog_dir(build, cache);

        if build.errors > 0 && !build.ignore_errors {
            info!("Build was aborted because {} errors were encountered while reading the catalog.", build.errors);
            info!("You can run faircamp with --ignore-errors if you want to build in spite of errors.");
            return Err(());
        }

        if catalog.releases.iter().any(|release| {
            let release_ref = release.borrow();
            release_ref.embedding || release_ref.tracks.iter().any(|track| track.embedding)
        }) {
            build.embeds_requested = true;
        }

        if catalog.home_image.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
            warn_discouraged!("The catalog home image is missing an image description.");
            build.missing_image_descriptions = true;
        }

        catalog.map_artists();

        if catalog.label_mode {
            for main_artist in &catalog.main_artists {
                if main_artist.borrow().external_page.is_some() { continue; }

                catalog.featured_artists.push(main_artist.clone());
                main_artist.borrow_mut().featured = true;
            }

            if catalog.feature_support_artists {
                for support_artist in &catalog.support_artists {
                    if support_artist.borrow().external_page.is_some() { continue; }

                    // Only assign support artist to catalog's featured artists if
                    // it hasn't already been assigned to them as a main artist
                    if !catalog.featured_artists.iter().any(|featured_artist| ArtistRc::ptr_eq(featured_artist, support_artist)) {
                        support_artist.borrow_mut().featured = true;
                        catalog.featured_artists.push(support_artist.clone());
                    }
                }
            }

            catalog.featured_artists.sort_unstable_by(|a, b| a.borrow().name.cmp(&b.borrow().name));

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

        if !catalog.validate_permalinks(build) {
            warn!("The build has been aborted because permalink conflicts were found, this kind of error needs to be resolved and cannot be ignored.");
            return Err(());
        }

        if let Some(site_metadata) = &mut catalog.site_metadata {
            if let Err(missing_filenames) = site_metadata.resolve_filename_references(&catalog.site_assets) {
                for filename in &missing_filenames {
                    error!("The filename reference {{{}}} inside site_metadata could not be resolved.", filename)
                }

                warn!("The build has been aborted because {} filenames in site_metadata could not be resolved, this kind of error needs to be resolved and cannot be ignored.", missing_filenames.len());
                return Err(());
            }
        }

        catalog.compute_asset_basenames();
        catalog.compute_subscribe_permalink(build);

        catalog.unlist_artists();

        Ok(catalog)
    }

    fn read_artist_dir(
        &mut self,
        build: &mut Build,
        cache: &mut Cache,
        fair_dir: FairDir,
        parent_overrides: &Overrides
    ) {
        if !fair_dir.audio_files.is_empty() {
            let error = format!("Audio files were encountered in the artist directory '{}' but will be ignored - if you meant to create a release, move these audio files to a separate directory", fair_dir.path.display());
            build.error(&error);
        }

        let artist_manifest = fair_dir.artist_manifest.as_ref().unwrap();

        let mut overrides = parent_overrides.clone();

        if build.verbose {
            info!("Reading artist manifest {}", artist_manifest.display());
        }
        manifest::read_artist_manifest(
            build,
            cache,
            self,
            &fair_dir.path,
            artist_manifest,
            &mut overrides
        );

        for dir_path in &fair_dir.dirs {
            self.read_unknown_dir(build, cache, &overrides, dir_path);
        }
    }

    fn read_catalog_dir(
        &mut self,
        build: &mut Build,
        cache: &mut Cache
    ) {
        if build.verbose {
            info!("Reading catalog directory {}", build.catalog_dir.display());
        }

        let fair_dir = FairDir::read(build, &build.catalog_dir.clone());

        if fair_dir.release_manifest.is_some() {
            let error = format!("A release.eno manifest may not be placed at the root of the catalog directory, however it was found there (at '{}'). Please move it into its own (release) directory", build.catalog_dir.display());
            build.error(&error);
        }

        if fair_dir.track_manifest.is_some() {
            let error = format!("A track.eno manifest may not be placed at the root of the catalog directory, however it was found there (at '{}'). Please move it into its own (track) directory", build.catalog_dir.display());
            build.error(&error);
        }

        let mut catalog_overrides = Overrides::default();
        let mut local_options = LocalOptions::new();

        if let Some(catalog_manifest) = &fair_dir.catalog_manifest {
            if build.verbose {
                info!("Reading catalog manifest {}", catalog_manifest.display());
            }
            manifest::read_catalog_manifest(
                build,
                cache,
                self,
                &fair_dir.path,
                &mut local_options,
                catalog_manifest,
                &mut catalog_overrides
            );
        }

        if let Some(artist_manifest) = &fair_dir.artist_manifest {
            if build.verbose {
                info!("Reading artist manifest {}", artist_manifest.display());
            }
            manifest::read_artist_manifest(
                build,
                cache,
                self,
                &fair_dir.path,
                artist_manifest,
                &mut catalog_overrides
            );
        }

        self.copy_link = catalog_overrides.copy_link;

        if local_options.more.is_some() {
            self.more = local_options.more;
        }
        self.more_label = catalog_overrides.more_label.clone();

        if !local_options.links.is_empty() {
            self.links = local_options.links;
        }

        if local_options.synopsis.is_some() {
            self.synopsis = local_options.synopsis;
        }

        self.theme = catalog_overrides.theme.clone();

        for dir_path in &fair_dir.dirs {
            self.read_unknown_dir(build, cache, &catalog_overrides, dir_path);
        }
    }

    fn read_release_dir(
        &mut self,
        build: &mut Build,
        cache: &mut Cache,
        fair_dir: FairDir,
        parent_overrides: &Overrides
    ) {
        let mut local_overrides = None;
        let mut local_options = LocalOptions::new();

        let mut release_tracks: Vec<Track> = Vec::new();

        if let Some(release_manifest) = &fair_dir.release_manifest {
            if build.verbose {
                info!("Reading release manifest {}", release_manifest.display());
            }
            manifest::read_release_manifest(
                build,
                cache,
                self,
                &fair_dir.path,
                &mut local_options,
                release_manifest,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }

        let finalized_overrides = local_overrides.as_ref().unwrap_or(parent_overrides);

        for dir_path in &fair_dir.dirs {
            let fair_subdir = FairDir::read(build, dir_path);

            if fair_subdir.catalog_manifest.is_some() {
                let error = format!("A catalog.eno manifest may only be placed at the root of the catalog directory, however it was found in a subdirectory (at '{}'). Please move it to the folder '{}'", fair_subdir.path.display(), build.catalog_dir.display());
                build.error(&error);
            }

            let mutually_exclusive_manifests =
                fair_subdir.artist_manifest.is_some() as usize +
                fair_subdir.release_manifest.is_some() as usize +
                fair_subdir.track_manifest.is_some() as usize;

            if mutually_exclusive_manifests > 1 {
                let error = format!("A directory in a faircamp catalog may only ever contain a catalog.eno, release.eno or track.eno manifest (one of them), but the directory '{}' contains {mutually_exclusive_manifests} of these. The directory will be ignored until this is resolved.", fair_subdir.path.display());
                build.error(&error);
                continue;
            }

            if fair_subdir.artist_manifest.is_some() {
                self.read_artist_dir(
                    build,
                    cache,
                    fair_subdir,
                    finalized_overrides
                );
                continue;
            }

            if fair_subdir.audio_files.len() == 1 {
                let result = self.read_track_dir(
                    build,
                    cache,
                    fair_subdir,
                    finalized_overrides
                );

                if let Some(track) = result {
                    release_tracks.push(track);
                }

                continue;
            }

            warn!("Ignoring release subdirectory '{}' - if you meant to make it a track directory it must contain exactly one audio file (plus a track.eno manifest and auxiliary files potentially)", fair_subdir.path.display());
        }

        for audio_path in &fair_dir.audio_files {
            let extension = audio_path.extension().unwrap().to_str().unwrap().to_lowercase().as_str().to_string();
            let path_relative_to_catalog = audio_path.strip_prefix(&build.catalog_dir).unwrap();

            if build.verbose {
                info!("Reading track {}", path_relative_to_catalog.display());
            }

            let transcodes = match cache.get_or_create_transcodes(build, path_relative_to_catalog, &extension) {
                Ok(transcodes) => transcodes,
                Err(err) => {
                    let error = format!("Skipping track {} due to decoding error ({err})", path_relative_to_catalog.display());
                    build.error(&error);
                    continue;
                }
            };

            let track = self.read_track(
                None,
                Vec::new(),
                LocalOptions::new(),
                finalized_overrides,
                transcodes
            );

            release_tracks.push(track);
        }

        if !release_tracks.is_empty() {
            // Process bare image paths into ImageRc representations
            let images: Vec<ImageRcView> = fair_dir.image_files
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

            // TODO: Print warning if all tracks have track numbers as tags but they don't start at 0/1 and don't increase monotonically
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
                    (None, None) => {
                        // If both tracks have no track number, sort by original source file name instead
                        let file_name_a = track_a.transcodes.file_meta.path.file_name().unwrap();
                        let file_name_b = track_b.transcodes.file_meta.path.file_name().unwrap();

                        file_name_a.cmp(file_name_b)
                    }
                }
            });

            let mut main_artists_to_map: Vec<String> = Vec::new();
            let mut support_artists_to_map: Vec<String> = Vec::new();

            // This sets main_artists_to_map and support_artists_to_map in
            // one of three ways, see comments in branches
            if !finalized_overrides.release_artists.is_empty() {
                // Here, main_artists_to_map is set manually through manifest metadata.
                for artist_name in &finalized_overrides.release_artists {
                    main_artists_to_map.push(artist_name.to_string());
                }

                // All artists that were associated with a track but not
                // manually set as main_artists_to_map are now added as
                // support_artists_to_map.
                for release_track in &release_tracks {
                    for track_artist_to_map in &release_track.artists_to_map {
                        if !main_artists_to_map.contains(track_artist_to_map) && !support_artists_to_map.contains(track_artist_to_map) {
                            support_artists_to_map.push(track_artist_to_map.clone());
                        }
                    }
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

                // All artists that were associated with a track but not
                // set as "album artist" on any of them are now added as
                // support_artists_to_map.
                for release_track in &release_tracks {
                    for track_artist_to_map in &release_track.artists_to_map {
                        if !main_artists_to_map.contains(track_artist_to_map) && !support_artists_to_map.contains(track_artist_to_map) {
                            support_artists_to_map.push(track_artist_to_map.clone());
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

            let title = match local_options.title {
                Some(title) => title,
                None => {
                    // To implicitly obtain the release title we get
                    // the 'album' metadata from each track in a release. As
                    // each track in a release could have a different 'album'
                    // specified, we count how often each distinct 'album'
                    // tag is present on a track in the release, and then
                    // when we create the release struct, we assign
                    // the 'album' title we've encountered most. (and this is
                    // what release_title_metrics is for => Vec<count, title>)
                    let mut release_title_metrics: Vec<(u32, String)> = Vec::new();

                    for track in &release_tracks {
                        if let Some(release_title) = &track.transcodes.borrow().source_meta.album {
                            if let Some(metric) = &mut release_title_metrics
                                .iter_mut()
                                .find(|(_count, title)| title == release_title) {
                                metric.0 += 1;
                            } else {
                                release_title_metrics.push((1, release_title.to_string()));
                            }
                        }
                    }

                    // Sort most often occuring title to the end of the Vec
                    release_title_metrics.sort_by(|a, b| a.0.cmp(&b.0));

                    release_title_metrics
                        .pop()
                        .map(|(_count, title)| title)
                        .unwrap_or_else(||
                            fair_dir.path
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string()
                        )

                }
            };

            let cover = match &local_options.cover {
                Some(described_image) => Some(described_image.clone()),
                None => pick_best_cover_image(&images)
            };

            if cover.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
                warn_discouraged!("The cover image for release '{}' is missing an image description.", title);
                build.missing_image_descriptions = true;
            }

            let mut extras = Vec::new();
            for image in images {
                if let Some(ref described_image) = cover {
                    // If the image we're iterating is the cover image for this release
                    // we don't include it as an extra (as it would be redundant).
                    if image.file_meta.path == described_image.file_meta.path {
                        continue
                    }
                }

                let extra = Extra::new(image.file_meta.clone());
                extras.push(extra);
            }

            for extra_path in fair_dir.extra_files {
                let path_relative_to_catalog = extra_path.strip_prefix(&build.catalog_dir).unwrap();
                let file_meta = FileMeta::new(build, path_relative_to_catalog);
                extras.push(Extra::new(file_meta));
            }

            let download_access = finalized_overrides.release_download_access.assemble(
                finalized_overrides,
                &finalized_overrides.release_price
            );

            let release_dir_relative_to_catalog = fair_dir.path.strip_prefix(&build.catalog_dir).unwrap().to_path_buf();

            let release = Release::new(
                finalized_overrides.copy_link,
                cover,
                local_options.release_date.take(),
                download_access,
                finalized_overrides.release_downloads.clone(),
                finalized_overrides.embedding,
                finalized_overrides.release_extras.clone(),
                extras,
                mem::take(&mut local_options.links),
                finalized_overrides.m3u_enabled,
                main_artists_to_map,
                local_options.more.take(),
                finalized_overrides.more_label.clone(),
                local_options.permalink.take(),
                release_dir_relative_to_catalog,
                finalized_overrides.speed_controls,
                support_artists_to_map,
                local_options.synopsis.take(),
                finalized_overrides.theme.clone(),
                title.to_string(),
                finalized_overrides.track_numbering.clone(),
                release_tracks,
                local_options.unlisted_release
            );

            self.releases.push(ReleaseRc::new(release));
        }
    }

    pub fn read_track(
        &mut self,
        cover: Option<DescribedImage>,
        extras: Vec<Extra>,
        mut local_options: LocalOptions,
        overrides: &Overrides,
        transcodes: TranscodesRcView
    ) -> Track {
        let artists_to_map = if !overrides.track_artists.is_empty() {
            overrides.track_artists.clone()
        } else {
            transcodes.borrow().source_meta.artists.to_vec()
        };

        let download_access = overrides.track_download_access.assemble(
            overrides,
            &overrides.track_price
        );

        let theme = overrides.theme.clone();

        Track::new(
            artists_to_map,
            overrides.copy_link,
            cover,
            download_access,
            overrides.track_downloads.clone(),
            overrides.embedding,
            overrides.track_extras,
            extras,
            local_options.links,
            local_options.more.take(),
            // TODO: There is a general design issue here: Overriding (= inheriting across
            // catalog/artist/release/track) the more_label makes sense from the perspective
            // of a generic term like "Learn more", but it's the wrong behavior if it's used
            // in a matter of "About the label", "About the album", "lyrics", etc.
            // Possibly needs different ways of specifying the more_label generically or for
            // catalog/release/track/artist specifically (without inheritance or with "targeted"
            // inheritance towards certain child nodes). But this needs to be carefully considered
            // as to stay manageable/compatible with potential future GUI usage.
            overrides.more_label.clone(),
            overrides.speed_controls,
            overrides.streaming_quality,
            local_options.synopsis.take(),
            overrides.tag_agenda.clone(),
            theme,
            local_options.title.take(),
            transcodes
        )
    }

    fn read_track_dir(
        &mut self,
        build: &mut Build,
        cache: &mut Cache,
        fair_dir: FairDir,
        parent_overrides: &Overrides
    ) -> Option<Track> {
        let mut local_options = LocalOptions::new();
        let mut local_overrides = None;

        if let Some(track_manifest) = &fair_dir.track_manifest {
            if build.verbose {
                info!("Reading track manifest {}", track_manifest.display());
            }
            manifest::read_track_manifest(
                build,
                cache,
                &fair_dir.path,
                &mut local_options,
                track_manifest,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }

        let finalized_overrides = local_overrides.as_ref().unwrap_or(parent_overrides);

        let audio_path = fair_dir.audio_files.first().unwrap();

        let extension = audio_path.extension().unwrap().to_str().unwrap().to_lowercase().as_str().to_string();
        let path_relative_to_catalog = audio_path.strip_prefix(&build.catalog_dir).unwrap();

        if build.verbose {
            info!("Reading track {}", path_relative_to_catalog.display());
        }

        let transcodes = match cache.get_or_create_transcodes(build, path_relative_to_catalog, &extension) {
            Ok(transcodes) => transcodes,
            Err(err) => {
                let error = format!("Skipping track {} due to decoding error ({err})", path_relative_to_catalog.display());
                build.error(&error);
                return None;
            }
        };

        // Process bare image paths into ImageRc representations
        let images: Vec<ImageRcView> = fair_dir.image_files
            .into_iter()
            .map(|image_path| {
                let path_relative_to_catalog = image_path.strip_prefix(&build.catalog_dir).unwrap();

                if build.verbose {
                    info!("Reading image {}", path_relative_to_catalog.display());
                }

                cache.get_or_create_image(build, path_relative_to_catalog)
            })
            .collect();

        let title = &local_options
            .title
            .as_ref()
            .cloned()
            .unwrap_or_else(||
                if let Some(album) = &transcodes.borrow().source_meta.album {
                    album.clone()
                } else {
                    transcodes.file_meta.path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            );

        let cover = match &local_options.cover {
            Some(described_image) => Some(described_image.clone()),
            None => pick_best_cover_image(&images)
        };

        if cover.as_ref().is_some_and(|described_image| described_image.description.is_none()) {
            warn_discouraged!("The cover image for track '{}' is missing an image description.", title);
            build.missing_image_descriptions = true;
        }

        let mut extras = Vec::new();
        for image in images {
            if let Some(ref described_image) = cover {
                // If the image we're iterating is the cover image for this release
                // we don't include it as an extra (as it would be redundant).
                if image.file_meta.path == described_image.file_meta.path {
                    continue
                }
            }

            let extra = Extra::new(image.file_meta.clone());
            extras.push(extra);
        }

        for extra_path in fair_dir.extra_files {
            let path_relative_to_catalog = extra_path.strip_prefix(&build.catalog_dir).unwrap();
            let file_meta = FileMeta::new(build, path_relative_to_catalog);
            extras.push(Extra::new(file_meta));
        }

        let track = self.read_track(
            cover,
            extras,
            local_options,
            finalized_overrides,
            transcodes
        );

        for dir_path in &fair_dir.dirs {
            // TODO: We could consider supporting artist directories as
            // subdirectories of track directories, as that somehow would
            // make sense.
            let error = format!("Subdirectories of track directories are currently not handled by faircamp, ignoring directory '{}'", dir_path.display());
            build.error(&error);
        }

        Some(track)
    }

    fn read_unknown_dir(
        &mut self,
        build: &mut Build,
        cache: &mut Cache,
        parent_overrides: &Overrides,
        path: &Path
    ) {
        let fair_dir = FairDir::read(build, path);

        if fair_dir.catalog_manifest.is_some() {
            let error = format!("A catalog.eno manifest may only be placed at the root of the catalog directory, however it was found in a subdirectory (at '{}'). Please move it to the folder '{}'", path.display(), build.catalog_dir.display());
            build.error(&error);
        }

        let mutually_exclusive_manifests =
            fair_dir.artist_manifest.is_some() as usize +
            fair_dir.release_manifest.is_some() as usize +
            fair_dir.track_manifest.is_some() as usize;

        if mutually_exclusive_manifests > 1 {
            let error = format!("A directory in a faircamp catalog may only ever contain a catalog.eno, release.eno or track.eno manifest (one of them), but the directory '{}' contains {mutually_exclusive_manifests} of these. The directory will be ignored until this is resolved.", path.display());
            build.error(&error);
            return;
        }

        if fair_dir.artist_manifest.is_some() {
            self.read_artist_dir(
                build,
                cache,
                fair_dir,
                parent_overrides
            );
            return;
        }

        if fair_dir.release_manifest.is_some() {
            self.read_release_dir(
                build,
                cache,
                fair_dir,
                parent_overrides
            );
            return;
        }

        if fair_dir.track_manifest.is_some() {
            let error = format!("A track.eno manifest may only be placed inside a track directory (that is, a subdirectory of a release directory), however it was found outside of any such directory configuration (at '{}'). Please move it accordingly.", path.display());
            build.error(&error);
            return;
        }

        if !fair_dir.audio_files.is_empty() {
            self.read_release_dir(
                build,
                cache,
                fair_dir,
                parent_overrides
            );
            return;
        }

        for dir_path in &fair_dir.dirs {
            self.read_unknown_dir(build, cache, parent_overrides, dir_path);
        }
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

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
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
    /// should be replaced with manually specified ones. Returns whether all
    /// permalinks were valid (i.e.: whether no conflicts were found).
    fn validate_permalinks(&self, build: &mut Build) -> bool {
        let mut no_conflicts = true;
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
                let error = format!("The {generated_or_assigned} permalink '{slug}' of the release '{title}' from directory '{release_dir}' conflicts with the {previous_usage_formatted}\n{PERMALINK_CONFLICT_RESOLUTION_HINT}");
                build.error(&error);
                no_conflicts = false;
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

                let resolution_hint = match &previous_usage {
                    PermalinkUsage::Artist(_) => {
                        indoc!(r#"
                            When two artist permalinks are in conflict, a likely cause is that it is actually one and the same artist,
                            whose name has just been spelled differently on different releases or tracks (e.g. "Alice" being spelled as
                            "alice" or "Älicë" too). In such cases there are three possible solutions:

                            1. Unify/correct the tags to use the same spelling on all audio files (e.g. using a tag editor)

                            2. Expliclity define the artist in an artist.eno manifest, defining aliases for them:

                               name: Alice
                               aliases:
                               - alice
                               - Älicë

                            3. Explicitly define the artist in a catalog.eno manifest using a shortcut artist definition with aliases:

                               artist:
                               name = Alice
                               alias = alice
                               alias = Älicë

                            If in your case there are actually two separate artists whose permalinks just happen to conflict,
                            use the 'permalink: example' option to manually specify a permalink on at least one of them to
                            resolve the conflict.
                        "#)
                    }
                    PermalinkUsage::Release(_) => PERMALINK_CONFLICT_RESOLUTION_HINT
                };

                let error = formatdoc!("
                    Two permalinks are in conflict (= two different pages are competing for the same URL):

                    A) The artist '{name}' has the {generated_or_assigned} permalink '{slug}'
                    B) {previous_usage_formatted}

                    {resolution_hint}
                ");

                build.error(&error);
                no_conflicts = false;
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

        no_conflicts
    }

    /// Writes all images (catalog home image, release/track covers, theme
    /// background images) and streaming audio files.
    pub fn write_assets(&mut self, build: &mut Build, cache: &mut Cache) {
        // Write catalog theme background image
        if let Some(image) = &self.theme.background_image {
            write_background_image(build, image);
        }

        if let Some(described_image) = &self.home_image {
            let mut image_mut = described_image.borrow_mut();
            let source_path = &described_image.file_meta.path;
            // Write home image as poster image for homepage
            let poster_assets = image_mut.artist_assets(build, source_path);

            for asset in &poster_assets.all() {
                let target_filename = asset.target_filename();

                util::hard_link_or_copy(
                    build.cache_dir.join(&asset.filename),
                    build.build_dir.join(&target_filename)
                );

                build.reserve_filename(target_filename);
                build.stats.add_image(asset.filesize_bytes);
            }

            // Write home image as feed image
            if build.base_url.is_some() && self.feeds.any_requested() {
                let source_path = &described_image.file_meta.path;
                let feed_image_asset = image_mut.feed_asset(build, source_path);

                util::hard_link_or_copy(
                    build.cache_dir.join(&feed_image_asset.filename),
                    build.build_dir.join(FeedImageAsset::TARGET_FILENAME)
                );

                build.reserve_filename(FeedImageAsset::TARGET_FILENAME);
                build.stats.add_image(feed_image_asset.filesize_bytes);
            }

            image_mut.persist_to_cache(&build.cache_dir);
        }

        for artist in self.featured_artists.iter_mut() {
            let artist_ref = artist.borrow();

            if let Some(described_image) = &artist_ref.image {
                // Write artist dir
                let artist_dir = build.build_dir.join(&artist_ref.permalink.slug);
                build.reserve_filename(artist_ref.permalink.slug.clone());
                util::ensure_dir_all(&artist_dir);

                // Write artist image as poster image

                let mut image_mut = described_image.borrow_mut();
                let source_path = &described_image.file_meta.path;
                let poster_assets = image_mut.artist_assets(build, source_path);

                for asset in &poster_assets.all() {
                    util::hard_link_or_copy(
                        build.cache_dir.join(&asset.filename),
                        artist_dir.join(asset.target_filename())
                    );

                    build.stats.add_image(asset.filesize_bytes);
                }

                image_mut.persist_to_cache(&build.cache_dir);
            }

            // Write artist theme background image
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

            // Write release dir
            let release_dir = build.build_dir.join(&release_mut.permalink.slug);
            build.reserve_filename(release_mut.permalink.slug.clone());
            util::ensure_dir_all(&release_dir);

            // Write release theme background image
            if let Some(image) = &release_mut.theme.background_image {
                write_background_image(build, image);
            }

            // Write release cover image
            if let Some(described_image) = &release_mut.cover {
                let mut image_mut = described_image.borrow_mut();
                let source_path = &described_image.file_meta.path;
                let cover_assets = image_mut.cover_assets(build, source_path);

                for asset in &cover_assets.all() {
                    util::hard_link_or_copy(
                        build.cache_dir.join(&asset.filename),
                        release_dir.join(asset.target_filename())
                    );

                    build.stats.add_image(asset.filesize_bytes);
                }

                image_mut.persist_to_cache(&build.cache_dir);
            } else {
                let procedural_cover = cache.get_or_create_procedural_cover(
                    build,
                    &release_mut.theme.cover_generator,
                    max_tracks_in_release,
                    &release_mut,
                );

                {
                    let mut procedural_cover_mut = procedural_cover.borrow_mut();

                    let mut write_to_build = |asset: &ProceduralCoverAsset, target_filename: &str| {
                        util::hard_link_or_copy(
                            build.cache_dir.join(&asset.filename),
                            release_dir.join(target_filename)
                        );
                        build.stats.add_image(asset.filesize_bytes);
                    };

                    write_to_build(&procedural_cover_mut.asset_120, ProceduralCover::FILENAME_120);
                    write_to_build(&procedural_cover_mut.asset_240, ProceduralCover::FILENAME_240);
                    write_to_build(&procedural_cover_mut.asset_480, ProceduralCover::FILENAME_480);
                    write_to_build(&procedural_cover_mut.asset_720, ProceduralCover::FILENAME_720);

                    procedural_cover_mut.unmark_stale();
                }

                release_mut.procedural_cover = Some(procedural_cover);
            }

            let cover_path = release_mut.cover
                .as_ref()
                .map(|described_image| build.catalog_dir.join(&described_image.file_meta.path));

            let release_slug = release_mut.permalink.slug.clone();

            let tag_mappings: Vec<TagMapping> = release_mut.tracks.iter().zip(TRACK_NUMBERS)
                .map(|(track, track_number)| TagMapping::new(&release_mut, track, track_number))
                .collect();

            for ((track, tag_mapping), track_number) in release_mut.tracks.iter_mut().zip(tag_mappings.iter()).zip(TRACK_NUMBERS) {
                let track_dir = release_dir.join(track_number.to_string());

                util::ensure_dir_all(&track_dir);

                // Write track theme background image
                if let Some(image) = &track.theme.background_image {
                    write_background_image(build, image);
                }

                // Write track cover image
                if let Some(described_image) = &track.cover {
                    let mut image_mut = described_image.borrow_mut();
                    let source_path = &described_image.file_meta.path;
                    let cover_assets = image_mut.cover_assets(build, source_path);

                    for asset in &cover_assets.all() {
                        util::hard_link_or_copy(
                            build.cache_dir.join(&asset.filename),
                            track_dir.join(asset.target_filename())
                        );

                        build.stats.add_image(asset.filesize_bytes);
                    }

                    image_mut.persist_to_cache(&build.cache_dir);
                }

                // Write track streaming audio files
                for streaming_format in track.streaming_quality.formats() {
                    let streaming_format_dir = track_dir.join(streaming_format.asset_dirname());

                    util::ensure_dir_all(&streaming_format_dir);

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

                    let hash = build.hash_with_salt(|hasher| {
                        release_slug.hash(hasher);
                        track_number.hash(hasher);
                        streaming_format.asset_dirname().hash(hasher);
                        track_filename.hash(hasher);
                    });

                    let hash_dir = streaming_format_dir.join(hash);

                    util::ensure_dir_all(&hash_dir);

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

            release_mut.write_downloadable_files(build);
        }
    }

    // Writing user-provided extra site assets entails detecting collisions
    // against all the directories/files we wrote ourselves already (which can
    // only be done after we have written them, hence this should be called
    // as the last build step).
    pub fn write_user_assets(&mut self, build: &mut Build) -> Result<(), Vec<String>> {
        let mut collisions = Vec::new();

        for site_asset in self.site_assets.iter_mut() {
            if build.reserve_filename(site_asset.filename.clone()) {
                util::hard_link_or_copy(
                    &site_asset.path,
                    build.build_dir.join(&site_asset.filename)
                );
            } else {
                collisions.push(site_asset.filename.clone());
            }
        }

        match collisions.is_empty() {
            true => Ok(()),
            false => Err(collisions)
        }
    }
}
