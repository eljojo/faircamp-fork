// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;
use std::fs::{self, DirEntry};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use enolib::HtmlPrinter;
use indoc::formatdoc;
use pulldown_cmark::{CodeBlockKind, Event, html, Parser, Tag, TagEnd};
use slug::slugify;

struct Docs {
    examples: Vec<Page>,
    index: Page,
    reference: Vec<Page>,
    topics: Vec<Page>,
    version: &'static str
}

#[derive(PartialEq)]
struct Page {
    content: String,
    slug: String,
    title: String
}

pub fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);

    let mut inside_eno_codeblock = false;

    let parser = parser.map(|event| {
        if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref language))) = event {
            if language.deref() == "eno" {
                inside_eno_codeblock = true;
            }
        } else if let Event::End(TagEnd::CodeBlock) = event {
            inside_eno_codeblock = false;
        } else if let Event::Text(ref text) = event {
            if inside_eno_codeblock {
                // Fenced code comes with a trailing line break here, we trim it away
                let eno_source = text.trim_end();
                let document = match enolib::parse(eno_source) {
                    Ok(document) => document,
                    Err(err) => panic!("Syntax error in {} ({})", text, err)
                };
                let syntax_highlighted = document.snippet_with_options(&HtmlPrinter, false);
                return Event::Html(syntax_highlighted.into())
            }
        }

        event
    });

    html::push_html(&mut html_output, parser);

    html_output
}

pub fn main() {
    let mut args = env::args();

    let manual_out_dir = match args.nth(1) {
        Some(path) => PathBuf::from(&path),
        None => {
            eprintln!("A single argument is required (directory path to which to write the manual), aborting.");
            return;
        }
    };

    let docs = read_docs();

    if manual_out_dir.exists() {
        let _ = fs::remove_dir_all(&manual_out_dir);
    }

    fs::create_dir(&manual_out_dir).unwrap();

    render_page(
        &manual_out_dir,
        &docs,
        &docs.index,
        docs.topics.first()
    );

    let mut topics_iter = docs.topics.iter().peekable();
    while let Some(page) = topics_iter.next() {
        render_page(
            &manual_out_dir,
            &docs,
            page,
            topics_iter.peek().copied().or_else(|| docs.examples.first())
        );
    }

    let mut examples_iter = docs.examples.iter().peekable();
    while let Some(page) = examples_iter.next() {
        render_page(
            &manual_out_dir,
            &docs,
            page,
            examples_iter.peek().copied().or_else(|| docs.reference.first())
        );
    }

    let mut reference_iter = docs.reference.iter().peekable();
    while let Some(page) = reference_iter.next() {
        render_page(
            &manual_out_dir,
            &docs,
            page,
            reference_iter.peek().copied()
        );
    }

    fs::write(
        manual_out_dir.join("favicon.svg"),
        include_bytes!("../../src/assets/favicon.svg")
    ).unwrap();

    fs::write(
        manual_out_dir.join("favicon_dark.png"),
        include_bytes!("../../src/assets/favicon_dark.png")
    ).unwrap();

    fs::write(
        manual_out_dir.join("favicon_light.png"),
        include_bytes!("../../src/assets/favicon_light.png")
    ).unwrap();

    fs::copy(
        "assets/fira-mono-v14-latin_latin-ext-regular.woff2",
        manual_out_dir.join("fira-mono-v14-latin_latin-ext-regular.woff2")
    ).unwrap();

    fs::copy(
        "assets/titillium-web-v15-latin_latin-ext-regular.woff2",
        manual_out_dir.join("titillium-web-v15-latin_latin-ext-regular.woff2")
    ).unwrap();

    fs::copy(
        "assets/titillium-web-v15-latin_latin-ext-italic.woff2",
        manual_out_dir.join("titillium-web-v15-latin_latin-ext-italic.woff2")
    ).unwrap();

    fs::copy(
        "assets/styles.css",
        manual_out_dir.join("styles.css")
    ).unwrap();
}

