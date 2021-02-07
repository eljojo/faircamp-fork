use crate::artist::Artist;
use crate::image::Image;
use crate::release::Release;

#[derive(Debug)]
pub struct Catalog {
    pub artists: Vec<Artist>,
    pub images: Vec<Image>, // TODO: Do we need these + what to do with them (also consider "label cover" aspect)
    pub releases: Vec<Release>
}

impl Catalog {
    pub fn init() -> Catalog {
        Catalog {
            artists: Vec::new(),
            images: Vec::new(),
            releases: Vec::new()
        }
    }
}