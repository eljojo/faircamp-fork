use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadOption,
    ImageFormat,
    Release,
    render::{image, layout, list_artists, play_icon},
    Track,
    util::{format_time, html_escape_outside_attribute}
};

pub mod checkout;
pub mod download;
pub mod embed;

const MAX_TRACK_DURATION_WIDTH_EM: f32 = 36.0;
const TRACK_HEIGHT_EM: f32 = 1.9;
const WAVEFORM_PADDING_EM: f32 = 0.6;
const WAVEFORM_HEIGHT: f32 = TRACK_HEIGHT_EM - 2.0 * WAVEFORM_PADDING_EM;

pub fn release_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../";

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
                    <a href="../download/{download_page_uid}{explicit_index}">Download Digital Release</a>
                    <div>{includes_text}</div>
                </div>
            "#,
            download_page_uid = download_page_uid,
            explicit_index = explicit_index,
            includes_text = includes_text
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
                        <a href="../checkout/{checkout_page_uid}{explicit_index}">Buy Digital Release</a> {price_label}
                        <div>{includes_text}</div>
                    </div>
                "#,
                checkout_page_uid=checkout_page_uid,
                explicit_index = explicit_index,
                includes_text=includes_text,
                price_label=price_label
            )
        }
    };

    let embed_widget = if release.embedding && build.base_url.is_some() {
        format!(
            r#"<a href="embed{explicit_index}">Embed</a>"#,
            explicit_index = explicit_index
        )
    } else {
        String::new()
    };

    let release_text = match &release.text {
        Some(text) => format!(r#"<div class="vpad">{}</div>"#, html_escape_outside_attribute(text)),
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
                MAX_TRACK_DURATION_WIDTH_EM * (track.cached_assets.source_meta.duration_seconds as f32 / longest_track_duration as f32)
            } else {
                0.0
            };
            let track_number = index + 1;

            formatdoc!(
                r#"
                    <div class="track">
                        <a class="track_title_wrapper">
                            <span class="track_title">{track_title}</span>
                            <span class="track_controls">{play_icon}</span>
                            <span class="track_number">{track_number}</span>
                        </a>
                        <div class="track_waveform">
                            <audio controls preload="metadata" src="{root_prefix}{track_src}"></audio>
                            <div class="track_progress_bar" data-max-width="{track_duration_width_em}"></div>
                            {waveform} <span class="track_duration">{track_duration}</span>
                        </div>
                    </div>
                "#,
                play_icon = play_icon(root_prefix),
                root_prefix = root_prefix,
                track_duration = format_time(track.cached_assets.source_meta.duration_seconds),
                track_duration_width_em = track_duration_width_em,
                track_number = release.track_numbering.format(track_number),
                track_src = track.get_as(release.streaming_format).as_ref().unwrap().filename,  // TODO: get_in_build(...) or such to differentate this from an intermediate cache asset request
                track_title = html_escape_outside_attribute(&track.title),
                waveform = waveform(track, track_number, track_duration_width_em)
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
                        <a class="big_play_button">
                            {play_icon}
                        </a>
                    </div>
                    <div style="margin: 0.4em 0 1em 0;">
                        <h1>{release_title}</h1>
                        <div>{artists}</div>
                    </div>
                </div>

                {tracks_rendered}

                <div class="align vpad">
                    <div></div>
                    <div>
                        {download_option_rendered}
                    </div>

                    <div></div>
                    <div>
                        {release_text}
                        {embed_widget}
                    </div>
                </div>
            </div>
        "##,
        artists = list_artists(explicit_index, root_prefix, &release.artists),
        cover = image(explicit_index, root_prefix, &release.cover, ImageFormat::Cover),
        download_option_rendered = download_option_rendered,
        embed_widget = embed_widget,
        play_icon = play_icon(root_prefix),
        release_text = html_escape_outside_attribute(&release_text),
        release_title = html_escape_outside_attribute(&release.title),
        tracks_rendered = tracks_rendered
    );

    layout(root_prefix, &body, build, catalog, &release.title)
}

fn waveform(track: &Track, track_number: usize, track_duration_width_em: f32) -> String {
    let step = 1;

    if let Some(peaks) = &track.cached_assets.source_meta.peaks {
        let num_peaks = peaks.len();
        let step_width = track_duration_width_em / num_peaks as f32;

        let mut enumerate_peaks = peaks.iter().step_by(step).enumerate();

        let mut d = format!(
            "M 0,{}",
            WAVEFORM_PADDING_EM + (1.0 - enumerate_peaks.next().unwrap().1) * WAVEFORM_HEIGHT
        );

        while let Some((index, peak)) = enumerate_peaks.next() {
            let command = format!(
                " L {x},{y}",
                x = index as f32 * step_width,
                y = WAVEFORM_PADDING_EM + (1.0 - peak) * WAVEFORM_HEIGHT
            );

            d.push_str(&command);
        }

        formatdoc!(
            r##"
                <svg class="waveform"
                     height="{viewbox_height}em"
                     viewBox="0 0 {viewbox_width} {viewbox_height}"
                     width="{viewbox_width}em"
                     xmlns="http://www.w3.org/2000/svg">
                    <defs>
                        <linearGradient id="progress_{track_number}">
                            <stop offset="0%" stop-color="hsl(var(--hue), var(--link-s), var(--link-l))" />
                            <stop offset="0.000001%" stop-color="hsl(var(--text-h), var(--text-s), var(--text-l))" />
                        </linearGradient>
                    </defs>
                    <style>
                        .levels_{track_number} {{ stroke: url(#progress_{track_number}); }}
                    </style>
                    <path class="levels_{track_number}" d="{d}" />
                </svg>
            "##,
            d = d,
            track_number = track_number,
            viewbox_height = TRACK_HEIGHT_EM,
            viewbox_width = track_duration_width_em
        )
    } else {
        String::new()
    }
}
