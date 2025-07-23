// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::SiteUrl;

#[derive(Clone, Debug)]
pub enum Link {
    Anchor {
        /// We store this including the leading '#'
        id: String,
        /// A mandatory, user-supplied label
        label: String
    },
    Full {
        /// Used in conjunction with rel="me" linking, when we want the link to be present
        /// to verify identity, but not display it.
        hidden: bool,
        /// A user-supplied label or an automatically constructed fallback
        /// (the url without http(s):// and without trailing slash).
        label: String,
        /// Indicates rel="me" linking (https://microformats.org/wiki/rel-me)
        rel_me: bool,
        url: String
    }
}

impl Link {
    pub fn anchor(
        id: String,
        label: String
    ) -> Link {
        Link::Anchor {
            id,
            label
        }
    }

    pub fn full(
        hidden: bool,
        label: Option<String>,
        rel_me: bool,
        url: impl Into<String>
    ) -> Link {
        let url = url.into();
        let label = label.unwrap_or(SiteUrl::pretty_display(&url).to_string());

        Link::Full {
            hidden,
            label,
            rel_me,
            url
        }
    }

    pub fn url(url: &str) -> Link {
        let hidden = false;
        let label = SiteUrl::pretty_display(url).to_string();
        let rel_me = false;
        let url = url.into();

        Link::Full {
            hidden,
            label,
            rel_me,
            url: url
        }
    }
}
