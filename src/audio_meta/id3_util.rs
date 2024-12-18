// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use id3::{Tag, TagLike, Version};

use super::trim_and_reject_empty;

pub struct Id3Util<'a> {
    tag: &'a Tag
}

impl Id3Util<'_> {
    pub fn album(&self) -> Option<String> {
        match self.tag.album() {
            Some(album) => self.patched_trim_and_reject_empty(album),
            None => None
        }
    }

    pub fn album_artists(&self) -> Vec<String> {
        match self.tag.album_artist() {
            Some(album_artist) => match self.patched_trim_and_reject_empty(album_artist) {
                Some(album_artist) => vec![album_artist],
                None => Vec::new()
            },
            None => Vec::new()
        }
    }

    pub fn artists(&self) -> Vec<String> {
        match self.tag.artists() {
            Some(artists) => artists
                .iter()
                .filter_map(|artist| self.patched_trim_and_reject_empty(artist))
                .collect(),
            None => Vec::new()
        }
    }


    pub fn new(tag: &Tag) -> Id3Util {
        Id3Util {
            tag
        }
    }

    /// Due to a bug in the id3 crate, in ID3v2.2 and ID3v2.3 tags
    /// the character '/' (slash) is replaced with '\0' (null byte).
    /// The issue is a bit more complex than that, hence unresolved,
    /// but as a practical workaround we are for the time being re-
    /// replacing '\0' with '/' when we encounter it. A bugreport
    /// for the underlying issue is found at the following url:
    /// https://github.com/polyfloyd/rust-id3/issues/103
    pub fn patched_trim_and_reject_empty(&self, string: &str) -> Option<String> {
        match self.tag.version() {
            Version::Id3v22 |
            Version::Id3v23 => {
                let repaired_string = string.replace('\0', "/");
                trim_and_reject_empty(&repaired_string)
            }
            Version::Id3v24 => trim_and_reject_empty(string)
        }
    }

    pub fn title(&self) -> Option<String> {
        match self.tag.title() {
            Some(title) => self.patched_trim_and_reject_empty(title),
            None => None
        }
    }
}
