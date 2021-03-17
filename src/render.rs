use indoc::formatdoc;
use std::rc::Rc;

use crate::{
    artist::Artist,
    audio_format,
    build::Build,
    catalog::Catalog,
    download_option::DownloadOption,
    image_format::ImageFormat,
    localization::WritingDirection,
    payment_option::PaymentOption,
    release::Release,
    track::Track,
    util
};

const SHARE_WIDGET: &str = include_str!("templates/share_widget.html");

fn layout(page_depth: usize, body: &str, build: &Build, catalog: &Catalog, title: &str) -> String {
    let root_prefix = "../".repeat(page_depth);
    
    let (feed_meta_link, feed_user_link) = match &build.base_url.is_some() {
        true => (
            format!(
                r#"<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="{root_prefix}feed.rss">"#,
                root_prefix=root_prefix
            ),
            format!(
                r#"<a href="{root_prefix}feed.rss">■ RSS</a>"#,
                root_prefix=root_prefix
            ),
        ),
        false => (String::new(), String::new())
    };
    
    let dir_attribute = match build.localization.writing_direction {
        WritingDirection::Ltr => "",
        WritingDirection::Rtl => "dir=\"rtl\""
    };
    
    format!(
        include_str!("templates/layout.html"),
        body=body,
        catalog_title=catalog.title.as_ref().map(|title| title.as_str()).unwrap_or("About"),
        dir_attribute=dir_attribute,
        feed_meta_link=feed_meta_link,
        feed_user_link=feed_user_link,
        root_prefix=root_prefix,
        title=title
    )
}

fn list_artists(page_depth: usize, artists: &Vec<Rc<Artist>>) -> String {
    artists
        .iter()
        .map(|artist|
            format!(
                r#"<a href="{root_prefix}{permalink}/">{name}</a>"#,
                name = artist.name,
                permalink = artist.permalink.get(),
                root_prefix = ("../".repeat(page_depth))
            )
        )
        .collect::<Vec<String>>()
        .join(", ") // TODO: Consider "Alice, Bob and Carol" as polish over "Alice, Bob, Carol" (something for later)
}

pub fn render_about(build: &Build, catalog: &Catalog) -> String {
    let text = catalog.text
        .as_ref()
        .map(|title| title.as_str())
        .unwrap_or("");
        
    let title = catalog.title
        .as_ref()
        .map(|title| title.as_str())
        .unwrap_or("Catalog");
    
    let body = formatdoc!(
        r#"
            <div class="center">
                <div class="vpad">
                    <h1>{title}</h1>
                </div>
                
                <div class="vpad">
                    {text}
                </div>
                
                {share_widget}
            </div>
        "#,
        share_widget=SHARE_WIDGET,
        text=text,
        title=title
    );
    
    layout(1, &body, build, catalog, title)
}

