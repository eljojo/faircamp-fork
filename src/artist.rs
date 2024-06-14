// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::{
    DescribedImage,
    HtmlAndStripped,
    Permalink,
    ReleaseRc,
    Theme
};

#[derive(Debug)]
pub struct Artist {
    pub aliases: Vec<String>,
    pub copy_link: bool,
    pub image: Option<DescribedImage>,
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
    pub fn new(copy_link: bool, name: &str, theme: Theme) -> Artist {
        let permalink = Permalink::generate(name);
        
        Artist {
            aliases: Vec::new(),
            copy_link,
            image: None,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            text: None,
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