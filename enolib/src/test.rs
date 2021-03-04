use crate::Element;

#[test]
fn test_parse() {
    let result = crate::parse("field: value");
    
    assert!(result.is_ok());
}

#[test]
fn test_escaped_key() {
    let result = crate::parse("`` `empty` ``");
    
    assert_eq!(
        &result.unwrap().first().unwrap().key,
        "`empty`"
    );
}