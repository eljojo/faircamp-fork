use indoc::formatdoc;
use std::rc::Rc;

use crate::{
    artist::Artist,
    audio_format,
    build_settings::BuildSettings,
    catalog::Catalog,
    download_option::DownloadOption,
    image_format::ImageFormat,
    payment_option::PaymentOption,
    release::Release
};

const SHARE_WIDGET: &str = include_str!("templates/share_widget.html");

fn layout(page_depth: usize, body: &str, build_settings: &BuildSettings, catalog: &Catalog, title: &str) -> String {
    let root_prefix = "../".repeat(page_depth);
    let (feed_meta_link, feed_user_link) = match &build_settings.base_url.is_some() {
        true => (
            format!(
                r#"<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="{root_prefix}feed.rss">"#,
                root_prefix=root_prefix
            ),
            format!(
                r#"<a href="{root_prefix}feed.rss">RSS</a>"#,
                root_prefix=root_prefix
            ),
        ),
        false => (String::new(), String::new())
    };
    
    format!(
        include_str!("templates/layout.html"),
        body=body,
        catalog_title=catalog.title.as_ref().map(|title| title.as_str()).unwrap_or("About"),
        feed_meta_link=feed_meta_link,
        feed_user_link=feed_user_link,
        root_prefix=root_prefix,
        title=title,
        version=env!("CARGO_PKG_VERSION")
    )
}

fn list_artists(page_depth: usize, artists: &Vec<Rc<Artist>>) -> String {
    artists
        .iter()
        .map(|artist|
            format!(
                r#"<a href="{root_prefix}{artist_slug}/">{artist_name}</a>"#,
                artist_slug=artist.slug,
                artist_name=artist.name,
                root_prefix=("../".repeat(page_depth))
            )
        )
        .collect::<Vec<String>>()
        .join(", ") // TODO: Consider "Alice, Bob and Carol" as polish over "Alice, Bob, Carol" (something for later)
}

pub fn render_about(build_settings: &BuildSettings, catalog: &Catalog) -> String {
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
    
    layout(1, &body, build_settings, catalog, title)
}

