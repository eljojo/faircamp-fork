<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Command-line arguments

Consult `faircamp --help` for the most authoritative and up-to-date information on available arguments.

That said here's a glimpse at some particularly interesting ones:

- `--build-dir <BUILD_DIR>` Override build directory (default is .faircamp_build/ inside the catalog directory). **Pay close attention where you point this to - this directory is wiped during the build process (!)**
- `--cache-dir <CACHE_DIR>` Override cache directory (default is .faircamp_cache/ inside the catalog directory). **Pay close attention where you point this to - this directory is wiped during the build process (!)**
- `--catalog-dir <CATALOG_DIR>` Override catalog directory (default is the current working directory)
- `--exclude <PATTERN>` Excludes all file paths that contain the specified pattern from being processed. Can be supplied multiple times. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--include <PATTERN>` Pass this so only file paths that contain the specified pattern will get processed. Can be supplied multiple times. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--manual` Opens the faircamp manual in your browser, does not do anything else
- `--no-clean-urls` Generate full links, e.g. "/my-album/index.html" instead of "/my-album/". Creates a build that is fully browsable from your local disk without a webserver
- `--preview` Locally previews the build in the browser after the build is finished (usually spins up an http server, except for builds with `--no-clean-urls` which can be directly browsed)
- `--theming-widget` Injects a small widget into the page which allows you to interactively explore different theme color configurations (see the reference page for `Theme`)