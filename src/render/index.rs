// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Build, Catalog, CrawlerMeta, ReleaseRc, Scripts};
use crate::icons;
use crate::render::{
    artist_image,
    copy_button,
    layout,
    releases
};
use crate::util::{html_escape_outside_attribute};

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let catalog_text = match &catalog.text {
        Some(html_and_stripped) => format!(
            r#"<div class="text padded">{}</div>"#,
            &html_and_stripped.html
        ),
        None => String::new()
    };

    let title_escaped = html_escape_outside_attribute(&catalog_title);

    let home_image = match &catalog.home_image {
        Some(home_image) => artist_image(
            build,
            index_suffix,
            root_prefix,
            "__home__", // TODO: Bad hack, solve properly
            home_image
        ),
        None => String::new()
    };

    let mut actions = Vec::new();
    let mut templates = String::new();

    let more_icon = icons::more(&build.locale.translations.more);
    let more_label = match &catalog.more_label {
        Some(label) => label,
        None => &build.locale.translations.more
    };

    let more_link = format!(r##"
        <a class="more" href="#description">
            {more_icon} {more_label}
        </a>
    "##);

    actions.push(more_link);

    if catalog.copy_link {
        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => {
                let url = base_url.join(build.index_suffix_file_only()).unwrap().to_string();
                ("content", url)
            }
            None => ("dynamic-url", String::new())
        };

        let copy_icon = icons::copy(None);
        let t_copy_link = &build.locale.translations.copy_link;
        let r_copy_link = copy_button(content_key, &content_value, &copy_icon, t_copy_link);
        actions.push(r_copy_link);

        let failed_icon = icons::failure(&build.locale.translations.failed);
        let success_icon = icons::success(&build.locale.translations.copied);
        templates.push_str(&format!(r#"
            <template id="copy_icon">
                {copy_icon}
            </template>
            <template id="failed_icon">
                {failed_icon}
            </template>
            <template id="success_icon">
                {success_icon}
            </template>
        "#));
    };

    if build.base_url.is_some() && catalog.feed_enabled {
        let t_feed = &build.locale.translations.feed;
        let feed_icon = icons::feed(&build.locale.translations.rss_feed);

        let feed_link = format!(r#"
            <a href="{root_prefix}feed.rss">
                {feed_icon}
                <span>{t_feed}</span>
            </a>
        "#);

        actions.push(feed_link);
    };

    for link in &catalog.links {
        let external_icon = icons::external(&build.locale.translations.external_link);

        let rel_me = if link.rel_me { r#"rel="me""# } else { "" };
        let url = &link.url;

        let r_link = if link.hidden {
            format!(r#"<a href="{url}" {rel_me} style="display: none;">hidden</a>"#)
        } else {
            let label = link.pretty_label();
            let e_label = html_escape_outside_attribute(&label);
            formatdoc!(r#"
                <a href="{url}" {rel_me} target="_blank">{external_icon} <span>{e_label}</span></a>
            "#)
        };

        actions.push(r_link);
    }

    let r_actions = if actions.is_empty() {
        String::new()
    } else {
        let joined = actions.join("");

        formatdoc!(r#"
            <div class="actions">
                {joined}
            </div>
        "#)
    };

    let public_releases: Vec<ReleaseRc> = catalog.releases
        .iter()
        .filter_map(|release| {
            match release.borrow().unlisted {
                true => None,
                false => Some(release.clone())
            }
        })
        .collect();

    let r_releases = releases(
        build,
        index_suffix,
        root_prefix,
        catalog,
        &public_releases
    );

    let synopsis = match &catalog.synopsis {
        Some(synopsis) => {
            formatdoc!(r#"
                <div style="margin-bottom: 1rem; margin-top: 1rem;">
                    {synopsis}
                </div>
            "#)
        }
        None => String::new()
    };

    // TODO: Make configurable
    let faircamp_notice = if true {
        let faircamp_version = env!("CARGO_PKG_VERSION");
        let t_this_site_was_created_with_faircamp = build.locale.translations.this_site_was_created_with_faircamp(r#"<a href="https://simonrepp.com/faircamp/">Faircamp</a>"#);
        formatdoc!(r#"
            <footer class="faircamp_notice" data-version="{faircamp_version}">
                {t_this_site_was_created_with_faircamp}
            </footer>
        "#)
    } else {
        String::new()
    };

    let body = formatdoc!(r##"
        <div class="page">
            <div class="page_split page_60vh">
                {home_image}
                <div class="abstract">
                    <h1>{title_escaped}</h1>
                    {synopsis}
                    {r_actions}
                </div>
            </div>
        </div>
        <div class="additional page">
            <div class="page_grid page_50vh">
                <div>
                    {r_releases}
                </div>
            </div>
        </div>
        <a class="scroll_target" id="description"></a>
        <div class="page">
            <div class="page_center page_50vh">
                <div style="max-width: 32rem;">
                    <div style="font-size: 1.4rem;">{title_escaped}</div>
                    {catalog_text}
                </div>
            </div>
            {faircamp_notice}
        </div>
        {templates}
    "##);

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::Clipboard,
        &catalog.theme,
        &catalog_title,
        CrawlerMeta::None,
        None
    )
}
