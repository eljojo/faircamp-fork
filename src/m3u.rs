// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// M3U format reference:
/// - https://en.wikipedia.org/wiki/M3U
/// - https://docs.fileformat.com/audio/m3u/

use std::hash::Hash;

use indoc::formatdoc;
use url::Url;

use crate::{
    Artist,
    Build,
    Catalog,
    Release,
    Track,
    TRACK_NUMBERS
};

/// Generate complete content of an M3U playlist for all (public) releases of
/// an artist.
pub fn generate_for_artist(base_url: &Url, build: &Build, artist: &Artist) -> String {
    let artist_url = base_url.join(&format!("{}/", artist.permalink.slug)).unwrap();
    let artist_name = &artist.name;

    let r_releases = artist.public_releases()
        .iter()
        .map(|release| {
            let release_ref = release.borrow();
            let release_url = base_url.join(&format!("{}/", release_ref.permalink.slug)).unwrap();
            let release_title = &release_ref.title;

            let r_tracks = generate_tracks(build, &release_ref, &release_url, &release_ref.tracks);

            let release_extimg = match &release_ref.cover {
                Some(described_image) => {
                    let image_ref = described_image.image.borrow();
                    let file_name = image_ref.cover_assets.as_ref().unwrap().playlist_image();
                    let file_url = release_url.join(&file_name).unwrap();
                    let hash = image_ref.hash.as_url_safe_base64();

                    format!("#EXTIMG:{file_url}?{hash}")
                }
                None => {
                    let procedural_cover = release_ref.procedural_cover.as_ref().unwrap();
                    let file_name = procedural_cover.borrow().filename_480();
                    let file_url = release_url.join(&file_name).unwrap();

                    format!("#EXTIMG:{file_url}")
                }
            };

            formatdoc!(r#"
                {release_extimg}
                #EXTALB:{release_title}
                {r_tracks}
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");


    let artist_extimg = match &artist.image {
        Some(described_image) => {
            let image_ref = described_image.image.borrow();
            let file_name = image_ref.artist_assets.as_ref().unwrap().playlist_image();
            let file_url = artist_url.join(&file_name).unwrap();
            let hash = image_ref.hash.as_url_safe_base64();

            format!("#EXTIMG:{file_url}?{hash}")
        }
        None => String::new()
    };

    formatdoc!(r#"
        #EXTM3U
        #EXTENC:UTF-8
        #PLAYLIST:{artist_name}
        {artist_extimg}
        {r_releases}
    "#)
}

/// Generate complete content of an M3U playlist for all (public) releases of
/// the catalog.
pub fn generate_for_catalog(base_url: &Url, build: &Build, catalog: &Catalog) -> String {
    let catalog_title = catalog.title();

    let r_releases = catalog.public_releases()
        .iter()
        .map(|release| {
            let release_ref = release.borrow();
            let release_url = base_url.join(&format!("{}/", release_ref.permalink.slug)).unwrap();
            let release_title = &release_ref.title;

            let r_tracks = generate_tracks(build, &release_ref, &release_url, &release_ref.tracks);

            let release_extimg = match &release_ref.cover {
                Some(described_image) => {
                    let image_ref = described_image.image.borrow();
                    let file_name = image_ref.cover_assets.as_ref().unwrap().playlist_image();
                    let file_url = release_url.join(&file_name).unwrap();
                    let hash = image_ref.hash.as_url_safe_base64();

                    format!("#EXTIMG:{file_url}?{hash}")
                }
                None => {
                    let procedural_cover = release_ref.procedural_cover.as_ref().unwrap();
                    let file_name = procedural_cover.borrow().filename_480();
                    let file_url = release_url.join(&file_name).unwrap();

                    format!("#EXTIMG:{file_url}")
                }
            };

            formatdoc!(r#"
                {release_extimg}
                #EXTALB:{release_title}
                {r_tracks}
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");


    let catalog_extimg = match &catalog.home_image {
        Some(described_image) => {
            let image_ref = described_image.image.borrow();
            let file_name = image_ref.artist_assets.as_ref().unwrap().playlist_image();
            let file_url = base_url.join(&file_name).unwrap();
            let hash = image_ref.hash.as_url_safe_base64();

            format!("#EXTIMG:{file_url}?{hash}")
        }
        None => String::new()
    };

    formatdoc!(r#"
        #EXTM3U
        #EXTENC:UTF-8
        #PLAYLIST:{catalog_title}
        {catalog_extimg}
        {r_releases}
    "#)
}

/// Generate complete content of an M3U playlist for a release
pub fn generate_for_release(base_url: &Url, build: &Build, release: &Release) -> String {
    let release_url = base_url.join(&format!("{}/", release.permalink.slug)).unwrap();
    let release_title = &release.title;

    let r_tracks = generate_tracks(build, release, &release_url, &release.tracks);

    let extimg = match &release.cover {
        Some(described_image) => {
            let image_ref = described_image.image.borrow();
            let file_name = image_ref.cover_assets.as_ref().unwrap().playlist_image();
            let file_url = release_url.join(&file_name).unwrap();
            let hash = image_ref.hash.as_url_safe_base64();

            format!("#EXTIMG:{file_url}?{hash}")
        }
        None => {
            let procedural_cover = release.procedural_cover.as_ref().unwrap();
            let file_name = procedural_cover.borrow().filename_480();
            let file_url = release_url.join(&file_name).unwrap();

            format!("#EXTIMG:{file_url}")
        }
    };

    formatdoc!(r#"
        #EXTM3U
        #EXTENC:UTF-8
        #PLAYLIST:{release_title}
        {extimg}
        #EXTALB:{release_title}
        {r_tracks}
    "#)
}

/// Generate M3U playlist content just for the tracks of a release, to be used
/// as a reusable function for generating either a playlist for an release or
/// for an entire catalog (multiple releases).
pub fn generate_tracks(
    build: &Build,
    release: &Release,
    release_url: &Url,
    tracks: &[Track]
) -> String {
    tracks
        .iter()
        .zip(TRACK_NUMBERS)
        .map(|(track, track_number)| {
            let track_number_formatted = release.track_numbering.format(track_number);

            let artists = track.artists
                .iter()
                .map(|artist| artist.borrow().name.clone())
                .collect::<Vec<String>>()
                .join(", ");

            let track_title = track.title();
            let title = match track_number_formatted.is_empty() {
                true => format!("{artists} – {track_title}"),
                false => format!("{artists} – {track_number_formatted} {track_title}")
            };

            let duration_seconds = track.transcodes.borrow().source_meta.duration_seconds as usize;

            let extinf = format!("#EXTINF:{duration_seconds}, {title}");

            let primary_streaming_format = track.streaming_quality.formats()[0];
            let format_dir = primary_streaming_format.asset_dirname();
            let format_extension = primary_streaming_format.extension();

            let track_filename = format!(
                "{basename}{format_extension}",
                basename = track.asset_basename.as_ref().unwrap()
            );

            let track_hash = build.hash_with_salt(|hasher| {
                release.permalink.slug.hash(hasher);
                track_number.hash(hasher);
                format_dir.hash(hasher);
                track_filename.hash(hasher);
            });

            let track_filename_urlencoded = urlencoding::encode(&track_filename);
            let src = format!("{track_number}/{format_dir}/{track_hash}/{track_filename_urlencoded}");
            let file_url = release_url.join(&src).unwrap();

            format!("{extinf}\n{file_url}")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
