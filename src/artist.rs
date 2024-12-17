// SPDX-FileCopyrightText: 2021-2024 Simon Repp
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
    pub featured: bool,
    pub image: Option<DescribedImage>,
    pub links: Vec<Link>,
    /// Optional override label for the button that (by default) says "More" on the
    /// artist page and points to the long-form text on the artist page.
    pub more_label: Option<String>,
    pub name: String,
    pub permalink: Permalink,
    pub releases: Vec<ReleaseRc>,
    pub text: Option<HtmlAndStripped>,
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
            featured: false,
            image: None,
            links: Vec::new(),
            more_label: None,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            text: None,
            theme: catalog.theme.clone(),
            unlisted: false
        }
    }

    /// This is how we create an artist if we encouter an artist that
    /// is manually defined in the catalog (via an artist manifest or
    /// through a short-form artist definition).
    pub fn new_manual(
        aliases: Vec<String>,
        copy_link: bool,
        image: Option<DescribedImage>,
        links: Vec<Link>,
        more_label: Option<String>,
        name: &str,
        permalink: Option<Permalink>,
        text: Option<HtmlAndStripped>,
        theme: Theme
    ) -> Artist {
        let permalink = permalink.unwrap_or_else(|| Permalink::generate(&name));

        Artist {
            aliases,
            copy_link,
            featured: false,
            image,
            links,
            more_label,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            text,
            theme,
            unlisted: false
        }
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