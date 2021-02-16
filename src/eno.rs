#[derive(Debug)]
pub enum Element {
    Empty {
        key: String
    },
    Field {
        content: FieldContent,
        key: String
    }
}

#[derive(Debug)]
pub struct Entry {
    key: String,
    value: String
}

#[derive(Debug)]
pub enum FieldContent {
    Entries(Vec<Entry>),
    Items(Vec<String>),
    None,
    Value(String)
}

pub fn parse(input: &str) -> Result<Vec<Element>, String> {
    let mut elements = Vec::new();
    
    for line in input.lines() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() || trimmed.starts_with(">") { continue }
        
        if trimmed.starts_with("--") {
            return Err(format!("Multiline fields are not (yet) supported in faircamp's manifest files. ('{}')", trimmed))
        } else if trimmed.starts_with("`") {
            return Err(format!("Escaped keys are not (yet) supported in faircamp's manifest files. ('{}')", trimmed))
        } else if trimmed.starts_with("-") {
            let item = trimmed[1..].trim().to_string();
            
            match elements.last_mut() {
                Some(Element::Field { content, .. }) => match content {
                    FieldContent::Items(items) => items.push(item),
                    FieldContent::None => *content = FieldContent::Items(vec![item]),
                    _ => return Err(format!("Item without field encountered in manifest. ('{}')", trimmed))
                }
                _ => return Err(format!("Item without field encountered in manifest. ('{}')", trimmed))
            }
        } else if trimmed.starts_with("|") {
            return Err(format!("Direct continuations are not (yet) supported in faircamp's manifest files. ('{}')", trimmed))
        } else if trimmed.starts_with("\\") {
            return Err(format!("Spaced continuations are not (yet) supported in faircamp's manifest files. ('{}')", trimmed))
        } else if trimmed.starts_with("#") {
            return Err(format!("Sections are not allowed in faircamp's manifest files. ('{}')", trimmed))
        } else {
            if let Some(index) = trimmed.find(|c| c == ':' || c == '=' || c == '<') {
                if trimmed[index..].starts_with(":") {
                    let field = Element::Field {
                        content: if trimmed[(index + 1)..].trim().is_empty() {
                            FieldContent::None
                        } else {
                            FieldContent::Value(trimmed[(index + 1)..].trim().to_string())
                        },
                        key: trimmed[..index].trim().to_string()
                    };
                    
                    elements.push(field);
                } else if trimmed[index..].starts_with("=") {
                    let entry = Entry {
                        key: trimmed[..index].trim().to_string(),
                        value: trimmed[(index + 1)..].trim().to_string()
                    };
                    
                    match elements.last_mut() {
                        Some(Element::Field { content, .. }) => match content {
                            FieldContent::Entries(entries) => entries.push(entry),
                            FieldContent::None => *content = FieldContent::Entries(vec![entry]),
                            _ => return Err(format!("Entry without field encountered in manifest. ('{}')", trimmed))
                        }
                        _ => return Err(format!("Entry without field encountered in manifest. ('{}')", trimmed))
                    }
                } else if trimmed[index..].starts_with("<") {
                    return Err(format!("Copies are not (yet) supported in faircamp's manifest files. ('{}')", trimmed));
                }
            } else {
                elements.push(Element::Empty { key: trimmed.to_string() })
            }
        }
    }
    
    Ok(elements)
}