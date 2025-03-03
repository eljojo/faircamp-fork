// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    CrawlerMeta,
    OpenGraphMeta,
    Scripts
};
use crate::icons;
use crate::render::{
    artist_image,
    copy_button,
    layout,
    releases,
    unlisted_badge
};
use crate::util::html_escape_outside_attribute;

pub fn artist_html(build: &Build, artist: &Artist, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";
    let translations = &build.locale.translations;

    let artist_name_escaped = html_escape_outside_attribute(&artist.name);

    let mut actions = Vec::new();

    let r_more = match &artist.more {
        Some(html_and_stripped) => {
            let more_icon = icons::more(&translations.more);
            let more_label = match &artist.more_label {
                Some(label) => label,
                None => *translations.more
            };
            let more_link = format!(r##"
                <a class="more" href="#more">
                    {more_icon} {more_label}
                </a>
            "##);

            actions.push(more_link);

            let artist_text = &html_and_stripped.html;
            formatdoc!(r#"
                <a class="scroll_target" id="more"></a>
                <div class="page">
                    <div class="page_center">
                        <div class="page_more">
                            <h1>{artist_name_escaped}</h1>
                            <div class="text">{artist_text}</div>
                        </div>
                    </div>
                </div>
            "#)
        }
        None => String::new()
    };

    let templates = if artist.copy_link {
        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => {
                let url = base_url.join(&format!("{}{index_suffix}", &artist.permalink.slug)).unwrap().to_string();
                ("content", url)
            }
            None => ("dynamic-url", String::new())
        };

        let copy_icon = icons::copy();
        let r_copy_link = copy_button(content_key, &content_value, &translations.copy_link);
        actions.push(r_copy_link);

        let failed_icon = icons::failure(&translations.failed);
        let success_icon = icons::success(&translations.copied);
        format!(r#"
            <template id="copy_icon">
                {copy_icon}
            </template>
            <template id="failed_icon">
                {failed_icon}
            </template>
            <template id="success_icon">
                {success_icon}
            </template>
        "#)
    } else {
        String::new()
    };

    for link in &artist.links {
        let external_icon = icons::external(&translations.external_link);

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

    let r_artist_image = match &artist.image {
        Some(artist_image_unpacked) => artist_image(
            build,
            index_suffix,
            root_prefix,
            &artist.permalink.slug,
            artist_image_unpacked
        ),
        None => String::new()
    };

    let name_unlisted = if artist.unlisted {
        format!("{artist_name_escaped} {}", unlisted_badge(build))
    } else {
        artist_name_escaped.clone()
    };

    let public_releases = artist.public_releases();

    let r_releases = releases(
        build,
        index_suffix,
        root_prefix,
        catalog,
        &public_releases
    );

    let synopsis = match &artist.synopsis {
        Some(synopsis) => {
            formatdoc!(r#"
                <div style="margin-bottom: 1rem; margin-top: 1rem;">
                    {synopsis}
                </div>
            "#)
        }
        None => String::new()
    };

    let body = formatdoc!(r##"
        <div class="page">
            <div class="page_split">
                {r_artist_image}
                <div class="abstract">
                    <h1>{name_unlisted}</h1>
                    {synopsis}
                    {r_actions}
                </div>
            </div>
        </div>
        <div class="page">
            <div class="page_grid">
                <div>
                    {r_releases}
                </div>
            </div>
        </div>
        {r_more}
        {templates}
    "##);

    let crawler_meta = if artist.unlisted { CrawlerMeta::NoIndexNoFollow } else { CrawlerMeta::None };

    let opengraph_meta = if catalog.opengraph {
        if let Some(base_url) = &build.base_url {
            let artist_slug = &artist.permalink.slug;
            let artist_url = base_url.join(&format!("{artist_slug}{index_suffix}")).unwrap();
            let mut meta = OpenGraphMeta::new(artist.name.clone(), artist_url);

            if let Some(synopsis) = &artist.synopsis {
                meta.description(synopsis);
            }

            if let Some(described_image) = &artist.image {
                let image = described_image.image.borrow();
                let image_url_prefix = base_url.join(&format!("{artist_slug}/")).unwrap();
                let opengraph_image = image.artist_assets.as_ref().unwrap().opengraph_image(&image_url_prefix);

                meta.image(opengraph_image);

                if let Some(description) = &described_image.description {
                    meta.image_alt(description);
                }
            }

            Some(meta)
        } else {
            None
        }
    } else {
        None
    };

    layout(
        root_prefix,
        &body,
        None,
        build,
        catalog,
        crawler_meta,
        Scripts::Clipboard,
        opengraph_meta,
        &artist.theme,
        &artist.name
    )
}
