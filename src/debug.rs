// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{
    ArtistRc,
    Catalog,
    DownloadAccess
};

fn format_artist(artist: &ArtistRc) -> String {
    let artist_ref = artist.borrow();

    if let Some(link) = &artist_ref.external_page {
        format!("{} (External page: {})", artist_ref.name, link)
    } else {
        format!("{} (Permalink: {})", artist_ref.name, artist_ref.permalink.slug)
    }
}

/// Prints debug information, e.g. to gain an understanding of how the catalog
/// files map to faircamp's internally generated data model.
pub fn debug_catalog(catalog: &Catalog) {
    let r_catalog_artist = match &catalog.artist {
        Some(artist) => artist.borrow().name.clone(),
        None => String::from("None")
    };

    let r_artists = match catalog.artists.is_empty() {
        true => String::from("Empty"),
        false => catalog.artists
            .iter()
            .map(|artist| format!("\n- {}", format_artist(artist)))
            .collect::<Vec<String>>()
            .join("")
    };

    let r_featured_artists = match catalog.featured_artists.is_empty() {
        true => String::from("Empty"),
        false => catalog.featured_artists
            .iter()
            .map(|artist| format!("\n- {}", artist.borrow().name))
            .collect::<Vec<String>>()
            .join("")
    };

    let r_feature_support_artists = if catalog.feature_support_artists { "Enabled" } else { "Disabled" };
    let r_label_mode = if catalog.label_mode { "Enabled" } else { "Disabled" };

    let r_releases = match catalog.releases.is_empty() {
        true => String::from("Empty"),
        false => catalog.releases
            .iter()
            .map(|release| {
                let release_ref = release.borrow();

                let r_download_access = match &release_ref.download_access {
                    DownloadAccess::Disabled => String::from("Disabled"),
                    DownloadAccess::Code { .. } => String::from("Code"),
                    DownloadAccess::External { link } => format!("External ({link})"),
                    DownloadAccess::Free => String::from("Free"),
                    DownloadAccess::Paycurtain { .. } => String::from("Paycurtain")
                };

                let r_release_formats = format!("({} release formats)", release_ref.download_formats.len());
                // TODO: Somewhere restore "{} track formats" track.download_formats.len() information (Enumerate tracks probably!)

                let r_main_artists = match release_ref.main_artists.is_empty() {
                    true => String::from("Empty"),
                    false => release_ref.main_artists
                        .iter()
                        .map(|artist| artist.borrow().name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                };
                let r_support_artists = match release_ref.support_artists.is_empty() {
                    true => String::from("Empty"),
                    false => release_ref.support_artists
                        .iter()
                        .map(|artist| artist.borrow().name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                };

                format!("\n- Title: {}\n  Main Artists: {r_main_artists}\n  Support Artists: {r_support_artists}\n  Downloads: {r_download_access}\n Download Formats: {r_release_formats}", release_ref.title)
            })
            .collect::<Vec<String>>()
            .join("")
    };

    let r_support_artists = match catalog.support_artists.is_empty() {
        true => String::from("Empty"),
        false => catalog.support_artists
            .iter()
            .map(|artist| format!("\n- {}", artist.borrow().name))
            .collect::<Vec<String>>()
            .join("")
    };

    let output = formatdoc!(r#"
        Label mode: {r_label_mode}
        Feature support artists: {r_feature_support_artists}

        Artists: {r_artists}

        Featured Artists: {r_featured_artists}

        Catalog artist: {r_catalog_artist}

        Releases: {r_releases}

        Support Artists: {r_support_artists}
    "#);

    println!("{output}");
}