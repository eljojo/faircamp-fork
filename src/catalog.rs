use sanitize_filename::sanitize;
use std::fs;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::{
    Artist,
    AssetIntent,
    Build,
    Cache,
    DownloadOption,
    Image,
    manifest::{self, LocalOptions, Overrides},
    Permalink,
    PermalinkUsage,
    Release,
    TagMapping,
    Track,
    TrackAssets,
    util
};

// TODO: Research if aac as input is easily possible, alternatively use ffmpeg to transcode it as a slow but functional workaround
const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["aiff", "flac", "mp3", "ogg", "opus", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["gif", "heif", "jpeg", "jpg", "png", "webp"];

const PERMALINK_CONFLICT_RESOLUTION_HINT: &str = "Hint: In order to resolve the conflict, explicitly specify non-conflicting permalinks for all involved artists/releases through metadata (see faircamp's README.md)";

#[derive(Debug)]
pub struct Catalog {
    /// Stores the primary artist for "single artist" catalogs
    pub artist: Option<Rc<RefCell<Artist>>>,
    /// All artists (main_artists + support_artists)
    pub artists: Vec<Rc<RefCell<Artist>>>,
    /// Whether support artists should get their own
    /// pages and be linked to them
    pub feature_support_artists: bool,
    /// Those artists that get their own page
    pub featured_artists: Vec<Rc<RefCell<Artist>>>,
    pub feed_image: Option<Rc<RefCell<Image>>>,
    pub home_image: Option<Rc<RefCell<Image>>>,
    pub label_mode: bool,
    pub main_artists: Vec<Rc<RefCell<Artist>>>,
    pub releases: Vec<Rc<RefCell<Release>>>,
    pub show_support_artists: bool,
    pub support_artists: Vec<Rc<RefCell<Artist>>>,
    pub text: Option<String>,
    title: Option<String>
}

/// Gets passed the images found in a release directory. Checks against a few
/// hardcoded filenames (the usual suspects) to determine which image is most
/// likely to be the intended release cover image.
fn pick_best_cover_image(images: Vec<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Image>>> {
    let mut cover_candidate_option: Option<(usize, _)> = None;

    for image in images {
        let priority = match image.borrow()
            .assets.borrow()
            .source_file_signature
            .path.file_stem().unwrap().to_str().unwrap() {
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

    cover_candidate_option.map(|cover_candidate| cover_candidate.1)
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

                    format!("{} - ", list)
                };
                let track_number = index + 1;
                let track_title = sanitize(&track.title);

                let track_basename = format!("{track_number:02} {track_artists}{track_title}");

                track.asset_basename = Some(track_basename);
            }
        }
    }

    pub fn create_artist(&mut self, name: &str) -> Rc<RefCell<Artist>> {
        let artist = Rc::new(RefCell::new(Artist::new(name)));
        self.artists.push(artist.clone());
        artist
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
                        if !release_mut.main_artists.iter().any(|main_artist| Rc::ptr_eq(main_artist, artist)) {
                            artist_mut.releases.push(release.clone());
                            release_mut.main_artists.push(artist.clone());
                        }

                        // Only assign artist to catalog's main artists if it hasn't already been assigned to them
                        if !self.main_artists.iter().any(|main_artist| Rc::ptr_eq(main_artist, artist)) {
                            self.main_artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = Rc::new(RefCell::new(Artist::new(&main_artist_to_map)));
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
                        if !release_mut.support_artists.iter().any(|support_artist| Rc::ptr_eq(support_artist, artist)) {
                            artist_mut.releases.push(release.clone());
                            release_mut.support_artists.push(artist.clone());
                        }

                        // Only assign artist to catalog's support artists if it hasn't already been assigned to them
                        if !self.support_artists.iter().any(|support_artist| Rc::ptr_eq(support_artist, artist)) {
                            self.support_artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = Rc::new(RefCell::new(Artist::new(&support_artist_to_map)));
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
                            if !track.artists.iter().any(|track_artist| Rc::ptr_eq(track_artist, artist)) {
                                track.artists.push(artist.clone());
                            }
                        }
                    }

                    if !any_artist_found {
                        // TODO: An artist created here curiously belongs neither to catalog.main_artists,
                        //       nor catalog.support_artists. This might indicate that in fact we never
                        //       enter into this branch at all?
                        let new_artist = Rc::new(RefCell::new(Artist::new(&track_artist_to_map)));
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
            feature_support_artists: false,
            featured_artists: Vec::new(),
            feed_image: None,
            home_image: None,
            label_mode: false,
            main_artists: Vec::new(),
            releases: Vec::new(),
            show_support_artists: false,
            support_artists: Vec::new(),
            text: None,
            title: None
        }
    }
    
    pub fn read(build: &mut Build, cache: &mut Cache) -> Result<Catalog, ()> {
        let mut catalog = Catalog::new();
        
        catalog.read_dir(&build.catalog_dir.clone(), build, cache, &Overrides::default()).unwrap();
        
        if let Some(markdown) = catalog.text.take() {
            catalog.text = Some(util::markdown_to_html(&markdown));
        }

        catalog.map_artists();

        if catalog.label_mode {
            catalog.featured_artists.extend(catalog.main_artists.iter().cloned());

            if catalog.feature_support_artists {
                for support_artist in &catalog.support_artists {
                    // Only assign support artist to catalog's featured artists if
                    // it hasn't already been assigned to them as a main artist
                    if !catalog.featured_artists.iter().any(|featured_artist| Rc::ptr_eq(featured_artist, support_artist)) {
                        catalog.featured_artists.push(support_artist.clone());
                    }
                }
            }
        } else {
            catalog.set_artist();
        }
        
        if !catalog.validate_permalinks() { return Err(()); }

        catalog.compute_asset_basenames();
        
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
        
        let mut images = Vec::new();
        // We get the 'album' metadata from each track in a release. As each track in a
        // release could have a different 'album' specified, we count how often each
        // distinct 'album' tag is present on a track in the release, and then when we
        // create the release struct, we assign the 'album' title we've encountered most.
        // (this is what release_title_metrics is for => Vec<count, title>)
        let mut release_title_metrics: Vec<(u32, String)> = Vec::new();
        let mut release_tracks: Vec<Track> = Vec::new();
        
        let mut dir_paths: Vec<PathBuf> = Vec::new();
        let mut image_paths: Vec<PathBuf> = Vec::new();
        let mut meta_paths: Vec<PathBuf> = Vec::new();
        let mut track_paths: Vec<(PathBuf, String)> = Vec::new();
        
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

                                if let Some(extension) = path
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
                                    } else {
                                        warn!("Ignoring unsupported file '{}'", path.display());
                                    }
                                } else {
                                    warn!("Ignoring unsupported file '{}'", path.display());
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
        
        for (track_path, extension) in &track_paths {
            let path_relative_to_catalog = track_path.strip_prefix(&build.catalog_dir).unwrap();

            if build.verbose {
                info!("Reading track {}", path_relative_to_catalog.display());
            }
            
            let assets = cache.get_or_create_track_assets(build, path_relative_to_catalog, extension);
            
            if let Some(release_title) = &assets.borrow().source_meta.album {
                if let Some(metric) = &mut release_title_metrics
                    .iter_mut()
                    .find(|(_count, title)| title == release_title) {
                    metric.0 += 1;
                } else {
                    release_title_metrics.push((1, release_title.to_string()));
                }
            }
            
            let track = self.read_track(
                track_path,
                local_overrides.as_ref().unwrap_or(parent_overrides),
                assets
            );
            
            release_tracks.push(track);
        }
        
        for image_path in &image_paths {
            let path_relative_to_catalog = image_path.strip_prefix(&build.catalog_dir).unwrap();

            if build.verbose {
                info!("Reading image {}", path_relative_to_catalog.display());
            }
            
            let assets = cache.get_or_create_image_assets(build, path_relative_to_catalog);
            
            images.push(Rc::new(RefCell::new(Image::new(assets, None))));
        }
        
        if !release_tracks.is_empty() {
            let assets = cache.get_or_create_release_assets(&release_tracks);
            
            release_tracks.sort_by(|a, b|
                a.assets.borrow().source_meta.track_number.cmp(
                    &b.assets.borrow().source_meta.track_number
                )
            );
            release_title_metrics.sort_by(|a, b| a.0.cmp(&b.0)); // sort most often occuring title to the end of the Vec
            
            let mut main_artists_to_map: Vec<String> = Vec::new();
            let mut support_artists_to_map: Vec<String> = Vec::new();

            // This sets main_artists_to_map in one of three ways, see comments in branches
            if let Some(artist_names) = &local_overrides.as_ref().unwrap_or(parent_overrides).release_artists {
                // Here, main_artists_to_map is set manually through manifest metadata
                for artist_name in artist_names {
                    main_artists_to_map.push(artist_name.to_string());
                }
            } else if release_tracks
                .iter()
                .any(|track| !track.assets.borrow().source_meta.album_artist.is_empty()) {
                // Here, main_artists_to_map is set through "album artist" tags found on at least one track
                for release_track in &release_tracks {
                    let album_artist = &release_track.assets.borrow().source_meta.album_artist;

                    for artist in album_artist {
                        if !main_artists_to_map.contains(artist) {
                            main_artists_to_map.push(artist.clone());
                        }
                    }
                }
            } else {
                // Here, main_artists_to_map is set through finding the artist(s) that appear in the "artist" tag on the highest number of tracks
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

                track_artist_metrics.sort_by(|a, b| b.0.cmp(&a.0)); // sort most often occuring artist(s) to the start of the Vec

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

            if local_overrides.as_ref().unwrap_or(parent_overrides).embedding {
                build.embeds_requested = true;
            }

            let cover = match &local_overrides.as_ref().unwrap_or(parent_overrides).release_cover {
                Some(image) => Some(image.clone()),
                None => pick_best_cover_image(images)
            };
            
            let release = Release::new(
                assets,
                cover,
                local_options.release_date,
                main_artists_to_map,
                local_overrides.as_ref().unwrap_or(parent_overrides),
                local_options.release_permalink,
                support_artists_to_map,
                title.to_string(),
                release_tracks
            );

            self.releases.push(Rc::new(RefCell::new(release)));
        } else if !images.is_empty() {
            // This dir is not a release dir (no tracks found), but it contains images.
            // Consider whether there might be anything to do with these images?
        }
        
        for dir_path in &dir_paths {
            self.read_dir(dir_path, build, cache, local_overrides.as_ref().unwrap_or(parent_overrides)).unwrap();
        }

        Ok(())
    }

    pub fn read_track(
        &mut self,
        path: &Path,
        overrides: &Overrides,
        assets: Rc<RefCell<TrackAssets>>
    ) -> Track {
        let artists_to_map = if let Some(artist_names) = &overrides.track_artists {
            artist_names.to_vec()
        } else {
            assets.borrow().source_meta.artist.to_vec()
        };
        
        let title = assets.borrow().source_meta.title
            .as_ref()
            .cloned()
            .unwrap_or(path.file_stem().unwrap().to_str().unwrap().to_string());
        
        Track::new(artists_to_map, assets, title)
    }

    // TODO: Should we have a manifest option for setting the catalog.artist manually in edge cases?
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
                        .any(|release_main_artist| Rc::ptr_eq(release_main_artist, artist)) {
                        num_releases += 1;
                    }
                    for track in &release_ref.tracks {
                        if track.artists
                            .iter()
                            .any(|track_artist| Rc::ptr_eq(track_artist, artist)) {
                            num_tracks += 1;
                        }
                    }
                }
                (artist.clone(), num_releases, num_tracks)
            })
            .collect::<Vec<(Rc<RefCell<Artist>>, usize, usize)>>();

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

    fn validate_permalinks(&mut self) -> bool {
        let mut generated_permalinks = (None, None, None, 0);
        let mut used_permalinks = HashMap::new();

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

        let mode = |permalink: &Permalink| -> &str {
            if permalink.generated { "auto-generated" } else { "user-assigned" }
        };

        let format_previous_usage = |previous_usage: &PermalinkUsage| -> String {
            match previous_usage {
                PermalinkUsage::Artist(artist) => {
                    let artist_ref = artist.borrow();
                    format!("the {} permalink of the artist '{}'", mode(&artist_ref.permalink), artist_ref.name)
                }
                PermalinkUsage::Release(release) => {
                    let release_ref = release.borrow();
                    format!("the {} permalink of the release '{}'", mode(&release_ref.permalink), release_ref.title)
                }
            }
        };

        for release in &self.releases {
            let release_ref = release.borrow();

            if let Some(previous_usage) = used_permalinks.get(&release_ref.permalink.slug) {
                let generated_or_assigned = mode(&release_ref.permalink);
                let slug = &release_ref.permalink.slug;
                let title = &release_ref.title;
                let previous_usage_formatted = format_previous_usage(previous_usage);
                let message = format!("The {generated_or_assigned} permalink '{slug}' of the release '{title}' conflicts with {previous_usage_formatted}");
                error!("{}\n{}", message, PERMALINK_CONFLICT_RESOLUTION_HINT);
                return false;
            } else {
                let usage = PermalinkUsage::Release(release);
                if release_ref.permalink.generated { add_generated_usage(&usage); }
                used_permalinks.insert(release_ref.permalink.slug.to_string(), usage);
            }
        }
        
        // TODO: We could think about validating this even for non-featured
        // artists already(especially, or maybe only if their permalinks were
        // user-assigned). This way the behavior would be a bit more stable
        // when someone suddenly "flips the switch" on label_mode and/or
        // feature_supported_artists.
        for artist in &self.featured_artists {
            let artist_ref = artist.borrow();
            if let Some(previous_usage) = used_permalinks.get(&artist_ref.permalink.slug) {
                let generated_or_assigned = mode(&artist_ref.permalink);
                let slug = &artist_ref.permalink.slug;
                let name = &artist_ref.name;
                let previous_usage_formatted = format_previous_usage(previous_usage);
                let message = format!("The {generated_or_assigned} permalink '{slug}' of the artist '{name}' conflicts with {previous_usage_formatted}");
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
        if let Some(background_image) = &build.theme.background_image {
            let background_image_mut = background_image.borrow_mut();
            let mut background_image_assets_mut = background_image_mut.assets.borrow_mut();
            let image_asset = background_image_assets_mut.background_asset(build, AssetIntent::Deliverable);
            
            util::hard_link_or_copy(
                build.cache_dir.join(&image_asset.filename),
                build.build_dir.join("background.jpg")
            );
            
            build.stats.add_image(image_asset.filesize_bytes);
            
            background_image_assets_mut.persist_to_cache(&build.cache_dir);
        }

        if let Some(feed_image) = &self.feed_image {
            let feed_image_mut = feed_image.borrow_mut();
            let mut feed_image_assets_mut = feed_image_mut.assets.borrow_mut();
            let image_asset = feed_image_assets_mut.feed_asset(build, AssetIntent::Deliverable);
            
            util::hard_link_or_copy(
                build.cache_dir.join(&image_asset.filename),
                build.build_dir.join("feed.jpg")
            );
            
            build.stats.add_image(image_asset.filesize_bytes);
            
            feed_image_assets_mut.persist_to_cache(&build.cache_dir);
        }

        if let Some(home_image) = &self.home_image {
            let image_mut = home_image.borrow_mut();
            let mut image_assets_mut = image_mut.assets.borrow_mut();
            let poster_assets = image_assets_mut.artist_asset(build, AssetIntent::Deliverable);

            for asset in &poster_assets.all() {
                util::hard_link_or_copy(
                    build.cache_dir.join(&asset.filename),
                    // TODO: Address the ugly __home__ hack soon
                    build.build_dir.join(format!("{}_{}_{}x{}.jpg", "__home__", asset.format, asset.width, asset.height))
                );

                build.stats.add_image(asset.filesize_bytes);
            }

            image_assets_mut.persist_to_cache(&build.cache_dir);
        }

        for artist in self.featured_artists.iter_mut() {
            let mut artist_mut = artist.borrow_mut();

            let permalink = artist_mut.permalink.slug.to_string();
            if let Some(image) = &mut artist_mut.image {
                let image_mut = image.borrow_mut();
                let mut image_assets_mut = image_mut.assets.borrow_mut();
                let poster_assets = image_assets_mut.artist_asset(build, AssetIntent::Deliverable);

                for asset in &poster_assets.all() {
                    util::hard_link_or_copy(
                        build.cache_dir.join(&asset.filename),
                        build.build_dir.join(format!("{}_{}_{}x{}.jpg", &permalink, asset.format, asset.width, asset.height))
                    );

                    build.stats.add_image(asset.filesize_bytes);
                }

                image_assets_mut.persist_to_cache(&build.cache_dir);
            }
        }

        let max_tracks_in_release = self.releases
            .iter()
            .map(|release| release.borrow().tracks.len())
            .max()
            .unwrap();

        for release in &self.releases {
            let mut release_mut = release.borrow_mut();

            let release_dir = build.build_dir.join(&release_mut.permalink.slug);

            util::ensure_dir(&release_dir);

            if let Some(image) = &mut release_mut.cover {
                let image_mut = image.borrow_mut();
                let mut image_assets_mut = image_mut.assets.borrow_mut();
                let cover_assets = image_assets_mut.cover_asset(build, AssetIntent::Deliverable);

                for asset in &cover_assets.all() {
                    util::hard_link_or_copy(
                        build.cache_dir.join(&asset.filename),
                        release_dir.join(format!("cover_{}.jpg", asset.edge_size))
                    );

                    build.stats.add_image(asset.filesize_bytes);
                }

                image_assets_mut.persist_to_cache(&build.cache_dir);
            } else {
                let svg = release_mut.generate_cover(&build.theme, max_tracks_in_release);
                fs::write(release_dir.join("cover.svg"), svg).unwrap();
            }
            
            let streaming_format = release_mut.streaming_format;

            let mut tag_mapping_option = if release_mut.rewrite_tags {
                Some(TagMapping {
                    album: Some(release_mut.title.clone()),
                    album_artist: if release_mut.main_artists.is_empty() {
                        None
                    } else {
                        Some(
                            release_mut.main_artists
                            .iter()
                            .map(|artist| artist.borrow().name.clone())
                            .collect::<Vec<String>>()
                            .join(", ")
                        )
                    },
                    artist: None,
                    title: None,
                })
            } else {
                None
            };

            let streaming_format_dir = build.build_dir
                .join(&release_mut.permalink.slug)
                .join(streaming_format.asset_dirname());

            util::ensure_dir(&streaming_format_dir);

            let release_slug = release_mut.permalink.slug.clone();

            for track in release_mut.tracks.iter_mut() {
                if let Some(tag_mapping) = &mut tag_mapping_option {
                    tag_mapping.artist = if track.artists.is_empty() {
                        None
                    } else {
                        Some(
                            track.artists
                            .iter()
                            .map(|artist| artist.borrow().name.clone())
                            .collect::<Vec<String>>()
                            .join(", ")
                        )
                    };
                    tag_mapping.title = Some(track.title.clone());
                }

                track.transcode_as(
                    streaming_format,
                    build,
                    AssetIntent::Deliverable,
                    &tag_mapping_option
                );

                let track_filename = format!(
                    "{basename}{extension}",
                    basename = track.asset_basename.as_ref().unwrap(),
                    extension = streaming_format.extension()
                );

                let hash = build.hash(
                    &release_slug,
                    streaming_format.asset_dirname(),
                    &track_filename
                );

                let hash_dir = streaming_format_dir.join(hash);

                util::ensure_dir(&hash_dir);

                let track_assets_ref = track.assets.borrow();
                let streaming_asset = track_assets_ref.get(streaming_format).as_ref().unwrap();

                util::hard_link_or_copy(
                    build.cache_dir.join(&streaming_asset.filename),
                    hash_dir.join(track_filename)
                );
                
                build.stats.add_track(streaming_asset.filesize_bytes);
                
                track.assets.borrow().persist_to_cache(&build.cache_dir);
            }

            if release_mut.download_option != DownloadOption::Disabled {
                release_mut.write_downloadable_files(build);
            }
        }
    }
}
