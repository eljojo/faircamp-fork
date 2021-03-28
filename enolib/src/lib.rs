use std::fmt;
use std::mem;

mod document;
mod parse;

#[cfg(test)]
mod test;

pub use document::{
    Attribute,
    Document,
    Element,
    FieldContent,
    Item,
    Kind
};
pub use parse::Error;
use parse::ParseContext;

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
            parse::associate_comment_with_document(&mut context);
            context.comment.clear();
            continue
        } else if trimmed.starts_with(">") {
            parse::read_comment(&mut context, trimmed);
        } else if trimmed.starts_with("--") {
            parse::read_embed(&mut context, trimmed)?;
        } else if trimmed.starts_with("`") {
            parse::read_attribute_empty_field_escaped_key(&mut context, trimmed)?;
        } else if trimmed.starts_with("-") {
            parse::read_item(&mut context, trimmed)?;
        } else if trimmed.starts_with("|") {
            parse::read_continuation(&mut context, trimmed)?;
        } else if trimmed.starts_with("\\") {
            context.next_continuation_direct = false;
            parse::read_continuation(&mut context, trimmed)?;
        } else if trimmed.starts_with("#") {
            parse::read_section(&mut context, trimmed)?;
        } else {
            parse::read_attribute_empty_field(&mut context, trimmed)?;
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