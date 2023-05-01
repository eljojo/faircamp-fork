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

pub fn embed_release_html(
    build: &Build,
    catalog: &Catalog,
    release: &Release,
    base_url: &Url
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let longest_track_duration = release.longest_track_duration();

    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_number = index + 1;

            let audio_sources = release.streaming_quality
                .formats()
                .iter()
                .map(|format| {
                    let format_dir = format.asset_dirname();
                    let format_extension = format.extension();

                    let track_filename = format!(
                        "{basename}{format_extension}",
                        basename = track.asset_basename.as_ref().unwrap()
                    ); 

                    let track_hash = build.hash(
                        &release.permalink.slug,
                        format_dir,
                        &track_filename
                    );

                    let source_type = format.source_type();
                    let src = format!("{release_prefix}{format_dir}/{track_hash}/{track_filename}");

                    format!(r#"<source src="{src}" type="{source_type}">"#)
                })
                .collect::<Vec<String>>()
                .join("\n");

            formatdoc!(
                r#"
                    <div class="track_title_wrapper">
                        <span class="track_number">{track_number:02}</span>
                        <a class="track_title">
                            {track_title} <span class="pause"></span>
                        </a>
                    </div>
                    <div class="track_waveform">
                        <audio controls preload="none">
                            {audio_sources}
                        </audio>
                        {waveform} <span class="track_duration">{track_duration}</span>
                    </div>
                "#,
                track_duration = format_time(track.assets.borrow().source_meta.duration_seconds),
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
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let track_duration = track.assets.borrow().source_meta.duration_seconds;

    let audio_sources = release.streaming_quality
        .formats()
        .iter()
        .map(|format| {
            let format_dir = format.asset_dirname();
            let format_extension = format.extension();

            let track_filename = format!(
                "{basename}{format_extension}",
                basename = track.asset_basename.as_ref().unwrap()
            ); 

            let track_hash = build.hash(
                &release.permalink.slug,
                format_dir,
                &track_filename
            );

            let source_type = format.source_type();
            let src = format!("{release_prefix}{format_dir}/{track_hash}/{track_filename}");

            format!(r#"<source src="{src}" type="{source_type}">"#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let track_rendered = formatdoc!(
        r#"
            <div class="track">
                <a class="track_controls outer">{play_icon}</a>
                <span class="track_number outer">{track_number}</span>
                <span class="track_header">
                    <a class="track_controls inner">{play_icon}</a>
                    <span class="track_number inner">{track_number}</span>
                    <a class="track_title" title="{track_title_attribute}">{track_title}</a>
                    <span class="duration"><span class="track_time"></span>{duration_formatted}</span>
                </span>
                <audio controls preload="none">
                    {audio_sources}
                </audio>
                {waveform}
            </div>
        "#,
        duration_formatted = format_time(track_duration),
        play_icon = play_icon(root_prefix),
        track_number = release.track_numbering.format(track_number),
        track_title = html_escape_outside_attribute(&track.title),
        track_title_attribute = html_escape_inside_attribute(&track.title),
        waveform = waveform(track)
    );

    let body = formatdoc!(
        r##"
            <div class="hcenter_unconstrained">
                <div class="vpad">
                    <div class="cover">{cover}</div>

                    <div class="release_label">
                        <h1>{release_title_escaped}</h1>
                        <div class="release_artists">{artists}</div>
                    </div>

                    <div data-longest-duration="{track_duration}" disable-relative-wavforms></div>
                    {track_rendered}

                    <div>
                        Listen to everything at <a href="{root_prefix}.{index_suffix}">{base_url}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(index_suffix, root_prefix, catalog, release),
        cover = cover_image(build, index_suffix, release_prefix, root_prefix, release),
        release_title_escaped = html_escape_outside_attribute(&release.title)
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}
