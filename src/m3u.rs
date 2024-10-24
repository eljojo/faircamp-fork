// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// M3U format reference:
/// - https://en.wikipedia.org/wiki/M3U
/// - https://docs.fileformat.com/audio/m3u/

use indoc::formatdoc;
use url::Url;

use crate::{Build, Release};

pub fn generate(base_url: &Url, build: &Build, release: &Release) -> String {
    let release_url = base_url.join(&format!("{}/", release.permalink.slug)).unwrap();

    let tracks = release.tracks
        .iter()
        .map(|track| {
            let artists = track.artists
                .iter()
                .map(|artist| artist.borrow().name.clone())
                .collect::<Vec<String>>()
                .join(", ");

            let title = format!("{artists} – {}", track.title());

            let duration_seconds = track.transcodes.borrow().source_meta.duration_seconds as usize;

            let extinf = format!("#EXTINF:{duration_seconds}, {title}");

            let primary_streaming_format = release.streaming_quality.formats()[0];
            let format_dir = primary_streaming_format.asset_dirname();
            let format_extension = primary_streaming_format.extension();

            let track_filename = format!(
                "{basename}{format_extension}",
                basename = track.asset_basename.as_ref().unwrap()
            );

            let track_hash = build.hash_path_with_salt(
                &release.permalink.slug,
                format_dir,
                &track_filename
            );

            let track_filename_urlencoded = urlencoding::encode(&track_filename);
            let src = format!("{format_dir}/{track_hash}/{track_filename_urlencoded}");
            let file_url = release_url.join(&src).unwrap();

            format!("{extinf}\n{file_url}")
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_title = &release.title;

    let extimg = match &release.cover {
        Some (described_image) => {
            let image_ref = described_image.image.borrow();
            let file_name = image_ref.cover_assets.as_ref().unwrap().playlist_image();
            let file_url = release_url.join(&file_name).unwrap();

            format!("#EXTIMG:{file_url}")
        }
        None => {
            String::new()
        }
    };

    formatdoc!(r#"
        #EXTM3U
        #EXTENC:UTF-8
        {extimg}
        #EXTALB:{release_title}
        {tracks}
    "#)
}
