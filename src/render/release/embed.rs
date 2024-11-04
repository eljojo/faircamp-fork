// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;
use url::Url;

use crate::{Build, Catalog, CrawlerMeta, Release, Scripts};
use crate::icons;
use crate::render::{compact_release_identifier, copy_button, layout};
use crate::util::{
    html_double_escape_inside_attribute,
    html_escape_inside_attribute,
    html_escape_outside_attribute
};

/// Returns a two-field tuple where the fields have the following use:
/// .0 => Intended to be directly copied via copy-to-clipboard
/// .1 => Intended to be rendered to the page, so people can copy it themselves.
/// The title parameter provides a text that indicates to screen-reader users
/// what to expect inside the iframe. See description at
/// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#accessibility_concerns
fn embed_code<T: std::fmt::Display>(
    base_url: &Url,
    index_suffix: &str,
    release_slug: &str,
    postfix: T,
    title: &str
) -> (String, String) {
    let title_double_escaped = html_double_escape_inside_attribute(title);
    let title_escaped = html_escape_inside_attribute(title);
    let url = base_url
        .join(&format!("{release_slug}/embed/{postfix}{index_suffix}"))
        .unwrap();
    
    let copy_code = html_escape_inside_attribute(
        &formatdoc!(r#"
            <iframe loading="lazy" src="{url}" style="min-width: 480px;" title="{title_escaped}"></iframe>
        "#)
    );

    let display_code = formatdoc!(r#"
        <div class="embed_code_wrapper">
            <pre class="embed_code"><span class="embed_syntax_special">&lt;</span>iframe
            loading<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"lazy"</span>
            src<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"{url}"</span>
            style<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"min-width: 480px;"</span>
            title<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"{title_double_escaped}"</span><span class="embed_syntax_special">&gt;</span>
        <span class="embed_syntax_special">&lt;/</span>iframe<span class="embed_syntax_special">&gt;</span></pre>
        </div>
    "#);

    (copy_code, display_code)
}

pub fn embed_choices_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    base_url: &Url
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../";
    let root_prefix = "../../";

    let copy_icon = icons::copy(None);

    let track_choices_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_number = index + 1;
            let track_title = track.title();

            let t_audio_player_widget_for_xxx =
                build.locale.translations.audio_player_widget_for_xxx(&track_title);

            let (embed_copy_code, embed_display_code) = embed_code(
                base_url,
                index_suffix,
                &release.permalink.slug,
                track_number,
                &t_audio_player_widget_for_xxx
            );

            let t_copy = &build.locale.translations.copy;
            let r_copy_button = copy_button("content", &embed_copy_code, &copy_icon, t_copy);
            let track_number_formatted = release.track_numbering.format(track_number);
            let track_title_escaped = html_escape_outside_attribute(&track_title);

            formatdoc!(r#"
                <div class="embed_split" style="margin-top: 2rem; position: relative;">
                    <div>
                        <span style="color: var(--fg-3);">{track_number_formatted}</span>
                        <span>{track_title_escaped}</span>
                    </div>
                    {r_copy_button}
                </div>
                {embed_display_code}
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_link = format!("..{index_suffix}");

    let r_compact_release_identifier = compact_release_identifier(
        build,
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let t_audio_player_widget_for_xxx =
        build.locale.translations.audio_player_widget_for_xxx(&release.title);

    let (embed_copy_code, embed_display_code) = embed_code(
        base_url,
        index_suffix,
        &release.permalink.slug,
        "all",
        &t_audio_player_widget_for_xxx
    );

    let t_copy = &build.locale.translations.copy;
    let r_copy_button = copy_button("content", &embed_copy_code, &copy_icon, t_copy);

    let copy_icon = icons::copy(None);
    let failed_icon = icons::failure(&build.locale.translations.failed);
    let success_icon = icons::success(&build.locale.translations.copied);
    let templates = format!(r#"
        <template id="copy_icon">
            {copy_icon}
        </template>
        <template id="failed_icon">
            {failed_icon}
        </template>
        <template id="success_icon">
            {success_icon}
        </template>
    "#);

    let t_embed = &build.locale.translations.embed;
    let t_embed_entire_release = &build.locale.translations.embed_entire_release;
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center page_100vh">
                <div>
                    <h1>{t_embed}</h1>
                    {r_compact_release_identifier}

                    <div style="margin-top: 2rem;">
                        <div class="embed_split">
                            <span>{t_embed_entire_release}</span>
                            {r_copy_button}
                        </div>
                        {embed_display_code}
                    </div>

                    {track_choices_rendered}
                </div>
            </div>
        </div>
        {templates}
    "#);

    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let breadcrumb = Some(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::Clipboard,
        &release.theme,
        &release.title,
        CrawlerMeta::NoIndexNoFollow,
        breadcrumb
    )
}
