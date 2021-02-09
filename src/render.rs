use indoc::formatdoc;
use std::rc::Rc;

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
                    <div class="layout">
                        <header>
                            <nav>
                                <a href="{root_prefix}">Catalog</a>
                                <a href="{root_prefix}artists/">Artists</a>
                            </nav>
                        </header>
                        <main>
                            {body}
                        </main>
                        <footer>
                            <span>2021 faircamp alpha</span>
                            <a href=".">^ top</a>
                        </footer>
                    </div>
                </body>
            </html>
        "#,
        body=body,
        root_prefix=("../".repeat(page_depth)),
        title=title
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
                Some(image) => format!(r#"<img class="cover" src="../{}.jpg">"#, image.uuid),
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
            <h1>{artist_name}</h1>
            <div class="releases">
                {releases}
            </div>
        "#,
        artist_name=artist.name,
        releases=releases_rendered
    );
    
    layout(1, &body, &artist.name)
}

pub fn render_artists(catalog: &Catalog) -> String {
    let artists_rendered = catalog.artists
        .iter()
        .map(|artist| {
            let artist_cover_rendered = match &artist.image {
                Some(image) => format!(r#"<img class="cover" src="{}.jpg">"#, image.uuid),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            formatdoc!(
                r#"
                    <div>
                        {artist_cover_rendered}
                        <a href="../{artist_slug}/">{artist_name}</a>
                    </div>
                "#,
                artist_cover_rendered=artist_cover_rendered,
                artist_slug=artist.slug,
                artist_name=artist.name
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    let body = formatdoc!(
        r#"
            <h1>Artists</h1>
            <div class="releases"> <!-- TODO: Generic class for the grid (or a specific "artists" class with similar behavior) -->
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
        Some(image) => format!(r#"<img class="cover" src="../../{}.jpg">"#, image.uuid),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    let body = formatdoc!(
        r#"
            {release_cover_rendered}
            
            <h1>Download {release_title}</h1>
            <div>by {artists_rendered}</div>
            
            <div><a download href="original.zip">Zip Archive</a></div>
        "#,
        artists_rendered=artists_rendered,
        release_cover_rendered=release_cover_rendered,
        release_title=release.title
    );
    
    layout(2, &body, &release.title)
}



pub fn render_release(release: &Release) -> String {
    let artists_rendered = list_artists(1, &release.artists);
    
    // TODO: Probably outsource that into impl DownloadOption (give it its own file I guess then)
    let download_option_rendered = match &release.download_option {
        DownloadOption::Disabled => String::new(),
        DownloadOption::Free(download_hash) => formatdoc!(
            r#"
                <div>
                    <a href="../download/{hash}/">Download Digital Release</a>
                    <div>{includes_text}</div>
                </div>
            "#,
            hash=download_hash,
            includes_text=DOWNLOAD_INCLUDES_TEXT
        ),
        DownloadOption::NameYourPrice => formatdoc!(
            r#"
                <div>
                    <a href="../download/todo">Buy Digital Release</a> Name Your Price
                    <div>{includes_text} {paying_text}</div>
                </div>
            "#,
            includes_text=DOWNLOAD_INCLUDES_TEXT,
            paying_text=PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayExactly(price) => formatdoc!(
            r#"
                <div>
                    <a href="../download/todo">Buy Digital Release</a> {price}
                    <div>{includes_text} {paying_text}</div>
                </div>
            "#,
            price=price,
            includes_text=DOWNLOAD_INCLUDES_TEXT,
            paying_text=PAYING_SUPPORTERS_TEXT
        ),
        DownloadOption::PayMinimum(price) => formatdoc!(
            r#"
                <div>
                    <a href="../download/todo">Buy Digital Release</a> {price} or more
                    <div>{includes_text} {paying_text}</div>
                </div>
            "#,
            price=price,
            includes_text=DOWNLOAD_INCLUDES_TEXT,
            paying_text=PAYING_SUPPORTERS_TEXT
        )
    };
    
    let release_cover_rendered = match &release.cover {
        Some(image) => format!(r#"<img class="cover" src="../{}.jpg">"#, image.uuid),
        None => String::from(r#"<div class="cover"></div>"#)
    };
    
    let tracks_rendered = release.tracks
        .iter()
        .enumerate()
        .map(|(index, track)|
            formatdoc!(
                r#"
                    {track_number}. {track_title} <audio controls src=\"../{track_src}\"></audio> {track_duration}
                "#,
                track_duration=track.duration_formatted(),
                track_number=index + 1,
                track_src=format!("{}.mp3", track.uuid),
                track_title=track.title
            )
        )
        .collect::<Vec<String>>()
        .join("<br><br>\n");

    let body = formatdoc!(
        r#"
            <h1>{release_title}</h1>
            <div>by {artists_rendered}</div>
            
            {download_option_rendered}
            
            {release_cover_rendered}

            <div>{tracks}</div>
            
            <div>{release_text}</div>
        "#,
        artists_rendered=artists_rendered,
        download_option_rendered=download_option_rendered,
        release_cover_rendered=release_cover_rendered,
        release_text=release.text.as_ref().unwrap_or(&String::new()),
        release_title=release.title,
        tracks=tracks_rendered
    );
    
    layout(1, &body, &release.title)
}

pub fn render_releases(catalog: &Catalog) -> String {
    let releases_rendered = catalog.releases
        .iter()
        .map(|release| {
            let release_cover_rendered = match &release.cover {
                Some(image) => format!(r#"<img class="cover" src="{}.jpg">"#, image.uuid),
                None => String::from(r#"<div class="cover"></div>"#)
            };
            
            formatdoc!(
                r#"
                    <div>
                        {release_cover_rendered}
                        <a href="{release_slug}/">{release_title}</a>
                    </div>
                "#,
                release_cover_rendered=release_cover_rendered,
                release_slug=release.slug,
                release_title=release.title
            )
        })
        .collect::<Vec<String>>()
        .join("<br><br>\n");
    
    let body = formatdoc!(
        r#"
            <h1>Catalog</h1>
            <div class="releases">
                {releases}
            </div>
        "#,
        releases=releases_rendered
    );
    
    layout(0, &body, "Catalog")
}