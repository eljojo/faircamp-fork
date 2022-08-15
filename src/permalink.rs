use std::cell::RefCell;
use std::rc::Rc;
use slug::slugify;

use crate::artist::Artist;
use crate::release::Release;

#[derive(Clone, Debug)]
pub struct Permalink {
    pub generated: bool,
    pub slug: String
}

pub enum PermalinkUsage<'a> {
    Artist(&'a Rc<RefCell<Artist>>),
    Release(&'a Release)
}

impl Permalink {
    pub fn generate(non_slug: &str) -> Permalink {
        Permalink {
            generated: true,
            slug: slugify(non_slug)
        }
    }

    pub fn new(slug: &str) -> Result<Permalink, String> {
        let slugified = slugify(slug);

        if slug == slugified {
            Ok(Permalink {
                generated: false,
                slug: slug.to_string()
            })
        } else {
            Err(format!("'{}' is not a valid permalink, an allowed version would be '{}'", slug, slugified))
        }
    }
}