use indoc::formatdoc;

use crate::release::Release;
use crate::types::{Artist, DownloadOption};

const DOWNLOAD_INCLUDES_TEXT: &str = "Includes high-quality download in MP3, FLAC and more.";
const PAYING_SUPPORTERS_TEXT: &str = "Paying supporters make a dignified life for artists possible, giving them some financial security in their life.";

pub fn render_release(artist: &Artist, release: &Release) -> String {
    // TODO: Probably outsource that into impl DownloadOption (give it its own file I guess then)
    let download_option_rendered = match &release.download_option {
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free => formatdoc!(
            r#"
                <a href="">Download Digital Release</a>
                {}
            "#,
            DOWNLOAD_INCLUDES_TEXT
        ),
        DownloadOption::NameYourPrice => formatdoc!(
            r#"
                <a href="">Buy Digital Release</a> Name Your Price
                {} {}
            "#,
            DOWNLOAD_INCLUDES_TEXT,
            PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayExactly(price) => formatdoc!(
            r#"
                <a href="">Buy Digital Release</a> {}
                {} {}
            "#,
            price,
            DOWNLOAD_INCLUDES_TEXT,
            PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayMinimum(price) => formatdoc!(
            r#"
                <a href="">Buy Digital Release</a> {} or more
                {} {}
            "#,
            price,
            DOWNLOAD_INCLUDES_TEXT,
            PAYING_SUPPORTERS_TEXT
        )
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
                    <link href="styles.css" rel="stylesheet">
                </head>
                <body>
                    <h1>{release_title}</h1>
                    <div>by <a href="{artist_href}">{artist_name}</a></div>
                    
                    {download_option_rendered}
                    
                    <div class="cover"></div>

                    <div>{tracks}</div>
                    
                    <div>{release_text}</div>
                </body>
            </html>
        "#,
        artist_href="TODO",
        artist_name=artist.name,
        download_option_rendered=download_option_rendered,
        release_text=release.text.as_ref().unwrap_or(&String::new()),
        release_title=release.title,
        tracks=tracks_rendered
    )
}

pub fn render_releases(artist: &Artist, releases: &Vec<Release>) -> String {
    let releases_rendered = releases
        .iter()
        .map(|release|
            format!(
                r#"<div class="cover"></div> <a href="{release_slug}">{release_title}</a>"#,
                release_slug=release.slug,
                release_title=release.title
            )
        )
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