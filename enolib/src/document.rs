#[derive(Debug)]
pub struct Attribute {
    pub comment: Option<String>,
    pub key: String,
    pub line_number: u32,
    pub value: String
}

#[derive(Debug)]
pub struct Document {
    pub comment: Option<String>,
    pub elements: Vec<Element>
}

#[derive(Debug)]
pub struct Element {
    pub comment: Option<String>,
    pub key: String,
    pub kind: Kind,
    pub line_number: u32
}

#[derive(Debug)]
pub enum FieldContent {
    Attributes(Vec<Attribute>),
    Items(Vec<Item>),
    None,
    Value(String)
}

#[derive(Debug)]
pub struct Item {
    pub comment: Option<String>,
    pub line_number: u32,
    pub value: String
}

#[derive(Debug)]
pub enum Kind {
    Embed(Option<String>),
    Empty,
    Field(FieldContent),
    Section(Vec<Element>)
}

impl Document {
    pub fn new() -> Document {
        Document {
            comment: None,
            elements: Vec::new()
        }
    }
}

impl Element {
    pub fn new(comment: Option<String>, key: String, kind: Kind, line_number: u32) -> Element {
        Element {
            comment,
            key,
            kind,
            line_number
        }
    }
}