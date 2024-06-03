// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::RefCell;
use std::rc::Rc;

use crate::{HtmlAndStripped, Image, Permalink, Release};

#[derive(Debug)]
pub struct Artist {
    pub aliases: Vec<String>,
    pub image: Option<Rc<RefCell<Image>>>,
    pub name: String,
    pub permalink: Permalink,
    pub releases: Vec<Rc<RefCell<Release>>>,
    pub text: Option<HtmlAndStripped>,
    pub unlisted: bool
}

impl Artist {
    pub fn new(name: &str) -> Artist {
        let permalink = Permalink::generate(name);
        
        Artist {
            aliases: Vec::new(),
            image: None,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            text: None,
            unlisted: false
        }
    }
}