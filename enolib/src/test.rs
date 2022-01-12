use indoc::formatdoc;

use crate::{FieldContent, Kind, parse};

#[test]
fn test_document_comment() {
    let result = parse(&formatdoc!(r#"
        >  alice
        > bob
        > carter
        
        empty
    "#));
    
    assert_eq!(
        &result.unwrap().comment.unwrap(),
        " alice\nbob\ncarter"
    );
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
    
    match &element.kind {
        Kind::Embed(Some(value)) => {
            assert_eq!(value, "value");
        }
        _ => panic!("Embed with value expected")
    }
}

#[test]
fn test_escaped_key() {
    let result = parse("`` `empty` ``");
    
    assert_eq!(
        &result.unwrap().elements.first().unwrap().key,
        "`empty`"
    );
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
    assert_eq!(&element.key, "section");
    
    match &element.kind {
        Kind::Section(elements) => {
            assert_eq!(elements.len(), 1);
            
            let element = elements.first().unwrap();
            
            assert_eq!(&element.key, "field");
            
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

