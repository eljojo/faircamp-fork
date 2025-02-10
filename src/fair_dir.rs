// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::{Path, PathBuf};

use crate::Build;

const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["aif", "aifc", "aiff", "alac", "flac", "mp3", "ogg", "opus", "wav"];
const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["gif", "heif", "jpeg", "jpg", "png", "webp"];
const UNSUPPORTED_AUDIO_EXTENSIONS: &[&str] = &["aac", "m4a"];

/// Convenience wrapper to generically pre-scan directories for
/// repeating/required patterns used in faircamp's folder hierarchy.
#[derive(Debug)]
pub struct FairDir {
    pub artist_manifest: Option<PathBuf>,
    pub audio_files: Vec<PathBuf>,
    pub catalog_manifest: Option<PathBuf>,
    pub dirs: Vec<PathBuf>,
    pub extra_files: Vec<PathBuf>,
    pub image_files: Vec<PathBuf>,
    pub path: PathBuf,
    pub release_manifest: Option<PathBuf>,
    pub track_manifest: Option<PathBuf>
}

impl FairDir {
    fn new(path: &Path) -> FairDir {
        FairDir {
            artist_manifest: None,
            audio_files: Vec::new(),
            catalog_manifest: None,
            dirs: Vec::new(),
            extra_files: Vec::new(),
            image_files: Vec::new(),
            path: path.to_owned(),
            release_manifest: None,
            track_manifest: None
        }
    }

    pub fn read(build: &mut Build, path: &Path) -> FairDir {
        let mut fair_dir = FairDir::new(path);

        if let Ok(dir_entries) = path.read_dir() {
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
                            let dir_canonicalized = path.canonicalize().unwrap();
                            for special_dir in &[&build.build_dir, &build.cache_dir] {
                                if let Ok(special_dir_canonicalized) = special_dir.canonicalize() {
                                    if dir_canonicalized == special_dir_canonicalized {
                                        if build.verbose {
                                            info!("Ignoring special directory {}", special_dir.display());
                                        }
                                        continue 'dir_entry_iter;
                                    }
                                }
                            }

                            for exclude_pattern in &build.exclude_patterns {
                                if let Some(dir_str) = path.to_str() {
                                    if dir_str.contains(exclude_pattern) {
                                        if build.verbose {
                                            info!("Ignoring directory {} and all below (excluded by pattern '{}')", path.display(), exclude_pattern);
                                        }
                                        continue 'dir_entry_iter;
                                    }
                                }
                            }

                            fair_dir.dirs.push(path);
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

                            if path.ends_with("artist.eno") {
                                fair_dir.artist_manifest = Some(path);
                            } else if path.ends_with("catalog.eno") {
                                fair_dir.catalog_manifest = Some(path);
                            } else if path.ends_with("release.eno") {
                                fair_dir.release_manifest = Some(path);
                            } else if path.ends_with("track.eno") {
                                fair_dir.track_manifest = Some(path);
                            } else if let Some(extension) = path
                                .extension()
                                .and_then(|osstr|
                                    osstr.to_str().map(|str|
                                        str.to_lowercase().as_str().to_string()
                                    )
                                ) {
                                if extension == "eno" {
                                    let error = format!("A manifest named '{}' was encountered at '{}', but this name is not recognized (allowed ones are 'artist.eno', 'catalog.eno', 'release.eno', and 'track.eno')", path.file_name().unwrap().to_string_lossy(), path.display());
                                    build.error(&error);
                                } else if SUPPORTED_AUDIO_EXTENSIONS.contains(&&extension[..]) {
                                    fair_dir.audio_files.push(path);
                                } else if SUPPORTED_IMAGE_EXTENSIONS.contains(&&extension[..]) {
                                    fair_dir.image_files.push(path);
                                } else if UNSUPPORTED_AUDIO_EXTENSIONS.contains(&&extension[..]) {
                                    let error = format!("Support for reading audio files with the extension '{extension}' from the catalog is not yet supported - please get in touch or open an issue if you need this");
                                    build.error(&error);
                                } else {
                                    fair_dir.extra_files.push(path);
                                }
                            } else {
                                fair_dir.extra_files.push(path);
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

        fair_dir
    }
}
