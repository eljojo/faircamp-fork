// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::{
    Catalog,
    DescribedImage,
    HtmlAndStripped,
    Link,
    Permalink,
    ReleaseRc,
    Theme
};

#[derive(Debug)]
pub struct Artist {
    pub aliases: Vec<String>,
    pub copy_link: bool,
    /// This is only set when an artist is created through a short-form
    /// definition in a manifest, and when providing a link through it.
    /// Its presence indicates that we don't generate an internal (featured)
    /// artist page, but instead link to the artist on an external page.
    pub external_page: Option<String>,
    /// While reading the catalog we annotate artists who are featured
    /// (i.e. get their own page) with this flag. This helps us to correctly
    /// link to their pages where needed.
    pub featured: bool,
    pub image: Option<DescribedImage>,
    pub links: Vec<Link>,
    /// Whether an m3u playlist should be generated and provided for this artist
    pub m3u: bool,
    pub more: Option<HtmlAndStripped>,
    /// Optional override label for the button that (by default) says "More" on the
    /// artist page and points to additional long-form content for the artist.
    pub more_label: Option<String>,
    pub name: String,
    pub permalink: Permalink,
    pub releases: Vec<ReleaseRc>,
    pub synopsis: Option<String>,
    pub theme: Theme,
    pub unlisted: bool
}

#[derive(Clone, Debug)]
pub struct ArtistRc {
    artist: Rc<RefCell<Artist>>,
}

impl Artist {
    /// This is how we create an artist if the catalog has no explicitly
    /// defined artist that matches a release/track's artist. We use the
    /// name that was given on the release/track and pull some default
    /// options from the catalog.
    pub fn new_automatic(catalog: &Catalog, name: &str) -> Artist {
        let permalink = Permalink::generate(name);

        Artist {
            aliases: Vec::new(),
            copy_link: catalog.copy_link,
            external_page: None,
            featured: false,
            image: None,
            links: Vec::new(),
            m3u: false,
            more: None,
            more_label: None,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            synopsis: None,
            theme: catalog.theme.clone(),
            unlisted: false
        }
    }

    /// This is how we create an artist if we encounter an artist that
    /// is manually defined in the catalog via an artist manifest.
    pub fn new_manual(
        aliases: Vec<String>,
        copy_link: bool,
        external_page: Option<String>,
        image: Option<DescribedImage>,
        links: Vec<Link>,
        m3u: bool,
        more: Option<HtmlAndStripped>,
        more_label: Option<String>,
        name: &str,
        permalink: Option<Permalink>,
        synopsis: Option<String>,
        theme: Theme
    ) -> Artist {
        let permalink = permalink.unwrap_or_else(|| Permalink::generate(name));

        Artist {
            aliases,
            copy_link,
            external_page,
            featured: false,
            image,
            links,
            m3u,
            more,
            more_label,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            synopsis,
            theme,
            unlisted: false
        }
    }

    /// This is how we create an artist if we encouter an artist that is
    /// manually defined in the catalog through a short-form artist
    /// definition.
    pub fn new_shortcut(
        aliases: Vec<String>,
        catalog: &Catalog,
        external_page: Option<String>,
        name: &str,
        permalink: Option<Permalink>
    ) -> Artist {
        let permalink = match external_page.is_some() {
            // TODO: In terms of modeling, the fact that we need to set a permalink
            //       for this, might indicate that we should maybe have a separate structure to
            //       hold external artists (but this would have other implications too).
            true => Permalink::uid(),
            false => permalink.unwrap_or_else(|| Permalink::generate(name))
        };

        Artist {
            aliases,
            copy_link: false,
            external_page,
            featured: false,
            image: None,
            links: Vec::new(),
            m3u: false,
            more: None,
            more_label: None,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            synopsis: None,
            theme: catalog.theme.clone(),
            unlisted: false
        }
    }

    pub fn public_releases(&self) -> Vec<ReleaseRc> {
        self.releases
            .iter()
            .filter_map(|release| {
                match release.borrow().unlisted {
                    true => None,
                    false => Some(release.clone())
                }
            })
            .collect()
    }

    /// Returns - if available - the file name of the artist image,
    /// without any prefixing (i.e. in the context of the artist directory)
    pub fn thumbnail_image_src(&self) -> Option<String> {
        self.image
            .as_ref()
            .map(|described_image| {
                let image_ref = described_image.borrow();
                let asset = &image_ref.artist_assets.as_ref().unwrap().fixed_max_320;
                let filename = asset.target_filename();
                let hash = image_ref.hash.as_url_safe_base64();

                format!("{filename}?{hash}")
            })
    }
}

impl ArtistRc {
    pub fn borrow(&self) -> Ref<'_, Artist> {
        self.artist.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Artist> {
        self.artist.borrow_mut()
    }

    pub fn new(artist: Artist) -> ArtistRc {
        ArtistRc {
            artist: Rc::new(RefCell::new(artist))
        }
    }

    pub fn ptr_eq(a: &ArtistRc, b: &ArtistRc) -> bool {
        Rc::ptr_eq(&a.artist, &b.artist)
    }
}