fn layout(body: &str, docs: &Docs, active_page: &Page) -> String {
    let section_links = |pages: &[Page]| {
        pages
            .iter()
            .map(|page| {
                let active = if page == active_page { r#"class="active" "# } else { "" };
                let slug = &page.slug;
                let title = &page.title;

                format!(r#"<a {active}href="{slug}.html">{title}</a>"#)
            })
            .collect::<Vec<String>>()
            .join("\n")
    };

    let examples = section_links(&docs.examples);
    let reference = section_links(&docs.reference);
    let topics = section_links(&docs.topics);
    let index_active = if active_page == &docs.index { r#" class="active""# } else { "" };

    let title = &active_page.title;

    let favicon_svg_hash = url_safe_hash_base64(include_bytes!("../../src/assets/favicon.svg"));
    let favicon_light_png_hash = url_safe_hash_base64(include_bytes!("../../src/assets/favicon_light.png"));
    let favicon_dark_png_hash = url_safe_hash_base64(include_bytes!("../../src/assets/favicon_dark.png"));
    let styles_css_hash = url_safe_hash_base64(include_bytes!("../assets/styles.css"));

    let index_class_active = if active_page == &docs.index { r#"class="active""# } else { "" };

    let version = docs.version;

    formatdoc!(r##"
        <!doctype html>
        <html>
            <head>
                <title>{title}</title>
                <meta charset="utf-8">
                <meta name="description" content="{title}">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link href="favicon.svg?{favicon_svg_hash}" rel="icon" type="image/svg+xml">
                <link href="favicon_light.png?{favicon_light_png_hash}" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                <link href="favicon_dark.png?{favicon_dark_png_hash}" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                <link href="styles.css?{styles_css_hash}" rel="stylesheet">
            </head>
            <body>
                <header>
                    <a class="title" href="index.html">
                        <span{index_active}>Faircamp {version}</span>
                        <svg width="1em" height="1em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                            <title>Faircamp</title>
                            <path d="m39.881 40.829 1.2736-3.7326c0.16935-0.49631 0.51868-0.91106 0.97898-1.1623l7.6911-4.1982c0.41894-0.22868 0.44885-0.81906 0.05516-1.0889l-5.3773-3.6858c-0.40252-0.2759-0.58586-0.77665-0.45667-1.2472l0.83396-3.0375c0.10596-0.38594 0.5597-0.55363 0.89118-0.32937l10.551 7.1384c1.1285 0.76347 1.0472 2.4511-0.14947 3.1025l-15.839 8.6228c-0.2544 0.13848-0.54634-0.10768-0.4528-0.38181zm-21.56-0.71651-10.639-7.1649c-1.1304-0.76131-1.0532-2.4495 0.14204-3.1044l16.55-9.0683c0.24626-0.13493 0.52768 0.10884 0.42925 0.37183l-1.4747 3.9402c-0.18436 0.4926-0.53964 0.90254-1.001 1.155l-8.1479 4.459c-0.41817 0.22884-0.44672 0.81879-0.0526 1.0869l5.4751 3.7252c0.40291 0.27414 0.58616 0.77467 0.45549 1.2442l-0.84196 3.0251c-0.10758 0.38652-0.56213 0.55415-0.89491 0.33004zm25.654-30.594c-3.4629 0-10.138 2.0227-15.593 24.68-4.1 17.03-7.8557 18.965-9.919 19.007-1.1051 0.0226-1.7076-1.5696-1.1711-1.9798 0.37253-0.22641 1.1535-0.80303 1.1535-1.5516 0-0.85436-0.66165-1.3217-1.521-1.3017-0.88964 0.02072-1.7065 0.98255-1.7505 2.3158 0 1.7735 1.2416 3.7938 3.7009 3.7938 4.335 0 11.759-5.0498 15.399-20.158 2.0661-8.5762 5.2058-23.614 10.028-23.614 1.5974 0 1.5175 1.4328 1.0664 1.8125-0.45106 0.37973-0.95632 0.78658-0.99971 1.5338-0.04845 0.83454 0.55753 1.8182 1.4467 1.7975 1.2133-0.02825 2.1069-0.91548 2.1069-2.3008 0-2.2179-1.6252-4.034-3.9476-4.034z"/>
                        </svg>
                    </a>
                    <a class="open_nav" href="#nav">☰</a>
                </header>

                <div class="split">
                    <nav id="nav">
                        <div class="nav_inner">
                            <a class="close_nav" href="#">✕</a>

                            <a {index_class_active} href="index.html">Overview</a>

                            <span>Topics</span>
                            {topics}

                            <span>Examples</span>
                            {examples}

                            <span>Reference</span>
                            {reference}

                            <span>More Resources</span>
                            <a href="https://simonrepp.com/faircamp/" target="_blank">Website</a>
                            <a href="https://codeberg.org/simonrepp/faircamp/" target="_blank">Source Code</a>
                        </div>
                    </nav>
                    <main>
                        {body}
                    </main>
                </div>
            </body>
        </html>
    "##)
}

fn read_docs() -> Docs {
    let index_content = markdown_to_html(include_str!("../index.md"));

    let faircamp_version_display = env!("FAIRCAMP_VERSION_DISPLAY");

    let index = Page {
        content: index_content,
        slug: String::from("index"),
        title: format!("Faircamp {faircamp_version_display}")
    };

    let examples = read_pages(Path::new("examples"));
    let reference = read_pages(Path::new("reference"));
    let topics = read_pages(Path::new("topics"));

    Docs {
        examples,
        index,
        reference,
        topics,
        version: faircamp_version_display
    }
}

fn read_pages(dir: &Path) -> Vec<Page> {
    let mut pages: Vec<DirEntry> = fs::read_dir(dir)
        .unwrap()
        .flatten()
        .collect();

    pages.sort_by_key(|dir_entry| dir_entry.file_name());

    pages
        .into_iter()
        .map(|dir_entry| {
            let path = dir_entry.path();
            let file_stem = path.file_stem().unwrap().to_string_lossy();

            let title = match file_stem.split_once(' ') {
                Some((prefix, suffix)) => {
                    match prefix.parse::<usize>() {
                        Ok(_) => {
                            // We use a custom (U+1234) unicode literal
                            // encoding syntax so that we can include
                            // characters that would conflict with the
                            // filesystem.
                            let mut suffix_parts = suffix.split("(U+");

                            let mut title = suffix_parts.next().unwrap().to_string();

                            for part in suffix_parts {
                                let (code, remainder) = part.split_once(')').unwrap();
                                let number = u32::from_str_radix(code, 16).unwrap();

                                title.push(char::from_u32(number).unwrap());
                                title.push_str(remainder);
                            }

                            title
                        }
                        Err(_) => file_stem.to_string()
                    }
                }
                None => file_stem.to_string()
            };

            let content_markdown = fs::read_to_string(&path).unwrap();
            let content = markdown_to_html(&content_markdown);

            let slug = slugify(&title);

            Page { content, slug, title }
        })
        .collect()
}

fn render_page(
    manual_out_dir: &Path,
    docs: &Docs,
    page: &Page,
    next_page: Option<&Page>
) {
    let content = &page.content;

    let body = if let Some(next_page) = next_page {
        let next_page_slug = &next_page.slug;
        let next_page_title = &next_page.title;

        formatdoc!(r#"
            {content}

            <div class="next_page">
                Next page: <a href="{next_page_slug}.html">{next_page_title}</a>
            </div>
        "#)
    } else {
        content.clone()
    };

    let html = layout(&body, docs, page);

    let out_path = manual_out_dir.join(&page.slug).with_extension("html");

    fs::write(out_path, html).unwrap();
}

pub fn url_safe_hash_base64(hashable: &impl Hash) -> String {
    let mut hasher = DefaultHasher::new();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();
    URL_SAFE_NO_PAD.encode(hash.to_le_bytes())
}
