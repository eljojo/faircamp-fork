use indoc::formatdoc;
use url::Url;

use crate::{
    Build,
    Catalog,
    ImageFormat,
    Release,
    render::{image, layout, list_artists, play_icon, release::waveform},
    Track,
    util::format_time,
    WritingDirection
};

/// The title parameter provides a text that indicates to screen-reader users
/// what to expect inside the iframe. See description at
/// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe#accessibility_concerns
fn embed_code<T: std::fmt::Display>(base_url: &Url, permalink_slug: &str, postfix: T, title: &str) -> String {
    format!(
        r#"<textarea class="embed_code" readonly="true">&lt;iframe loading="lazy" src="{base_url}{permalink_slug}/embed/{postfix}" title="{title}"&gt;&lt;/iframe&gt;</textarea>"#,
        base_url = base_url,
        permalink_slug = permalink_slug,
        postfix = postfix,
        title = title
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
                    <div class="track_title_wrapper">
                        <span class="track_number">{track_number:02}</span>
                        <a class="track_title">
                            {track_title} <span class="pause"></span>
                        </a>
                        {embed_code}
                    </div>
                "#,
                embed_code = embed_code(base_url, &release.permalink.slug, track_number, "Audio player widget for one track"),
                track_number = track_number,
                track_title = track.title
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let body = formatdoc!(
        r##"
            <div class="center_unconstrained">
                <div class="release_grid vpad">
                    <div class="cover">
                        {cover}
                    </div>

                    <div style="margin: 0.4em 0 1em 0;">
                        <h1>{release_title}</h1>
                        <div>{artists}</div>
                    </div>

                    Embed the entire release
                    {embed_code}

                    {track_choices_rendered}
                </div>
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &release.artists),
        cover = image(explicit_index, root_prefix, &release.cover, ImageFormat::Cover),
        embed_code = embed_code(base_url, &release.permalink.slug, "all", "Audio player widget for all tracks of a release"),
        release_title = release.title,
        track_choices_rendered = track_choices_rendered
    );

    layout(root_prefix, &body, build, catalog, &release.title)
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
            let track_duration_width_em = if longest_track_duration > 0 {
                36.0 * (track.cached_assets.source_meta.duration_seconds as f32 / longest_track_duration as f32)
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
                track_number = track_number,
                track_src = track.get_as(release.streaming_format).as_ref().unwrap().filename,  // TODO: get_in_build(...) or such to differentate this from an intermediate cache asset request
                track_title = track.title,
                waveform = waveform(track, track_number, track_duration_width_em)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let body = formatdoc!(
        r##"
            <div class="center_unconstrained">
                <div class="release_grid vpad">
                    <div class="cover">
                        {cover}
                    </div>

                    <div style="justify-self: end; align-self: end; margin: 0.4em 0 1em 0;">
                        <a class="track_play">
                            {play_icon}
                        </a>
                    </div>
                    <div style="margin: 0.4em 0 1em 0;">
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
        artists = list_artists(explicit_index, root_prefix, &release.artists),
        base_url = base_url,
        cover = image(explicit_index, root_prefix, &release.cover, ImageFormat::Cover),
        explicit_index = explicit_index,
        play_icon = play_icon(root_prefix),
        release_title = release.title,
        root_prefix = root_prefix,
        tracks_rendered = tracks_rendered
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}

fn embed_layout(root_prefix: &str, body: &str, build: &Build, catalog: &Catalog, title: &str) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    
    let feed_user_link = match &build.base_url.is_some() {
        true => format!(
            r#"<a href="{root_prefix}feed.rss">RSS</a>"#,
            root_prefix = root_prefix
        ),
        false => String::new()
    };

    let dir_attribute = match build.localization.writing_direction {
        WritingDirection::Ltr => "",
        WritingDirection::Rtl => "dir=\"rtl\""
    };

    let theming_widget = if build.theming_widget {
        include_str!("../../templates/theming_widget.html")
    } else {
        ""
    };

    format!(
        include_str!("../../templates/embed.html"),
        body = body,
        catalog_title = catalog.title(),
        dir_attribute = dir_attribute,
        explicit_index = explicit_index,
        feed_user_link = feed_user_link,
        root_prefix = root_prefix,
        theming_widget = theming_widget,
        title = title
    )
}

pub fn embed_track_html(build: &Build, catalog: &Catalog, release: &Release, track: &Track, track_number: usize, base_url: &Url) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../../";

    let track_duration = track.cached_assets.source_meta.duration_seconds;
    let track_duration_width_em = if track_duration > 0 { 36.0 } else { 0.0 };

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
        track_title = track.title,
        waveform = waveform(track, track_number, track_duration_width_em)
    );

    let body = formatdoc!(
        r##"
            <div class="center_unconstrained">
                <div class="release_grid vpad">
                    <div class="cover">
                        {cover}
                    </div>

                    <div style="justify-self: end; align-self: end; margin: 0.4em 0 1em 0;">
                        <a class="track_play">
                            {play_icon}
                        </a>
                    </div>
                    <div style="margin: 0.4em 0 1em 0;">
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
        artists = list_artists(explicit_index, root_prefix, &release.artists),
        base_url = base_url,
        cover = image(explicit_index, root_prefix, &release.cover, ImageFormat::Cover),
        explicit_index = explicit_index,
        play_icon = play_icon(root_prefix),
        release_title = release.title,
        root_prefix = root_prefix,
        track_rendered = track_rendered
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}
