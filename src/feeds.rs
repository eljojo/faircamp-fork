// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Build, Catalog};

use translations::Translations;

mod atom;
mod generic_rss;
mod media_rss;
mod podcast_rss;
mod rss;

#[derive(Debug)]
pub struct Feeds {
    pub atom: bool,
    pub generic_rss: bool,
    pub media_rss: bool,
    pub podcast_rss: bool
}

impl Feeds {
    pub const ALL: Feeds = Feeds {
        atom: true,
        generic_rss: true,
        media_rss: false, // TODO: Change to true once media rss is implemented
        podcast_rss: true
    };

    pub const ATOM_FILENAME: &str = "feed.atom";

    pub const ATOM_ONLY: Feeds = Feeds {
        atom: true,
        generic_rss: false,
        media_rss: false,
        podcast_rss: false
    };

    pub const DEFAULT: Feeds = Feeds {
        atom: true,
        generic_rss: true,
        media_rss: false, // TODO: Change to true once media rss is implemented
        podcast_rss: false
    };

    pub const DISABLED: Feeds = Feeds {
        atom: false,
        generic_rss: false,
        media_rss: false,
        podcast_rss: false
    };

    pub const GENERIC_RSS_FILENAME: &str = "feed.rss";

    pub const GENERIC_RSS_ONLY: Feeds = Feeds {
        atom: false,
        generic_rss: true,
        media_rss: false,
        podcast_rss: false
    };

    pub const MEDIA_RSS_FILENAME: &str = "media.rss";

    pub const MEDIA_RSS_ONLY: Feeds = Feeds {
        atom: false,
        generic_rss: false,
        media_rss: false, // TODO: Change to true once media rss is implemented
        podcast_rss: false
    };

    pub const PODCAST_RSS_FILENAME: &str = "podcast.rss";

    pub const PODCAST_RSS_ONLY: Feeds = Feeds {
        atom: false,
        generic_rss: false,
        media_rss: false,
        podcast_rss: true
    };

    /// Whether any type of feed is enabled
    pub fn any_requested(&self) -> bool {
        self.atom ||
        self.generic_rss ||
        self.media_rss ||
        self.podcast_rss
    }

    /// Generate all enabled feeds, writing them to the build directory.
    pub fn generate(&self, build: &mut Build, catalog: &Catalog) {
        if self.atom {
            atom::atom(build, catalog);
            build.reserve_filename(Feeds::ATOM_FILENAME);
        }

        if self.generic_rss {
            generic_rss::generic_rss(build, catalog);
            build.reserve_filename(Feeds::GENERIC_RSS_FILENAME);
        }

        if self.media_rss {
            media_rss::media_rss(build, catalog);
            build.reserve_filename(Feeds::MEDIA_RSS_FILENAME);
        }

        if self.podcast_rss {
            podcast_rss::podcast_rss(build, catalog);
            build.reserve_filename(Feeds::PODCAST_RSS_FILENAME);
        }
    }

    /// <link> tags to be placed in the <head> of the page to point browser to
    /// available feeds for the site.
    pub fn meta_link_tags(
        &self,
        root_prefix: &str,
        translations: &Translations
    ) -> String {
        let mut tags = Vec::new();

        if self.atom {
            let filename = Feeds::ATOM_FILENAME;
            tags.push(format!(r#"
                <link rel="alternate" type="application/atom+xml" title="Atom" href="{root_prefix}{filename}">
            "#));
        }

        if self.generic_rss {
            let filename = Feeds::GENERIC_RSS_FILENAME;
            let t_generic_rss = &translations.generic_rss;
            tags.push(format!(r#"
                <link rel="alternate" type="application/rss+xml" title="{t_generic_rss}" href="{root_prefix}{filename}">
            "#));
        }

        if self.media_rss {
            let filename = Feeds::MEDIA_RSS_FILENAME;
            tags.push(format!(r#"
                <link rel="alternate" type="application/rss+xml" title="Media RSS" href="{root_prefix}{filename}">
            "#));
        }

        if self.podcast_rss {
            let filename = Feeds::PODCAST_RSS_FILENAME;
            tags.push(format!(r#"
                <link rel="alternate" type="application/rss+xml" title="Podcast RSS" href="{root_prefix}{filename}">
            "#));
        }

        tags.join("\n")
    }
}
