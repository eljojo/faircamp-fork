use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    rc::Rc
};

use crate::{
    artist::Artist,
    asset_cache::{AssetIntent, CacheManifest},
    build::Build,
    image::Image,
    image_format::ImageFormat,
    manifest::{LocalOptions, Overrides},
    permalink::{Permalink, PermalinkUsage},
    release::Release,
    track::{CachedTrackAssets, Track},
    manifest,
    util
};

// TODO: Verify if ogg even works already
// TODO: See if aac, aiff as input is easily possible
const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "opus", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "png"];

const PERMALINK_CONFLICT_RESOLUTION_HINT: &str = "Hint: In order to resolve the conflict, explicitly specify non-conflicting permalinks for all involved artists/releases through metadata (see faircamp's README.md)";

#[derive(Debug)]
pub struct Catalog {
    pub artists: Vec<Rc<RefCell<Artist>>>,
    // Contains an absolute path to the file (validity is checked when reading manifests)
    pub feed_image: Option<PathBuf>,
    pub images: Vec<Rc<RefCell<Image>>>, // TODO: Do we need these + what to do with them (also consider "label cover" aspect)
    pub releases: Vec<Release>,
    pub text: Option<String>,
    title: Option<String>
}

/// Gets passed the images found in a release directory. Checks against a few
/// hardcoded filenames (the usual suspects) to determine which image is most
/// likely to be the intended release cover image.
fn pick_best_cover_image(images: Vec<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<Image>>> {
    let mut cover_candidate_option: Option<(usize, _)> = None;

    for image in images {
        let priority = match image.borrow().source_file.file_stem().unwrap().to_str().unwrap() {
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

    cover_candidate_option.map(|cover_candidate| cover_candidate.1.clone())
}

impl Catalog {
    pub fn create_artist(&mut self, name: &str) -> Rc<RefCell<Artist>> {
        let artist = Rc::new(RefCell::new(Artist::new(name)));
        self.artists.push(artist.clone());
        artist
    }

    // TODO: Here or earlier ensure that the artist names don't collide
    fn map_artists(&mut self) {
        for release in self.releases.iter_mut() {
            for release_artist_to_map in release.artists_to_map.drain(..) {
                let release_artist_to_map_lowercase = release_artist_to_map.to_lowercase();
                let mut any_artist_found = false;
                for artist in &self.artists {
                    let artist_ref = artist.borrow();
                    if artist_ref.name.to_lowercase() == release_artist_to_map_lowercase ||
                        artist_ref.aliases.iter().any(|alias| alias.to_lowercase() == release_artist_to_map_lowercase) {
                        any_artist_found = true;

                        // Only assign artist to release if it hasn't already been assigned to it
                        if !release.artists.iter().any(|release_artist| Rc::ptr_eq(release_artist, artist)) {
                            release.artists.push(artist.clone());
                        }
                    }
                }

                if !any_artist_found {
                    let new_artist = Rc::new(RefCell::new(Artist::new(&release_artist_to_map)));
                    self.artists.push(new_artist.clone());
                    release.artists.push(new_artist.clone());
                }
            }

            for track in release.tracks.iter_mut() {
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
                        let new_artist = Rc::new(RefCell::new(Artist::new(&track_artist_to_map)));
                        self.artists.push(new_artist.clone());
                        track.artists.push(new_artist.clone());
                    }
                }
            }
        }
    }
    
    pub fn new() -> Catalog {
        Catalog {
            artists: Vec::new(),
            feed_image: None,
            images: Vec::new(),
            releases: Vec::new(),
            text: None,
            title: None
        }
    }
    
    pub fn read(build: &mut Build, cache_manifest: &mut CacheManifest) -> Result<Catalog, ()> {
        let mut catalog = Catalog::new();
        
        catalog.read_dir(&build.catalog_dir.clone(), build, cache_manifest, &Overrides::default()).unwrap();
        
        if let Some(markdown) = catalog.text.take() {
            catalog.text = Some(util::markdown_to_html(&markdown));
        }

        catalog.map_artists();
        
        if !catalog.validate_permalinks() { return Err(()); }
        
        Ok(catalog)
    }
    
    fn read_dir(
        &mut self,
        dir: &Path,
        build: &mut Build,
        cache_manifest: &mut CacheManifest,
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
                            if filename.starts_with(".") {
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

                                if build.include_patterns.len() > 0 {
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

                                if let Some(extension) = path.extension()
                                    .map(|osstr|
                                        osstr.to_str().map(|str|
                                            str.to_lowercase().as_str().to_string()
                                        )
                                    )
                                    .flatten() {
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
                cache_manifest,
                self,
                &mut local_options,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }
        
        for (track_path, extension) in &track_paths {
            if build.verbose {
                info!("Reading track {}", track_path.display());
            }
            
            let cached_assets = cache_manifest.take_or_create_track_assets(track_path, extension);
            
            if let Some(release_title) = &cached_assets.source_meta.album {
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
                cached_assets
            );
            
            release_tracks.push(track);
        }
        
        for image_path in &image_paths {
            if build.verbose {
                info!("Reading image {}", image_path.display());
            }
            
            let cached_assets = cache_manifest.take_or_create_image_assets(image_path);
            
            images.push(Rc::new(RefCell::new(Image::new(cached_assets, None, image_path))));
        }
        
        if !release_tracks.is_empty() {
            let cached_assets = cache_manifest.take_or_create_release_assets(&release_tracks);
            
            release_tracks.sort_by(|a, b|
                a.cached_assets.source_meta.track_number.cmp(
                    &b.cached_assets.source_meta.track_number
                )
            );
            release_title_metrics.sort_by(|a, b| a.0.cmp(&b.0)); // sort most often occuring title to the end of the Vec
            
            let mut release_artists_to_map: Vec<String> = Vec::new();
            if let Some(artist_names) = &local_overrides.as_ref().unwrap_or(parent_overrides).release_artists {
                for artist_name in artist_names {
                    release_artists_to_map.push(artist_name.to_string());
                }
            } else {
                for release_track in &release_tracks {
                    for track_artist_to_map in &release_track.artists_to_map {
                        if release_artists_to_map
                        .iter()
                        .find(|release_artist_to_map| release_artist_to_map == &track_artist_to_map)
                        .is_none() {
                            release_artists_to_map.push(track_artist_to_map.clone());
                        }
                    }
                }
            }
            
            let title = &local_overrides
                .as_ref()
                .unwrap_or(parent_overrides)
                .release_title
                .as_ref()
                .map(|title| title.clone())
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
                release_artists_to_map,
                cached_assets,
                cover,
                local_overrides.as_ref().unwrap_or(parent_overrides),
                local_options.release_permalink,
                title.to_string(),
                release_tracks
            );

            self.releases.push(release);
        } else if !images.is_empty() {
            // TODO: Some future logic/configuration lookup for  associating images with an artist
            // TODO: Right now might as well drop these (artist images need to be directly assigned anyway currently)
            self.images.append(&mut images);
        }
        
        for dir_path in &dir_paths {
            self.read_dir(dir_path, build, cache_manifest, local_overrides.as_ref().unwrap_or(&parent_overrides)).unwrap();
        }

        Ok(())
    }

    pub fn read_track(
        &mut self,
        path: &Path,
        overrides: &Overrides,
        cached_assets: CachedTrackAssets
    ) -> Track {
        let artists_to_map = if let Some(artist_names) = &overrides.track_artists {
            artist_names.iter().map(|name| name.clone()).collect()
        } else if let Some(name) = cached_assets.source_meta.artist.as_ref() {
            vec![name.to_string()]
        } else {
            vec![]
        };
        
        let source_file = path.to_path_buf();
        let title = cached_assets.source_meta.title
            .as_ref()
            .map(|title| title.clone())
            .unwrap_or(path.file_stem().unwrap().to_str().unwrap().to_string());
        
        Track::new(artists_to_map, cached_assets, source_file, title)
    }
    
    pub fn set_title(&mut self, title: String) -> Option<String> {
        self.title.replace(title)
    }
    
    pub fn title(&self) -> String {
        self.title.as_ref().cloned().unwrap_or(String::from("Faircamp catalog"))
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
                    PermalinkUsage::Release(release) => format!("release '{}'", release.title)
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
                    format!("the {} permalink of the release '{}'", mode(&release.permalink), release.title)
                }
            }
        };

        for release in &self.releases {
            if let Some(previous_usage) = used_permalinks.get(&release.permalink.slug) {
                let message = format!("The {} permalink '{}' of the release '{}' conflicts with {}", mode(&release.permalink), release.permalink.slug, release.title, format_previous_usage(previous_usage));
                error!("{}\n{}", message, PERMALINK_CONFLICT_RESOLUTION_HINT);
                return false;
            } else {
                let usage = PermalinkUsage::Release(&release);
                if release.permalink.generated { add_generated_usage(&usage); }
                used_permalinks.insert(release.permalink.slug.to_string(), usage);
            }
        }
        
        // TODO: We only need to validate this for those artists that are actually accessible via their own page
        // TODO: We do not yet differentiate between artists that get their own page 
        //       (i.e. because we implicitly/explicitly provide that option/data for it in the manifest) and those that don't
        for artist in &self.artists {
            let artist_ref = artist.borrow();
            if let Some(previous_usage) = used_permalinks.get(&artist_ref.permalink.slug) {
                let message = format!("The {} permalink '{}' of the artist '{}' conflicts with {}", mode(&artist_ref.permalink), artist_ref.permalink.slug, artist_ref.name, format_previous_usage(previous_usage));
                error!("{}\n{}", message, PERMALINK_CONFLICT_RESOLUTION_HINT);
                return false;
            } else {
                let usage = PermalinkUsage::Artist(&artist);
                if artist_ref.permalink.generated { add_generated_usage(&usage); }
                used_permalinks.insert(artist_ref.permalink.slug.to_string(), usage);
            }
        }

        match generated_permalinks {
            (None, None, None, 0) => (),
            (Some(first), None, None, 0) => warn!("The {} has no user-assigned permalink, it is recommended to assign one.", first),
            (Some(first), Some(second), None, 0) => warn!("The {} and the {} have no user-assigned permalinks, it is recommended to assign some.", first, second),
            (Some(first), Some(second), Some(third), 0) => warn!("The {}, the {} and the {} have no user-assigned permalinks, it is recommended to assign some.", first, second, third),
            (Some(first), Some(second), Some(third), further) => warn!("The {}, the {}, the {} and {} other entities have no user-assigned permalinks, it is recommended to assign some.", first, second, third, further),
            _ => unreachable!()
        }

        return true;
    }
    
    pub fn write_assets(&mut self, build: &mut Build) {
        for artist in self.artists.iter_mut() {
            let mut artist_mut = artist.borrow_mut();

            if let Some(image) = &mut artist_mut.image {
                let mut image_mut = image.borrow_mut();
                let image_asset = image_mut.get_or_transcode_as(&ImageFormat::Jpeg, build, AssetIntent::Deliverable);
                
                fs::copy(
                    build.cache_dir.join(&image_asset.filename),
                    build.build_dir.join(&image_asset.filename)
                ).unwrap();
                
                build.stats.add_image(image_asset.filesize_bytes);
                
                image_mut.cached_assets.persist(&build.cache_dir);
            }
        }

        for release in self.releases.iter_mut() {            
            if let Some(image) = &mut release.cover {
                let mut image_mut = image.borrow_mut();
                let image_asset = image_mut.get_or_transcode_as(&ImageFormat::Jpeg, build, AssetIntent::Deliverable);
                
                fs::copy(
                    build.cache_dir.join(&image_asset.filename),
                    build.build_dir.join(&image_asset.filename)
                ).unwrap();
                
                build.stats.add_image(image_asset.filesize_bytes);
                
                image_mut.cached_assets.persist(&build.cache_dir);
            }
            
            for track in release.tracks.iter_mut() {
                let streaming_asset = track.get_or_transcode_as(&release.streaming_format, build, AssetIntent::Deliverable);
                
                fs::copy(
                    build.cache_dir.join(&streaming_asset.filename),
                    build.build_dir.join(&streaming_asset.filename)
                ).unwrap();
                
                build.stats.add_track(streaming_asset.filesize_bytes);
                
                track.cached_assets.persist(&build.cache_dir);
            }
            
            release.write_download_archives(build);
        }
    }
}
