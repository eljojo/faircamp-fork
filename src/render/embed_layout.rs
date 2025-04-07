// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    AssetHashes,
    Build,
    GENERATOR_INFO,
    SiteUrl,
    Theme
};
use crate::icons;
use crate::util::html_escape_outside_attribute;

use super::player_icon_templates;

pub struct EmbedLayout;

impl EmbedLayout {
    pub fn render(
        body: &str,
        build: &Build,
        link_url: &str,
        root_prefix: &str,
        theme: &Theme,
        title: &str
    ) -> String {
        let translations = &build.locale.translations;

        let dir_attribute = if build.locale.text_direction.is_rtl() { r#"dir="rtl""# } else { "" };

        let display_link_url = SiteUrl::pretty_display(link_url);
        let embeds_css_hash = AssetHashes::EMBEDS_CSS;
        let embeds_js_hash = build.asset_hashes.embeds_js.as_ref().unwrap();

        let external_icon = icons::external(&translations.external_link);

        let lang = &build.locale.language;

        let templates = player_icon_templates(translations);

        let theme_css_hash = build.asset_hashes.theme_css
            .get(&theme.stylesheet_filename())
            .unwrap();
        let theme_stylesheet_filename = theme.stylesheet_filename();

        let title_escaped = html_escape_outside_attribute(title);

        let t_javascript_is_disabled_listen_at_xxx = translations
            .javascript_is_disabled_listen_at_xxx(
                &format!(r#"<a href="{link_url}">{external_icon} {display_link_url}</a>"#)
            );
        formatdoc!(r#"
            <!DOCTYPE html>
            <html {dir_attribute} lang="{lang}">
                <head>
                    <title>{title_escaped}</title>
                    <meta charset="utf-8">
                    <meta name="generator" content="{GENERATOR_INFO}">
                    <meta name="robots" content="noindex, nofollow">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <link href="{root_prefix}{theme_stylesheet_filename}?{theme_css_hash}" rel="stylesheet">
                    <link href="{root_prefix}embeds.css?{embeds_css_hash}" rel="stylesheet">
                    <script defer src="{root_prefix}embeds.js?{embeds_js_hash}"></script>
                </head>
                <body>
                    <script>document.body.classList.add('js_enabled');</script>
                    <main>
                        {body}
                    </main>
                    <aside class="js_notice">
                        <div>
                            {t_javascript_is_disabled_listen_at_xxx}
                        </div>
                    </aside>
                    {templates}
                </body>
            </html>
        "#)
    }
}
