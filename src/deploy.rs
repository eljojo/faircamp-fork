use crate::build_settings::BuildSettings;
use crate::rsync;

pub fn deploy(build_settings: &BuildSettings) {
    if let Some(destination) = &build_settings.deploy_destination {
        info!("Deployment started");
        rsync::sync(&build_settings.build_dir, destination).unwrap();
        info!("Deployment finished");
    } else {
        error!("No deployment destination specified, provide one with --deploy-destination")
    }
}