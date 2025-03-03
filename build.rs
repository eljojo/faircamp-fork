// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Build faircamp with (e.g.) FAIRCAMP_PKG_VERSION=2.0.0~pre1 to override
/// the version that is displayed and reported in resulting builds.

use std::env;
use std::process::Command;

fn main() {
    let version_detailed;
    let version_display;
    if let Ok(override_version) = env::var("FAIRCAMP_PKG_VERSION") {
        version_detailed = override_version.clone();
        version_display = override_version;
    } else {
        version_detailed = env!("CARGO_PKG_VERSION").to_string();
        version_display = concat!(env!("CARGO_PKG_VERSION_MAJOR"), '.', env!("CARGO_PKG_VERSION_MINOR")).to_string();
    }

    let mut git = Command::new("git");
    git.args(["rev-parse", "--short", "HEAD"]);
    let revision = match git.output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_) => String::from("unknown revision")
    };

    println!("cargo:rerun-if-env-changed=FAIRCAMP_PKG_VERSION");
    println!("cargo:rustc-env=FAIRCAMP_REVISION={revision}");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DETAILED={version_detailed}");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DISPLAY={version_display}");
}
