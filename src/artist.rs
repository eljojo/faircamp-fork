use crate::{
    catalog::Permalink,
    image::Image
};

#[derive(Debug)]
pub struct Artist {
    pub image: Option<Image>,
    pub links: Vec<Link>, // TODO: Revisit this - we want that? (as in: maybe leave up to user to provide this in text)
    pub location: Option<String>, // TODO: Revisit this - we want that? (as in: maybe leave up to user to provide this in text)
    pub name: String,
    pub permalink: Permalink,
    pub text: Option<String>
}

impl Artist {
    pub fn new(name: String, permalink: Option<String>) -> Artist {
        let permalink = Permalink::new(permalink, &name);
        
        Artist {
            image: None,
            links: Vec::new(),
            location: None,
            name,
            permalink,
            text: None
        }
    }
    
    pub fn new_from_manifest(
        name: String,
        permalink: Option<String>,
        text: Option<String>
    ) -> Artist {
        let permalink = Permalink::new(permalink, &name);
        
        Artist {
            image: None,
            links: Vec::new(),
            location: None,
            name,
            permalink,
            text
        }
    }
}

#[derive(Debug)]
pub struct Link {
    pub label: String,
    pub url: String
}