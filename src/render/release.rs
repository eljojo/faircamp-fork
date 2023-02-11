use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadOption,
    Release,
    render::{
        cover_image,
        layout,
        list_artists,
        play_icon,
        share_link,
        share_overlay
    },
    Track,
    util::{
        format_time,
        html_escape_inside_attribute,
        html_escape_outside_attribute
    }
};

pub mod checkout;
pub mod download;
pub mod embed;

pub fn release_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";

    let download_link = match &release.download_option {
        DownloadOption::Codes { .. } => {
            let page_hash = build.hash_generic(&[&release.permalink.slug, "checkout"]);

            let t_download_with_unlock_code = &build.locale.strings.download_with_unlock_code;
            formatdoc!(r#"
                <a href="checkout/{page_hash}{index_suffix}">{t_download_with_unlock_code}</a>
            "#)
        },
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free => {
            let page_hash = build.hash_generic(&[&release.permalink.slug, "download"]);

            let t_download = &build.locale.strings.download;
            formatdoc!(r#"
                <a href="download/{page_hash}{index_suffix}">{t_download}</a>
            "#)
        },
        DownloadOption::Paid(_currency, _range) => {
            if release.payment_options.is_empty() {
                String::new()
            } else {
                let checkout_page_hash = build.hash_generic(&[&release.permalink.slug, "checkout"]);

                let t_buy = &build.locale.strings.buy;
                formatdoc!(r#"
                    <a href="checkout/{checkout_page_hash}{index_suffix}">{t_buy}</a>
                "#)
            }
        }
    };

    let t_embed = &build.locale.strings.embed;
    let embed_link = if release.embedding && build.base_url.is_some() {
        format!(r#"<a href="embed{index_suffix}">{t_embed}</a>"#)
    } else {
        String::new()
    };

    let release_text = match &release.text {
        Some(text) => format!(r#"<div class="vpad">{}</div>"#, text),
        None => String::new()
    };

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

            let track_filename = format!(
                "{basename}{extension}",
                basename = track.asset_basename.as_ref().unwrap(),
                extension = release.streaming_format.extension()
            ); 

            let track_hash = build.hash(
                &release.permalink.slug,
                release.streaming_format.asset_dirname(),
                &track_filename
            );

            formatdoc!(
                r#"
                    <div class="track">
                        <a class="track_controls outer">{play_icon}</a>
                        <span class="track_number outer">{track_number}</span>
                        <span class="track_header">
                            <a class="track_controls inner">{play_icon}</a>
                            <span class="track_number inner">{track_number}</span>
                            <a class="track_title" title="{track_title_attribute}">{track_title}</a>
                            <span class="duration"><span class="track_time"></span>{duration}</span>
                        </span>
                        <audio controls preload="metadata" src="{streaming_format_dir}/{track_hash}/{track_filename}"></audio>
                        {waveform}
                    </div>
                "#,
                play_icon = play_icon(root_prefix),
                duration = format_time(track.assets.borrow().source_meta.duration_seconds),
                streaming_format_dir = release.streaming_format.asset_dirname(),
                track_number = release.track_numbering.format(track_number),
                track_title = html_escape_outside_attribute(&track.title),
                track_title_attribute = html_escape_inside_attribute(&track.title),
                waveform = waveform(track)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let share_url = match &build.base_url {
        Some(base_url) => base_url
            .join(&format!("{}{}", &release.permalink.slug, index_suffix))
            .unwrap()
            .to_string(),
        None => String::new()
    };

    let share_link_rendered = share_link(build);
    let share_overlay_rendered = share_overlay(build, &share_url);

    let mut action_links = String::new();

    if !download_link.is_empty() {
        action_links.push_str(&download_link);
        action_links.push_str(" &nbsp; ");
    }

    if !embed_link.is_empty() {
        action_links.push_str(&embed_link);
        action_links.push_str(" &nbsp; ");
    }

    action_links.push_str(&share_link_rendered);

    let body = formatdoc!(
        r##"
            <div class="center_release mobile_hpadding">
                <div class="cover">
                    {cover}
                </div>

                <div style="justify-self: end; align-self: end; margin: .4rem 0 1rem 0;">
                    <a class="big_play_button">
                        {play_icon}
                    </a>
                </div>

                <div style="margin: .4rem 0 1rem 0;">
                    <h1 style="margin-bottom: .2rem;">{release_title_escaped}</h1>
                    <div>{artists}</div>
                </div>

                <br>

                <div data-longest-duration="{longest_track_duration}"></div>
                {tracks_rendered}
            </div>
            <div class="additional">
                <div class="center_release mobile_hpadding">
                    <div>
                        {action_links}
                    </div>
                    <div>
                        {release_text}
                    </div>
                </div>
            </div>
            {share_overlay_rendered}
        "##,
        artists = list_artists(index_suffix, root_prefix, &catalog, &release.artists),
        cover = cover_image(build, index_suffix, "", root_prefix, &release.cover, None),
        play_icon = play_icon(root_prefix)
    );

    let breadcrumbs = &[
        format!(r#"<a href=".{index_suffix}">{release_title_escaped}</a>"#)
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}

fn waveform(track: &Track) -> String {
    if let Some(peaks) = &track.assets.borrow().source_meta.peaks {
        let peaks_base64 = peaks.iter()
            .map(|peak| {
                // Limit range to 0-63
                let peak64 = ((peak / 1.0) * 63.0) as u8;
                let base64 = match peak64 {
                    0..=25 => (peak64 + 65) as char, // shift to 65-90 (A-Z)
                    26..=51 => (peak64 + 71) as char, // shift to 97-122 (a-z)
                    52..=61 => (peak64 - 4) as char, // shift to 48-57 (0-9)
                    62 => '+', // map to 43 (+)
                    63 => '/', // map to 48 (/)
                    _ => unreachable!() 
                };
                base64.to_string()
            })
            .collect::<Vec<String>>()
            .join("");

        let duration_seconds = track.assets.borrow().source_meta.duration_seconds;

        formatdoc!(r#"
            <svg class="waveform"
                 data-duration="{duration_seconds}"
                 data-peaks="{peaks_base64}">
                <path class="progress"/>
                <path class="base"/>
            </svg>
        "#)
    } else {
        String::new()
    }
}
