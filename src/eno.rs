use std::{
    fmt,
    str::Lines
};

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
    pub key: String,
    pub value: String
}

#[derive(Debug)]
pub enum FieldContent {
    Entries(Vec<Entry>),
    Items(Vec<String>),
    None,
    Value(String)
}

/// `line` is 1-indexed (i.e. the first line is line 1, not 0)
pub struct Error {
    pub line: u32,
    pub message: String
}

pub struct ParseContext<'a> {
    elements: Vec<Element>,
    line_number: u32,
    lines: Lines<'a>,
    /// memorizes direct/spaced continuation state while reading values that can be continued
    next_continuation_direct: bool
}

impl Error {
    pub fn new(message: String, line_number: u32) -> Error {
        Error {
            line: line_number,
            message
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// TODO: Collect column for error (and for inspection of document via API)

pub fn parse(input: &str) -> Result<Vec<Element>, Error> {
    let mut context = ParseContext {
        elements: Vec::new(),
        line_number: 0,
        lines: input.lines(),
        next_continuation_direct: true
    };
    
    while let Some(line) = context.lines.next() {
        let trimmed = line.trim();
        
        context.line_number += 1;
        
        if trimmed.is_empty() || trimmed.starts_with(">") { continue }
        
        if trimmed.starts_with("--") {
            read_embed(&mut context, trimmed)?;
        } else if trimmed.starts_with("`") {
            return Err(Error::new(format!("Escaped keys are not (yet) supported in faircamp's manifest files. ('{}')", trimmed), context.line_number))
        } else if trimmed.starts_with("-") {
            read_item(&mut context, trimmed)?;
        } else if trimmed.starts_with("|") {
            read_continuation(&mut context, trimmed)?;
        } else if trimmed.starts_with("\\") {
            context.next_continuation_direct = false;
            read_continuation(&mut context, trimmed)?;
        } else if trimmed.starts_with("#") {
            read_section(&mut context, trimmed)?;
        } else {
            read_empty_entry_field(&mut context, trimmed)?;
        }
    }
    
    Ok(context.elements)
}

fn read_continuation(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    let value = trimmed[1..].trim();
    
    if !value.is_empty() {
        match context.elements.last_mut() {
            Some(Element::Field { content, .. }) => match content {
                FieldContent::Entries(entries) => {
                    let existing = &mut entries.last_mut().unwrap().value;
                    
                    if existing.is_empty() || context.next_continuation_direct {
                        existing.push_str(value)
                    } else {
                        existing.push_str(" ");
                        existing.push_str(value);
                    }
                }
                FieldContent::Items(items) => {
                    let existing = items.last_mut().unwrap();
                    
                    if existing.is_empty() || context.next_continuation_direct {
                        existing.push_str(value)
                    } else {
                        existing.push_str(" ");
                        existing.push_str(value);
                    }
                }
                FieldContent::None => *content = FieldContent::Value(value.to_string()),
                FieldContent::Value(existing) => {
                    if existing.is_empty() || context.next_continuation_direct {
                        existing.push_str(value)
                    } else {
                        existing.push_str(" ");
                        existing.push_str(value);
                    }
                }
            }
            _ => return Err(Error::new(format!("Continuation without continuable field encountered ('{}')", trimmed), context.line_number))
        }
        
        context.next_continuation_direct = true;
    }
    
    Ok(())
}
    
fn read_embed(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    match trimmed.find(|c| c != '-') {
        Some(begin_operator_len) => {
            let key = trimmed[begin_operator_len..].trim();
            let mut value = None;
            
            while let Some(line) = context.lines.next() {
                context.line_number += 1;
                
                let trimmed = line.trim();
                
                if trimmed.starts_with("--") {
                    if let Some(end_operator_len) = trimmed.find(|c| c != '-') {
                        if begin_operator_len == end_operator_len && key == trimmed[end_operator_len..].trim() {
                            context.elements.push(Element::Field {
                                content: FieldContent::Value(value.unwrap_or(String::new())),
                                key: key.to_string()
                            });
                            
                            return Ok(());
                        }
                    }
                }
                
                match &mut value {
                    Some(value) => value.push_str(&format!("\n{}", line)),
                    None => value = Some(String::from(line))
                }
            }
            
            return Err(Error::new(format!("Embed with key '{}' is never terminated", key), context.line_number));
        },
        None => return Err(Error::new(format!("Embed without key ('{}')", trimmed), context.line_number))
    }
}

fn read_empty_entry_field(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
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
            
            context.elements.push(field);
            context.next_continuation_direct = true;
        } else if trimmed[index..].starts_with("=") {
            let entry = Entry {
                key: trimmed[..index].trim().to_string(),
                value: trimmed[(index + 1)..].trim().to_string()
            };
            
            match context.elements.last_mut() {
                Some(Element::Field { content, .. }) => match content {
                    FieldContent::Entries(entries) => entries.push(entry),
                    FieldContent::None => *content = FieldContent::Entries(vec![entry]),
                    _ => return Err(Error::new(format!("Entry without field encountered. ('{}')", trimmed), context.line_number))
                }
                _ => return Err(Error::new(format!("Entry without field encountered. ('{}')", trimmed), context.line_number))
            }
            
            context.next_continuation_direct = true;
        } else if trimmed[index..].starts_with("<") {
            return Err(Error::new(format!("Copies are not (yet) supported. ('{}')", trimmed), context.line_number));
        }
    } else {
        context.elements.push(Element::Empty { key: trimmed.to_string() })
    }
    
    Ok(())
}

fn read_item(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    let item = trimmed[1..].trim().to_string();
    
    match context.elements.last_mut() {
        Some(Element::Field { content, .. }) => match content {
            FieldContent::Items(items) => items.push(item),
            FieldContent::None => *content = FieldContent::Items(vec![item]),
            _ => return Err(Error::new(format!("Item without field encountered ('{}')", trimmed), context.line_number))
        }
        _ => return Err(Error::new(format!("Item without field encountered ('{}')", trimmed), context.line_number))
    }
    
    Ok(())
}

fn read_section(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    match trimmed.find(|c| c != '#') {
        Some(section_operator_len) => {
            let rightwards = trimmed[section_operator_len..].trim();
            
            if rightwards.starts_with("`") {
                match rightwards.find(|c| c != '`') {
                    Some(escape_operator_len) => {
                        let remainder = rightwards[escape_operator_len..].trim();
                        
                        // TODO: Handle escaped
                    }
                    None => return Err(Error::new(format!("Section with an empty escaped key in manifest. ('{}')", trimmed), context.line_number))
                }
            } else {
                if let Some(copy_operator_position) = rightwards.find(|c| c == '<') {
                    let key = rightwards[..copy_operator_position].trim();
                    let template = rightwards[(copy_operator_position + 1)..].trim();
                } else {
                    let key = rightwards;
                }
            }
        }
        None => return Err(Error::new(format!("Section without key in manifest. ('{}')", trimmed), context.line_number))
    }
    
    Ok(())
}
