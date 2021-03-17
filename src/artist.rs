use crate::{
    catalog::Permalink,
    image::Image
};

#[derive(Debug)]
pub struct Artist {
    pub image: Option<Image>,
    pub links: Vec<Link>,
    pub location: Option<String>,
    pub name: String,
    pub permalink: Permalink
}

impl Artist {
    pub fn init(name: String, permalink: Option<String>) -> Artist {
        let permalink = Permalink::new(permalink, &name);
        
        Artist {
            image: None,
            links: Vec::new(),
            location: None,
            name,
            permalink
        }
    }
}

#[derive(Debug)]
pub struct Link {
    pub label: String,
    pub url: String
}