use std::{
    fmt,
    str::Lines
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String
}

#[derive(Debug)]
pub struct Document {
    pub comment: Option<String>,
    pub elements: Vec<Element>
}

#[derive(Debug)]
pub struct Element {
    pub key: String,
    pub kind: Kind
}

#[derive(Debug)]
pub enum Kind {
    Embed(Option<String>),
    Empty,
    Field(FieldContent),
    Section
}

#[derive(Debug)]
pub enum FieldContent {
    Attributes(Vec<Attribute>),
    Items(Vec<String>),
    None,
    Value(String)
}

/// `line` is 1-indexed (i.e. the first line is line 1, not 0)
#[derive(Debug)]
pub struct Error {
    pub line: u32,
    pub message: String
}

pub struct ParseContext<'a> {
    comment: Vec<(u32, String)>, // (line_number, value)
    document: Document,
    line_number: u32,
    lines: Lines<'a>,
    /// memorizes direct/spaced continuation state while reading values that can be continued
    next_continuation_direct: bool
}

impl Document {
    fn new() -> Document {
        Document {
            comment: None,
            elements: Vec::new()
        }
    }
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

fn associate_comment_with_document(context: &mut ParseContext) {
    let (mut shared_indentation_bytes, shared_indentation_string) = match context.comment.first() {
        Some((1, value)) =>  match value.find(|c: char| !c.is_whitespace()) {
            Some(index) => (index, value[..index].to_string()),
            None => (0, String::new())
        }
        _ => return
    };
        
    for (_line_number, value) in &context.comment[1..] {
        for (index, (shared_c, c)) in shared_indentation_string
            .chars()
            .zip(value.chars())
            .enumerate() {
            if index < shared_indentation_bytes {
                if c != shared_c {
                    shared_indentation_bytes = index;
                    break;
                }    
            } else {
                break;
            }
        }
    }
    
    let comment = context.comment
        .drain(..)
        .map(|(_line_number, value)| value[shared_indentation_bytes..].to_string())
        .collect::<Vec<String>>()
        .join("\n");
    
    context.document.comment = Some(comment);
}

pub fn parse(input: &str) -> Result<Document, Error> {
    let mut context = ParseContext {
        comment: Vec::new(),
        document: Document::new(),
        line_number: 0,
        lines: input.lines(),
        next_continuation_direct: true
    };
    
    while let Some(line) = context.lines.next() {
        let trimmed = line.trim();
        
        context.line_number += 1;
        
        if trimmed.is_empty() {
            associate_comment_with_document(&mut context);
            context.comment.clear();
            continue
        } else if trimmed.starts_with(">") {
            read_comment(&mut context, trimmed);
        } else if trimmed.starts_with("--") {
            read_embed(&mut context, trimmed)?;
        } else if trimmed.starts_with("`") {
            read_attribute_empty_field_escaped_key(&mut context, trimmed)?;
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
            read_attribute_empty_field(&mut context, trimmed)?;
        }
    }
    
    Ok(context.document)
}

fn read_attribute_empty_field_escaped_key(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    if let Some(escape_operator_len) = trimmed.find(|c| c != '`') {
        let remainder = trimmed[escape_operator_len..].trim();
        
        let mut chars = remainder.chars().enumerate();
        
        'search_terminator: while let Some((index, c)) = chars.next() {
            if c == '`' {
                for _ in 1..escape_operator_len {
                    match chars.next() {
                        Some((_index, '`')) => (),
                        Some(_) => continue 'search_terminator,
                        None => break 'search_terminator
                    }
                }
                
                let key = remainder[..index].trim().to_string();
                if key.is_empty() {
                    return Err(Error::new(format!("Escaped key is empty ('{}')", trimmed), context.line_number))
                }
                
                let remainder = remainder[index + escape_operator_len..].trim();
                
                return read_attribute_empty_field_remainder(context, key, remainder);
            }
        }
    }
    
    Err(Error::new(format!("Escaped key is never terminated ('{}')", trimmed), context.line_number))
}
    
