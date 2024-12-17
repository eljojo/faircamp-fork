// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use chrono::NaiveDate;
use enolib::prelude::*;
use enolib::{Attribute, Item};

use crate::{
    DescribedImage,
    DownloadOption,
    DownloadsConfig,
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
mod artist_release;
mod catalog;
mod catalog_release;
mod obsolete;
mod release;

pub use artist::read_artist_manifest;
pub use artist_catalog_release::read_artist_catalog_release_option;
pub use artist_release::read_artist_release_option;
pub use catalog::read_catalog_manifest;
pub use catalog_release::read_catalog_release_option;
pub use obsolete::read_obsolete_option;
pub use release::read_release_manifest;

/// Options specified in a manifest that only apply to everything found in the
/// same folder as the manifest. For instance a permalink can only uniquely
/// apply to one artist or release, thus it is a local option only.
#[derive(Clone)]
pub struct LocalOptions {
    pub links: Vec<Link>,
    /// Applies to artist or release
    pub permalink: Option<Permalink>,
    pub release_date: Option<NaiveDate>,
    pub release_title: Option<String>,
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
    pub downloads: DownloadOption,
    pub downloads_config: DownloadsConfig,
    pub embedding: bool,
    pub m3u_enabled: bool,
    pub more_label: Option<String>,
    pub payment_info: Option<String>,
    pub price: Price,
    pub release_artists: Vec<String>,
    pub release_cover: Option<DescribedImage>,
    pub release_synopsis: Option<String>,
    pub release_text: Option<HtmlAndStripped>,
    pub streaming_quality: StreamingQuality,
    pub tag_agenda: TagAgenda,
    pub theme: Theme,
    pub track_artists: Vec<String>,
    pub track_numbering: TrackNumbering,
    pub unlock_info: Option<String>
}

impl LocalOptions {
    pub fn new() -> LocalOptions {
        LocalOptions {
            links: Vec::new(),
            release_date: None,
            permalink: None,
            release_title: None,
            unlisted_release: false
        }
    }
}

impl Overrides {
    pub fn default() -> Overrides {
        Overrides {
            copy_link: true,
            download_codes: Vec::new(),
            downloads: DownloadOption::Free,
            downloads_config: DownloadsConfig::default(),
            embedding: false,
            m3u_enabled: false,
            more_label: None,
            payment_info: None,
            price: Price::default(),
            release_artists: Vec::new(),
            release_cover: None,
            release_synopsis: None,
            release_text: None,
            streaming_quality: StreamingQuality::Standard,
            tag_agenda: TagAgenda::normalize(),
            theme: Theme::new(),
            track_artists: Vec::new(),
            track_numbering: TrackNumbering::ArabicDotted,
            unlock_info: None
        }
    }
}

fn attribute_error_with_snippet(
    attribute: &Attribute,
    manifest_path: &Path,
    error: &str
) {
    let snippet = attribute.snippet();
    error!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), attribute.line_number, snippet, error);
}

fn element_error_with_snippet(
    element: &Box<dyn SectionElement>,
    manifest_path: &Path,
    error: &str
) {
    let snippet = element.snippet();
    error!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), element.line_number(), snippet, error);
}

fn item_error_with_snippet(
    item: &Item,
    manifest_path: &Path,
    error: &str
) {
    let snippet = item.snippet();
    error!("Error in {}:{}:\n\n{}\n\n{}", manifest_path.display(), item.line_number, snippet, error);
}
