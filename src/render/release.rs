use indoc::formatdoc;
use std::rc::Rc;

use crate::{
    artist::Artist,
    build::Build,
    catalog::Catalog,
    download_option::DownloadOption,
    release::Release,
    render::{SHARE_WIDGET, cover, layout},
    track::Track,
    util::format_time
};

pub mod checkout;
pub mod download;
pub mod embed;

fn list_artists(root_prefix: &str, artists: &Vec<Rc<Artist>>) -> String {
    artists
        .iter()
        .map(|artist|
            format!(
                r#"<a href="{root_prefix}{permalink}/">{name}</a>"#,
                name = artist.name,
                permalink = artist.permalink.get(),
                root_prefix = root_prefix
            )
        )
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn release_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let root_prefix = &"../".repeat(1);

    let formats_list = release.download_formats
        .iter()
        .map(|format| format.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let includes_text = format!("Available Formats: {}", formats_list);

    let download_option_rendered = match &release.download_option {
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free { download_page_uid } => formatdoc!(
            r#"
                <div class="vpad">
                    <a href="../download/{download_page_uid}/">Download Digital Release</a>
                    <div>{includes_text}</div>
                </div>
            "#,
            download_page_uid=download_page_uid,
            includes_text=includes_text
        ),
        DownloadOption::Paid { checkout_page_uid, currency, range, .. } => {
            let price_label = if range.end == f32::INFINITY {
                if range.start > 0.0 {
                    format!(
                        "{currency_symbol}{min_price} {currency_code} or more",
                        currency_code=currency.code(),
                        currency_symbol=currency.symbol(),
                        min_price=range.start
                    )
                } else {
                    format!("Name Your Price ({})", currency.code())
                }
            } else if range.start == range.end {
                format!(
                    "{currency_symbol}{price} {currency_code}",
                    currency_code=currency.code(),
                    currency_symbol=currency.symbol(),
                    price=range.start
                )
            } else if range.start > 0.0 {
                format!(
                    "{currency_symbol}{min_price}-{currency_symbol}{max_price} {currency_code}",
                    currency_code=currency.code(),
                    currency_symbol=currency.symbol(),
                    max_price=range.end,
                    min_price=range.start
                )
            } else {
                format!(
                    "Up to {currency_symbol}{max_price} {currency_code}",
                    currency_code=currency.code(),
                    currency_symbol=currency.symbol(),
                    max_price=range.end
                )
            };

            formatdoc!(
                r#"
                    <div class="vpad">
                        <a href="../checkout/{checkout_page_uid}/">Buy Digital Release</a> {price_label}
                        <div>{includes_text}</div>
                    </div>
                "#,
                checkout_page_uid=checkout_page_uid,
                includes_text=includes_text,
                price_label=price_label
            )
        }
    };

    let embed_widget = if release.embedding {
        r#"<a href="embed/">Embed</a>"#
    } else {
        ""
    };

    let release_text = match &release.text {
        Some(text) => format!(r#"<div class="vpad">{}</div>"#, text),
        None => String::new()
    };

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
                        <audio controls preload="metadata" src="../{track_src}"></audio>
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
                <!-- TODO: This one needs to be conditional depending on download/buy option-->
                <!-- div>
                    <a href="#download_buy_todo">$</a>
                </div -->

                <div class="release_grid vpad">
                    <div></div>
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

                    <div></div>
                    <div>
                        {download_option_rendered}
                    </div>

                    <div></div>
                    <div>
                        {release_text}
                        {embed_widget}
                        {share_widget}
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(root_prefix, &release.artists),
        cover = cover(root_prefix, release),
        download_option_rendered = download_option_rendered,
        embed_widget = embed_widget,
        release_text = release_text,
        release_title = release.title,
        share_widget = SHARE_WIDGET,
        tracks_rendered = tracks_rendered
    );

    layout(root_prefix, &body, build, catalog, &release.title)
}

fn waveform(track: &Track, index: usize, track_duration_width_em: f32) -> String {
    let step = 1;

    if let Some(peaks) = &track.cached_assets.source_meta.peaks {
        let height = 10;
        let width = peaks.len();

        let mut enumerate_peaks = peaks.iter().step_by(step).enumerate();

        let mut d = format!("M 0,{}", (1.0 - enumerate_peaks.next().unwrap().1) * height as f32);

        while let Some((index, peak)) = enumerate_peaks.next() {
            let command = format!(
                " L {x},{y}",
                x = index * step,
                y = (1.0 - peak) * height as f32
            );

            d.push_str(&command);
        }

        formatdoc!(
            r##"
                <svg class="waveform"
                     preserveAspectRatio="none"
                     style="width: {track_duration_width_em}em;"
                     viewBox="0 0 {width} {height}"
                     xmlns="http://www.w3.org/2000/svg">
                    <defs>
                        <linearGradient id="progress_{index}">
                            <stop offset="0%" stop-color="hsl(var(--hue), var(--link-s), var(--link-l))" />
                            <stop offset="0.000001%" stop-color="hsl(var(--text-h), var(--text-s), var(--text-l))" />
                        </linearGradient>
                    </defs>
                    <style>
                        .levels_{index} {{ stroke: url(#progress_{index}); }}
                    </style>
                    <path class="levels_{index}" d="{d}" />
                </svg>
            "##,
            d = d,
            height = height,
            index = index,
            track_duration_width_em = track_duration_width_em,
            width = width
        )
    } else {
        String::new()
    }
}
