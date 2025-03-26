// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    Release
};
use crate::util::html_escape_outside_attribute;

use super::Layout;
use super::compact_release_identifier;

/// Renders content for pages found under /[release_permalink]/[unlock_permalink]/[hash]/index.html
pub fn release_unlock_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    unlock_info: &Option<String>
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let mut layout = Layout::new();

    layout.no_indexing();

    let custom_or_default_unlock_info = unlock_info
        .as_ref()
        .map(|text| text.to_string())
        .unwrap_or(build.locale.translations.default_unlock_info.to_string());

    let t_unlock_permalink = &build.locale.translations.unlock_permalink;
    let page_hash = build.hash_with_salt(|hasher| {
        release.permalink.slug.hash(hasher);
        t_unlock_permalink.hash(hasher);
    });

    let t_download_code_seems_incorrect = &build.locale.translations.download_code_seems_incorrect;
    let t_downloads_permalink = &build.locale.translations.downloads_permalink;
    let t_enter_code_here = &build.locale.translations.enter_code_here;
    let t_unlock = &build.locale.translations.unlock;
    let t_unlock_manual_instructions = &build.locale.translations.unlock_manual_instructions(&page_hash, index_suffix);
    let content = formatdoc!(r#"
        <div class="unlock_scripted">
            {custom_or_default_unlock_info}

            <br><br>

            <form id="unlock">
                <input class="download_code" placeholder="{t_enter_code_here}" type="text">
                <button name="unlock">{t_unlock}</button>
            </form>
            <script>
                document.querySelector('#unlock').addEventListener('submit', event => {{
                    event.preventDefault();
                    const code = document.querySelector('.download_code').value;
                    const url = `../../{t_downloads_permalink}/${{code}}{index_suffix}`;
                    // TODO: Is this a problem in local-only viewing (file://...)? Test/follow up.
                    fetch(url, {{ method: 'HEAD', mode: 'no-cors' }})
                        .then(response => {{
                            if (response.ok) {{
                                window.location = url;
                            }} else {{
                                alert('{t_download_code_seems_incorrect}');
                            }}
                        }})
                        .catch(error => {{
                            alert('{t_download_code_seems_incorrect}');
                        }});
                }});
            </script>
        </div>
        <div class="unlock_manual">
            {t_unlock_manual_instructions}
        </div>
    "#);

    let release_link = format!("../..{index_suffix}");

    let compact_release_identifier_rendered = compact_release_identifier(
        build,
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let t_unlock_downloads = &build.locale.translations.unlock_downloads;
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center">
                <div style="max-width: 28rem;">
                    <h1>{t_unlock_downloads}</h1>
                    {compact_release_identifier_rendered}
                    {content}
                </div>
            </div>
        </div>
    "#);

    let release_title = &release.title;
    let release_title_escaped = html_escape_outside_attribute(release_title);

    layout.add_breadcrumb(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_unlock_downloads} – {release_title}");

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &release.theme,
        &page_title
    )
}
