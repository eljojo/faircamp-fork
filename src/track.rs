// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::PathBuf;

use crate::{
    ArtistRc,
    Asset,
    AssetIntent,
    AudioFormat,
    Build,
    ffmpeg,
    HeuristicAudioMeta,
    HtmlAndStripped,
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
    pub heuristic_audio_meta: Option<HeuristicAudioMeta>,
    pub text: Option<HtmlAndStripped>,
    pub theme: Theme,
    pub transcodes: TranscodesRcView
}

impl Track {
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
    
    pub fn new(
        artists_to_map: Vec<String>,
        copy_link: bool,
        theme: Theme,
        transcodes: TranscodesRcView
    ) -> Track {
        Track {
            artists: Vec::new(),
            artists_to_map,
            asset_basename: None,
            copy_link,
            heuristic_audio_meta: None,
            // TODO: Wire up with manifests so it can be set
            text: None,
            transcodes,
            theme
        }
    }

    pub fn title(&self) -> String {
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
