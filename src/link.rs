// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use url::Url;

#[derive(Clone, Debug)]
pub struct Link {
    /// Used in conjunction with rel="me" linking, when we want the link to be present
    /// to verify identity, but not display it.
    pub hidden: bool,
    pub label: Option<String>,
    /// Indicates rel="me" linking (https://microformats.org/wiki/rel-me)
    pub rel_me: bool,
    pub url: Url
}

impl Link {
    pub fn new(
        hidden: bool,
        label: Option<String>,
        rel_me: bool,
        url: Url
    ) -> Link {
        Link {
            hidden,
            label,
            rel_me,
            url
        }
    }

    /// Returns either the label itself, or as a fallback the url
    /// without the http(s):// part and without trailing slash.
    pub fn pretty_label(&self) -> String {
        match &self.label {
            Some(label) => label.clone(),
            None => self.url.to_string()
                .split_once("://")
                .unwrap()
                .1
                .trim_end_matches('/')
                .to_string()
        }
    }
}