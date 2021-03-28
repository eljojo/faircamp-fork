use std::{
    fmt,
    mem,
    str::Lines
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Attribute {
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
    pub key: String,
    pub kind: Kind,
    pub line_number: u32,
    template: Option<String>
}

/// `line` is 1-indexed (i.e. the first line is line 1, not 0)
#[derive(Debug)]
pub struct Error {
    pub line: u32,
    pub message: String
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

pub struct ParseContext<'a> {
    comment: Vec<(u32, String)>, // (line_number, value)
    document: Document,
    line_number: u32,
    lines: Lines<'a>,
    /// memorizes direct/spaced continuation state while reading values that can be continued
    next_continuation_direct: bool,
    section_depth: usize,
    section_elements: Vec<Element>,
    section_key: String,
    section_line_number: u32
}

impl Document {
    fn new() -> Document {
        Document {
            comment: None,
            elements: Vec::new()
        }
    }
}

impl Element {
    fn new(key: String, kind: Kind, line_number: u32) -> Element {
        Element {
            key,
            kind,
            line_number,
            template: None
        }
    }
    
    fn new_copy(key: String, line_number: u32, template: String) -> Element {
        Element {
            key,
            kind: Kind::Empty,
            line_number,
            template: Some(template)
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
        next_continuation_direct: true,
        section_depth: 0,
        section_elements: Vec::new(),
        section_key: String::new(),
        section_line_number: 0
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
    
    // TODO: DRY - identical code in read_section
    if context.section_depth == 0 {
        context.document.elements.append(&mut context.section_elements);
    } else {
        let key = mem::take(&mut context.section_key);
        let kind = Kind::Section(mem::take(&mut context.section_elements));
        let section = Element::new(key, kind, context.line_number);
        
        fn deep_append(depth: usize, elements: &mut Vec<Element>, section: Element) {
            if depth == 0 {
                elements.push(section);
            } else {
                match elements.last_mut() {
                    Some(Element { kind: Kind::Section(subelements), .. }) => deep_append(depth - 1, subelements, section),
                    _ => unreachable!() // we know the last element exists and must be a section
                } 
            }
        }
        
        deep_append(context.section_depth - 1, &mut context.document.elements, section);
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
        let empty = Element::new(key, Kind::Empty, context.line_number);
        
        context.section_elements.push(empty);
    } else if remainder.starts_with(":") {
        let field_content = if remainder[1..].trim().is_empty() {
            FieldContent::None
        } else {
            FieldContent::Value(remainder[1..].trim().to_string())
        };
        
        let field = Element::new(key, Kind::Field(field_content), context.line_number);
        
        context.section_elements.push(field);
        context.next_continuation_direct = true;
    } else if remainder.starts_with("=") {
        let attribute = Attribute {
            key,
            line_number: context.line_number,
            value: remainder[1..].trim().to_string()
        };
        
        match context.section_elements.last_mut() {
            Some(Element { kind: Kind::Field(content), .. }) => match content {
                FieldContent::Attributes(attributes) => attributes.push(attribute),
                FieldContent::None => *content = FieldContent::Attributes(vec![attribute]),
                _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", remainder), context.line_number)) // TODO: Here and elsewhere we are printing remainder/trimmed, but in reality we need to be printing the line, verbatim
            }
            _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", remainder), context.line_number))
        }
        
        context.next_continuation_direct = true;
    } else if remainder.starts_with("<") {
        let template = if remainder[1..].trim().is_empty() {
            return Err(Error::new(format!("Copy operator needs to be followed by a template key. ('{}')", remainder), context.line_number));
        } else {
            remainder[1..].trim().to_string()
        };
        
        let copy = Element::new_copy(key, context.line_number, template);
        
        context.document.elements.push(copy);
        
        // TODO: Remove when implemented
        eprintln!("Copies are not yet resolved during parsing - element will appear as an empty for now. ('{}')", remainder);
    } else {
        return Err(Error::new(format!("Invalid syntax following a valid escaped key. ('{}')", remainder), context.line_number));
    }
    
    Ok(())
}
    
fn read_attribute_empty_field(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    if let Some(index) = trimmed.find(|c| c == ':' || c == '=' || c == '<') {
        if trimmed[index..].starts_with(":") {
            let key = trimmed[..index].trim().to_string();            
            let field_content = if trimmed[(index + 1)..].trim().is_empty() {
                FieldContent::None
            } else {
                FieldContent::Value(trimmed[(index + 1)..].trim().to_string())
            };
            
            let field = Element::new(key, Kind::Field(field_content), context.line_number);
            
            context.section_elements.push(field);
            context.next_continuation_direct = true;
        } else if trimmed[index..].starts_with("=") {
            let attribute = Attribute {
                key: trimmed[..index].trim().to_string(),
                line_number: context.line_number,
                value: trimmed[(index + 1)..].trim().to_string()
            };
            
            match context.section_elements.last_mut() {
                Some(Element { kind: Kind::Field(content), .. }) => match content {
                    FieldContent::Attributes(attributes) => attributes.push(attribute),
                    FieldContent::None => *content = FieldContent::Attributes(vec![attribute]),
                    _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", trimmed), context.line_number))
                }
                _ => return Err(Error::new(format!("Attribute without field encountered. ('{}')", trimmed), context.line_number))
            }
            
            context.next_continuation_direct = true;
        } else if trimmed[index..].starts_with("<") {
            let key = trimmed[..index].trim().to_string();
            let template = if trimmed[index + 1..].trim().is_empty() {
                return Err(Error::new(format!("Copy operator needs to be followed by a template key. ('{}')", trimmed), context.line_number));
            } else {
                trimmed[index + 1..].trim().to_string()
            };
            
            let copy = Element::new_copy(key, context.line_number, template);
            
            context.document.elements.push(copy);
            
            // TODO: Remove when implemented
            eprintln!("Copies are not yet resolved during parsing - element will appear as an empty for now. ('{}')", trimmed);
        }
    } else {
        let empty = Element::new(trimmed.to_string(), Kind::Empty, context.line_number);
        
        context.section_elements.push(empty);
    }
    
    Ok(())
}

fn read_comment(context: &mut ParseContext, trimmed: &str) {
    context.comment.push((context.line_number, trimmed[1..].to_string()));
}

fn read_continuation(context: &mut ParseContext, trimmed: &str) -> Result<(), Error> {
    let value = trimmed[1..].trim();
    
    if !value.is_empty() {
        match context.section_elements.last_mut() {
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
                    
                    if existing.value.is_empty() || context.next_continuation_direct {
                        existing.value.push_str(value)
                    } else {
                        existing.value.push_str(" ");
                        existing.value.push_str(value);
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
            let begin_line_number = context.line_number;
            let key = trimmed[begin_operator_len..].trim();
            let mut value = None;
            
            while let Some(line) = context.lines.next() {
                context.line_number += 1;
                
                let trimmed = line.trim();
                
                if trimmed.starts_with("--") {
                    if let Some(end_operator_len) = trimmed.find(|c| c != '-') {
                        if begin_operator_len == end_operator_len && key == trimmed[end_operator_len..].trim() {
                            let embed = Element::new(key.to_string(), Kind::Embed(value), begin_line_number);
                            
                            context.section_elements.push(embed);
                            
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
    let item = Item {
        line_number: context.line_number,
        value: trimmed[1..].trim().to_string()
    };
    
    match context.section_elements.last_mut() {
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
            if section_operator_len > context.section_depth + 1 {
                return Err(Error::new(format!("Section hierarchy layer skip encountered. ('{}')", trimmed), context.line_number));
            }
            
            let rightwards = trimmed[section_operator_len..].trim();
            
            if rightwards.starts_with("`") {
                match rightwards.find(|c| c != '`') {
                    Some(escape_operator_len) => {
                        let remainder = rightwards[escape_operator_len..].trim();
                        
                        // TODO: Handle escaped
                    }
                    None => return Err(Error::new(format!("Section with an empty escaped key. ('{}')", trimmed), context.line_number))
                }
            } else {
                if let Some(copy_operator_position) = rightwards.find(|c| c == '<') {
                    let key = rightwards[..copy_operator_position].trim();
                    let template = if rightwards[copy_operator_position + 1..].starts_with("<") {
                        // TODO: Carry on and implement deep copy flag/behavior
                        rightwards[copy_operator_position + 2..].trim()
                    } else {
                        rightwards[copy_operator_position + 1..].trim()
                    };
                    
                    if template.is_empty() {
                        return Err(Error::new(format!("Copy operator needs to be followed by a template key. ('{}')", trimmed), context.line_number));
                    }
                    
                    // TODO: Implement further
                    // let key = mem::take(&mut context.section_key);
                    // let kind = Kind::Section(mem::take(&mut context.section_elements));
                    // let section = Element::new(key, kind, context.section_line_number);
                    // 
                    // fn deep_append(depth: usize, elements: &mut Vec<Element>, section: Element) {
                    //     if depth == 0 {
                    //         elements.push(section);
                    //     } else {
                    //         match elements.last_mut() {
                    //             Some(Element { kind: Kind::Section(subelements), .. }) => deep_append(depth - 1, subelements, section),
                    //             _ => unreachable!() // we know the last element exists and must be a section
                    //         } 
                    //     }
                    // }
                    // 
                    // deep_append(context.section_depth - 1, &mut context.document.elements, section);
                    // 
                    // // TODO: Remove when implemented
                    // eprintln!("Copies are not yet resolved during parsing - element will appear as an empty for now. ('{}')", trimmed);
                } else {
                    if context.section_depth == 0 {
                        context.document.elements.append(&mut context.section_elements);
                    } else {
                        let key = mem::take(&mut context.section_key);
                        let kind = Kind::Section(mem::take(&mut context.section_elements));
                        let section = Element::new(key, kind, context.section_line_number);
                            
                        fn deep_append(depth: usize, elements: &mut Vec<Element>, section: Element) {
                            if depth == 0 {
                                elements.push(section);
                            } else {
                                match elements.last_mut() {
                                    Some(Element { kind: Kind::Section(subelements), .. }) => deep_append(depth - 1, subelements, section),
                                    _ => unreachable!() // we know the last element exists and must be a section
                                } 
                            }
                        }
                        
                        deep_append(context.section_depth - 1, &mut context.document.elements, section);
                    }
                    
                    context.section_depth = section_operator_len;
                    context.section_key = rightwards.to_string();
                    context.section_line_number = context.line_number;
                }
            }
        }
        None => return Err(Error::new(format!("Section without key in manifest. ('{}')", trimmed), context.line_number))
    }
    
    Ok(())
}
