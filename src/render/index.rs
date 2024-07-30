// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Build, Catalog, CrawlerMeta, ReleaseRc};
use crate::icons;
use crate::render::{
    artist_image,
    copy_button,
    cover_image_tiny,
    layout,
    releases
};
use crate::util::{format_time, html_escape_outside_attribute};

pub fn index_html(build: &Build, catalog: &Catalog) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let catalog_text = match &catalog.text {
        Some(html_and_stripped) => format!(
            r#"<div class="text">{}</div>"#,
            &html_and_stripped.html
        ),
        None => String::new()
    };

    // catalog.featured_artists is only populated in label mode, otherwise empty
    let featured_artists = catalog.featured_artists
        .iter()
        .filter(|artist| !artist.borrow().unlisted)
        .map(|artist| {
            let artist_ref = artist.borrow();
            let name = &artist_ref.name;
            let permalink = &artist_ref.permalink.slug;

            let releases = artist_ref.public_releases()
                .map(|release| {
                    let release_ref = release.borrow();
                    let release_prefix = format!("{}/", release_ref.permalink.slug);
                    cover_image_tiny(build, &release_prefix, &release_ref.cover, &release_prefix)
                })
                .collect::<Vec<String>>()
                .join("\n");

            formatdoc!(r#"
                <div class="artist">
                    <a href="{root_prefix}{permalink}{index_suffix}">{name}</a>
                    {releases}
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("");

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

    let mut templates = String::new();

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

    let public_releases_count = public_releases.len();
    let public_tracks_count: usize = public_releases.iter().map(|release| release.borrow().tracks.len()).sum();
    let total_listening_duration = public_releases
        .iter()
        .map(|release|
            release.borrow().tracks
                .iter()
                .map(|track| track.transcodes.borrow().source_meta.duration_seconds)
                .sum::<f32>()
        ).sum();
    let total_listening_duration_formatted = format_time(total_listening_duration);

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

    let grid_icon = icons::grid();
    let list_icon = icons::list();
    let scroll_icon = icons::scroll();
    let t_more = &build.locale.translations.more;
    let t_more_info = &build.locale.translations.more_info;
    let t_top = &build.locale.translations.top;
    let t_releases = &build.locale.translations.releases;
    let t_tracks = &build.locale.translations.tracks;
    let body = formatdoc!(r##"
        <div class="page" data-overview>
            <div class="page_split">
                {home_image}
                <div style="max-width: 26rem;">
                    <h1>{title_escaped}</h1>
                    <div class="actions primary">
                        <a class="emphasized" href="#releases">
                            {grid_icon}
                            {t_releases}
                        </a>
                        <!--a class="emphasized" href="#artists">
                            {list_icon}
                            Artists
                        </a-->
                    </div>
                    {synopsis}
                    <a class="scroll_link" href="#description">
                        {scroll_icon}
                        {t_more_info}
                    </a>
                </div>
            </div>
        </div>
        <a class="scroll_target" id="releases"></a>
        <div class="additional page">
            <div class="page_grid">
                <div>
                    {r_releases}
                </div>
            </div>
        </div>
        <!--a class="scroll_target" id="artists"></a>
        <div class="page">
            <div class="page_center">
                <div>
                    {featured_artists}
                </div>
            </div>
        </div-->
        <a class="scroll_target" id="description"></a>
        <div class="additional page" data-description>
            <div class="page_center">
                <div style="max-width: 32rem;">
                    <div>{title_escaped}</div>
                    <div>{public_releases_count} {t_releases}</div>
                    <div>{public_tracks_count} {t_tracks}</div>
                    <div>{total_listening_duration_formatted}</div>
                    {r_actions}
                    {catalog_text}
                </div>
            </div>
            {faircamp_notice}
        </div>
        <div class="scroll_hints">
            <a class="up" href="#">
                {scroll_icon} {t_top}
            </a>
            <a class="down" href="#description">
                <span>{scroll_icon}</span> {t_more}
            </a>
        </div>
        {templates}
    "##);

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &catalog.theme,
        &catalog_title,
        CrawlerMeta::None
    )
}
