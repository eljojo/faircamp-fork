// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_derive::{Deserialize, Serialize};

use crate::{ArtistRc, Release, Track};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct TagMapping {
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    /// Track number
    pub track: Option<usize>
}

impl TagMapping {
    pub fn new(
        release: &Release,
        track: &Track,
        track_number: usize
    ) -> Option<TagMapping> {
        if !release.rewrite_tags { return None; }

        let same_track_artists = release.tracks
            .iter()
            .all(|track| {
                track.artists
                    .iter()
                    .zip(release.main_artists.iter())
                    .all(|(track_artist, main_artist)| ArtistRc::ptr_eq(track_artist, main_artist))
            });

        let album = Some(release.title.clone());

        let album_artist = match !release.main_artists.is_empty() && !same_track_artists {
            true => Some(
                release.main_artists
                .iter()
                .map(|artist| artist.borrow().name.clone())
                .collect::<Vec<String>>()
                .join(", ")
            ),
            false => None
        };

        // TODO: If there are no track artists, should we use release.main_artists instead?
        let artist = match !track.artists.is_empty() {
            true => Some(
                track.artists
                .iter()
                .map(|artist| artist.borrow().name.clone())
                .collect::<Vec<String>>()
                .join(", ")
            ),
            false => None
        };

        let title = Some(track.title());

        // TODO: Maybe rethink this one (also with new additions of heuristic audio meta)
        // This does intentionally not (directly) utilize track number metadata
        // gathered from the original audio files, here's why:
        // - If all tracks came with track number metadata, the tracks will have
        //   been sorted by it, and hence we arrive at the same result anyway (except
        //   if someone supplied track number metadata that didn't regularly go from
        //   1 to [n] in steps of 1, which is however quite an edge case and raises
        //   questions also regarding presentation on the release page itself.)
        // - If no track metadata was supplied, we here use the same order as has
        //   been determined when the Release is built (alphabetical)
        // - If there was a mix of tracks with track numbers and tracks without, it's
        //   going to be a bit of a mess (hard to do anything about it), but this will
        //   also show on the release page itself already
        let track = Some(track_number);

        let tag_mapping = TagMapping {
            album,
            album_artist,
            artist,
            title,
            track
        };

        Some(tag_mapping)
    }
}
