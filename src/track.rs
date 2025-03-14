// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::PathBuf;

use crate::{
    ArtistRc,
    Asset,
    AssetIntent,
    AudioFormat,
    Build,
    DescribedImage,
    DownloadAccess,
    DownloadFormat,
    Extra,
    ffmpeg,
    HeuristicAudioMeta,
    HtmlAndStripped,
    Link,
    StreamingQuality,
    TagAgenda,
    TagMapping,
    Theme,
    Transcode,
    TranscodesRcView,
    util
};
use crate::util::generic_hash;

#[derive(Debug)]
pub struct Track {
    /// The final mapped artists (including metadata). Used in assembling the final page.
    pub artists: Vec<ArtistRc>,
    /// Names/aliases that should be mapped to this track, coming from the
    /// audio file metadata or from manifest overrides. Only relevant as an
    /// intermediate step before actual mapping.
    pub artists_to_map: Vec<String>,
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download/stream asset filenames.
    pub asset_basename: Option<String>,
    pub copy_link: bool,
    pub cover: Option<DescribedImage>,
    pub download_access: DownloadAccess,
    pub download_formats: Vec<DownloadFormat>,
    pub embedding: bool,
    pub extra_downloads: bool,
    pub extras: Vec<Extra>,
    // TODO: Re-check if we need this post-creation (if not we don't need to store it on Track)
    pub heuristic_audio_meta: Option<HeuristicAudioMeta>,
    pub links: Vec<Link>,
    pub more: Option<HtmlAndStripped>,
    /// Optional custom label for the button that (by default) says "More" on the
    /// track page and points to additional long-form content for the track.
    /// For tracks this label is also displayed in the track list on a release page.
    pub more_label: Option<String>,
    pub streaming_quality: StreamingQuality,
    pub synopsis: Option<String>,
    /// Describes if/how audio file tags (metadata) should be written to the
    /// transcoded track assets (e.g. copying original tags, removing all tags,
    /// or some other specified behavior).
    pub tag_agenda: TagAgenda,
    pub theme: Theme,
    /// An explicit title coming from the manifest, but this can also be
    /// missing and instead be given by audio file metadata, or the audio
    /// file name itself (either as heuristic audio meta or taking the raw
    /// file name).
    title: Option<String>,
    pub transcodes: TranscodesRcView
}

impl Track {
    /// Returns - if available - the file name of the track cover,
    /// without any prefixing (i.e. in the context of the track directory)
    pub fn cover_image_micro_src(&self) -> Option<String> {
        self.cover
            .as_ref()
            .map(|described_image| {
                let image_ref = described_image.borrow();
                let asset = &image_ref.cover_assets.as_ref().unwrap().max_160;
                let filename = asset.target_filename();
                let hash = image_ref.hash.as_url_safe_base64();
                format!("{filename}?{hash}")
            })
    }

    pub fn download_assets_available(&self) -> bool {
        !self.download_formats.is_empty() ||
        (self.extra_downloads && !self.extras.is_empty())
    }

    pub fn new(
        artists_to_map: Vec<String>,
        copy_link: bool,
        cover: Option<DescribedImage>,
        download_access: DownloadAccess,
        download_formats: Vec<DownloadFormat>,
        embedding: bool,
        extra_downloads: bool,
        extras: Vec<Extra>,
        links: Vec<Link>,
        more: Option<HtmlAndStripped>,
        more_label: Option<String>,
        streaming_quality: StreamingQuality,
        synopsis: Option<String>,
        tag_agenda: TagAgenda,
        theme: Theme,
        title: Option<String>,
        transcodes: TranscodesRcView
    ) -> Track {
        Track {
            artists: Vec::new(),
            artists_to_map,
            asset_basename: None,
            copy_link,
            cover,
            download_access,
            download_formats,
            embedding,
            extra_downloads,
            extras,
            heuristic_audio_meta: None,
            links,
            more,
            more_label,
            streaming_quality,
            synopsis,
            tag_agenda,
            title,
            transcodes,
            theme
        }
    }

    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => {
                if let Some(title) = &self.transcodes.borrow().source_meta.title {
                    title.clone()
                } else if let Some(heuristic_audio_meta) = &self.heuristic_audio_meta {
                    heuristic_audio_meta.title.clone()
                } else {
                    self.transcodes.file_meta.path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            }
        }
    }

    pub fn transcode_as(
        &mut self,
        target_format: AudioFormat,
        build: &Build,
        asset_intent: AssetIntent,
        tag_mapping: &TagMapping,
        cover_path: &Option<PathBuf>
    ) {
        let mut transcodes_mut = self.transcodes.borrow_mut();

        if let Some(transcode) = transcodes_mut.get_mut(target_format, generic_hash(tag_mapping)) {
            if asset_intent == AssetIntent::Deliverable {
                transcode.asset.unmark_stale();
            }
        } else {
            let target_filename = format!("{}{}", util::uid(), target_format.extension());

            info_transcoding!("{:?} to {}", self.transcodes.file_meta.path, target_format);
            ffmpeg::transcode(
                cover_path,
                &build.catalog_dir.join(&self.transcodes.file_meta.path),
                &build.cache_dir.join(&target_filename),
                transcodes_mut.source_meta.format_family,
                target_format,
                tag_mapping
            ).unwrap();

            let asset = Asset::new(build, target_filename, asset_intent);
            transcodes_mut.formats.push(Transcode::new(asset, target_format, generic_hash(tag_mapping)));
        }
    }
}
