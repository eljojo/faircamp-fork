// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use slug::slugify;

use crate::{ArtistRc, ReleaseRc};
use crate::util::uid;

#[derive(Clone, Debug)]
pub struct Permalink {
    pub generated: bool,
    pub slug: String
}

pub enum PermalinkUsage<'a> {
    Artist(&'a ArtistRc),
    Release(&'a ReleaseRc)
}

impl Permalink {
    pub fn generate(non_slug: &str) -> Permalink {
        Permalink {
            generated: true,
            slug: slugify(non_slug)
        }
    }

    pub fn generated_or_assigned_str(&self) -> &str {
        if self.generated { "auto-generated" } else { "user-assigned" }
    }

    pub fn new(slug: &str) -> Result<Permalink, String> {
        let slugified = slugify(slug);

        if slug == slugified {
            Ok(Permalink {
                generated: false,
                slug: slug.to_string()
            })
        } else {
            Err(format!("'{}' is not a valid permalink, an allowed version would be '{}'", slug, slugified))
        }
    }

    pub fn uid() -> Permalink {
        Permalink {
            generated: false,
            slug: uid()
        }
    }
}

impl<'a> PermalinkUsage<'a> {
    pub fn as_string(&self) -> String {
        match self {
            PermalinkUsage::Artist(artist) => {
                let artist_ref = artist.borrow();
                let generated_or_assigned = artist_ref.permalink.generated_or_assigned_str();
                let name = &artist_ref.name;

                format!("{generated_or_assigned} permalink of the artist '{name}'")
            }
            PermalinkUsage::Release(release) => {
                let release_ref = release.borrow();
                let generated_or_assigned = release_ref.permalink.generated_or_assigned_str();
                let release_dir = release_ref.source_dir.display();
                let title = &release_ref.title;

                format!("{generated_or_assigned} permalink of the release '{title}' from directory '{release_dir}'")
            }
        }
    }
}