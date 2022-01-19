use std::{
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
    release::Release,
    track::{CachedTrackAssets, Track},
    manifest,
    util
};

// TODO: Verify if ogg even works already
// TODO: See if aac, aiff as input is easily possible
const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "png"];

#[derive(Debug)]
pub struct Catalog {
    pub artists: Vec<Rc<Artist>>,
    pub feed_image: Option<String>,
    pub images: Vec<Image>, // TODO: Do we need these + what to do with them (also consider "label cover" aspect)
    pub releases: Vec<Release>,
    pub text: Option<String>,
    pub title: Option<String>
}

#[derive(Debug)]
pub enum Permalink {
    Generated(String),
    UserAssigned(String)
}

impl Catalog {
    pub fn get_or_create_artist(&mut self, new_artist_name: Option<&str>) -> Rc<Artist> {
        if let Some(new_artist_name) = new_artist_name {
            self.artists
                .iter()
                .find(|artist| &artist.name == new_artist_name)
                .map(|existing_artist| existing_artist.clone())
                .unwrap_or_else(|| {
                    let new_artist = Rc::new(Artist::new(new_artist_name.to_string(), None));
                    self.artists.push(new_artist.clone());
                    new_artist
                })
        } else {
            self.artists
                .iter()
                .find(|artist| artist.name == "UNKNOWN_SPECIAL_STRING")
                .map(|existing_artist| existing_artist.clone())
                .unwrap_or_else(|| {
                    let new_artist = Rc::new(Artist::new(String::from("UNKNOWN_SPECIAL_STRING"), None));
                    self.artists.push(new_artist.clone());
                    new_artist
                })
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
        
        // TODO: Put permalink collision check logic (the stuff below) into its own function
        let mut permalinks = HashMap::new();
        
        for release in &catalog.releases {
            match &release.permalink {
                Permalink::Generated(permalink) => if let Some(_previous_permalink) = permalinks.get(&permalink) {
                    error!("Conflicting permalinks"); // TODO: Proper error message that indicates what conflicts with what and how to resolve it
                    return Err(()); 
                } else {
                    permalinks.insert(permalink, &release.permalink);
                }
                Permalink::UserAssigned(permalink) => if let Some(_previous_permalink) = permalinks.get(&permalink) {
                    error!("Conflicting permalinks"); // TODO: Proper error message that indicates what conflicts with what and how to resolve it
                    return Err(());
                } else {
                    permalinks.insert(permalink, &release.permalink);
                }
            }
        }
        
        // TODO: We only need to validate this for those artists that are actually accessible via their own page
        // TODO: We do not yet differentiate between artists that get their own page 
        //       (i.e. because we implicitly/explicitly provide that option/data for it in the manifest) and those that don't
        for artist in &catalog.artists {
            match &artist.permalink {
                Permalink::Generated(permalink) => if let Some(_previous_permalink) = permalinks.get(&permalink) {
                    error!("Conflicting permalinks"); // TODO: Proper error message that indicates what conflicts with what and how to resolve it
                    return Err(()); 
                } else {
                    permalinks.insert(permalink, &artist.permalink);
                }
                Permalink::UserAssigned(permalink) => if let Some(_previous_permalink) = permalinks.get(&permalink) {
                    error!("Conflicting permalinks"); // TODO: Proper error message that indicates what conflicts with what and how to resolve it
                    return Err(());
                } else {
                    permalinks.insert(permalink, &artist.permalink);
                }
            }
        }
        
        Ok(catalog)
    }
    
    fn read_dir(
        &mut self,
        dir: &Path,
        build: &mut Build,
        cache_manifest: &mut CacheManifest,
        parent_overrides: &Overrides
    ) -> Result<(), String> {
        let mut local_options = LocalOptions::new();
        let mut local_overrides = None;
        
        let mut images: Vec<Image> = Vec::new();
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
                for dir_entry_result in dir_entries {
                    if let Ok(dir_entry) = dir_entry_result {
                        if let Some(filename) = dir_entry.file_name().to_str() {
                            if filename.starts_with(".") {
                                info!("Ignoring hidden file '{}'", filename);
                                continue
                            }
                        }
                        
                        if let Ok(file_type) = dir_entry.file_type() {
                            let path = dir_entry.path();
                            
                            if file_type.is_dir() {
                                dir_paths.push(path);
                            } else if file_type.is_file() {
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
            info!("Reading meta {}", meta_path.display());
            
            manifest::apply_options(
                meta_path,
                build,
                self,
                &mut local_options,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }
        
        for (track_path, extension) in &track_paths {
            info!("Reading track {}", track_path.display());
            
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
            info!("Reading image {}", image_path.display());
            
            let cached_assets = cache_manifest.take_or_create_image_assets(image_path);
            
            images.push(Image::init(cached_assets, image_path));
        }
        
        if !release_tracks.is_empty() {
            let cached_assets = cache_manifest.take_or_create_release_assets(&release_tracks);
            
            release_tracks.sort_by(|a, b|
                a.cached_assets.source_meta.track_number.cmp(
                    &b.cached_assets.source_meta.track_number
                )
            );
            release_title_metrics.sort_by(|a, b| a.0.cmp(&b.0)); // sort most often occuring title to the end of the Vec
            
            let mut release_artists: Vec<Rc<Artist>> = Vec::new();
            if let Some(artist_names) = &local_overrides.as_ref().unwrap_or(parent_overrides).release_artists {
                for artist_name in artist_names {
                    let artist = self.get_or_create_artist(Some(&artist_name));
                    release_artists.push(artist);
                }
            } else {
                for release_track in &release_tracks {
                    for track_artist in &release_track.artists {
                        if release_artists
                        .iter()
                        .find(|release_artist| Rc::ptr_eq(release_artist, track_artist))
                        .is_none() {
                            release_artists.push(track_artist.clone());
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
            
            let release = Release::init(
                release_artists,
                cached_assets,
                images,
                local_overrides.as_ref().unwrap_or(parent_overrides),
                local_options.release_permalink,
                title.to_string(),
                release_tracks
            );

            self.releases.push(release);
        } else if !images.is_empty() {
            // TODO: Some future logic/configuration lookup for  associating images with an artist
            self.images.append(&mut images);
        }
        
        for dir_path in &dir_paths {
            info!("Reading directory {}", dir_path.display());
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
        let artists = if let Some(artist_names) = &overrides.track_artists {
            artist_names
                .iter()
                .map(|name| self.get_or_create_artist(Some(name)))
                .collect()
        } else {
            // TODO: We unconditionally assign an artist to the track - including a None artist,
            //       that is, one that we give an UNKNOWN_SPECIAL_STRING identifier. Is there
            //       any sensible reason not to just *not* have an artist for a track (?) 
            vec![self.get_or_create_artist(cached_assets.source_meta.artist.as_ref().map(|name| name.as_str()))]
        };
        
        let source_file = path.to_path_buf();
        let title = cached_assets.source_meta.title
            .as_ref()
            .map(|title| title.clone())
            .unwrap_or(path.file_stem().unwrap().to_str().unwrap().to_string());
        
        Track::new(artists, cached_assets, source_file, title)
    }
    
    pub fn write_assets(&mut self, build: &mut Build) {
        for release in self.releases.iter_mut() {            
            if let Some(image) = &mut release.cover {
                let image_asset = image.get_or_transcode_as(&ImageFormat::Jpeg, build, AssetIntent::Deliverable);
                
                fs::copy(
                    build.cache_dir.join(&image_asset.filename),
                    build.build_dir.join(&image_asset.filename)
                ).unwrap();
                
                build.stats.add_image(image_asset.filesize_bytes);
                
                image.cached_assets.persist(&build.cache_dir);
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

impl Permalink {
    pub fn get(&self) -> &str {
        match self {
            Permalink::Generated(string) => string,
            Permalink::UserAssigned(string) => string
        }
    }
    
    pub fn new(user_assigned: Option<String>, fallback: &str) -> Permalink {
        match user_assigned {
            Some(permalink) => Permalink::UserAssigned(permalink),
            None => Permalink::Generated(slug::slugify(fallback))
        }
    }
}