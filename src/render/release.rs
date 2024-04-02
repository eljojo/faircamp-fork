use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadOption,
    Release,
    Theme,
    Track
};
use crate::render::{
    cover_image,
    layout,
    list_artists,
    play_icon,
    share_link,
    share_overlay
};
use crate::util::{
    format_time,
    html_escape_inside_attribute,
    html_escape_outside_attribute
};

pub mod checkout;
pub mod download;
pub mod embed;

pub fn release_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let root_prefix = "../";

    let download_link = match &release.download_option {
        DownloadOption::Codes { .. } => {
            let t_unlock_permalink = &build.locale.translations.unlock_permalink;
            let page_hash = build.hash_generic(&[&release.permalink.slug, t_unlock_permalink]);

            let unlock_icon = include_str!("../icons/unlock.svg");
            let t_downloads = &build.locale.translations.downloads;
            formatdoc!(r#"
                <a href="{t_unlock_permalink}/{page_hash}{index_suffix}">
                    {unlock_icon}
                    <span>{t_downloads}</span>
                </a>
            "#)
        },
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free => {
            let t_downloads_permalink = &build.locale.translations.downloads_permalink;
            let page_hash = build.hash_generic(&[&release.permalink.slug, t_downloads_permalink]);

            let download_icon = include_str!("../icons/download.svg");
            let t_downloads = &build.locale.translations.downloads;
            formatdoc!(r#"
                <a href="{t_downloads_permalink}/{page_hash}{index_suffix}">
                    {download_icon}
                    <span>{t_downloads}</span>
                </a>
            "#)
        },
        DownloadOption::Paid(_currency, _range) => {
            if release.payment_options.is_empty() {
                String::new()
            } else {
                let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                let page_hash = build.hash_generic(&[&release.permalink.slug, t_purchase_permalink]);

                let buy_icon = include_str!("../icons/buy.svg");
                let t_downloads = &build.locale.translations.downloads;
                formatdoc!(r#"
                    <a href="{t_purchase_permalink}/{page_hash}{index_suffix}">
                        {buy_icon}
                        <span>{t_downloads}</span>
                    </a>
                "#)
            }
        }
    };

    let embed_link = if release.embedding && build.base_url.is_some() {
        let embed_icon = include_str!("../icons/embed.svg");
        let t_embed = &build.locale.translations.embed;
        formatdoc!(r#"
            <a href="embed{index_suffix}">
                {embed_icon}
                <span>{t_embed}</span>
            </a>
        "#)
    } else {
        String::new()
    };

    let release_text = match &release.text {
        Some(html_and_stripped) => format!(
            r#"<div class="vpad" style="margin-top: 1.5rem;">{}</div>"#,
            &html_and_stripped.html
        ),
        None => String::new()
    };

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

                    let track_filename_urlencoded = urlencoding::encode(&track_filename);
                    let src = format!("{format_dir}/{track_hash}/{track_filename_urlencoded}");

                    let source_type = format.source_type();
                     format!(r#"<source src="{src}" type="{source_type}">"#)
                })
                .collect::<Vec<String>>()
                .join("\n");

            formatdoc!(
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
                duration_formatted = format_time(track.assets.borrow().source_meta.duration_seconds),
                play_icon = play_icon(root_prefix),
                track_number = release.track_numbering.format(track_number),
                track_title = html_escape_outside_attribute(&track.title),
                track_title_attribute = html_escape_inside_attribute(&track.title),
                waveform = waveform(&build.theme, track)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let share_url = match &build.base_url {
        Some(base_url) => base_url
            .join(&format!("{}{index_suffix}", &release.permalink.slug))
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

    let relative_waveforms = if build.theme.relative_waveforms { "" } else { "data-disable-relative-waveforms " };

    let body = formatdoc!(
        r##"
            <div class="vcenter_page_outer">
                <div class="hcenter_narrow mobile_hpadding vcenter_page vpad_adaptive">
                    <div class="cover">{cover}</div>

                    <div class="release_label">
                        <h1>{release_title_escaped}</h1>
                        <div class="release_artists">{artists}</div>
                    </div>

                    <div {relative_waveforms}data-longest-duration="{longest_track_duration}"></div>
                    {tracks_rendered}
                </div>
                <div class="additional">
                    <div class="mobile_hpadding">
                        <div class="action_links">
                            {action_links}
                        </div>
                        {release_text}
                    </div>
                </div>
            </div>
            {share_overlay_rendered}
        "##,
        artists = list_artists(index_suffix, root_prefix, catalog, release),
        cover = cover_image(build, index_suffix, "", root_prefix, release)
    );

    let breadcrumbs = &[
        format!(r#"<a href=".{index_suffix}">{release_title_escaped}</a>"#)
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}

pub fn waveform(theme: &Theme, track: &Track) -> String {
    if let Some(peaks) = &track.assets.borrow().source_meta.peaks {
        let peaks_base64 = peaks.iter()
            .map(|peak| {
                // In https://codeberg.org/simonrepp/faircamp/issues/11#issuecomment-858690
                // the "_ => unreachable!()" branch below was hit, probably due to a slight
                // peak overshoot > 1.0 (1.016 already leads to peak64 being assigned 64).
                // We know that some decoders can produce this kind of overshoot, ideally
                // we should be normalizing (=limiting) these peaks to within 0.0-1.0
                // already when we compute/store/cache them. For now we prevent the panic
                // locally here as a patch.
                // TODO:
                // - Implement normalizing/limiting at the point of decoding/caching
                // - Implement an integrity check of all peaks at cache retrieval time (?),
                //   triggering a correction and cache update/removal if found - this is
                //   only meant as a temporary measure, to be phased out in some months/
                //   years.
                //   OR: Better yet use the cache layout versioning
                //   flag to trigger a cache update for all updated faircamp
                //   versions, so all peaks are correctly recalculated for everyone then.
                // - Then also remove this peak_limited correction and rely on the raw
                //   value again.
                let peak_limited = if theme.waveforms {
                    if *peak > 1.0 { 1.0 } else { *peak }
                } else {
                    0.5
                };

                // Limit range to 0-63
                let peak64 = ((peak_limited / 1.0) * 63.0) as u8;
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