fn read_attribute_empty_field_remainder(context: &mut ParseContext, key: String, remainder: &str) -> Result<(), Error> {
    if remainder.is_empty() {
        context.document.elements.push(Element {
            key,
            kind: Kind::Empty
        });
    } else if remainder.starts_with(":") {
        let element = Element {
            key,
            kind: Kind::Field(
                if remainder[1..].trim().is_empty() {
                    FieldContent::None
                } else {
                    FieldContent::Value(remainder[1..].trim().to_string())
                }
            )
        };
        
        context.document.elements.push(element);
        context.next_continuation_direct = true;
    } else if remainder.starts_with("=") {
        let attribute = Attribute {
            key,
            value: remainder[1..].trim().to_string()
        };
        
        match context.document.elements.last_mut() {
            Some(Element { kind: Kind::Field(content), .. }) => match content {
                FieldContent::Attributes(attributes) => attributes.push(attribute),
                FieldContent::None => *content = FieldContent::Attributes(vec![attribute]),
                _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", remainder), context.line_number)) // TODO: Here and elsewhere we are printing remainder/trimmed, but in reality we need to be printing the line, verbatim
            }
            _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", remainder), context.line_number))
        }
        
        context.next_continuation_direct = true;
    } else if remainder.starts_with("<") {
        // TODO: Implement
        // let element = Element {
        //     key,
        //     kind: Kind::Field(FieldContent::None)
        // };
        // context.document.elements.push(element);
        
        return Err(Error::new(format!("Copies are not (yet) supported. ('{}')", remainder), context.line_number));
    } else {
        return Err(Error::new(format!("Invalid syntax following a valid escaped key. ('{}')", remainder), context.line_number));
    }
    
    Ok(())
}
    
fn read_attribute_empty_field(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    if let Some(index) = trimmed.find(|c| c == ':' || c == '=' || c == '<') {
        if trimmed[index..].starts_with(":") {
            let element = Element {
                key: trimmed[..index].trim().to_string(),
                kind: Kind::Field(
                    if trimmed[(index + 1)..].trim().is_empty() {
                        FieldContent::None
                    } else {
                        FieldContent::Value(trimmed[(index + 1)..].trim().to_string())
                    }
                )
            };
            
            context.document.elements.push(element);
            context.next_continuation_direct = true;
        } else if trimmed[index..].starts_with("=") {
            let attribute = Attribute {
                key: trimmed[..index].trim().to_string(),
                value: trimmed[(index + 1)..].trim().to_string()
            };
            
            match context.document.elements.last_mut() {
                Some(Element { kind: Kind::Field(content), .. }) => match content {
                    FieldContent::Attributes(attributes) => attributes.push(attribute),
                    FieldContent::None => *content = FieldContent::Attributes(vec![attribute]),
                    _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", trimmed), context.line_number))
                }
                _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", trimmed), context.line_number))
            }
            
            context.next_continuation_direct = true;
        } else if trimmed[index..].starts_with("<") {
            // TODO: Implement
            // let element = Element {
            //     key: trimmed[..index].trim().to_string(),
            //     kind: Kind::Field(FieldContent::None)
            // };
            // context.document.elements.push(element);
            
            return Err(Error::new(format!("Copies are not (yet) supported. ('{}')", trimmed), context.line_number));
        }
    } else {
        context.document.elements.push(Element {
            key: trimmed.to_string(),
            kind: Kind::Empty
        })
    }
    
    Ok(())
}

fn read_comment(context: &mut ParseContext, trimmed: &str) {
    context.comment.push((context.line_number, trimmed[1..].to_string()));
}

fn read_continuation(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    let value = trimmed[1..].trim();
    
    if !value.is_empty() {
        match context.document.elements.last_mut() {
            Some(Element { kind: Kind::Field(content), .. }) => match content {
                FieldContent::Attributes(attributes) => {
                    let existing = &mut attributes.last_mut().unwrap().value;
                    
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
                            context.document.elements.push(Element {
                                key: key.to_string(),
                                kind: Kind::Embed(value)
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

fn read_item(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    let item = trimmed[1..].trim().to_string();
    
    match context.document.elements.last_mut() {
        Some(Element { kind: Kind::Field(content), .. }) => match content {
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
