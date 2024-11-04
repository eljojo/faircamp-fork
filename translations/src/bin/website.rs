// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Escape e.g. "me&you" so it can be rendered into an attribute,
/// e.g. as <img alt="me&quot;you" src="...">
pub fn html_escape_inside_attribute(string: &str) -> String {
    string.replace('&', "&amp;")
          .replace('<', "&lt;")
          .replace('>', "&gt;")
          .replace('"', "&quot;")
          .replace('\'', "&#39;")
}

/// Escape e.g. "love>hate" so it can be rendered into the page,
/// e.g. as <span>love&gt;hate</span>
pub fn html_escape_outside_attribute(string: &str) -> String {
    string.replace('&', "&amp;")
          .replace('<', "&lt;")
          .replace('>', "&gt;")
}

pub fn main() {
    let mut args = env::args();

    let website_out_dir = match args.nth(1) {
        Some(path) => PathBuf::from(&path),
        None => {
            eprintln!("A single argument is required (directory path to which to write the translation website), aborting.");
            return;
        }
    };

    if website_out_dir.exists() {
        let _ = fs::remove_dir_all(&website_out_dir);
    }

    fs::create_dir(&website_out_dir).unwrap();

    let mut body = String::new();

    // TODO: `translation` should be plural, but that collides with the module name,
    // maybe "strings"/"Strings" instead?
    for translation in translations::all_translations() {
        let code = translation.0;

        let mut strings = String::new();

        for string in translation.1.all_strings() {
            let key = string.0;
            let is_multiline = string.2;
            let status = string.1.status();

            let value = if status != "untranslated" { string.1 } else { "" };

            let r_input = if is_multiline {
                let value_escaped = html_escape_outside_attribute(value);
                format!(r#"<textarea readonly>{value_escaped}</textarea>"#)
            } else {
                let value_escaped = html_escape_inside_attribute(value);
                format!(r#"<input readonly value="{value_escaped}">"#)
            };


            let r_string = formatdoc!(r#"
                <div class="string">
                    <code class="{status}">{key}</code>
                    {r_input}
                </div>
            "#);

            strings.push_str(&r_string);
        }

        let percent_reviewed = translation.1.percent_reviewed();
        let percent_translated = translation.1.percent_translated();

        let section = formatdoc!(r#"
            <h2>{code} ({percent_translated}% translated, {percent_reviewed}% reviewed)</h2>

            <div class="strings">
                {strings}
            </div>
        "#);

        body.push_str(&section);
    }

    let html = layout(&body);

    fs::write(website_out_dir.join("index.html"), html).unwrap();

    fs::write(
        website_out_dir.join("favicon.svg"),
        include_bytes!("../../../src/assets/favicon.svg")
    ).unwrap();

    fs::write(
        website_out_dir.join("favicon_dark.png"),
        include_bytes!("../../../src/assets/favicon_dark.png")
    ).unwrap();

    fs::write(
        website_out_dir.join("favicon_light.png"),
        include_bytes!("../../../src/assets/favicon_light.png")
    ).unwrap();

    fs::copy(
        "assets/scripts.js",
        website_out_dir.join("scripts.js")
    ).unwrap();

    fs::copy(
        "assets/styles.css",
        website_out_dir.join("styles.css")
    ).unwrap();
}

fn layout(body: &str) -> String {
    formatdoc!(r##"
        <!doctype html>
        <html>
            <head>
                <title>Faircamp Translations</title>
                <meta charset="utf-8">
                <meta name="description" content="Easily accessible translation contributions for Faircamp">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link href="favicon.svg" rel="icon" type="image/svg+xml">
                <link href="favicon_light.png" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                <link href="favicon_dark.png" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                <link href="styles.css?0" rel="stylesheet">
            </head>
            <body>
                <header>
                    <span>Faircamp Translations</span>
                    <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                        <title>Faircamp</title>
                        <path d="m46.739 32.391-9.0123 4.9051 0.58674-2.9505 5.1633-2.8163-4.1776-2.8633 0.58674-2.9505 7.2756 4.9286zm-22.625 4.9051-7.2756-4.9051 0.42245-1.7468 9.0123-4.9286-0.56327 2.9505-5.1868 2.8633 4.1776 2.8163zm14.632-19.062c-4.2114 0-7.2885 4.6842-9.799 15.112-2.5104 10.427-4.81 11.612-6.0734 11.638-0.67667 0.01381-1.0456-0.96107-0.71705-1.2122 0.2281-0.13864 0.67976-0.49247 0.70632-0.95004 0.02966-0.51099-0.40513-0.80927-0.93131-0.79703-0.54473 0.0127-0.99994 0.58986-1.0339 1.1848-0.0031 0.05482-0.0017 0.10857-0.01283 0.63607-0.01113 0.52749 0.611 1.92 1.9896 1.92 3.9236 0 7.7931-4.51 9.6802-12.343 1.2651-5.2512 3.1875-14.459 6.1404-14.459 0.97806 0 0.92916 0.8773 0.65297 1.1098-0.27618 0.23251-0.58556 0.48163-0.61212 0.93918-0.02967 0.51099 0.14424 1.1179 0.88584 1.1006 0.74292-0.01727 1.2641-0.56811 1.2918-1.3344 0.0023-0.05967-2e-3 -0.11806-0.01221-0.17492-0.02172-1.0411-0.63078-2.3695-2.1553-2.3695z"/>
                    </svg>
                </header>
                <main>
                    <h3 style="color: red;">Heads up: The translation tool is work-in-progress, editing and submitting translations will be possible in a few days</h3>
                    {body}
                </main>
            </body>
        </html>
    "##)
}
