#[derive(Debug)]
pub struct Artist {
    pub image: Option<String>,
    pub links: Vec<Link>,
    pub location: Option<String>,
    pub name: String
}

impl Artist {
    pub fn init(name: String) -> Artist {
        Artist {
            image: None,
            links: Vec::new(),
            location: None,
            name
        }
    }
}

#[derive(Debug)]
pub struct Link {
    pub label: String,
    pub url: String
}