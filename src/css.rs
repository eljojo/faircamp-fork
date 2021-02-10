use indoc::indoc;

pub const DEFAULT: &str = indoc!(r#"
    a {
        color: #ddd;
        text-decoration: none;
    }
    a.play {
        cursor: pointer;
        margin-right: .3em;
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
    h1 {
        font-size: 1.8em;
        margin: 0;
    }
    header > nav { padding: .6em; }
    header > nav > *:not(:first-child) { margin-left: .6em; }
    img { display: block; }
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
    .muted { color: #444; }
    .releases {
        display: grid;
        grid-template-columns: repeat(auto-fit, 200px);
    }
    .vpad { margin: 1em 0; }
"#);
