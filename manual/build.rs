// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Build the manual with (e.g.) FAIRCAMP_VERSION=2.0.0~pre1 to override
/// the version that is displayed in the built manual.

use std::env;

fn main() {
    let version_display = match env::var("FAIRCAMP_VERSION") {
        Ok(override_version) => override_version,
        Err(_) => concat!(env!("CARGO_PKG_VERSION_MAJOR"), '.', env!("CARGO_PKG_VERSION_MINOR")).to_string()
    };

    println!("cargo:rerun-if-env-changed=FAIRCAMP_VERSION");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DISPLAY={version_display}");
}
