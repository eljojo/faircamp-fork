use log::{error, info};

use crate::build::Build;
use crate::rsync;

pub fn deploy(build: &Build) {
    if let Some(destination) = &build.deploy_destination {
        info!("Deployment started");
        rsync::sync(&build.build_dir, destination).unwrap();
        info!("Deployment finished");
    } else {
        error!("No deployment destination specified, provide one with --deploy-destination")
    }
}