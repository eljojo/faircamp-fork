use indoc::indoc;

pub const DEFAULT: &str = indoc!(r#"
    body {
        background-color: #444;
        color: #bbb;
    }
    footer { color: #666; }
    .cover {
        background-color: #222;
        height: 20em;
        width: 20em;
    }
"#);
