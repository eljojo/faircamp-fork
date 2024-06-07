// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Utc};
use url::Url;

use crate::{
    Args,
    CacheOptimization,
    ImageProcessor,
    Locale,
    util
};

pub struct Build {
    pub base_url: Option<Url>,
    pub build_begin: DateTime<Utc>,
    pub build_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub cache_optimization: CacheOptimization,
    pub catalog_dir: PathBuf,
    pub clean_urls: bool,
    pub deploy_destination: Option<String>,
    pub embeds_requested: bool,
    pub exclude_patterns: Vec<String>,
    pub image_processor: ImageProcessor,
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

impl Build {
    // TODO: Name more appropriately - hash_with_salt or such
    pub fn hash(
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

    // TODO: Name more appropriately - hash_generic_with_salt or such
    pub fn hash_generic(
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
            base_url: None,
            build_begin: Utc::now(),
            build_dir,
            cache_dir,
            cache_optimization: CacheOptimization::Default,
            catalog_dir,
            clean_urls: !args.no_clean_urls,
            deploy_destination: args.deploy_destination.clone(),
            embeds_requested: false,
            exclude_patterns: args.exclude_patterns.clone(),
            include_patterns: args.include_patterns.clone(),
            image_processor: ImageProcessor::new(),
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
        let elapsed = Utc::now().signed_duration_since(self.build_begin).num_seconds();
        
        info_stats!("{}", &self.stats.to_string());
        info_stats!("Build finished in {:.2} seconds", elapsed);
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
            num_archives=self.num_archives,
            num_extras=self.num_extras,
            num_images=self.num_images,
            num_tracks=self.num_tracks,
            bytes_used_archives=util::format_bytes(self.bytes_used_archives),
            bytes_used_extras=util::format_bytes(self.bytes_used_extras),
            bytes_used_images=util::format_bytes(self.bytes_used_images),
            bytes_used_tracks=util::format_bytes(self.bytes_used_tracks)
        )
    }
}