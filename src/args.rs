use clap::Clap;
use std::path::PathBuf;

#[clap(version = env!("CARGO_PKG_VERSION"))]
#[derive(Clap, Debug)]
pub struct Args {
    /// Override build directory (by default it's .faircamp_build/ inside the current working directory)
    #[clap(long = "build-dir", short = 'b')]
    pub build_dir: Option<PathBuf>,
    
    /// Override cache directory (by default it's .faircamp_cache/ inside the current working directory)
    #[clap(long = "cache-dir", short = 'c')]
    pub cache_dir: Option<PathBuf>,

    /// Wipes the asset cache BEFORE building - it then gets newly populated during building.
    #[clap(long = "wipe-cache", short = 'w')]
    pub wipe_cache: bool
}
