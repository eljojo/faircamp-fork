use clap::Clap;
use std::path::PathBuf;

#[clap(version = env!("CARGO_PKG_VERSION"))]
#[derive(Clap, Debug)]
pub struct Args {
    /// Override build directory (default is .faircamp_build/ inside the current working directory)
    #[clap(long = "build-dir")]
    pub build_dir: Option<PathBuf>,
    
    /// Override cache directory (default is .faircamp_cache/ inside the current working directory)
    #[clap(long = "cache-dir")]
    pub cache_dir: Option<PathBuf>,
    
    /// Override catalog directory (default is the current working directory)
    #[clap(long = "catalog-dir")]
    pub catalog_dir: Option<PathBuf>,
    
    /// Deploys to the configured server via rsync after the build is finished
    #[clap(long = "deploy", short = 'd')]
    pub deploy: bool,
    
    /// Configures the deploy destination (passed to rsync as [DEST] argument), e.g. "user@example.com:/var/www/example.com/music/"
    #[clap(long = "deploy-destination")]
    pub deploy_destination: Option<String>,
    
    /// Locally previews the build in the browser after the build is finished
    #[clap(long = "preview", short = 'p')]
    pub preview: bool,

    /// Wipes the asset cache BEFORE building - it then gets newly populated during building.
    #[clap(long = "wipe-cache")]
    pub wipe_cache: bool
}
