use indoc::formatdoc;
use std::rc::Rc;

use crate::{
    artist::Artist,
    catalog::Catalog,
    download_option::DownloadOption,
    release::Release
};

const PAYING_SUPPORTERS_TEXT: &str = "Paying supporters make a dignified life for artists possible, giving them some financial security in their life.";

fn layout(page_depth: usize, body: &str, title: &str) -> String {
    format!(
        include_str!("assets/layout.html"),
        body=body,
        root_prefix=("../".repeat(page_depth)),
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

pub fn render_artist(artist: &Rc<Artist>, catalog: &Catalog) -> String {
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
                    r#"<img alt="Release cover" class="cover" src="../{image_uuid}.jpg">"#,
                    image_uuid=image.uuid
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
        share_widget=share_widget(&artist.slug)
    );
    
    layout(1, &body, &artist.name)
}

pub fn render_artists(catalog: &Catalog) -> String {
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
                            r#"<img alt="Release cover" class="cover" src="../{image_uuid}.jpg">"#,
                            image_uuid=image.uuid
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
    
    layout(1, &body, "Artists")
}

pub fn render_download(release: &Release) -> String {
    let artists_rendered = list_artists(2, &release.artists);
    
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(
            r#"<img alt="Release cover" class="cover" src="../../{image_uuid}.jpg">"#,
            image_uuid=image.uuid
        ),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    // TODO: Possibly DRY this up (used in a very similar fashion in render_release)
    // TODO: Dynamically assign the "Recommended format", depending on what formats are available
    let format_availability = &[
        (release.download_formats.mp3_v0, "MP3 (VBR/V0) - Recommended Format"),
        (release.download_formats.mp3_320, "MP3 (CBR/320kbps)"),
        (release.download_formats.mp3_128, "MP3 (CBR/128kbps)"),
        (release.download_formats.flac, "FLAC"),
        (release.download_formats.aac, "AAC"),
        (release.download_formats.ogg_vorbis, "Ogg Vorbis"),
        (release.download_formats.wav, "WAV"),
        (release.download_formats.aiff, "AIFF")
    ];
    
    
    let download_links = format_availability
        .iter()
        .filter_map(|(enabled, label)|
            if *enabled { 
                Some(
                    formatdoc!(
                        r#"
                            <div>
                                <a download href="todo.zip">Download {label}</a>
                            </div>
                        "#,
                        label=label
                    )
                )
            } else {
                None
            }
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
    
    layout(2, &body, &release.title)
}



pub fn render_release(release: &Release) -> String {
    let artists_rendered = list_artists(1, &release.artists);
    
    let format_availability = &[
        (release.download_formats.aac, "AAC"),
        (release.download_formats.flac, "FLAC"),
        (
            release.download_formats.mp3_128 ||
            release.download_formats.mp3_320 ||
            release.download_formats.mp3_v0,
            "MP3"
        ),
        (release.download_formats.ogg_vorbis, "Ogg Vorbis")
    ];
    
    let includes_text = if format_availability.iter().any(|(enabled, _label)| *enabled) {
        let formats_list = format_availability
            .iter()
            .filter_map(|(enabled, label)| if *enabled { Some(label.to_string()) } else { None })
            .collect::<Vec<String>>()
            .join(",");
        
        format!("Includes high-quality download as {}", formats_list)
    } else {
        String::from("Includes medium-quality download as MP3 128")
    };
    
    // TODO: Probably outsource that into impl DownloadOption (give it its own file I guess then)
    let download_option_rendered = match &release.download_option {
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free(download_hash) => formatdoc!(
            r#"
                <div class="vpad">
                    <a href="../download/{hash}/">Download Digital Release</a>
                    <div>{includes_text}</div>
                </div>
            "#,
            hash=download_hash,
            includes_text=includes_text
        ),
        DownloadOption::NameYourPrice => formatdoc!(
            r#"
                <div class="vpad">
                    <a href="../download/todo">Buy Digital Release</a> Name Your Price
                    <div>{includes_text} {paying_text}</div>
                </div>
            "#,
            includes_text=includes_text,
            paying_text=PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayExactly(price) => formatdoc!(
            r#"
                <div class="vpad">
                    <a href="../download/todo">Buy Digital Release</a> {price}
                    <div>{includes_text} {paying_text}</div>
                </div>
            "#,
            price=price,
            includes_text=includes_text,
            paying_text=PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayMinimum(price) => formatdoc!(
            r#"
                <div class="vpad">
                    <a href="../download/todo">Buy Digital Release</a> {price} or more
                    <div>{includes_text} {paying_text}</div>
                </div>
            "#,
            price=price,
            includes_text=includes_text,
            paying_text=PAYING_SUPPORTERS_TEXT
        )
    };
    
    // TODO: Here and elsewhere DRY this up, repeats multiple times
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(
            r#"<img alt="Release cover" class="cover" src="../{image_uuid}.jpg">"#,
            image_uuid=image.uuid
        ),
        None => String::from(r#"<div class="cover vpad"></div>"#)
    };
    
    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)|
            formatdoc!(
                r#"
                    <div>
                        <a class="play">▶️</a><span class="muted">{track_number:02}</span> {track_title} <audio controls src="../{track_src}"></audio> <span class="muted">{track_duration}</span>
                    </div>
                "#,
                track_duration=track.duration_formatted(),
                track_number=index + 1,
                track_src=format!("{}{}", track.uuid, release.streaming_format.suffix_and_extension()),
                track_title=track.title
            )
        )
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
                
                {share_widget}
                
                <div>{release_text}</div>
            </div>
        "##,
        artists_rendered=artists_rendered,
        download_option_rendered=download_option_rendered,
        release_cover_rendered=release_cover_rendered,
        release_text=release.text.as_ref().unwrap_or(&String::new()),
        release_title=release.title,
        share_widget=share_widget(&release.slug), // TODO: Build absolute url
        tracks_rendered=tracks_rendered
    );
    
    layout(1, &body, &release.title)
}

pub fn render_releases(catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .map(|release| {
            let artists_rendered = list_artists(0, &release.artists);
            
            let release_cover_rendered = match &release.cover {
                Some(image) => format!(
                    r#"<img alt="Release cover" class="cover" src="{image_uuid}.jpg">"#,
                    image_uuid=image.uuid
                ),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            formatdoc!(
                r#"
                    <div>
                        {release_cover_rendered}
                        <a href="{release_slug}/">{release_title}</a>
                        <div>{artists_rendered}</div>
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
    
    layout(0, &body, "Catalog")
}

fn share_widget(url: &str) -> String {
    format!(r#"<a class="disabled share" data-url="{}" title="Not available in your browser">Share</a>"#, url)
}