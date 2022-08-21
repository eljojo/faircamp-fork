use std::cell::RefCell;
use std::rc::Rc;

use crate::{Image, Permalink};

#[derive(Debug)]
pub struct Artist {
    pub aliases: Vec<String>,
    pub image: Option<Rc<RefCell<Image>>>,
    pub name: String,
    pub permalink: Permalink,
    pub text: Option<String>
}

impl Artist {
    pub fn new(name: &str) -> Artist {
        let permalink = Permalink::generate(name);
        
        Artist {
            aliases: Vec::new(),
            image: None,
            name: name.to_string(),
            permalink,
            text: None
        }
    }
}

#[derive(Debug)]
pub struct Link {
    pub label: String,
    pub url: String
}