use indoc::formatdoc;

use crate::{FieldContent, Kind, parse};

#[test]
fn test_attributes() {
    let result = parse(&formatdoc!(r#"
        > comment
        field:
        attribute = value
    "#));

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.comment.as_deref(), Some("comment"));
    assert_eq!(element.key, "field");
    assert_eq!(element.line_number, 2);

    match &element.kind {
        Kind::Field(FieldContent::Attributes(attributes)) => {
            assert_eq!(attributes.len(), 1);

            let attribute = attributes.first().unwrap();

            assert_eq!(attribute.key, "attribute");
            assert_eq!(attribute.line_number, 3);
            assert_eq!(attribute.value, "value");
        }
        _ => panic!("Field with attributes expected")
    }
}

#[test]
fn test_continuations() {
    let result = parse(&formatdoc!(r#"
        field:
        | a
        \ b
        | c
    "#));

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    match &element.kind {
        Kind::Field(FieldContent::Value(value)) => {
            assert_eq!(value, "a bc");
        }
        _ => panic!("Field with value expected")
    }
}

#[test]
fn test_document_comment() {
    let result = parse(&formatdoc!(r#"
        >  alice
        > bob
        > carter

        empty
    "#));

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.line_number, 5);
    assert_eq!(document.comment.as_deref(), Some(" alice\nbob\ncarter"));
}

#[test]
fn test_embed() {
    let result = parse(&formatdoc!(r#"
        -- embed
        value
        -- embed
    "#));

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.key, "embed");
    assert_eq!(element.line_number, 1);

    match &element.kind {
        Kind::Embed(Some(value)) => {
            assert_eq!(value, "value");
        }
        _ => panic!("Embed with value expected")
    }
}

#[test]
fn test_empty() {
    let result = parse("empty");

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.key, "empty");
    assert_eq!(element.line_number, 1);

    match element.kind {
        Kind::Empty => (),
        _ => panic!("Empty expected")
    }
}

#[test]
fn test_empty_with_escaped_key() {
    let result = parse("`` `empty` ``");

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.key, "`empty`");

    match element.kind {
        Kind::Empty => (),
        _ => panic!("Empty expected")
    }
}

#[test]
fn test_items() {
    let result = parse(&formatdoc!(r#"
        field:
        > comment1
        - item1
        > comment2
        - item2
    "#));

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.key, "field");
    assert_eq!(element.line_number, 1);

    match &element.kind {
        Kind::Field(FieldContent::Items(items)) => {
            assert_eq!(items.len(), 2);

            let mut iter = items.iter();

            let first = iter.next().expect("First item expected");
            assert_eq!(first.comment.as_deref(), Some("comment1"));
            assert_eq!(first.line_number, 3);
            assert_eq!(first.value,"item1");

            let second = iter.next().expect("Second item expected");
            assert_eq!(second.comment.as_deref(), Some("comment2"));
            assert_eq!(second.line_number, 5);
            assert_eq!(second.value,"item2");
        }
        _ => panic!("Field with items expected")
    }
}

#[test]
fn test_section() {
    let result = parse(&formatdoc!(r#"
        > associated comment
        # section
        field: value
    "#));

    let document = result.unwrap();
    let element = document.elements.first().unwrap();

    assert_eq!(element.comment.as_deref(), Some("associated comment"));
    assert_eq!(element.key, "section");

    match &element.kind {
        Kind::Section(elements) => {
            assert_eq!(elements.len(), 1);

            let element = elements.first().unwrap();

            assert_eq!(element.key, "field");
            assert_eq!(element.line_number, 3);

            match &element.kind {
                Kind::Field(FieldContent::Value(value)) => {
                    assert_eq!(value, "value");
                }
                _ => panic!("Field with value expected")
            }
        }
        _ => panic!("Section expected")
    }
}

#[test]
fn test_parse() {
    let result = parse("field: value");

    assert!(result.is_ok());
}
