use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    image::Image,
    permalink::Permalink
};

#[derive(Debug)]
pub struct Artist {
    pub aliases: Vec<String>,
    pub image: Option<Rc<RefCell<Image>>>,
    pub links: Vec<Link>, // TODO: Revisit this - we want that? (as in: maybe leave up to user to provide this in text)
    pub location: Option<String>, // TODO: Revisit this - we want that? (as in: maybe leave up to user to provide this in text)
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
            links: Vec::new(),
            location: None,
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