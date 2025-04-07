// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    AssetHashes,
    Build,
    Catalog,
    GENERATOR_INFO,
    OpenGraphMeta,
    Theme
};
use crate::icons;
use crate::util::{
    html_escape_inside_attribute,
    html_escape_outside_attribute
};

use super::player_icon_templates;

pub struct Layout {
    breadcrumb: Option<String>,
    /// If true we inject a script tag for clipboard.js into the head of the
    /// page and append templates for icons (copy/failed/success) used at
    /// runtime to the end of the page.
    clipboard_script: bool,
    /// If true we inject noindex/nofollow meta into the head of the page
    no_indexing: bool,
    opengraph_meta: Option<OpenGraphMeta>,
    /// If true we inject a script tag for player.js into the head of the page
    /// and append templates for icons (loading/pause/play) used at runtime
    /// to the end of the page.
    player_script: bool
}

fn theming_widget(build: &Build, catalog: &Catalog) -> String {
    let accent_brightening = &catalog.theme.accent_brightening;
    let accent_chroma = match &catalog.theme.accent_chroma {
        Some(chroma) => chroma.to_string(),
        None => String::from("null")
    };
    let accent_hue = match catalog.theme.accent_hue {
        Some(hue) => hue.to_string(),
        None => String::from("null")
    };
    let background_alpha = &catalog.theme.background_alpha;
    let base = catalog.theme.base.to_key();
    let base_chroma = &catalog.theme.base_chroma;
    let base_hue = catalog.theme.base_hue;
    let build_begin = build.build_begin;
    let dynamic_range = catalog.theme.dynamic_range;

    let mut script = formatdoc!(r#"
        const BUILD_OPTIONS = {{
            'accent_brightening': {accent_brightening},
            'accent_chroma': {accent_chroma},
            'accent_hue': {accent_hue},
            'background_alpha': {background_alpha},
            'base': '{base}',
            'base_chroma': {base_chroma},
            'base_hue': {base_hue},
            'build_time': '{build_begin}',
            'dynamic_range': {dynamic_range}
        }};
    "#);

    let dark_js = crate::theme::DARK.print_js("DARK_THEME");
    let light_js = crate::theme::LIGHT.print_js("LIGHT_THEME");

    script.push_str(&dark_js);
    script.push_str(&light_js);
    script.push_str(include_str!("../assets/theming_widget.js"));

    formatdoc!(r#"
        <div class="theming_widget advanced">
            <div class="levels">
                <div class="beam"></div>
                <div class="level" data-name="background_1">
                    <div>bg-1</div>
                </div>
                <div class="level" data-name="background_2">
                    <div>bg-2</div>
                </div>
                <div class="level" data-name="background_3">
                    <div>bg-3</div>
                </div>
                <div class="level" data-name="background_middleground">
                    <div>bg-mg</div>
                </div>
                <div class="level" data-name="foreground_1">
                    <div>fg-1</div>
                </div>
                <div class="level" data-name="foreground_2">
                    <div>fg-2</div>
                </div>
                <div class="level" data-name="foreground_3">
                    <div>fg-3</div>
                </div>
                <div class="level" data-name="foreground_middleground">
                    <div>fg-mg</div>
                </div>
                <div class="level" data-name="middleground">
                    <div>mg</div>
                </div>
            </div>
            <div class="palette">
                <div class="tone">
                    <span>bg-acc</span>
                    <div style="background: var(--bg-acc);"></div>
                    <div class="monochrome" data-name="background_accent"></div>
                </div>
                <div class="tone">
                    <span>mg-acc</span>
                    <div style="background: var(--mg-acc);"></div>
                    <div class="monochrome" data-name="middleground_accent"></div>
                </div>
                <div class="tone">
                    <span>bg-1</span>
                    <div style="background: var(--bg-1);"></div>
                    <div class="monochrome" data-name="background_1"></div>
                </div>
                <div class="tone">
                    <span>bg-2</span>
                    <div style="background: var(--bg-2);"></div>
                    <div class="monochrome" data-name="background_2"></div>
                </div>
                <div class="tone">
                    <span>bg-3</span>
                    <div style="background: var(--bg-3);"></div>
                    <div class="monochrome" data-name="background_3"></div>
                </div>
                <div class="tone">
                    <span>bg-mg</span>
                    <div style="background: var(--bg-mg);"></div>
                    <div class="monochrome" data-name="background_middleground"></div>
                </div>
                <div class="tone">
                    <span>mg</span>
                    <div style="background: var(--mg);"></div>
                    <div class="monochrome" data-name="middleground"></div>
                </div>
                <div class="tone">
                    <span>fg-mg</span>
                    <div style="background: var(--fg-mg);"></div>
                    <div class="monochrome" data-name="foreground_middleground"></div>
                </div>
                <div class="tone">
                    <span>fg-3</span>
                    <div style="background: var(--fg-3);"></div>
                    <div class="monochrome" data-name="foreground_3"></div>
                </div>
                <div class="tone">
                    <span>fg-2</span>
                    <div style="background: var(--fg-2);"></div>
                    <div class="monochrome" data-name="foreground_2"></div>
                </div>
                <div class="tone">
                    <span>fg-1</span>
                    <div style="background: var(--fg-1);"></div>
                    <div class="monochrome" data-name="foreground_1"></div>
                </div>
            </div>
            <div class="controls">
                <textarea class="manifest" readonly></textarea>
            </div>
        </div>
        <script>{script}</script>
    "#)
}

impl Layout {
    pub fn add_breadcrumb(&mut self, breadcrumb: String) {
        self.breadcrumb = Some(breadcrumb);
    }

    pub fn add_clipboard_script(&mut self) {
        self.clipboard_script = true;
    }

    pub fn add_opengraph_meta(&mut self, opengraph_meta: OpenGraphMeta) {
        self.opengraph_meta = Some(opengraph_meta);
    }

    pub fn add_player_script(&mut self) {
        self.player_script = true;
    }

    pub fn new() -> Layout {
        Layout {
            breadcrumb: None,
            clipboard_script: false,
            no_indexing: false,
            opengraph_meta: None,
            player_script: false
        }
    }

    /// For pages that should not be indexed by crawlers (search engines
    /// etc.), call this method on the layout, this adds a noindex and
    /// nofollow meta tag for crawlers.
    pub fn no_indexing(&mut self) {
        self.no_indexing = true;
    }

    pub fn render(
        &self,
        body: &str,
        build: &Build,
        catalog: &Catalog,
        root_prefix: &str,
        theme: &Theme,
        title: &str
    ) -> String {
        let index_suffix = build.index_suffix();
        let mut templates = String::new();
        let translations = &build.locale.translations;

        let mut extra_meta = String::new();

        let mut add_extra_meta = |tags: &str| {
            extra_meta.push_str(tags);
            extra_meta.push('\n');
        };

        if build.base_url.is_some() && catalog.feeds.any_requested() {
            let feed_tags = catalog.feeds.meta_link_tags(root_prefix, translations);
            add_extra_meta(&feed_tags);
        }

        let dir_attribute = if build.locale.text_direction.is_rtl() { r#"dir="rtl""# } else { "" };

        let faircamp_signature = if catalog.faircamp_signature {
            let faircamp_icon = icons::faircamp(None);
            let faircamp_version_display = env!("FAIRCAMP_VERSION_DISPLAY");

            formatdoc!(r#"
                <a class="faircamp_signature" href="https://simonrepp.com/faircamp/" target="_blank">
                    {faircamp_icon}
                    <span>Faircamp {faircamp_version_display}</span>
                </a>
            "#)
        } else {
            String::new()
        };

        let r_theming_widget = if build.theming_widget {
            theming_widget(build, catalog)
        } else {
            String::new()
        };

        let breadcrumb = match &self.breadcrumb {
            Some(link) => format!(" <span>›</span> {link}"),
            None => String::from("")
        };

        if self.clipboard_script {
            let clipboard_js_hash = AssetHashes::CLIPBOARD_JS;
            let clipboard_script_tag = format!(r#"<script defer src="{root_prefix}clipboard.js?{clipboard_js_hash}"></script>"#);

            add_extra_meta(&clipboard_script_tag);

            let copy_icon = icons::COPY;
            let failed_icon = icons::failure(&translations.failed);
            let success_icon = icons::success(&translations.copied);

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
        }

        if self.player_script {
            let player_js_hash = build.asset_hashes.player_js.as_ref().unwrap();
            let player_script_tag = format!(r#"<script defer src="{root_prefix}player.js?{player_js_hash}"></script>"#);

            add_extra_meta(&player_script_tag);

            templates.push_str(&player_icon_templates(translations));
        }

        let browse_icon = icons::BROWSE;
        let browser_js_hash = build.asset_hashes.browser_js.as_ref().unwrap();
        let catalog_title = html_escape_outside_attribute(&catalog.title());


        if self.no_indexing {
            add_extra_meta(r#"<meta name="robots" content="noindex, nofollow">"#);
        }

        if let Some(favicon_tags) = catalog.favicon.header_tags(build, root_prefix) {
            add_extra_meta(&favicon_tags);
        }

        let faircamp_icon = icons::faircamp(Some("Faircamp"));
        let lang = &build.locale.language;

        if let Some(meta) = &self.opengraph_meta {
            let opengraph_tags = meta.tags(build, catalog);
            add_extra_meta(&opengraph_tags);
        }

        let site_css_hash = build.asset_hashes.site_css.as_ref().unwrap();
        let theme_css_hash = build.asset_hashes.theme_css.get(&theme.stylesheet_filename()).unwrap();
        let theme_stylesheet_filename = theme.stylesheet_filename();

        let title_escaped_outside_attribute = html_escape_outside_attribute(title);

        // TODO: We are currently using the title for the meta description content (too),
        //       which is not ideal - this could rather be something specific to the
        //       page, e.g. the synopsis, and maybe we should leave it out if we don't
        //       have anything to add to the title anyway? (that's in <title> already)
        let title_escaped_inside_attribute = html_escape_inside_attribute(title);

        let close_icon = icons::failure(&translations.close);

        let t_browse = &translations.browse;
        let t_javascript_is_disabled_text = &translations.javascript_is_disabled_text;
        let t_search = &translations.search;
        let t_skip_to_main_content = &translations.skip_to_main_content;

        // User-supplied site metadata is appended last in order to guarantee
        // its precendence when overriding (e.g.) native styles.
        if let Some(site_metadata) = &catalog.site_metadata {
            add_extra_meta(&site_metadata.render(root_prefix));
        }

        formatdoc!(r##"
            <!DOCTYPE html>
            <html {dir_attribute} lang="{lang}">
                <head>
                    <title>{title_escaped_outside_attribute}</title>
                    <meta charset="utf-8">
                    <meta name="description" content="{title_escaped_inside_attribute}">
                    <meta name="generator" content="{GENERATOR_INFO}">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <link href="{root_prefix}{theme_stylesheet_filename}?{theme_css_hash}" rel="stylesheet">
                    <link href="{root_prefix}site.css?{site_css_hash}" rel="stylesheet">
                    <script defer src="{root_prefix}browser.js?{browser_js_hash}"></script>
                    {extra_meta}
                </head>
                <body>
                    <script>document.body.classList.add('js_enabled');</script>
                    <a class="skip_to_content" href="#content">{t_skip_to_main_content}</a>
                    <div class="layout">
                        <header>
                            <div>
                                <a id="logo" href="{root_prefix}.{index_suffix}">
                                    {faircamp_icon}
                                    <span>{catalog_title}</span>
                                </a>
                                {breadcrumb}
                            </div>
                            <button class="browse">
                                {t_browse}
                                {browse_icon}
                            </button>
                        </header>
                        <main id="content">
                            {body}
                        </main>
                        <footer>
                            <span>
                                <a href="{root_prefix}">{catalog_title}</a>
                                <button class="browse">{browse_icon} {t_browse}</button>
                            </span>
                            {faircamp_signature}
                        </footer>
                    </div>
                    <div id="browser" data-root-prefix="{root_prefix}">
                        <div>
                            <input autocomplete="off" placeholder="{t_search}" type="search">
                            <div role="status"></div>
                            <div id="results"></div>
                        </div>
                        <button>
                            {close_icon}
                        </button>
                    </div>
                    {r_theming_widget}
                    <aside class="js_notice">{t_javascript_is_disabled_text}</aside>
                    {templates}
                </body>
            </html>
        "##)
    }
}
