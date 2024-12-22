// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::hash_map::{DefaultHasher, HashMap};
use std::env;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Utc};
use url::Url;

use crate::{Args, ImageProcessor, Locale};
use crate::util::format_bytes;

/// When we link to assets on the rendered pages, we append a unique asset
/// hash to each path (e.g. "player.js?g1VVfPoEjUw"), which is derived from
/// the file content of the asset. We do this in order to prompt browsers to
/// fetch new, uncached assets when their content has changed. This struct
/// groups together those hashes for all assets we use.
pub struct AssetHashes {
    pub browser_js: Option<String>,
    pub clipboard_js: Option<String>,
    pub embeds_css: Option<String>,
    pub embeds_js: Option<String>,
    pub favicon_custom: Option<String>,
    pub favicon_dark_png: Option<String>,
    pub favicon_light_png: Option<String>,
    pub favicon_svg: Option<String>,
    pub player_js: Option<String>,
    pub site_css: Option<String>,
    pub theme_css: HashMap<String, String>
}

pub struct Build {
    pub asset_hashes: AssetHashes,
    pub base_url: Option<Url>,
    pub build_begin: DateTime<Utc>,
    pub build_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub catalog_dir: PathBuf,
    pub clean_urls: bool,
    pub deploy_destination: Option<String>,
    /// Whether at least one embed was requested to be generated somewhere.
    /// This lets us know to generate some css/js used in embeds only, and/or
    /// to print a warning in case the base_url is missing and we hence
    /// couldn't generate any embeds in spite of them being requested.
    pub embeds_requested: bool,
    /// Counts errors during build
    pub errors: usize,
    pub exclude_patterns: Vec<String>,
    pub image_processor: ImageProcessor,
    /// Forces continuation of build even when there are errors in the
    /// manifests or during building in general.
    pub ignore_errors: bool,
    pub include_patterns: Vec<String>,
    pub locale: Locale,
    /// If we encounter missing image descriptions during the build we set this flag.
    /// This lets us know to inject optional css used for indicating these images.
    pub missing_image_descriptions: bool,
    pub post_build_action: PostBuildAction,
    pub stats: Stats,
    pub theming_widget: bool,
    /// Most asset urls contain a deterministically random (=hashed) path
    /// segment. Out of the box, a static default string is used as a salt
    /// for hashing, which means that initially all urls remain stable
    /// between deployments. Faircamp's configuration allows to either
    /// override the salt manually(thereby keeping urls valid until a
    /// different salt is set), or let it be automatically randomized on each
    /// deployment, thereby invalidating all download asset urls on each
    /// deployment.
    pub url_salt: String,
    pub verbose: bool
}

#[derive(Debug, PartialEq)]
pub enum PostBuildAction {
    None,
    Deploy,
    Preview { port: Option<u16> }
}

pub struct Stats {
    bytes_used_archives: u64,
    bytes_used_extras: u64,
    bytes_used_images: u64,
    bytes_used_tracks: u64,
    num_archives: u32,
    num_extras: u32,
    num_images: u32,
    num_tracks: u32
}

impl AssetHashes {
    pub fn new() -> AssetHashes {
        AssetHashes {
            browser_js: None,
            clipboard_js: None,
            embeds_css: None,
            embeds_js: None,
            favicon_custom: None,
            favicon_dark_png: None,
            favicon_light_png: None,
            favicon_svg: None,
            player_js: None,
            site_css: None,
            theme_css: HashMap::default()
        }
    }
}

impl Build {
    pub fn error(&mut self, error: &str) {
        error!("{}", error);
        self.errors += 1;
    }

    pub fn hash_path_with_salt(
        &self,
        release_slug: &str,
        format_dir: &str,
        filename: &str
    ) -> String {
        let mut hasher = DefaultHasher::new();
        
        release_slug.hash(&mut hasher);
        format_dir.hash(&mut hasher);
        filename.hash(&mut hasher);
        self.url_salt.hash(&mut hasher);

        URL_SAFE_NO_PAD.encode(hasher.finish().to_le_bytes())
    }

    pub fn hash_with_salt(
        &self,
        inputs: &[&str]
    ) -> String {
        let mut hasher = DefaultHasher::new();

        for input in inputs {
            input.hash(&mut hasher);
        }

        self.url_salt.hash(&mut hasher);

        URL_SAFE_NO_PAD.encode(hasher.finish().to_le_bytes())
    }

    /// When we construct site-internal linking urls, we always
    /// append an index suffix. For instance we might build this:
    /// root_prefix ("../") + permalink ("foo") + index_suffix ("/") = "../foo/"
    /// If clean_urls is disabled however, we always append an index_suffix "/index.html",
    /// so that above example would result in "../foo/index.html".
    pub fn index_suffix(&self) -> &str {
        match self.clean_urls {
            true => "/",
            false => "/index.html"
        }
    }

