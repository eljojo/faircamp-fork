use indoc::formatdoc;

use crate::artist::Artist;
use crate::catalog::Catalog;
use crate::download_option::DownloadOption;
use crate::release::Release;

const DOWNLOAD_INCLUDES_TEXT: &str = "Includes high-quality download in MP3, FLAC and more.";
const PAYING_SUPPORTERS_TEXT: &str = "Paying supporters make a dignified life for artists possible, giving them some financial security in their life.";

fn layout(page_depth: usize, body: &str, title: &str) -> String {
    formatdoc!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>{title}</title>
                    <meta charset="utf-8">
                    <meta name="description" content="{title}">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <!-- TODO: <link rel="alternate" type="application/rss+xml" title="RSS Feed" href="{root_prefix}feed.rss"> -->
                    <link href="{root_prefix}styles.css" rel="stylesheet">
                </head>
                <body>
                    {body}
                </body>
            </html>
        "#,
        body=body,
        root_prefix=("../".repeat(page_depth)),
        title=title
    )
}

pub fn render_download(artist: &Artist, release: &Release) -> String {
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(r#"<img class="cover" src="../../{}">"#, image.transcoded_file),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    let body = formatdoc!(
        r#"
            {release_cover_rendered}
            
            <h1>Download {release_title}</h1>
            <div>by <a href="{artist_href}">{artist_name}</a></div>
            
            <div><a download href="original.zip">Zip Archive</a></div>
        "#,
        artist_href="TODO",
        artist_name=artist.name,
        release_cover_rendered=release_cover_rendered,
        release_title=release.title
    );
    
    layout(2, &body, &release.title)
}

pub fn render_release(artist: &Artist, release: &Release) -> String {
    // TODO: Probably outsource that into impl DownloadOption (give it its own file I guess then)
    let download_option_rendered = match &release.download_option {
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free(download_hash) => formatdoc!(
            r#"
                <a href="../download/{hash}/">Download Digital Release</a>
                {includes_text}
            "#,
            hash=download_hash,
            includes_text=DOWNLOAD_INCLUDES_TEXT
        ),
        DownloadOption::NameYourPrice => formatdoc!(
            r#"
                <a href="../download/todo">Buy Digital Release</a> Name Your Price
                {includes_text} {paying_text}
            "#,
            includes_text=DOWNLOAD_INCLUDES_TEXT,
            paying_text=PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayExactly(price) => formatdoc!(
            r#"
                <a href="../download/todo">Buy Digital Release</a> {price}
                {includes_text} {paying_text}
            "#,
            price=price,
            includes_text=DOWNLOAD_INCLUDES_TEXT,
            paying_text=PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayMinimum(price) => formatdoc!(
            r#"
                <a href="../download/todo">Buy Digital Release</a> {price} or more
                {includes_text} {paying_text}
            "#,
            price=price,
            includes_text=DOWNLOAD_INCLUDES_TEXT,
            paying_text=PAYING_SUPPORTERS_TEXT
        )
    };
    
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(r#"<img class="cover" src="../{}">"#, image.transcoded_file),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)|
            format!(
                "{track_number}. {track_title} <audio controls src=\"../{track_transcoded_file}\"></audio>",
                track_number=index + 1,
                track_transcoded_file=track.transcoded_file,
                track_title=track.title
            )
        )
        .collect::<Vec<String>>()
        .join("<br><br>\n");

    let body = formatdoc!(
        r#"
            <h1>{release_title}</h1>
            <div>by <a href="{artist_href}">{artist_name}</a></div>
            
            {download_option_rendered}
            
            {release_cover_rendered}

            <div>{tracks}</div>
            
            <div>{release_text}</div>
        "#,
        artist_href="TODO",
        artist_name=artist.name,
        download_option_rendered=download_option_rendered,
        release_cover_rendered=release_cover_rendered,
        release_text=release.text.as_ref().unwrap_or(&String::new()),
        release_title=release.title,
        tracks=tracks_rendered
    );
    
    layout(1, &body, &release.title)
}

pub fn render_releases(artist: &Artist, catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .map(|release| {
            let release_cover_rendered = match &release.cover {
                Some(image) => format!(r#"<img class="cover" src="{}">"#, image.transcoded_file),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            format!(
                r#"{release_cover_rendered} <a href="{release_slug}/">{release_title}</a>"#,
                release_cover_rendered=release_cover_rendered,
                release_slug=release.slug,
                release_title=release.title
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    let body = formatdoc!(
        r#"
            <h1>{artist_name}</h1>
            <div>{releases}</div>
        "#,
        artist_name=artist.name,
        releases=releases_rendered
    );
    
    layout(0, &body, &artist.name)
}