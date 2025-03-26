// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use url::Url;

use crate::Build;

pub struct SiteUrl {
    /// Guaranteed to end on trailing slash
    normalized: String
}

impl SiteUrl {
    /// Return the root url of the site, with index.html conditionally
    /// appended based on the build.clean_urls setting.
    pub fn index(&self, build: &Build) -> String {
        match build.clean_urls {
            true => self.normalized.clone(),
            false => self.normalized.clone() + "index.html"
        }
    }

    /// path must be without leading slash e.g. "foo/bar.ext"
    pub fn join_file(&self, path: impl AsRef<str>) -> String {
        self.normalized.clone() + path.as_ref()
    }

    pub fn join_index(&self, build: &Build, path: impl AsRef<str>) -> String {
        match build.clean_urls {
            true => self.normalized.clone() + path.as_ref() + "/",
            false => self.normalized.clone() + path.as_ref() + "/index.html"
        }
    }

    // Join the path to the url so that it ends with a trailing slash
    pub fn join_prefix(&self, path: impl AsRef<str>) -> String {
        self.normalized.clone() + path.as_ref() + "/"
    }

    fn new(normalized: String) -> SiteUrl {
        SiteUrl {
            normalized
        }
    }

    pub fn parse(input: &str) -> Result<SiteUrl, String> {
        // Ensure the url has a trailing slash so that further url
        // construction during build is done correctly.
        let normalized_url = match input.ends_with('/') {
            true => input.to_string(),
            false => format!("{input}/")
        };

        match Url::parse(&normalized_url) {
            Ok(_) => Ok(SiteUrl::new(normalized_url)),
            Err(err) => Err(err.to_string())
        }
    }

    // The url as prefix, that is ending in a trailing slash
    pub fn prefix(&self) -> &str {
        &self.normalized
    }

    /// Returns the passed url without http(s):// and without trailing slash.
    pub fn pretty_display(url: &str) -> &str {
        url
            .split_once("://")
            .unwrap()
            .1
            .trim_end_matches('/')
    }

    pub fn without_trailing_slash(&self) -> &str {
        &self.normalized[..(self.normalized.len() - 1)]
    }

    /// Returns this url without http(s):// and without trailing slash.
    pub fn without_scheme_and_trailing_slash(&self) -> &str {
        let without_scheme = self.normalized.split_once("://").unwrap().1;
        &without_scheme[..(without_scheme.len() - 1)]
    }
}