pub fn render_artist(build: &Build, artist: &Rc<Artist>, catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .filter_map(|release| {
            if release.artists
                .iter()
                .find(|release_artist| Rc::ptr_eq(release_artist, artist))
                .is_none() {
                return None;
            }
            
            let cover = match &release.cover {
                Some(image) => format!(
                    r#"<img alt="Release cover" class="cover" src="../{filename}">"#,
                    filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
                ),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            let release_rendered = formatdoc!(
                r#"
                    <div>
                        {cover}
                        <a href="../{permalink}/">{title}</a>
                    </div>
                "#,
                cover = cover,
                permalink = release.permalink.get(),
                title = release.title
            );
            
            Some(release_rendered)
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    let body = formatdoc!(
        r#"
            <div class="center">
                TODO: Artist image

                <div class="vpad">
                    <h1>{artist_name}</h1>
                </div>
                
                <div class="vpad">
                    TODO: Artist text
                </div>
                
                {releases_rendered}
                
                {share_widget}
            </div>
        "#,
        artist_name=artist.name,
        releases_rendered=releases_rendered,
        share_widget=SHARE_WIDGET
    );
    
    layout(1, &body, build, catalog, &artist.name)
}

pub fn render_checkout(build: &Build, catalog: &Catalog, release: &Release, download_page_uid: &str) -> String {
    let artists_rendered = list_artists(2, &release.artists);
    
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(
            r#"<img alt="Release cover" class="cover" src="../../{filename}">"#,
            filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
        ),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    let payment_options = &release.payment_options
        .iter()
        .map(|option|
            match &option {
                PaymentOption::Custom(html) => {
                    format!(
                        r#"
                            <div>
                                <div>{message}</div>
                                <a href="../../download/{download_page_uid}/">I have made the payment — Continue</a>
                            </div>
                        "#,
                        download_page_uid=download_page_uid,
                        message=html.to_string()
                    )
                },
                PaymentOption::Liberapay(account_name) => {
                    let liberapay_url = format!("https://liberapay.com/{}", account_name);
                    
                    format!(
                        r#"
                            <div>
                                <div>
                                    Pay on liberapay: <a href="{liberapay_url}">{liberapay_url}</a>
                                </div>
                                <a href="../../download/{download_page_uid}/">I have made the payment — Continue</a>
                            </div>
                        "#,
                        download_page_uid=download_page_uid,
                        liberapay_url=liberapay_url
                    )
                }
            }
        )
        .collect::<Vec<String>>()
        .join("\n");
    
    let body = formatdoc!(
        r#"
            {release_cover_rendered}
            
            <h1>Buy {release_title}</h1>
            <div>{artists_rendered}</div>
            
            {payment_options}
        "#,
        artists_rendered=artists_rendered,
        payment_options=payment_options,
        release_cover_rendered=release_cover_rendered,
        release_title=release.title
    );
    
    layout(2, &body, build, catalog, &release.title)
}

pub fn render_download(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let artists_rendered = list_artists(2, &release.artists);
    
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(
            r#"<img alt="Release cover" class="cover" src="../../{filename}">"#,
            filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
        ),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    let download_links = audio_format::sorted_and_annotated_for_download(&release.download_formats)
        .iter()
        .map(|(format, annotation)|
            formatdoc!(
                r#"
                    <div>
                        <a download href="../../{filename}">Download {label}{annotation}</a>
                    </div>
                "#,
                annotation=annotation.as_ref().map(|annotation| annotation.as_str()).unwrap_or(""),
                filename=release.cached_assets.get(format).as_ref().unwrap().filename,
                label=format.user_label()
            )
        )
        .collect::<Vec<String>>()
        .join("\n");
    
    let body = formatdoc!(
        r#"
            {release_cover_rendered}
            
            <h1>Download {release_title}</h1>
            <div>{artists_rendered}</div>
            
            {download_links}
        "#,
        artists_rendered=artists_rendered,
        download_links=download_links,
        release_cover_rendered=release_cover_rendered,
        release_title=release.title
    );
    
    layout(2, &body, build, catalog, &release.title)
}

pub fn render_release(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let artists_rendered = list_artists(1, &release.artists);
    
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
    
    // TODO: Here and elsewhere DRY this up, repeats multiple times
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(
            r#"<img alt="Release cover" class="cover" src="../{filename}">"#,
            filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
        ),
        None => String::from(r#"<div class="cover vpad"></div>"#)
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
                track_duration = util::format_time(track.cached_assets.source_meta.duration_seconds),
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
                    <div>
                        {release_cover_rendered}
                    </div>
                    
                    <div style="justify-self: end; align-self: end; margin: 0.4em 0 1em 0;">
                        <a class="track_play">
                            <span style="transform: scaleX(80%) translate(9%, -5%) scale(90%);">▶</span>
                        </a>
                    </div>
                    <div style="margin: 0.4em 0 1em 0;">
                        <h1>{release_title}</h1>
                        <div>{artists_rendered}</div>
                    </div>
                    
                    {tracks_rendered}
                    
                    <div></div>
                    <div>
                        {download_option_rendered}
                    </div>
                    
                    <div></div>
                    <div>
                        {release_text}
                        {share_widget}
                    </div>
                </div>
            </div>
        "##,
        artists_rendered=artists_rendered,
        download_option_rendered=download_option_rendered,
        release_cover_rendered=release_cover_rendered,
        release_text=release_text,
        release_title=release.title,
        share_widget=SHARE_WIDGET,
        tracks_rendered=tracks_rendered
    );
    
    layout(1, &body, build, catalog, &release.title)
}

pub fn render_releases(build: &Build, catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .map(|release| {
            let artists_rendered = list_artists(0, &release.artists);
            
            let cover = match &release.cover {
                Some(image) => format!(
                    r#"<img alt="Release cover" src="{filename}">"#,
                    filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
                ),
                None => String::from(r#"<div></div>"#)
            };
            
            let track_snippets = release.tracks
                .iter()
                .enumerate()
                .map(|(index, track)| waveform_snippet(track, index, 2.0))
                .collect::<Vec<String>>();
            
            formatdoc!(
                r#"
                    <div class="vpad" style="display: flex;">
                        <a class="cover_listing" href="{permalink}/">
                            {cover}
                        </a>
                        <div>
                            <a class="large" href="{permalink}/" style="color: #fff;">{title} <span class="runtime">{runtime}</span></a>
                            <div>{artists_rendered}</div>
                            <span class="">{track_snippets}</span>
                        </div>
                    </div>
                "#,
                artists_rendered = artists_rendered,
                cover = cover,
                permalink = release.permalink.get(),
                runtime = util::format_time(release.runtime),
                title = release.title,
                track_snippets = track_snippets.join("&nbsp;&nbsp;&nbsp;&nbsp;")
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    let body = formatdoc!(
        r#"
            <div class="releases">
                {releases}
            </div>
        "#,
        releases=releases_rendered
    );
    
    layout(0, &body, build, catalog, catalog.title.as_ref().map(|title| title.as_str()).unwrap_or("Catalog"))
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

fn waveform_snippet(track: &Track, index: usize, track_duration_width_em: f32) -> String {
    let step = 1;
    
    if let Some(peaks) = &track.cached_assets.source_meta.peaks {
        let height = 10;
        let width = 50;
        let mut enumerate_peaks = peaks.iter().skip(width * 2).step_by(step).enumerate(); 
        
        let mut d = format!("M 0,{}", (1.0 - enumerate_peaks.next().unwrap().1) * height as f32);
        
        while let Some((index, peak)) = enumerate_peaks.next() {
            if index > width { break; }
            
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