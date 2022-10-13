use indoc::formatdoc;
use url::Url;

use crate::{
    Build,
    Catalog,
    Release,
    render::{cover_image, layout, list_artists, play_icon},
    render::release::{MAX_TRACK_DURATION_WIDTH_EM, waveform},
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
                    <div>
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

    let release_prefix = format!(
        "{root_prefix}{permalink}/",
        permalink = release.permalink.slug
    );

    let body = formatdoc!(
        r##"
            <div class="center_release">
                <div class="cover">
                    {cover}
                </div>

                <div style="margin: 0.4rem 0 1rem 0;">
                    <h1>{release_title}</h1>
                    <div>{artists}</div>
                </div>

                Embed the entire release<br><br>
                {embed_code}

                <br><br><br>

                {track_choices_rendered}
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        embed_code = embed_code(base_url, &release.permalink.slug, "all", "Audio player widget for all tracks of a release"),
        release_title = html_escape_outside_attribute(&release.title)
    );

    layout(root_prefix, &body, build, catalog, &release.title, None)
}

pub fn embed_release_html(build: &Build, catalog: &Catalog, release: &Release, base_url: &Url) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../../";

    let longest_track_duration = release.tracks
        .iter()
        .map(|track| track.cached_assets.source_meta.duration_seconds)
        .max()
        .unwrap();

    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_duration_width_rem = if longest_track_duration > 0 {
                MAX_TRACK_DURATION_WIDTH_EM * (track.cached_assets.source_meta.duration_seconds as f32 / longest_track_duration as f32)
            } else {
                0.0
            };
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
                track_duration = format_time(track.cached_assets.source_meta.duration_seconds),
                track_src = track.get_as(release.streaming_format).as_ref().unwrap().filename,  // TODO: get_in_build(...) or such to differentate this from an intermediate cache asset request
                track_title = html_escape_outside_attribute(&track.title),
                waveform = waveform(track, track_number, track_duration_width_rem)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = format!(
        "{root_prefix}{permalink}/",
        permalink = release.permalink.slug
    );

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
    let dir_attribute = match build.localization.writing_direction {
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

    let track_duration = track.cached_assets.source_meta.duration_seconds;
    let track_duration_width_rem = if track_duration > 0 { MAX_TRACK_DURATION_WIDTH_EM } else { 0.0 };

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
        track_src = track.get_as(release.streaming_format).as_ref().unwrap().filename,  // TODO: get_in_build(...) or such to differentate this from an intermediate cache asset request
        track_title = html_escape_outside_attribute(&track.title),
        waveform = waveform(track, track_number, track_duration_width_rem)
    );

    let release_prefix = format!(
        "{root_prefix}{permalink}/",
        permalink = release.permalink.slug
    );

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
