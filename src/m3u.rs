// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// M3U format reference:
/// - https://en.wikipedia.org/wiki/M3U
/// - https://docs.fileformat.com/audio/m3u/

use std::hash::Hash;

use indoc::formatdoc;

use crate::{
    Artist,
    Build,
    Catalog,
    Release,
    SiteUrl,
    Track,
    TRACK_NUMBERS
};

pub const M3U_PLAYLIST_FILENAME: &str = "playlist.m3u";

/// Generate complete content of an M3U playlist for all (public) releases of
/// an artist.
pub fn generate_for_artist(
    artist: &Artist,
    base_url: &SiteUrl,
    build: &Build
) -> String {
    let r_releases = artist.public_releases()
        .iter()
        .map(|release| {
            let release_ref = release.borrow();
            let release_slug = &release_ref.permalink.slug;
            let release_title = &release_ref.title;

            let r_tracks = generate_tracks(
                base_url,
                build,
                &release_ref,
                &release_ref.tracks
            );

            let release_cover_url = match &release_ref.cover {
                Some(described_image) => {
                    let image_ref = described_image.borrow();
                    let file_name = image_ref.cover_assets_unchecked().playlist_image();
                    let hash = image_ref.hash.as_url_safe_base64();

                    base_url.join_file(format!("{release_slug}/{file_name}?{hash}"))
                }
                None => {
                    let file_name = release_ref.procedural_cover_480_filename_unchecked();
                    base_url.join_file(format!("{release_slug}/{file_name}"))
                }
            };

            formatdoc!(r#"
                #EXTIMG:{release_cover_url}
                #EXTALB:{release_title}
                {r_tracks}
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let artist_extimg = match &artist.image {
        Some(described_image) => {
            let artist_slug = &artist.permalink.slug;
            let image_ref = described_image.borrow();
            let file_name = image_ref.artist_assets.as_ref().unwrap().playlist_image();
            let hash = image_ref.hash.as_url_safe_base64();

            let artist_image_url = base_url.join_file(format!("{artist_slug}/{file_name}?{hash}"));

            format!("#EXTIMG:{artist_image_url}")
        }
        None => String::new()
    };

    let artist_name = &artist.name;

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
pub fn generate_for_catalog(build: &Build, catalog: &Catalog) -> String {
    let base_url = build.base_url_unchecked();
    let catalog_title = catalog.title();

    let r_releases = catalog.public_releases()
        .iter()
        .map(|release| {
            let release_ref = release.borrow();
            let release_slug = &release_ref.permalink.slug;
            let release_title = &release_ref.title;

            let r_tracks = generate_tracks(
                base_url,
                build,
                &release_ref,
                &release_ref.tracks
            );

            let release_cover_url = match &release_ref.cover {
                Some(described_image) => {
                    let image_ref = described_image.borrow();
                    let file_name = image_ref.cover_assets_unchecked().playlist_image();
                    let hash = image_ref.hash.as_url_safe_base64();

                    base_url.join_file(format!("{release_slug}/{file_name}?{hash}"))
                }
                None => {
                    let file_name = release_ref.procedural_cover_480_filename_unchecked();
                    base_url.join_file(format!("{release_slug}/{file_name}"))
                }
            };

            formatdoc!(r#"
                #EXTIMG:{release_cover_url}
                #EXTALB:{release_title}
                {r_tracks}
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");


    let catalog_extimg = match &catalog.home_image {
        Some(described_image) => {
            let image_ref = described_image.borrow();
            let file_name = image_ref.artist_assets.as_ref().unwrap().playlist_image();
            let hash = image_ref.hash.as_url_safe_base64();

            let file_url = base_url.join_file(format!("{file_name}?{hash}"));

            format!("#EXTIMG:{file_url}")
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
pub fn generate_for_release(
    base_url: &SiteUrl,
    build: &Build,
    release: &Release
) -> String {
    let release_slug = &release.permalink.slug;
    let release_title = &release.title;

    let r_tracks = generate_tracks(
        base_url,
        build,
        release,
        &release.tracks
    );

    let release_cover_url = match &release.cover {
        Some(described_image) => {
            let image_ref = described_image.borrow();
            let file_name = image_ref.cover_assets_unchecked().playlist_image();
            let hash = image_ref.hash.as_url_safe_base64();

            base_url.join_file(format!("{release_slug}/{file_name}?{hash}"))
        }
        None => {
            let file_name = release.procedural_cover_480_filename_unchecked();
            base_url.join_file(format!("{release_slug}/{file_name}"))
        }
    };

    formatdoc!(r#"
        #EXTM3U
        #EXTENC:UTF-8
        #PLAYLIST:{release_title}
        #EXTIMG:{release_cover_url}
        #EXTALB:{release_title}
        {r_tracks}
    "#)
}

/// Generate M3U playlist content just for the tracks of a release, to be used
/// as a reusable function for generating either a playlist for an release or
/// for an entire catalog (multiple releases).
pub fn generate_tracks(
    base_url: &SiteUrl,
    build: &Build,
    release: &Release,
    tracks: &[Track]
) -> String {
    let release_slug = &release.permalink.slug;

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
                release_slug.hash(hasher);
                track_number.hash(hasher);
                format_dir.hash(hasher);
                track_filename.hash(hasher);
            });

            let track_filename_urlencoded = urlencoding::encode(&track_filename);
            let file_url = base_url.join_file(
                format!("{release_slug}/{track_number}/{format_dir}/{track_hash}/{track_filename_urlencoded}")
            );

            format!("{extinf}\n{file_url}")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
