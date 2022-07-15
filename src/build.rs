use chrono::{DateTime, Utc};
use std::{env, path::PathBuf};
use url::Url;

use crate::{
    args::Args,
    asset_cache::CacheOptimization,
    localization::Localization,
    styles::Theme,
    util
};

pub struct Build {
    pub base_url: Option<Url>,
    pub build_begin: DateTime<Utc>,
    pub build_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub cache_optimization: CacheOptimization,
    pub catalog_dir: PathBuf,
    pub deploy_destination: Option<String>,
    pub embeds_requested: bool,
    pub localization: Localization,
    pub post_build_action: PostBuildAction,
    pub stats: Stats,
    pub theme: Option<Theme>,
    pub theming_widget: bool
}

pub enum PostBuildAction {
    None,
    Deploy,
    Preview
}

pub struct Stats {
    bytes_used_archives: u64,
    bytes_used_images: u64,
    bytes_used_tracks: u64,
    num_archives: u32,
    num_images: u32,
    num_tracks: u32
}

impl Build {
    pub fn init(args: &Args) -> Build {
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
        
        let post_build_action = PostBuildAction::init(args);
        
        Build {
            base_url: None,
            build_begin: Utc::now(),
            build_dir,
            cache_dir,
            cache_optimization: CacheOptimization::Default,
            catalog_dir,
            deploy_destination: args.deploy_destination.clone(),
            embeds_requested: false,
            localization: Localization::defaults(),
            post_build_action,
            stats: Stats::empty(),
            theme: None,
            theming_widget: args.theming_widget
        }
    }
    
    pub fn print_stats(&self) {
        let elapsed = Utc::now().signed_duration_since(self.build_begin).num_seconds();
        
        info_stats!("{}", &self.stats.to_string());
        info_stats!("Build finished in {:.2} seconds", elapsed);
    }
}

impl PostBuildAction {
    pub fn init(args: &Args) -> PostBuildAction {
        if args.deploy {
            if args.preview {
                panic!("Provided options --deploy and --preview are mutually exclusive.")
            } else {
                PostBuildAction::Deploy
            }
        } else if args.preview {
            PostBuildAction::Preview
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
    
    pub fn add_image(&mut self, filesize_bytes: u64) {
        self.bytes_used_images += filesize_bytes;
        self.num_images += 1;
    }
    
    pub fn add_track(&mut self, filesize_bytes: u64) {
        self.bytes_used_tracks += filesize_bytes;
        self.num_tracks += 1;
    }
    
    pub fn empty() -> Stats {
        Stats {
            bytes_used_archives: 0,
            bytes_used_images: 0,
            bytes_used_tracks: 0,
            num_archives: 0,
            num_images: 0,
            num_tracks: 0
        }
    }
    
    pub fn to_string(&self) -> String {
        format!(
            "{num_archives} archives ({bytes_used_archives}), {num_tracks} tracks ({bytes_used_tracks}) and {num_images} images ({bytes_used_images}) written",
            num_archives=self.num_archives,
            num_images=self.num_images,
            num_tracks=self.num_tracks,
            bytes_used_archives=util::format_bytes(self.bytes_used_archives),
            bytes_used_images=util::format_bytes(self.bytes_used_images),
            bytes_used_tracks=util::format_bytes(self.bytes_used_tracks)
        )
    }
}