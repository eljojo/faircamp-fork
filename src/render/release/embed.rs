use indoc::formatdoc;
use url::Url;

use crate::{
    build::Build,
    catalog::Catalog,
    localization::WritingDirection,
    release::Release,
    render::{cover, list_artists, release::waveform},
    util::format_time
};

pub fn embed_html(build: &Build, catalog: &Catalog, release: &Release, base_url: &Url) -> String {
    let root_prefix = &"../".repeat(2);

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
                track_number = index + 1,
                track_src = track.get_as(&release.streaming_format).as_ref().unwrap().filename,  // TODO: get_in_build(...) or such to differentate this from an intermediate cache asset request
                track_title = track.title,
                waveform = waveform(track, index, track_duration_width_em)
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
                            <span style="transform: scaleX(80%) translate(9%, -5%) scale(90%);">▶</span>
                        </a>
                    </div>
                    <div style="margin: 0.4em 0 1em 0;">
                        <h1>{release_title}</h1>
                        <div>{artists}</div>
                    </div>

                    {tracks_rendered}

                    <div>
                        Listen to everything at <a href="{root_prefix}">{base_url}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(root_prefix, &release.artists),
        base_url = base_url,
        cover = cover(root_prefix, release),
        release_title = release.title,
        root_prefix = root_prefix,
        tracks_rendered = tracks_rendered
    );

    embed_layout(root_prefix, &body, build, catalog, &release.title)
}

fn embed_layout(root_prefix: &str, body: &str, build: &Build, catalog: &Catalog, title: &str) -> String {
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
        feed_user_link = feed_user_link,
        root_prefix = root_prefix,
        theming_widget = theming_widget,
        title = title
    )
}
