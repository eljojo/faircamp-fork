use slug;

use crate::image::Image;

#[derive(Debug)]
pub struct Artist {
    pub image: Option<Image>,
    pub links: Vec<Link>,
    pub location: Option<String>,
    pub name: String,
    pub slug: String
}

impl Artist {
    pub fn init(name: String) -> Artist {
        let slug = slug::slugify(&name);
        
        Artist {
            image: None,
            links: Vec::new(),
            location: None,
            name,
            slug
        }
    }
}

#[derive(Debug)]
pub struct Link {
    pub label: String,
    pub url: String
}