use std::env;
use std::path::PathBuf;
use std::time::Instant;

use crate::{
    args::Args,
    message,
    util
};

// TODO: Now that this holds stats as well it should be just "Build"
//       or something like that, as it is more than just settings.
pub struct BuildSettings {
    pub build_dir: PathBuf,
    begin_instant: Instant,
    pub cache_dir: PathBuf,
    pub catalog_dir: PathBuf,
    pub deploy_destination: Option<String>,
    pub host_original_media: bool, // TODO: ?
    pub post_build_action: PostBuildAction,
    pub stats: Stats
}

pub enum PostBuildAction {
    None,
    Deploy,
    Preview
}

pub struct Stats {
    bytes_used_images: u64,
    bytes_used_tracks: u64,
    num_images: u32,
    num_tracks: u32
}

impl BuildSettings {
    pub fn init(args: &Args) -> BuildSettings {
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
        
        BuildSettings {
            begin_instant: Instant::now(),
            build_dir,
            cache_dir,
            catalog_dir,
            deploy_destination: args.deploy_destination.clone(),
            host_original_media: false,
            post_build_action,
            stats: Stats::empty()
        }
    }
    
    pub fn print_stats(&self) {
        message::stats(&self.stats.to_string());
        message::stats(
            &format!("Build finished in {:.2} seconds", self.begin_instant.elapsed().as_secs_f32())
        );
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
            bytes_used_images: 0,
            bytes_used_tracks: 0,
            num_images: 0,
            num_tracks: 0
        }
    }
    
    pub fn to_string(&self) -> String {
        // TODO: Add track statistics as soon as we have them
        format!(
            "{num_images} images written ({bytes_used_images})",
            num_images=self.num_images,
            bytes_used_images=util::format_bytes(self.bytes_used_images)
        )
    }
}