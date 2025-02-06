// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::iter::zip;
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use indoc::formatdoc;

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

fn layout(body: &str) -> String {
    let favicon_svg_hash = url_safe_hash_base64(include_bytes!("../../../src/assets/favicon.svg"));
    let favicon_light_png_hash = url_safe_hash_base64(include_bytes!("../../../src/assets/favicon_light.png"));
    let favicon_dark_png_hash = url_safe_hash_base64(include_bytes!("../../../src/assets/favicon_dark.png"));
    let scripts_js_hash = url_safe_hash_base64(include_bytes!("../../assets/scripts.js"));
    let styles_css_hash = url_safe_hash_base64(include_bytes!("../../assets/styles.css"));

    formatdoc!(r##"
        <!doctype html>
        <html>
            <head>
                <title>Faircamp Translations</title>
                <meta charset="utf-8">
                <meta name="description" content="Easily accessible translation contributions for Faircamp">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link href="favicon.svg?{favicon_svg_hash}" rel="icon" type="image/svg+xml">
                <link href="favicon_light.png?{favicon_light_png_hash}" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                <link href="favicon_dark.png?{favicon_dark_png_hash}" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                <script defer src="scripts.js?{scripts_js_hash}"></script>
                <link href="styles.css?{styles_css_hash}" rel="stylesheet">
            </head>
            <body>
                <header>
                    <div>
                        <span class="count">0 changes</span>
                        <button disabled id="clear">Clear</button>
                        <a href="#review">Review</a>
                    </div>
                    <div class="message">
                        <div class="activity"></div>
                        <span class="text"></span>
                    </div>
                </header>
                <main>
                    <section>
                        <h1>
                            <svg aria-hidden="true" width="1em" height="1em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                                <path d="m46.739 32.391-9.0123 4.9051 0.58674-2.9505 5.1633-2.8163-4.1776-2.8633 0.58674-2.9505 7.2756 4.9286zm-22.625 4.9051-7.2756-4.9051 0.42245-1.7468 9.0123-4.9286-0.56327 2.9505-5.1868 2.8633 4.1776 2.8163zm14.632-19.062c-4.2114 0-7.2885 4.6842-9.799 15.112-2.5104 10.427-4.81 11.612-6.0734 11.638-0.67667 0.01381-1.0456-0.96107-0.71705-1.2122 0.2281-0.13864 0.67976-0.49247 0.70632-0.95004 0.02966-0.51099-0.40513-0.80927-0.93131-0.79703-0.54473 0.0127-0.99994 0.58986-1.0339 1.1848-0.0031 0.05482-0.0017 0.10857-0.01283 0.63607-0.01113 0.52749 0.611 1.92 1.9896 1.92 3.9236 0 7.7931-4.51 9.6802-12.343 1.2651-5.2512 3.1875-14.459 6.1404-14.459 0.97806 0 0.92916 0.8773 0.65297 1.1098-0.27618 0.23251-0.58556 0.48163-0.61212 0.93918-0.02967 0.51099 0.14424 1.1179 0.88584 1.1006 0.74292-0.01727 1.2641-0.56811 1.2918-1.3344 0.0023-0.05967-2e-3 -0.11806-0.01221-0.17492-0.02172-1.0411-0.63078-2.3695-2.1553-2.3695z"/>
                            </svg>
                            <span>Help to translate faircamp</span>
                        </h1>
                        <p>
                            Open a language section below to begin translating.
                            If your language is missing, start in the last section ("New Language").
                            When you're done proceed to <a href="#review">review and submit</a>
                            your changes. See <a href="#help">help</a> for additional
                            instructions.
                        </p>
                        <p>
                            <span>Display filter:</span>
                            <input autocomplete="off" id="hide-reviewed" type="checkbox">
                            <label for="hide-reviewed">Hide complete languages and translations</label>
                        </p>
                    </section>
                    <section class="unbounded">
                        <h2>Languages</h2>
                        {body}
                    </section>
                    <section>
                        <a id="review"></a>
                        <h2>Review and submit</h2>
                        <div class="submission">
                            <pre>First make some changes, then they will appear here.</pre>
                        </div>
                        <p>
                            Please quickly review if your content looks about right in the box above,
                            then copy all of it, and send it in your preferred way:
                        </p>
                        <ul>
                            <li><strong>Via Email</strong>: <a href="mailto:simon@fdpl.io">simon@fdpl.io</a></li>
                            <li><strong>Via Fediverse</strong>: <a href="https://post.lurk.org/@freebliss">@freebliss@post.lurk.org</a></li>
                            <li><strong>Via Codeberg</strong>: As an <a href="https://codeberg.org/simonrepp/faircamp/issues/new">issue</a> (or a pull request)</li>
                        </ul>
                        <p>
                            If this is your first contribution to faircamp, possibly include your
                            name and, if you want, a url to your website/online presence - I will
                            include you in faircamp's credits. You can also choose to contribute
                            anonymously, I fully respect that too.
                        </p>
                    </section>
                    <section>
                        <a id="help"></a>
                        <h2>Help</h2>
                        <ul>
                            <li>
                                <strong>Privacy</strong>: All data is stored in your browser (locally). No data leaves your computer until you manually submit it.
                            </li>
                            <li>
                                <strong>Safety</strong>: Your changes are continually saved. You can safely close this page/tab at any point. When you come back to this page (on the same computer and browser) all your changes will be restored.
                            </li>
                            <li>
                                <strong>Scope</strong>: You can work on changes for multiple languages (including changes for a new language) at the same time.
                            </li>
                            <li>
                                <strong>Translate (red)</strong>: Read the <span class="label">Reference</span> text and write your translation into the <span class="label">Proposed</span> field.
                            </li>
                            <li>
                                <strong>Review (yellow)</strong>: Read the <span class="label">Current</span> text and check whether it correctly translates the <span class="label">Reference</span> text.
                                When correct, simply copy/paste it to the <span class="label">Proposed</span> field, otherwise put a corrected version in the <span class="label">Proposed</span> field. 
                            </li>
                            <li>
                                <strong>Re-review (green)</strong>: You may double-check strings that have already been reviewed, but only propose changes when they are really necessary.
                            </li>
                        </ul>
                    </section>
                </main>
            </body>
        </html>
    "##)
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

    let mut all_languages = translations::all_languages();

    all_languages.sort_by_key(|language| language.name);

    all_languages.push(translations::new_language());

    for language in all_languages {
        let language_code = language.code;

        let mut strings = String::new();

        for (reference_string, string) in zip(translations::EN.all_strings(), language.translations.all_strings()) {
            let translation_key = string.0;
            let is_multiline = string.2;
            let status = string.1.status();

            let reference_value = reference_string.1;
            let r_reference_en = if language_code == "en" {
                String::new()
            } else {
                let value_escaped = html_escape_outside_attribute(reference_value);
                formatdoc!(r#"
                    <div>
                        <span>Reference</span>
                        <span>en</span>
                        <span>{value_escaped}</span>
                    </div>
                "#)
            };

            let r_current_xx = if status == "untranslated" {
                String::new()
            } else {
                let value_escaped = html_escape_outside_attribute(string.1);
                formatdoc!(r#"
                    <div>
                        <span>Current</span>
                        <span>{language_code}</span>
                        <span>{value_escaped}</span>
                    </div>
                "#)
            };

            let r_proposal_xx = if is_multiline {
                format!(r#"<textarea autocomplete="off" data-translation-key="{translation_key}"></textarea>"#)
            } else {
                format!(r#"<input autocomplete="off" data-translation-key="{translation_key}">"#)
            };

            let data_reviewed = if status == "reviewed" { "data-reviewed" } else { "" };

            let r_string = formatdoc!(r#"
                <div class="string" {data_reviewed}>
                    <div>
                        <span></span>
                        <div class="status {status}"></div>
                        <code>{translation_key}</code>
                    </div>
                    {r_reference_en}
                    {r_current_xx}
                    <div>
                        <span>Proposed</span>
                        <span>{language_code}</span>
                        {r_proposal_xx}
                    </div>
                </div>
            "#);

            strings.push_str(&r_string);
        }

        let percent_reviewed = language.translations.percent_reviewed();
        let percent_translated = language.translations.percent_translated();

        let percent_unreviewed = percent_translated - percent_reviewed;
        let percent_untranslated = 100.0 - percent_translated;

        let count_unreviewed = language.translations.count_unreviewed();
        let r_to_review = if count_unreviewed > 0 {
            format!(r#"<span class="badge unreviewed">{count_unreviewed} to review</span>"#)
        } else {
            String::new()
        };

        let count_untranslated = language.translations.count_untranslated();
        let r_to_translate = if count_untranslated > 0 {
            format!(r#"<span class="badge untranslated">{count_untranslated} to translate</span>"#)
        } else {
            String::new()
        };

        let data_reviewed = if percent_reviewed < 100.0 { "" } else { "data-reviewed" };

        let language_name = language.name;
        let section = formatdoc!(r#"
            <details {data_reviewed}>
                <summary class="language">
                    <div class="progress">
                        <div class="bar reviewed" style="width: {percent_reviewed}%;"></div>
                        <div class="bar unreviewed" style="width: {percent_unreviewed}%;"></div>
                        <div class="bar untranslated" style="width: {percent_untranslated}%;"></div>
                    </div>
                    <span>{language_name} ({language_code})</span>
                    {r_to_translate}
                    {r_to_review}
                    <span class="count"></span>
                </summary>
                <div class="strings" data-language-code="{language_code}">
                    {strings}
                </div>
            </details>
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

pub fn url_safe_hash_base64(hashable: &impl Hash) -> String {
    let mut hasher = DefaultHasher::new();
    hashable.hash(&mut hasher);
    let hash = hasher.finish();
    URL_SAFE_NO_PAD.encode(hash.to_le_bytes())
}
