use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::{
    artist::Artist,
    asset_cache::{
        CacheManifest,
        CachedImageAssets,
        CachedTrackAssets,
        SourceFileSignature
    },
    audio_meta::AudioMeta,
    build_settings::BuildSettings,
    ffmpeg::TranscodeFormat,
    image::Image,
    manifest::Overrides,
    message,
    release::Release,
    track::Track,
    manifest,
    util
};

// TODO: Verify if ogg even works already
// TODO: See if aac as input is easily possible
const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["flac", "mp3", "ogg", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "png"];

#[derive(Debug)]
pub struct Catalog {
    pub artists: Vec<Rc<Artist>>,
    pub images: Vec<Image>, // TODO: Do we need these + what to do with them (also consider "label cover" aspect)
    pub releases: Vec<Release>
}

impl Catalog {
    fn init_empty() -> Catalog {
        Catalog {
            artists: Vec::new(),
            images: Vec::new(),
            releases: Vec::new()
        }
    }
    
    pub fn read(catalog_dir: &Path) -> Catalog {
        let mut catalog = Catalog::init_empty();
        catalog.read_dir(catalog_dir, &Overrides::default()).unwrap();
        catalog
    }
    
    fn read_dir(&mut self, dir: &Path, parent_overrides: &Overrides) -> Result<(), String> {
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
                                    if extension == "txt" {
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
            
            manifest::apply_overrides(
                meta_path,
                local_overrides.get_or_insert_with(|| parent_overrides.clone())
            );
        }
        
        for (track_path, extension) in &track_paths {
            info!("Reading track {}", track_path.display());
            
            let audio_meta = AudioMeta::extract(track_path, extension);
            
            if let Some(release_title) = &audio_meta.album {
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
                util::is_lossless(extension),
                audio_meta,
                local_overrides.as_ref().unwrap_or(parent_overrides)
            );
            
            release_tracks.push(track);
        }
        
        for image_path in &image_paths {
            info!("Reading image {}", image_path.display());
            images.push(Image::init(image_path, util::uuid()));
        }
        
        if !release_tracks.is_empty() {
            release_tracks.sort_by(|a, b| a.number.cmp(&b.number));
            release_title_metrics.sort_by(|a, b| a.0.cmp(&b.0)); // sort most often occuring title to the end of the Vec
            
            let mut release_artists: Vec<Rc<Artist>> = Vec::new();
            if let Some(artist_names) = &local_overrides.as_ref().unwrap_or(parent_overrides).release_artists {
                for artist_name in artist_names {
                    let artist = self.track_artist(Some(artist_name.clone()));
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
            
            let release = Release::init(
                release_artists,
                local_overrides.as_ref().unwrap_or(parent_overrides).download_formats.clone(),
                local_overrides.as_ref().unwrap_or(parent_overrides).download_option.clone(),
                images,
                local_overrides.as_ref().unwrap_or(parent_overrides).release_text.clone(),
                release_title_metrics
                    .pop()
                    .map(|(_count, title)| title) 
                    .unwrap_or_else(|| dir.file_name().unwrap().to_str().unwrap().to_string()),
                release_tracks
            );

            self.releases.push(release);
        } else if !images.is_empty() {
            // TODO: Some future logic/configuration lookup for  associating images with an artist
            self.images.append(&mut images);
        }
        
        for dir_path in &dir_paths {
            info!("Reading directory {}", dir_path.display());
            self.read_dir(dir_path, local_overrides.as_ref().unwrap_or(&parent_overrides)).unwrap();
        }

        Ok(())
    }

    pub fn read_track(
        &mut self,
        path: &Path,
        lossless_source: bool,
        audio_meta: AudioMeta,
        overrides: &Overrides
    ) -> Track {
        let artists = if let Some(artist_names) = &overrides.track_artists {
            artist_names
                .iter()
                .map(|name| self.track_artist(Some(name.to_string())))
                .collect()
        } else {
            vec![self.track_artist(audio_meta.artist)]
        };
        
        let source_file = path.to_path_buf();
        let title = audio_meta.title.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
        
        Track::init(
            artists,
            audio_meta.duration_seconds,
            lossless_source,
            audio_meta.track_number,
            source_file,
            title,
            util::uuid()
        )
    }
    
    // TODO: track_artist is confusing because does it mean "track the artist" or "the track artist"
    pub fn track_artist(&mut self, new_artist_name: Option<String>) -> Rc<Artist> {
        if let Some(new_artist_name) = new_artist_name {
            self.artists
                .iter()
                .find(|artist| artist.name == new_artist_name)
                .map(|existing_artist| existing_artist.clone())
                .unwrap_or_else(|| {
                    let new_artist = Rc::new(Artist::init(new_artist_name));
                    self.artists.push(new_artist.clone());
                    new_artist
                })
        } else {
            self.artists
                .iter()
                .find(|artist| artist.name == "UNKNOWN_SPECIAL_STRING")
                .map(|existing_artist| existing_artist.clone())
                .unwrap_or_else(|| {
                    let new_artist = Rc::new(Artist::init(String::from("UNKNOWN_SPECIAL_STRING")));
                    self.artists.push(new_artist.clone());
                    new_artist
                })
        }
    }
    
    pub fn write_assets(&self, build_settings: &BuildSettings) {
        let mut cache_manifest = CacheManifest::retrieve(&build_settings.cache_dir);
        
        for release in &self.releases {            
            if let Some(image) = &release.cover {
                let source_file_signature = SourceFileSignature::init(&image.source_file);
                
                let mut cached_image_assets = match cache_manifest.images
                    .iter_mut()
                    .find(|cached_image| cached_image.source_file_signature == source_file_signature) {
                    Some(cached_image_assets) => cached_image_assets,
                    None => {
                        cache_manifest.images.push(CachedImageAssets::new(source_file_signature));
                        cache_manifest.images.last_mut().unwrap()
                    }    
                };
                
                release.write_image_assets(build_settings, &mut cached_image_assets, image, &TranscodeFormat::Jpeg);
            }
            
            // TODO: Check release.download_option to see if we even need to transcode and copy tracks
            
            for track in &release.tracks {
                let source_file_signature = SourceFileSignature::init(&track.source_file);
                
                let mut cached_track_assets = match cache_manifest.tracks
                    .iter_mut()
                    .find(|cached_track| cached_track.source_file_signature == source_file_signature) {
                    Some(cached_track_assets) => cached_track_assets,
                    None => {
                        cache_manifest.tracks.push(CachedTrackAssets::new(source_file_signature));
                        cache_manifest.tracks.last_mut().unwrap()
                    }    
                };
                
                release.write_track_assets(build_settings, &mut cached_track_assets, track, &TranscodeFormat::Mp3Cbr128);
                
                if release.download_formats.aac {
                    release.write_track_assets(build_settings, &mut cached_track_assets, track, &TranscodeFormat::Aac);
                }
                
                if release.download_formats.flac {
                    if !track.lossless_source {
                        message::discouraged(&format!("Track {} comes from a lossy format, offering it in a lossless format is wasteful and misleading to those who will download it.", &track.source_file.display()));
                    }
                    
                    release.write_track_assets(build_settings, &mut cached_track_assets, track, &TranscodeFormat::Flac);
                }
                
                if release.download_formats.mp3_320 {
                    release.write_track_assets(build_settings, &mut cached_track_assets, track, &TranscodeFormat::Mp3Cbr320);
                }
                
                if release.download_formats.mp3_v0 {
                    release.write_track_assets(build_settings, &mut cached_track_assets, track, &TranscodeFormat::Mp3VbrV0);
                }
                
                if release.download_formats.ogg_vorbis {
                    release.write_track_assets(build_settings, &mut cached_track_assets, track, &TranscodeFormat::OggVorbis);
                }
            }
        }
        
        cache_manifest.persist(&build_settings.cache_dir);
    }
}