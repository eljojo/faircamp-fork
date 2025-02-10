// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Build faircamp with (e.g.) FAIRCAMP_PKG_VERSION=2.0.0~pre1 to override
/// CARGO_PKG_VERSION in resulting builds.

use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=FAIRCAMP_PKG_VERSION");

    if let Ok(override_version) = env::var("FAIRCAMP_PKG_VERSION") {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={override_version}");
    }
}
