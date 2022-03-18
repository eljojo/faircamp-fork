use indoc::formatdoc;

use crate::{
    build::Build,
    catalog::Catalog,
    release::Release,
    render::{cover, layout, list_artists, release::waveform},
    util::format_time
};

pub fn embed_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
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
                        <!-- TODO: For this link we need the base_url, so we can actually pull
                                   this out of the "feed" options and make this a required-ish
                                   thing in the "catalog" options itself. Required-*ish* because
                                   we could still conditionally just not activate the embed
                                   feature if the base_url is missing - which makes sense to support. -->
                        Listen to everything at <a href="TODO">{catalog_title}</a>
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(root_prefix, &release.artists),
        catalog_title = catalog.title(),
        cover = cover(root_prefix, release),
        release_title = release.title,
        tracks_rendered = tracks_rendered
    );

    layout(root_prefix, &body, build, catalog, &release.title)
}
