<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Command-line arguments

Consult `faircamp --help` for the most authoritative and up-to-date information on available arguments.

That said here's a glimpse at some particularly interesting ones:

- `--build-dir <BUILD_DIR>` Override build directory (default is .faircamp_build/ inside the catalog directory). **Pay close attention where you point this to - this directory is wiped during the build process (!)**
- `--cache-dir <CACHE_DIR>` Override cache directory (default is .faircamp_cache/ inside the catalog directory). **Pay close attention where you point this to - this directory is wiped during the build process (!)**
- `--catalog-dir <CATALOG_DIR>` Override catalog directory (default is the current working directory)
- `--debug` Print debug information for the catalog (no build is performed)
- `--exclude <PATTERN>` Excludes all file paths that contain the specified pattern from being processed. Can be supplied multiple times. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--ignore-errors` By default, a build is interrupted when there are errors (e.g. invalid manifest options). With this option the build continues anyway when there are errors. Note that some critical errors can not be ignored (permalink conflicts, notably).
- `--include <PATTERN>` Pass this so only file paths that contain the specified pattern will get processed. Can be supplied multiple times. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--manual` Opens the faircamp manual in your browser, does not do anything else
- `--no-clean-urls` Generate full links, e.g. "/my-album/index.html" instead of "/my-album/". Creates a build that is fully browsable from your local disk without a webserver
- `--preview` Locally previews the build in the browser after the build is finished (usually spins up an http server, except for builds with `--no-clean-urls` which can be directly browsed)
- `--preview-ip` Can be set in conjunction with --preview to manually configure the ip used by the preview server (otherwise faircamp chooses 127.0.0.1 on its own)
- `--preview-port` Can be set in conjunction with --preview to manually configure the port used by the preview server (otherwise faircamp chooses an available port on its own)
- `--theming-widget` Injects a small widget into the page which allows you to interactively explore different theme color configurations (see the reference page for `Theme`)
