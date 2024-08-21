// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::Catalog;

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
			.map(|artist| format!("\n- {}", artist.borrow().name))
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

	let r_releases = match catalog.releases.is_empty() {
		true => String::from("Empty"),
		false => catalog.releases
			.iter()
			.map(|release| {
				let release_ref = release.borrow();
				let r_main_artists = match release_ref.main_artists.is_empty() {
					true => String::from("Empty"),
					false => release_ref.main_artists
						.iter()
						.map(|artist| format!("{}", artist.borrow().name))
						.collect::<Vec<String>>()
						.join(", ")
				};
				let r_support_artists = match release_ref.support_artists.is_empty() {
					true => String::from("Empty"),
					false => release_ref.support_artists
						.iter()
						.map(|artist| format!("{}", artist.borrow().name))
						.collect::<Vec<String>>()
						.join(", ")
				};

				format!("\n- Title: {}\n  Main Artists: {r_main_artists}\n  Support Artists: {r_support_artists}", release_ref.title)
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
		Artists: {r_artists}

		Featured Artists: {r_featured_artists}

		Catalog artist: {r_catalog_artist}

		Releases: {r_releases}

		Support Artists: {r_support_artists}
	"#);

	println!("{output}");
}