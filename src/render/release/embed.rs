use indoc::formatdoc;
use url::Url;

use crate::{
    Build,
    Catalog,
    Release,
    render::{
        compact_release_identifier,
        copy_button,
        cover_image,
        layout,
        list_artists,
        play_icon
    },
    render::release::waveform,
    Track,
    util::{
        format_time,
        html_double_escape_inside_attribute,
        html_escape_inside_attribute,
        html_escape_outside_attribute
    },
    WritingDirection
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
            <iframe loading="lazy" src="{url}" title="{title_escaped}"></iframe>
        "#)
    );

    let display_code = formatdoc!(r#"
        <div class="embed_code_wrapper">
            <pre class="embed_code"><span class="embed_syntax_special">&lt;</span>iframe
            loading<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"lazy"</span>
            src<span class="embed_syntax_special">=</span><span class="embed_syntax_value">"{url}"</span>
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

    let track_choices_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_number = index + 1;
            let track_title = html_escape_outside_attribute(&track.title);

            let t_audio_player_widget_for_track =
                build.locale.translations.audio_player_widget_for_track(&track.title);

            let (embed_copy_code, embed_display_code) = embed_code(
                base_url,
                index_suffix,
                &release.permalink.slug,
                track_number,
                &t_audio_player_widget_for_track
            );

            let r_copy_button = copy_button(build, Some(&embed_copy_code));

            formatdoc!(r#"
                <div class="hcenter_wide embed_split mobile_hpadding" style="margin-top: 2rem; position: relative;">
                    <div style="font-size: var(--subtly-larger);">
                        <span class="track_number">{track_number:02}</span>
                        <span>{track_title}</span>
                    </div>
                    {r_copy_button}
                </div>
                {embed_display_code}
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_link = format!("..{index_suffix}");

    let compact_release_identifier_rendered = compact_release_identifier(
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let t_audio_player_widget_for_release = 
        build.locale.translations.audio_player_widget_for_release(&release.title);

    let (embed_copy_code, embed_display_code) = embed_code(
        base_url,
        index_suffix,
        &release.permalink.slug,
        "all",
        &t_audio_player_widget_for_release
    );

    let r_copy_button = copy_button(build, Some(&embed_copy_code));

    let t_embed = &build.locale.translations.embed;
    let t_embed_entire_release = &build.locale.translations.embed_entire_release;
    let body = formatdoc!(r##"
        <div class="hcenter_wide margin_page_top mobile_hpadding">
            <h1>{t_embed}</h1>

            {compact_release_identifier_rendered}
        </div>

        <div style="margin-top: 2rem;">
            <div class="hcenter_wide embed_split mobile_hpadding">
                <span style="font-size: var(--subtly-larger);">{t_embed_entire_release}</span>
                {r_copy_button}
            </div>
            {embed_display_code}
        </div>

        {track_choices_rendered}

        <div class="margin_page_bottom"></div>
    "##);

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let embed_icon = include_str!("../../icons/embed.svg");
    let t_embed = &build.locale.translations.embed;
    let breadcrumbs = &[
        format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#),
        format!(r#"<a href=".{index_suffix}">{embed_icon} {t_embed}</a>"#)
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}

pub fn embed_release_html(build: &Build, catalog: &Catalog, release: &Release, base_url: &Url) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../../";

    let longest_track_duration = release.tracks
        .iter()
        .map(|track| track.assets.borrow().source_meta.duration_seconds)
        .max()
        .unwrap();

    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_number = index + 1;

            formatdoc!(
                r#"
                    <div class="track_title_wrapper">
                        <span class="track_number">{track_number:02}</span>
                        <a class="track_title">
                            {track_title} <span class="pause"></span>
                        </a>
                    </div>
                    <div class="track_waveform">
                        <audio controls preload="metadata" src="../../{track_src}"></audio>
                        {waveform} <span class="track_duration">{track_duration}</span>
                    </div>
                "#,
                track_duration = format_time(track.assets.borrow().source_meta.duration_seconds),
                track_src = track.assets.borrow().get(release.streaming_format).as_ref().unwrap().filename,
                track_title = html_escape_outside_attribute(&track.title),
                waveform = waveform(track)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = "../../";

    let body = formatdoc!(
        r##"
            <div class="hcenter_unconstrained">
                <div class="vpad">
                    <div class="cover">
                        {cover}
                    </div>

                    <div class="release_label">
                        <h1>{release_title}</h1>
                        <div class="release_artists">{artists}</div>
                    </div>

                    <div data-longest-duration="{longest_track_duration}"></div>
                    {tracks_rendered}

                    <div>
                        Listen to everything at <a href="{root_prefix}.{index_suffix}">{base_url}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(index_suffix, root_prefix, catalog, release),
        cover = cover_image(build, index_suffix, release_prefix, root_prefix, release),
        release_title = html_escape_outside_attribute(&release.title)
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}

fn embed_layout(root_prefix: &str, body: &str, build: &Build, catalog: &Catalog, title: &str) -> String {
    let dir_attribute = match build.locale.writing_direction {
        WritingDirection::Ltr => "",
        WritingDirection::Rtl => "dir=\"rtl\""
    };

    format!(
        include_str!("../../templates/embed.html"),
        body = body,
        catalog_title = html_escape_outside_attribute(&catalog.title()),
        dir_attribute = dir_attribute,
        root_prefix = root_prefix,
        title = html_escape_outside_attribute(title)
    )
}

pub fn embed_track_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    track: &Track,
    track_number: usize,
    base_url: &Url
) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../../../";

    let track_duration = track.assets.borrow().source_meta.duration_seconds;

    let track_rendered = formatdoc!(
        r#"
            <div class="track_title_wrapper">
                <span class="track_number">{track_number:02}</span>
                <a class="track_title">
                    {track_title} <span class="pause"></span>
                </a>
            </div>
            <div class="track_waveform">
                <audio controls preload="metadata" src="../../{track_src}"></audio>
                {waveform} <span class="track_duration">{track_duration}</span>
            </div>
        "#,
        track_duration = format_time(track_duration),
        track_src = track.assets.borrow().get(release.streaming_format).as_ref().unwrap().filename,
        track_title = html_escape_outside_attribute(&track.title),
        waveform = waveform(track)
    );

    let release_prefix = "../../";

    let body = formatdoc!(
        r##"
            <div class="hcenter_unconstrained">
                <div class="vpad">
                    <div class="cover">
                        {cover}
                    </div>

                    <div style="justify-self: end; align-self: end; margin: .4rem 0 1rem 0;">
                        <a class="big_play_button">
                            {play_icon}
                        </a>
                    </div>
                    <div style="margin: .4rem 0 1rem 0;">
                        <h1>{release_title}</h1>
                        <div>{artists}</div>
                    </div>

                    {track_rendered}

                    <div>
                        Listen to everything at <a href="{root_prefix}.{index_suffix}">{base_url}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(index_suffix, root_prefix, catalog, release),
        cover = cover_image(build, index_suffix, release_prefix, root_prefix, release),
        play_icon = play_icon(root_prefix),
        release_title = html_escape_outside_attribute(&release.title)
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}
