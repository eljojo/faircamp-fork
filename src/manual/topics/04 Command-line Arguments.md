# Command-line arguments

Consult `faircamp --help` for the most authoritative and up-to-date information on available arguments.

That said here's a glimpse at some particularly interesting ones:

- `--build-dir <BUILD_DIR>` Override build directory (default is .faircamp_build/ inside the catalog directory)
- `--cache-dir <CACHE_DIR>` Override cache directory (default is .faircamp_cache/ inside the catalog directory)
- `--catalog-dir <CATALOG_DIR>` Override catalog directory (default is the current working directory)
- `--exclude <EXCLUDE_PATTERNS>` Excludes all file paths that contain the specified pattern from being processed. Multiple can be supplied. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--include <INCLUDE_PATTERNS>` Pass this so only file paths that contain the specified pattern will get processed. Multiple can be supplied. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--no-clean-urls` Generate full links, e.g. "/my-album/index.html" instead of "/my-album/". Creates a build that is fully browsable from your local disk without a webserver
- `--preview` Opens the built site locally in your browser after building so you can check it out
- `--theming-widget` Injects a small widget into the page which allows you to interactively explore different theme color configurations (see the reference page for `Theme`)