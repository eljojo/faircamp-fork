// SPDX-FileCopyrightText: 2022-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    OpenGraphMeta
};
use crate::icons;
use crate::util::html_escape_outside_attribute;

use super::Layout;
use super::{
    artist_image,
    copy_button,
    releases,
    unlisted_badge
};

pub fn artist_html(artist: &Artist, build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";
    let translations = &build.locale.translations;

    let mut layout = Layout::new();

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

    if artist.copy_link {
        layout.add_clipboard_script();

        let (content_key, content_value) = match &build.base_url {
            Some(base_url) => ("content", base_url.join_index(build, &artist.permalink.slug)),
            None => ("dynamic-url", String::new())
        };

        let r_copy_link = copy_button(content_key, &content_value, &translations.copy_link);
        actions.push(r_copy_link);
    }

    for link in &artist.links {
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

    let r_artist_image = match &artist.image {
        Some(artist_image_unpacked) => artist_image(
            "",
            build,
            artist_image_unpacked,
            root_prefix
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
    "##);

    if artist.unlisted {
        layout.no_indexing();
    }

    if catalog.opengraph {
        if let Some(base_url) = &build.base_url {
            let artist_slug = &artist.permalink.slug;
            let artist_url = base_url.join_index(build, artist_slug);

            let mut meta = OpenGraphMeta::new(artist.name.clone(), artist_url);

            if let Some(synopsis) = &artist.synopsis {
                meta.description(synopsis);
            }

            if let Some(described_image) = &artist.image {
                let artist_prefix = base_url.join_prefix(artist_slug);
                let opengraph_image = described_image
                    .borrow()
                    .artist_opengraph_image(&artist_prefix);

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
        &artist.theme,
        &artist.name
    )
}
