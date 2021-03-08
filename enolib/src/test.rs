use indoc::formatdoc;

#[test]
fn test_comments() {
    let result = crate::parse(&formatdoc!(r#"
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
fn test_escaped_key() {
    let result = crate::parse("`` `empty` ``");
    
    assert_eq!(
        &result.unwrap().elements.first().unwrap().key,
        "`empty`"
    );
}

#[test]
fn test_parse() {
    let result = crate::parse("field: value");
    
    assert!(result.is_ok());
}

