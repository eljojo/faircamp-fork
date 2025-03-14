// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_derive::{Deserialize, Serialize};

use crate::{
    ArtistRc,
    Release,
    SourceHash,
    Track
};

/// This is the final mapping of a cover image to be embedded into an output audio file.
/// It only stores a source hash without any path information because faircamp anyway
/// knows from where to get the cover image (= from the release struct).
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum ImageEmbed {
    Copy,
    Write(SourceHash)
}

/// Set behavior for a single tag:
/// Copy - Copy 1:1 from source audio file
/// Remove - Leave out in output audio file
/// Rewrite - Rewrite from explicit/implicit data that faircamp gathers (or copy)
#[derive(Clone, Debug)]
pub enum TagAction {
    Copy,
    Remove,
    Rewrite
}

/// Set behavior for all tags:
/// Copy - Copy all tags 1:1 from source audio file
/// Custom - Define behavior on a per-tag basis (see [TagAction])
/// Remove - Write no tags at all to the output file
#[derive(Clone, Debug)]
pub enum TagAgenda {
    Copy,
    Custom {
        album: TagAction,
        album_artist: TagAction,
        artist: TagAction,
        image: TagAction,
        title: TagAction,
        track: TagAction
    },
    Remove
}

/// Based on [TagAgenda] this is the final mapping of concrete values
/// that are written to an output audio file, or in the case of copying/removing
/// the abstract instruction to copy/remove all of them.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum TagMapping {
    Copy,
    Custom {
        album: Option<String>,
        album_artist: Option<String>,
        artist: Option<String>,
        image: Option<ImageEmbed>,
        title: Option<String>,
        /// Track number
        track: Option<usize>
    },
    Remove
}

impl TagAction {
    pub fn from_key(key: &str) -> Result<TagAction, String> {
        match key {
            "copy" => Ok(TagAction::Copy),
            "remove" => Ok(TagAction::Remove),
            "rewrite" => Ok(TagAction::Rewrite),
            _ => {
                let err = format!("Unknown tag action '{key}' - supported are 'copy', 'remove' and 'rewrite'");
                Err(err)
            }
        }
    }
}

impl TagAgenda {
    pub fn normalize() -> TagAgenda {
        TagAgenda::Custom {
            album: TagAction::Rewrite,
            album_artist: TagAction::Rewrite,
            artist: TagAction::Rewrite,
            image: TagAction::Remove,
            title: TagAction::Rewrite,
            track: TagAction::Rewrite
        }
    }

    pub fn set(&mut self, tag_key: &str, action_key: &str) -> Result<(), String> {
        match self {
            TagAgenda::Copy => {
                *self = TagAgenda::Custom {
                    album: TagAction::Copy,
                    album_artist: TagAction::Copy,
                    artist: TagAction::Copy,
                    image: TagAction::Copy,
                    title: TagAction::Copy,
                    track: TagAction::Copy
                }
            }
            TagAgenda::Custom { .. } => (),
            TagAgenda::Remove => {
                *self = TagAgenda::Custom {
                    album: TagAction::Remove,
                    album_artist: TagAction::Remove,
                    artist: TagAction::Remove,
                    image: TagAction::Remove,
                    title: TagAction::Remove,
                    track: TagAction::Remove
                }
            }
        }

        if let TagAgenda::Custom { album, album_artist, artist, image, title, track } = self {
            match TagAction::from_key(action_key) {
                Ok(action) => {
                    match tag_key {
                        "album" => *album = action,
                        "album_artist" => *album_artist = action,
                        "artist" => *artist = action,
                        "image" => *image = action,
                        "title" => *title = action,
                        "track" => *track = action,
                        _ => {
                            let err = format!("Unknown tag key '{tag_key}' - supported are 'album', 'album_artist', 'artist', 'image', 'title' and 'track'");
                            return Err(err);
                        }
                    }
                }
                Err(err) => return Err(err)
            }
        }

        Ok(())
    }
}