    pub fn index_suffix_file_only(&self) -> &str {
        match self.clean_urls {
            true => "",
            false => "index.html"
        }
    }

    pub fn new(args: &Args) -> Build {
        let catalog_dir = args.catalog_dir
            .as_ref()
            .map(|path| path.to_path_buf())
            .unwrap_or_else(||
                env::current_dir()
                    .expect("Current working directory can not be determined or is unaccessible")
            );
            
        let build_dir = args.build_dir
            .as_ref()
            .map(|path| path.to_path_buf())
            .unwrap_or_else(|| catalog_dir.join(".faircamp_build"));
            
        let cache_dir = args.cache_dir
            .as_ref()
            .map(|path| path.to_path_buf())
            .unwrap_or_else(|| catalog_dir.join(".faircamp_cache"));

        let post_build_action = PostBuildAction::new(args);

        let locale = if args.debug_translations { Locale::keys() } else { Locale::default() };

        Build {
            asset_hashes: AssetHashes::new(),
            base_url: None,
            build_begin: Utc::now(),
            build_dir,
            cache_dir,
            catalog_dir,
            clean_urls: !args.no_clean_urls,
            deploy_destination: args.deploy_destination.clone(),
            embeds_requested: false,
            errors: 0,
            exclude_patterns: args.exclude_patterns.clone(),
            include_patterns: args.include_patterns.clone(),
            image_processor: ImageProcessor::new(),
            ignore_errors: args.ignore_errors,
            locale,
            missing_image_descriptions: false,
            post_build_action,
            stats: Stats::new(),
            theming_widget: args.theming_widget,
            url_salt: String::from(""), // changing this can invalidate urls of already deployed faircamp sites, handle with care
            verbose: args.verbose
        }
    }
    
    pub fn print_stats(&self) {
        let elapsed_time_delta = Utc::now().signed_duration_since(self.build_begin);

        let elapsed_time_string = if elapsed_time_delta.num_seconds() == 0 {
            format!("{} milliseconds", elapsed_time_delta.num_milliseconds())
        } else if elapsed_time_delta.num_minutes() == 0 {
            format!("{} seconds", elapsed_time_delta.num_seconds())
        } else {
            format!("{} minutes", elapsed_time_delta.num_minutes())
        };

        info_stats!("{}", &self.stats.to_string());
        info_stats!("Build finished in {}", elapsed_time_string);
    }
}

impl PostBuildAction {
    pub fn new(args: &Args) -> PostBuildAction {
        if args.deploy {
            if args.preview {
                panic!("Provided options --deploy and --preview are mutually exclusive.")
            } else {
                PostBuildAction::Deploy
            }
        } else if args.preview {
            PostBuildAction::Preview {
                port: args.preview_port
            }
        } else {
            PostBuildAction::None
        }
    }
}

impl Stats {
    pub fn add_archive(&mut self, filesize_bytes: u64) {
        self.bytes_used_archives += filesize_bytes;
        self.num_archives += 1;
    }

    pub fn add_extra(&mut self, filesize_bytes: u64) {
        self.bytes_used_extras += filesize_bytes;
        self.num_extras += 1;
    }

    pub fn add_image(&mut self, filesize_bytes: u64) {
        self.bytes_used_images += filesize_bytes;
        self.num_images += 1;
    }
    
    pub fn add_track(&mut self, filesize_bytes: u64) {
        self.bytes_used_tracks += filesize_bytes;
        self.num_tracks += 1;
    }
    
    pub fn new() -> Stats {
        Stats {
            bytes_used_archives: 0,
            bytes_used_extras: 0,
            bytes_used_images: 0,
            bytes_used_tracks: 0,
            num_archives: 0,
            num_extras: 0,
            num_images: 0,
            num_tracks: 0
        }
    }
    
    pub fn to_string(&self) -> String {
        format!(
            "{num_archives} archives ({bytes_used_archives}), {num_tracks} tracks ({bytes_used_tracks}), {num_images} images ({bytes_used_images}) and {num_extras} extras ({bytes_used_extras}) written",
            num_archives = self.num_archives,
            num_extras = self.num_extras,
            num_images = self.num_images,
            num_tracks = self.num_tracks,
            bytes_used_archives = format_bytes(self.bytes_used_archives),
            bytes_used_extras = format_bytes(self.bytes_used_extras),
            bytes_used_images = format_bytes(self.bytes_used_images),
            bytes_used_tracks = format_bytes(self.bytes_used_tracks)
        )
    }
}