// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use chrono::NaiveDate;
use enolib::prelude::*;
use enolib::{Attribute, Item};

use crate::{
    DescribedImage,
    DownloadAccessOption,
    DownloadFormat,
    ExtraDownloads,
    HtmlAndStripped,
    Link,
    Permalink,
    Price,
    StreamingQuality,
    TagAgenda,
    Theme,
    TrackNumbering
};

const MAX_SYNOPSIS_CHARS: usize = 256;

mod artist;
mod artist_catalog_release;
mod artist_catalog_release_track;
mod artist_release;
mod catalog;
mod catalog_release;
mod obsolete;
mod release;
mod release_track;
mod track;

pub use artist::read_artist_manifest;
pub use artist_catalog_release::{
    ARTIST_CATALOG_RELEASE_OPTIONS,
    read_artist_catalog_release_option
};
pub use artist_catalog_release_track::{
    ARTIST_CATALOG_RELEASE_TRACK_OPTIONS,
    read_artist_catalog_release_track_option
};
pub use artist_release::{
    ARTIST_RELEASE_OPTIONS,
    read_artist_release_option
};
pub use catalog::read_catalog_manifest;
pub use catalog_release::{
    CATALOG_RELEASE_OPTIONS,
    read_catalog_release_option
};
pub use obsolete::{read_obsolete_option, read_obsolete_theme_attribute};
pub use release::read_release_manifest;
pub use release_track::{
    RELEASE_TRACK_OPTIONS,
    read_release_track_option
};
pub use track::read_track_manifest;

/// Options specified in a manifest that only apply to everything found in the
/// same folder as the manifest. For instance a permalink can only uniquely
/// apply to one artist or release, thus it is a local option only.
#[derive(Clone)]
pub struct LocalOptions {
    /// Used by release and track
    pub cover: Option<DescribedImage>,
    pub links: Vec<Link>,
    /// Used by artist, release and track
    pub more: Option<HtmlAndStripped>,
    /// Used by artist and release
    pub permalink: Option<Permalink>,
    pub release_date: Option<NaiveDate>,
    /// Used by artist, release and track
    pub synopsis: Option<String>,
    /// Used by release and track
    pub title: Option<String>,
    pub unlisted_release: bool
}

/// Options specified in a manifest that apply to everything in the same
/// folder, but which are also passed down and applied to child folders
/// (unless overriden there once again). For instance one might enable
/// downloads in a manifest in the root folder of the catalog, this would
/// apply to everything in the catalog then, however one can also disable it
/// in a manifest further down the hierarchy, hence it is an override.
#[derive(Clone)]
pub struct Overrides {
    pub copy_link: bool,
    pub download_codes: Vec<String>,
    pub embedding: bool,
    pub m3u_enabled: bool,
    pub more_label: Option<String>,
    pub payment_info: Option<String>,
    pub release_artists: Vec<String>,
    pub release_download_access: DownloadAccessOption,
    pub release_downloads: Vec<DownloadFormat>,
    pub release_extras: ExtraDownloads,
    pub release_price: Price,
    pub speed_controls: bool,
    pub streaming_quality: StreamingQuality,
    pub tag_agenda: TagAgenda,
    pub theme: Theme,
    pub track_artists: Vec<String>,
    pub track_download_access: DownloadAccessOption,
    pub track_downloads: Vec<DownloadFormat>,
    pub track_extras: bool,
    pub track_numbering: TrackNumbering,
    pub track_price: Price,
    pub unlock_info: Option<String>
}

impl LocalOptions {
    pub fn new() -> LocalOptions {
        LocalOptions {
            cover: None,
            links: Vec::new(),
            more: None,
            permalink: None,
            release_date: None,
            synopsis: None,
            title: None,
            unlisted_release: false
        }
    }
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            copy_link: true,
            download_codes: Vec::new(),
            embedding: false,
            m3u_enabled: false,
            more_label: None,
            payment_info: None,
            release_artists: Vec::new(),
            release_download_access: DownloadAccessOption::Free,
            release_downloads: Vec::new(),
            release_extras: ExtraDownloads::BUNDLED,
            release_price: Price::default(),
            speed_controls: false,
            streaming_quality: StreamingQuality::Standard,
            tag_agenda: TagAgenda::normalize(),
            theme: Theme::new(),
            track_artists: Vec::new(),
            track_download_access: DownloadAccessOption::Free,
            track_downloads: Vec::new(),
            track_extras: true,
            track_numbering: TrackNumbering::ArabicDotted,
            track_price: Price::default(),
            unlock_info: None
        }
    }
}

fn attribute_error_with_snippet(
    attribute: &Attribute,
    manifest_path: &Path,
    error: &str
) -> String {
    let snippet = attribute.snippet();
    format!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), attribute.line_number, snippet, error)
}

fn element_error_with_snippet(
    element: &Box<dyn SectionElement>,
    manifest_path: &Path,
    error: &str
) -> String {
    let snippet = element.snippet();
    format!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), element.line_number(), snippet, error)
}

fn item_error_with_snippet(
    item: &Item,
    manifest_path: &Path,
    error: &str
) -> String {
    let snippet = item.snippet();
    format!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), item.line_number, snippet, error)
}

fn not_supported_error(
    manifest_name: &str,
    option_key: &str,
    supported_options_groups: &[&[&str]]
) -> String {
    let mut supported_options_sorted = Vec::new();

    for supported_option_group in supported_options_groups {
        for option in *supported_option_group {
            supported_options_sorted.push(*option);
        }
    }

    supported_options_sorted.sort();

    let r_supported_options = supported_options_sorted.join(", ");

    format!("This '{option_key}' option was not recognized (check for typos and that the option is supported inside a(n) {manifest_name} manifest).\n\nInside this manifest ({manifest_name}) the following keys are supported:\n{r_supported_options}")
}

#[cfg(not(target_os = "windows"))]
fn platform_printer() -> Box<enolib::TerminalPrinter> {
    Box::new(enolib::TerminalPrinter)
}

// TODO: Replace with terminal capability based approach, this is
//       just to temporarily fix the situation in Windows command prompt.
#[cfg(target_os = "windows")]
fn platform_printer() -> Box<enolib::TextPrinter> {
    Box::new(enolib::TextPrinter)
}
