#[derive(Debug)]
pub struct Artist {
    pub image: Option<String>,
    pub links: Vec<Link>,
    pub location: Option<String>,
    pub name: String
}

#[derive(Debug)]
pub struct Link {
    pub label: String,
    pub url: String
}