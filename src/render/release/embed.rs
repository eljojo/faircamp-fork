use indoc::formatdoc;
use url::Url;

use crate::{
    Build,
    Catalog,
    Release,
    render::{cover_image, layout, list_artists, play_icon},
    render::release::waveform,
    Track,
    util::{format_time, html_escape_outside_attribute},
    WritingDirection
};

/// The title parameter provides a text that indicates to screen-reader users
/// what to expect inside the iframe. See description at
/// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#accessibility_concerns
fn embed_code<T: std::fmt::Display>(base_url: &Url, permalink_slug: &str, postfix: T, title: &str) -> String {
    format!(
        r#"<textarea class="embed_code" onfocus="this.select()" readonly="true">&lt;iframe loading="lazy" src="{base_url}{permalink_slug}/embed/{postfix}" title="{title}"&gt;&lt;/iframe&gt;</textarea>"#
    )
}

pub fn embed_choices_html(build: &Build, catalog: &Catalog, release: &Release, base_url: &Url) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../";

    let track_choices_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_number = index + 1;
            formatdoc!(
                r#"
                    <div style="position: relative;">
                        <span class="track_number">{track_number:02}</span>
                        <span>{track_title}</span><br><br>
                        {embed_code}
                    </div>
                "#,
                embed_code = embed_code(base_url, &release.permalink.slug, track_number, "Audio player widget for one track"),
                track_title = html_escape_outside_attribute(&track.title)
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");

    let release_prefix = "../";

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let t_embed_release = &build.locale.strings.embed_release;
    let t_embed_entire_release = &build.locale.strings.embed_release;
    let body = formatdoc!(
        r##"
            <div class="center_release mobile_hpadding">
                <h1>{t_embed_release}</h1>

                <br><br>

                <div style="align-items: center; display: flex;">
                    <div style="margin-right: .8rem; max-width: 4rem">
                        {cover}
                    </div>
                    <div>
                        <div>{release_title_escaped}</div>
                        <div>{artists}</div>
                    </div>
                </div>

                <br><br>

                {t_embed_entire_release}<br><br>
                {embed_code}

                <br><br><br>

                {track_choices_rendered}
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        embed_code = embed_code(base_url, &release.permalink.slug, "all", "Audio player widget for all tracks of a release")
    );

    let t_embed = &build.locale.strings.embed;
    let breadcrumbs = &[
        format!(r#"<a href="..{explicit_index}">{release_title_escaped}</a>"#),
        format!("<span>{t_embed}</span>")
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}

pub fn embed_release_html(build: &Build, catalog: &Catalog, release: &Release, base_url: &Url) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
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
            <div class="center_unconstrained">
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

                    <div data-longest-duration="{longest_track_duration}"></div>
                    {tracks_rendered}

                    <div>
                        Listen to everything at <a href="{root_prefix}.{explicit_index}">{base_url}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        play_icon = play_icon(root_prefix),
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

pub fn embed_track_html(build: &Build, catalog: &Catalog, release: &Release, track: &Track, track_number: usize, base_url: &Url) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
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
            <div class="center_unconstrained">
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
                        Listen to everything at <a href="{root_prefix}.{explicit_index}">{base_url}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        play_icon = play_icon(root_prefix),
        release_title = html_escape_outside_attribute(&release.title)
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}