pub fn render_artist(build_settings: &BuildSettings, artist: &Rc<Artist>, catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .filter_map(|release| {
            if release.artists
                .iter()
                .find(|release_artist| Rc::ptr_eq(release_artist, artist))
                .is_none() {
                return None;
            }
            
            let release_cover_rendered = match &release.cover {
                Some(image) => format!(
                    r#"<img alt="Release cover" class="cover" src="../{filename}">"#,
                    filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
                ),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            let release_rendered = formatdoc!(
                r#"
                    <div>
                        {release_cover_rendered}
                        <a href="../{release_slug}/">{release_title}</a>
                    </div>
                "#,
                release_cover_rendered=release_cover_rendered,
                release_slug=release.slug,
                release_title=release.title
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
    
    layout(1, &body, build_settings, catalog, &artist.name)
}

pub fn render_artists(build_settings: &BuildSettings, catalog: &Catalog) -> String {
    let artists_rendered = catalog.artists
        .iter()
        .map(|artist| {
            let releases_rendered = catalog.releases
                .iter()
                .filter_map(|release| {
                    if release.artists
                        .iter()
                        .find(|release_artist| Rc::ptr_eq(release_artist, artist))
                        .is_none() {
                        return None;
                    }
                    
                    let release_cover_rendered = match &release.cover {
                        Some(image) => format!(
                            r#"<img alt="Release cover" class="cover" src="../{filename}">"#,
                            filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
                        ),
                        None => String::from(r#"<div class="cover"></div>"#)
                    };
                    
                    let release_rendered = formatdoc!(
                        r#"
                            <div>
                                <a href="../{release_slug}/" title="{release_title}">{release_cover_rendered}</a>
                            </div>
                        "#,
                        release_cover_rendered=release_cover_rendered,
                        release_slug=release.slug,
                        release_title=release.title
                    );
                    
                    Some(release_rendered)
                })
                .collect::<Vec<String>>()
                .join("<br><br>\n");
            
            formatdoc!(
                r#"
                    <div>
                        <a href="../{artist_slug}/">{artist_name}</a>
                        {releases_rendered}
                    </div>
                "#,
                artist_slug=artist.slug,
                artist_name=artist.name,
                releases_rendered=releases_rendered
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    let body = formatdoc!(
        r#"
            <div>
                {artists_rendered}
            </div>
        "#,
        artists_rendered=artists_rendered
    );
    
    layout(1, &body, build_settings, catalog, "Artists")
}

pub fn render_checkout(build_settings: &BuildSettings, catalog: &Catalog, release: &Release, download_page_uid: &str) -> String {
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
    
    layout(2, &body, build_settings, catalog, &release.title)
}

pub fn render_download(build_settings: &BuildSettings, catalog: &Catalog, release: &Release) -> String {
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
    
    layout(2, &body, build_settings, catalog, &release.title)
}

pub fn render_release(build_settings: &BuildSettings, catalog: &Catalog, release: &Release) -> String {
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
        .map(|track| track.cached_assets.source_meta.duration_seconds.unwrap_or(0))
        .max();
    
    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let track_duration_width_em = match (track.cached_assets.source_meta.duration_seconds, longest_track_duration) {
                (Some(this_duration), Some(longest_duration)) => (36.0 * (this_duration as f32 / longest_duration as f32)) as u32,
                _ => 0
            };
        
            formatdoc!(
                r#"
                    <div class="track">
                        <a class="muted track_number">{track_number}</a>
                        <span class="track_title">
                            <div class="track_duration_bar" style="width: {track_duration_width_em}em;"></div>
                            <span class="muted track_duration_text" style="left: {track_duration_width_em}em;">{track_duration}</span>
                            {track_title}
                        </span>
                        <audio controls src="../{track_src}"></audio>
                    </div>
                "#,
                track_duration=track.duration_formatted(),
                track_duration_width_em=track_duration_width_em,
                track_number=index + 1,
                track_src=track.get_as(&release.streaming_format).as_ref().unwrap().filename,  // TODO: get_in_build(...) or such to differentate this from an intermediate cache asset request
                track_title=track.title
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let body = formatdoc!(
        r##"
            <div class="center">
                {release_cover_rendered}

                <div class="apart hsplit vpad">
                    <div>
                        <h1>{release_title}</h1>
                        <div>{artists_rendered}</div>
                    </div>
                    
                    <!-- TODO: This one needs to be conditional actually depending on download/buy option-->
                    <div>
                        <a href="#download_buy_todo">$</a>
                    </div>
                </div>
                
                <div class="vpad">
                    {tracks_rendered}
                </div>
                
                {download_option_rendered}
                
                {release_text}
                
                {share_widget}
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
    
    layout(1, &body, build_settings, catalog, &release.title)
}

pub fn render_releases(build_settings: &BuildSettings, catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .map(|release| {
            let artists_rendered = list_artists(0, &release.artists);
            
            let release_cover_rendered = match &release.cover {
                Some(image) => format!(
                    r#"<img alt="Release cover" src="{filename}">"#,
                    filename=image.get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename
                ),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            formatdoc!(
                r#"
                    <div>
                        <a class="cover" href="{release_slug}/">
                            {release_cover_rendered}
                        </a>
                        <div class="vpad">
                            <a class="large" href="{release_slug}/">{release_title}</a>
                            <div>{artists_rendered}</div>
                        </div>
                    </div>
                "#,
                artists_rendered=artists_rendered,
                release_cover_rendered=release_cover_rendered,
                release_slug=release.slug,
                release_title=release.title
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
    
    layout(0, &body, build_settings, catalog, catalog.title.as_ref().map(|title| title.as_str()).unwrap_or("Catalog"))
}