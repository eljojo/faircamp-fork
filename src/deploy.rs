// SPDX-FileCopyrightText: 2021-2022 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Build, rsync};

pub fn deploy(build: &Build) {
    if let Some(destination) = &build.deploy_destination {
        info!("Deployment started");
        rsync::sync(&build.build_dir, destination).unwrap();
        info!("Deployment finished");
    } else {
        error!("No deployment destination specified, provide one with --deploy-destination");
    }
}