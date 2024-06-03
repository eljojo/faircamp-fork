// SPDX-FileCopyrightText: 2021-2023 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[cfg_attr(feature = "image", clap(version = concat!(env!("CARGO_PKG_VERSION"), " (compiled without libvips)")))]
#[cfg_attr(feature = "libvips", clap(version = concat!(env!("CARGO_PKG_VERSION"), " (compiled with libvips)")))]
pub struct Args {
    /// Reports cached assets that currently appear obsolete and their consumed disk space (no build is performed)
    #[clap(long = "analyze-cache")]
    pub analyze_cache: bool,
    
    /// Override build directory (default is .faircamp_build/ inside the catalog directory)
    #[clap(long = "build-dir")]
    pub build_dir: Option<PathBuf>,
    
    /// Override cache directory (default is .faircamp_cache/ inside the catalog directory)
    #[clap(long = "cache-dir")]
    pub cache_dir: Option<PathBuf>,
    
    /// Override catalog directory (default is the current working directory)
    #[clap(long = "catalog-dir")]
    pub catalog_dir: Option<PathBuf>,

    /// The site is rendered with every translatable interface string showing its key instead of its value (=translation)
    #[clap(long = "debug-translations")]
    pub debug_translations: bool,

    /// Deploys to the configured server via rsync after the build is finished
    #[clap(long = "deploy", short = 'd')]
    pub deploy: bool,
    
    /// Configures the deploy destination (passed to rsync as [DEST] argument), e.g. "user@example.com:/var/www/example.com/music/"
    #[clap(long = "deploy-destination")]
    pub deploy_destination: Option<String>,

    /// Excludes all file paths that contain the specified pattern from being processed. Multiple can be supplied. Matching is done by simple case-sensitive string comparison - no glob/regex.
    #[clap(long = "exclude")]
    pub exclude_patterns: Vec<String>,

    /// Pass this so only file paths that contain the specified pattern will get processed. Multiple can be supplied. Matching is done by simple case-sensitive string comparison - no glob/regex.
    #[clap(long = "include")]
    pub include_patterns: Vec<String>,

    /// Opens the faircamp manual in your browser, does not do anything else.
    #[clap(long = "manual")]
    pub manual: bool,

    /// Generate full links, e.g. "/my-album/index.html" instead of "/my-album/". Creates a build that is fully browsable from your local disk without a webserver
    #[clap(long = "no-clean-urls")]
    pub no_clean_urls: bool,

    /// Reclaims disk space by removing all cached assets that were not used for the last build and exits (no build is performed)
    #[clap(long = "optimize-cache")]
    pub optimize_cache: bool,
    
    /// Locally previews the build in the browser after the build is finished (usually spins up an http server, except for builds with --no-clean-urls which can be directly browsed)
    #[clap(long = "preview", short = 'p')]
    pub preview: bool,

    /// Can be set in conjunction with --preview to manually configure the port used by the preview server (otherwise faircamp chooses an available port on its own)
    #[clap(long = "preview-port")]
    pub preview_port: Option<u16>,

    /// Injects a small widget into the page which allows you to interactively explore different theme color configurations
    #[clap(long = "theming-widget")]
    pub theming_widget: bool,

    /// Show more messages during build
    #[clap(long = "verbose", short = 'v')]
    pub verbose: bool,

    /// Wipes the build and cache directory and exits (no build is performed)
    #[clap(long = "wipe-all")]
    pub wipe_all: bool,
    
    /// Wipes the build directory and exits (no build is performed)
    #[clap(long = "wipe-build")]
    pub wipe_build: bool,
    
    /// Wipes the cache directory and exits (no build is performed)
    #[clap(long = "wipe-cache")]
    pub wipe_cache: bool
}