impl TagMapping {
    pub fn new(
        release: &Release,
        track: &Track,
        track_number: usize
    ) -> TagMapping {
        match &track.tag_agenda {
            TagAgenda::Copy => TagMapping::Copy,
            TagAgenda::Custom {
                album: album_action,
                album_artist: album_artist_action,
                artist: artist_action,
                image: image_action,
                title: title_actiion,
                track: track_action
            } => {
                let album_mapped = match album_action {
                    TagAction::Copy => track.transcodes.borrow().source_meta.album.clone(),
                    TagAction::Remove => None,
                    TagAction::Rewrite => Some(release.title.clone())
                };

                let album_artist_mapped = match album_artist_action {
                    TagAction::Copy => {
                        let transcodes_ref = track.transcodes.borrow();
                        match transcodes_ref.source_meta.album_artists.is_empty() {
                            true => None,
                            false => Some(transcodes_ref.source_meta.album_artists.join(", "))
                        }
                    }
                    TagAction::Remove => None,
                    TagAction::Rewrite => {
                        if release.main_artists.is_empty() ||
                            release.tracks.iter().all(|track| {
                                track.artists.len() == release.main_artists.len() &&
                                track.artists
                                    .iter()
                                    .zip(release.main_artists.iter())
                                    .all(|(track_artist, main_artist)| ArtistRc::ptr_eq(track_artist, main_artist))
                            }) {
                            // The album artist tag is not needed when ...
                            // - We don't know the release's main artist(s) (not sure if that can even be the case, but for correctness sake)
                            // - Each track is by the same artist(s), which are at the same time also the release's main artist(s)
                            None
                        } else {
                            let album_artists = release.main_artists
                                .iter()
                                .map(|artist| artist.borrow().name.clone())
                                .collect::<Vec<String>>()
                                .join(", ");

                            Some(album_artists)
                        }
                    }
                };

                let artist_mapped = match artist_action {
                    TagAction::Copy => {
                        let transcodes_ref = track.transcodes.borrow();
                        match transcodes_ref.source_meta.artists.is_empty() {
                            true => None,
                            false => Some(transcodes_ref.source_meta.artists.join(", "))
                        }
                    }
                    TagAction::Remove => None,
                    TagAction::Rewrite => {
                        // TODO: If there are no track artists, should we use release.main_artists instead?
                        match track.artists.is_empty() {
                            true => None,
                            false => Some(
                                track.artists
                                .iter()
                                .map(|artist| artist.borrow().name.clone())
                                .collect::<Vec<String>>()
                                .join(", ")
                            )
                        }
                    }
                };

                let image_mapped = match image_action {
                    TagAction::Copy => Some(ImageEmbed::Copy),
                    TagAction::Remove => None,
                    TagAction::Rewrite => {
                        if let Some(described_image) = &track.cover {
                            Some(ImageEmbed::Write(described_image.borrow().hash.clone()))
                        } else if let Some(described_image) = &release.cover {
                            Some(ImageEmbed::Write(described_image.borrow().hash.clone()))
                        } else {
                            None
                        }
                    }
                };

                let title_mapped = match title_actiion {
                    TagAction::Copy => track.transcodes.borrow().source_meta.title.clone(),
                    TagAction::Remove => None,
                    TagAction::Rewrite => Some(track.title())
                };

                let track_mapped = match track_action {
                    TagAction::Copy => track.transcodes.borrow().source_meta.track_number.map(|track_number| track_number as usize),
                    TagAction::Remove => None,
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
                    TagAction::Rewrite => Some(track_number)
                };

                TagMapping::Custom {
                    album: album_mapped,
                    album_artist: album_artist_mapped,
                    artist: artist_mapped,
                    image: image_mapped,
                    title: title_mapped,
                    track: track_mapped
                }
            }
            TagAgenda::Remove => TagMapping::Remove
        }
    }
}
