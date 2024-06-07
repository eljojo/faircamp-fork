// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    ArtistRc,
    Asset,
    AssetIntent,
    AudioFormat,
    Build,
    ffmpeg,
    HeuristicAudioMeta,
    TagMapping,
    Theme,
    TranscodesRc,
    util
};

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
    pub heuristic_audio_meta: Option<HeuristicAudioMeta>,
    pub theme: Theme,
    pub transcodes: TranscodesRc
}

impl Track {
    pub fn transcode_as(
        &mut self,
        format: AudioFormat,
        build: &Build,
        asset_intent: AssetIntent,
        tag_mapping_option: &Option<TagMapping>
    ) {
        let mut transcodes_mut = self.transcodes.borrow_mut();

        if let Some(asset) = transcodes_mut.get_mut(format) {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let target_filename = format!("{}{}", util::uid(), format.extension());

            info_transcoding!("{:?} to {}", transcodes_mut.source_file_signature.path, format);
            ffmpeg::transcode(
                &build.catalog_dir.join(&transcodes_mut.source_file_signature.path),
                &build.cache_dir.join(&target_filename),
                format,
                tag_mapping_option
            ).unwrap();

            transcodes_mut.get_mut(format).replace(Asset::new(build, target_filename, asset_intent));
        }
    }
    
    pub fn new(
        artists_to_map: Vec<String>,
        theme: Theme,
        transcodes: TranscodesRc
    ) -> Track {
        Track {
            artists: Vec::new(),
            artists_to_map,
            asset_basename: None,
            heuristic_audio_meta: None,
            transcodes,
            theme
        }
    }

    pub fn title(&self) -> String {
        let transcodes_ref = self.transcodes.borrow();
        if let Some(title) = &transcodes_ref.source_meta.title {
            title.clone()
        } else if let Some(heuristic_audio_meta) = &self.heuristic_audio_meta {
            heuristic_audio_meta.title.clone()
        } else {
            transcodes_ref.source_file_signature.path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
