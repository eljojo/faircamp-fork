use indoc::indoc;

pub const DEFAULT: &str = indoc!(r#"
    a {
        color: #ddd;
        text-decoration: none;
    }
    body {
        background-color: #191919;
        color: #bbb;
        font-size: 18px;
        margin: 0;
    }
    footer {
        bottom: 0;
        color: #666;
        display: flex;
        justify-content: space-between;
        left: 0;
        position: absolute;
        width: 100%
    }
    footer > * { padding: .6em; }
    header > nav { padding: .6em; }
    header > nav > *:not(:first-child) { margin-left: .6em; }
    main { padding: .6em; }
    .cover {
        background-color: #222;
        height: 20em;
        width: 20em;
    }
    .layout {
        height: 100vh;
        overflow-x: hidden;
        overflow-y: auto;
    }
    .releases {
        display: grid;
        grid-template-columns: repeat(auto-fit, 200px);
    }
"#);
