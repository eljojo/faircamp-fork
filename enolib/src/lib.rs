use std::fmt;

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
        section_comment: None,
        section_depth: 0,
        section_elements: Vec::new(),
        section_key: String::new(),
        section_line_number: 0
    };

    while let Some(line) = context.lines.next() {
        let trimmed = line.trim();

        context.line_number += 1;

        if trimmed.is_empty() {
            match context.comment.first() {
                Some((1, _)) => context.document.comment = context.assemble_comment(),
                Some(_) => context.comment.clear(),
                _ => ()
            }
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

    context.attach_section_elements();
    context.comment.clear();

    Ok(context.document)
}
