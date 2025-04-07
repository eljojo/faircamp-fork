// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::M3U_PLAYLIST_FILENAME;
use crate::{
    Build,
    Catalog,
    OpenGraphMeta
};
use crate::icons;
use crate::util::html_escape_outside_attribute;

use super::Layout;
use super::{artist_image, copy_button, releases};

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "";
    let translations = &build.locale.translations;
    
    let mut layout = Layout::new();

    let catalog_title = catalog.title();

    let title_escaped = html_escape_outside_attribute(&catalog_title);

    let home_image = match &catalog.home_image {
        Some(home_image) => artist_image(
            root_prefix,
            build,
            home_image,
            root_prefix
        ),
        None => String::new()
    };

    let mut actions = Vec::new();

    let r_more = match &catalog.more {
        Some(html_and_stripped) => {
            let more = &html_and_stripped.html;
            let more_icon = icons::more(&translations.more);
            let more_label = match &catalog.more_label {
                Some(label) => label,
                None => *translations.more
            };
            let more_link = format!(r##"
                <a class="more" href="#more">
                    {more_icon} {more_label}
                </a>
            "##);

            actions.push(more_link);

            format!(r#"
                <a class="scroll_target" id="more"></a>
                <div class="page">
                    <div class="page_center">
                        <div class="page_more">
                            <h1>{title_escaped}</h1>
                            <div class="text">{more}</div>
                        </div>
                    </div>
                </div>
            "#)
        }
        None => String::new()
    };


    if catalog.copy_link {
        layout.add_clipboard_script();

        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => ("content", base_url.index(build)),
            None => ("dynamic-url", String::new())
        };

        let t_copy_link = &translations.copy_link;
        let r_copy_link = copy_button(content_key, &content_value, t_copy_link);
        actions.push(r_copy_link);
    }

    if build.base_url.is_some() {
        if catalog.feeds.any_requested() {
            let t_subscribe = &translations.subscribe;
            let feed_icon = icons::feed(&translations.feed);
            let subscribe_slug = catalog.subscribe_permalink.as_ref().unwrap();

            let subscribe_link = format!(r#"
                <a href="{root_prefix}{subscribe_slug}{index_suffix}">
                    {feed_icon}
                    <span>{t_subscribe}</span>
                </a>
            "#);

            actions.push(subscribe_link);
        }

        if catalog.m3u  {
            let t_m3u_playlist = &translations.m3u_playlist;
            let stream_icon = icons::STREAM;

            let m3u_playlist_link = formatdoc!(r#"
                <a href="{M3U_PLAYLIST_FILENAME}">
                    {stream_icon}
                    <span>{t_m3u_playlist}</span>
                </a>
            "#);

            actions.push(m3u_playlist_link);
        }
    }

    for link in &catalog.links {
        let external_icon = icons::external(&translations.external_link);

        let rel_me = if link.rel_me { r#"rel="me""# } else { "" };
        let url = &link.url;

        let r_link = if link.hidden {
            format!(r#"<a href="{url}" {rel_me} style="display: none;">hidden</a>"#)
        } else {
            let label = link.pretty_label();
            let e_label = html_escape_outside_attribute(label);
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

    let public_releases = catalog.public_releases();
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

    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_split">
                {home_image}
                <div class="abstract">
                    <h1>{title_escaped}</h1>
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
    "#);

    if catalog.opengraph {
        if let Some(base_url) = &build.base_url {
            let catalog_url = base_url.index(build);
            let mut meta = OpenGraphMeta::new(catalog.title(), catalog_url);

            if let Some(synopsis) = &catalog.synopsis {
                meta.description(synopsis);
            }

            if let Some(described_image) = &catalog.home_image {
                let opengraph_image = described_image
                    .borrow()
                    .artist_opengraph_image(base_url.prefix());

                meta.image(opengraph_image);

                if let Some(description) = &described_image.description {
                    meta.image_alt(description);
                }
            }

            layout.add_opengraph_meta(meta);
        }
    }

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &catalog.theme,
        &catalog_title
    )
}
