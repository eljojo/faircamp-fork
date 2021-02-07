use indoc::formatdoc;

use crate::artist::Artist;
use crate::catalog::Catalog;
use crate::download_option::DownloadOption;
use crate::release::Release;

const DOWNLOAD_INCLUDES_TEXT: &str = "Includes high-quality download in MP3, FLAC and more.";
const PAYING_SUPPORTERS_TEXT: &str = "Paying supporters make a dignified life for artists possible, giving them some financial security in their life.";

pub fn render_download(artist: &Artist, release: &Release) -> String {
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(r#"<img class="cover" src="../../{}">"#, image.transcoded_file),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    formatdoc!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Download {release_title}</title>
                    <meta charset="utf-8">
                    <meta name="description" content="Download {release_title}">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <link rel="alternate" type="application/rss+xml" title="RSS Feed" href="feed.rss"> <!--TODO-->
                    <link href="../../styles.css" rel="stylesheet">
                </head>
                <body>
                    {release_cover_rendered}
                    
                    <h1>Download {release_title}</h1>
                    <div>by <a href="{artist_href}">{artist_name}</a></div>
                    
                    <div><a download href="original.zip">Zip Archive</a></div>
                </body>
            </html>
        "#,
        artist_href="TODO",
        artist_name=artist.name,
        release_cover_rendered=release_cover_rendered,
        release_title=release.title
    )
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

    formatdoc!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>{release_title}</title>
                    <meta charset="utf-8">
                    <meta name="description" content="{release_title}">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <link rel="alternate" type="application/rss+xml" title="RSS Feed" href="feed.rss"> <!--TODO-->
                    <link href="../styles.css" rel="stylesheet">
                </head>
                <body>
                    <h1>{release_title}</h1>
                    <div>by <a href="{artist_href}">{artist_name}</a></div>
                    
                    {download_option_rendered}
                    
                    {release_cover_rendered}

                    <div>{tracks}</div>
                    
                    <div>{release_text}</div>
                </body>
            </html>
        "#,
        artist_href="TODO",
        artist_name=artist.name,
        download_option_rendered=download_option_rendered,
        release_cover_rendered=release_cover_rendered,
        release_text=release.text.as_ref().unwrap_or(&String::new()),
        release_title=release.title,
        tracks=tracks_rendered
    )
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
                r#"{release_cover_rendered} <a href="{release_slug}">{release_title}</a>"#,
                release_cover_rendered=release_cover_rendered,
                release_slug=release.slug,
                release_title=release.title
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    formatdoc!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>{artist_name}</title>
                    <meta charset="utf-8">
                    <meta name="description" content="{artist_name}">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <link rel="alternate" type="application/rss+xml" title="RSS Feed" href="feed.rss"> <!--TODO-->
                    <link href="styles.css" rel="stylesheet">
                </head>
                <body>
                    <h1>{artist_name}</h1>

                    <div>{releases}</div>
                </body>
            </html>
        "#,
        artist_name=artist.name,
        releases=releases_rendered
    )
}