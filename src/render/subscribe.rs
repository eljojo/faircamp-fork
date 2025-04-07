// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use translations::Translations;

use crate::{
    Build,
    Catalog,
    Feeds,
    SiteUrl
};

use super::Layout;
use super::copy_button;

pub fn feed_choice(
    base_url: &SiteUrl,
    filename: &str,
    label: &str,
    translations: &Translations
) -> String {
    let url = base_url.join_file(filename);
    let copy_button = copy_button("content", &url, &translations.copy_link);

    formatdoc!(r#"
        <div>
            <br><br>
            <div class="embed_split">
                <h2>
                    {label}
                </h2>
                {copy_button}
            </div>
            <br>
            <a href="{url}">
                {url}
            </a>
        </div>
    "#)
}

/// The subscription choices page, providing direct links to the feed files,
/// info about the feed types, copy buttons, etc.
pub fn subscribe_html(build: &Build, catalog: &Catalog) -> String {
    let base_url = build.base_url_unchecked();
    let root_prefix = "../";
    let translations = &build.locale.translations;

    let mut layout = Layout::new();

    layout.add_clipboard_script();
    layout.no_indexing();

    let mut feed_choices = Vec::new();

    if catalog.feeds.atom {
        let r_feed_choice = feed_choice(
            base_url,
            Feeds::ATOM_FILENAME,
            "Atom",
            translations
        );

        feed_choices.push(r_feed_choice);
    }

    if catalog.feeds.generic_rss {
        let r_feed_choice = feed_choice(
            base_url,
            Feeds::GENERIC_RSS_FILENAME,
            &translations.generic_rss,
            translations
        );

        feed_choices.push(r_feed_choice);
    }

    if catalog.feeds.media_rss {
        let r_feed_choice = feed_choice(
            base_url,
            Feeds::MEDIA_RSS_FILENAME,
            "Media RSS",
            translations
        );

        feed_choices.push(r_feed_choice);
    }

    if catalog.feeds.podcast_rss {
        let r_feed_choice = feed_choice(
            base_url,
            Feeds::PODCAST_RSS_FILENAME,
            "Podcast RSS",
            translations
        );

        feed_choices.push(r_feed_choice);
    }

    let feed_choices = feed_choices.join("\n");

    // TODO: Offer feed choice hints to visitors: What are the respective formats,
    //       which one should you pick and why, how do you subscribe to them, etc.

    let t_subscribe = &build.locale.translations.subscribe;
    let body = formatdoc!(
        r##"
            <div class="page">
                <div class="page_center">
                    <div style="max-width: 28rem;">
                        <h1>{t_subscribe}</h1>

                        {feed_choices}
                    </div>
                </div>
            </div>
        "##
    );

    let catalog_title = catalog.title();

    let page_title = format!("{t_subscribe} – {catalog_title}");

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &catalog.theme,
        &page_title
    )
}
