use std::env;
use std::path::PathBuf;

use crate::args::Args;

pub struct BuildSettings {
    pub build_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub catalog_dir: PathBuf,
    pub host_original_media: bool,
    pub post_build_action: PostBuildAction,
    pub transcode_flac: bool,
    pub transcode_mp3_320cbr: bool,
    pub transcode_mp3_256vbr: bool
}

pub enum PostBuildAction {
    None,
    Deploy,
    Preview
}

impl BuildSettings {
    pub fn init(args: &Args) -> BuildSettings {
        let catalog_dir = env::current_dir()
            .expect("Current working directory can not be determined or is unaccessible");
            
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
            build_dir,
            cache_dir,
            catalog_dir,
            host_original_media: false,
            post_build_action,
            transcode_flac: true,
            transcode_mp3_320cbr: true,
            transcode_mp3_256vbr: false
        }
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