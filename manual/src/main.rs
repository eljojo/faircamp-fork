// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use enolib::HtmlPrinter;
use indoc::formatdoc;
use pulldown_cmark::{CodeBlockKind, Event, html, Parser, Tag, TagEnd};
use slug::slugify;
use std::env;
use std::fs::{self, DirEntry};
use std::ops::Deref;
use std::path::{Path, PathBuf};

struct Docs {
    examples: Vec<Page>,
    index: Page,
    reference: Vec<Page>,
    topics: Vec<Page>
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

    formatdoc!(r##"
        <!doctype html>
        <html>
            <head>
                <title>{title}</title>
                <meta charset="utf-8">
                <meta name="description" content="{title}">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link href="favicon.svg" rel="icon" type="image/svg+xml">
                <link href="favicon_light.png" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                <link href="favicon_dark.png" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                <link href="styles.css?1" rel="stylesheet">
            </head>
            <body>
                <header>
                    <a class="title" href="index.html">
                        <span{index_active}>Faircamp Manual</span>
                        <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                            <title>Faircamp</title>
                            <path d="m46.739 32.391-9.0123 4.9051 0.58674-2.9505 5.1633-2.8163-4.1776-2.8633 0.58674-2.9505 7.2756 4.9286zm-22.625 4.9051-7.2756-4.9051 0.42245-1.7468 9.0123-4.9286-0.56327 2.9505-5.1868 2.8633 4.1776 2.8163zm14.632-19.062c-4.2114 0-7.2885 4.6842-9.799 15.112-2.5104 10.427-4.81 11.612-6.0734 11.638-0.67667 0.01381-1.0456-0.96107-0.71705-1.2122 0.2281-0.13864 0.67976-0.49247 0.70632-0.95004 0.02966-0.51099-0.40513-0.80927-0.93131-0.79703-0.54473 0.0127-0.99994 0.58986-1.0339 1.1848-0.0031 0.05482-0.0017 0.10857-0.01283 0.63607-0.01113 0.52749 0.611 1.92 1.9896 1.92 3.9236 0 7.7931-4.51 9.6802-12.343 1.2651-5.2512 3.1875-14.459 6.1404-14.459 0.97806 0 0.92916 0.8773 0.65297 1.1098-0.27618 0.23251-0.58556 0.48163-0.61212 0.93918-0.02967 0.51099 0.14424 1.1179 0.88584 1.1006 0.74292-0.01727 1.2641-0.56811 1.2918-1.3344 0.0023-0.05967-2e-3 -0.11806-0.01221-0.17492-0.02172-1.0411-0.63078-2.3695-2.1553-2.3695z"/>
                        </svg>
                    </a>
                    <a class="open_nav" href="#nav">☰</a>
                </header>

                <div class="split">
                    <nav id="nav">
                        <div class="nav_inner">
                            <a class="close_nav" href="#">✕</a>

                            <span>Topics</span>
                            {topics}

                            <span>Examples</span>
                            {examples}

                            <span>Reference</span>
                            {reference}
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

    let index = Page {
        content: index_content,
        slug: String::from("index"),
        title: String::from("Faircamp Manual")
    };

    let examples = read_pages(&Path::new("examples"));
    let reference = read_pages(&Path::new("reference"));
    let topics = read_pages(&Path::new("topics"));

    Docs {
        examples,
        index,
        reference,
        topics
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
                        Ok(_) => suffix.to_string(),
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
