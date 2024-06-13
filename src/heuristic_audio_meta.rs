// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde_derive::{Serialize, Deserialize};

use crate::Track;

/// This is initialized and subsequently fed one track after the other,
/// analyzing the track's filename for the most probable separator pattern
/// between track number and title. At the end the gathered metrics ("what
/// kind of separator pattern is found how often?") are used to determine
/// which separator pattern has most likely been used in the naming of the
/// files of an release. If the evidence is inconclusive (e.g. wildly
/// different separator patterns found), no separator pattern is assumed
/// and consequently no heuristic audio meta gathered from the filenames.
struct CommonSeparatorPattern {
    colon_space: usize,
    dot_space: usize,
    space: usize,
    space_dash_space: usize,
    total: usize,
    unseparated: usize
}

/// Describes a certain pattern for separating track number and filename.
/// Provides functionality for removing said pattern (falling back to most
/// likely similar patterns if there are errors, e.g. for SpaceDashSpace
/// the separator " - " will be removed, but if there happens to be no
/// "-" it will fall back to just removing the whitespace instead).
enum SeparatorPattern {
    /// "01:( )Example"
    ColonSpace,
    /// "01.( )Example"
    DotSpace,
    /// "01 Example"
    Space,
    /// "01( )-( )Example"
    SpaceDashSpace,
    /// "01Example"
    Unseparated
}

/// Limited metadata (only track number and title) that was determined from the name
/// of an audio file, e.g. "01 - Foo.mp3" could heuristically be interpreted to carry
/// the track number 1 and the title "Foo".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeuristicAudioMeta {
    pub title: String,
    pub track_number: u32
}

impl HeuristicAudioMeta {
    /// Analyzes all tracks of a release (looking only at each filename), and if
    /// it finds a reasonable pattern, extracts and stores heuristically determined
    /// metadata (track number and title) on each track.
    ///
    /// The criteria is roughly:
    /// - All filenames need to start with numbers such as "0", "00", "13", etc.
    /// - These numbers must start at 0 or 1 and increase monotonically throughout.
    /// - All filenames need a common separator following the track number.
    ///   Some recognized variants: ".", ". ", " ", " - ", "-" (any amount of whitespace is recognized in these patterns)
    pub fn compute(release_tracks: &mut Vec<Track>) {
        let mut items = Vec::new();

        for track in &mut *release_tracks {
            let file_stem = track.transcodes.file_meta.path.file_stem().unwrap().to_string_lossy();

            match file_stem.find(|c: char| !c.is_ascii_digit()) {
                Some(split_index) => {
                    if split_index == 0 { return; } // No track number

                    let track_number = file_stem[..split_index].parse::<u32>().unwrap();
                    let remainder = file_stem[split_index..].to_string();

                    items.push((track_number, remainder.to_string()));
                }
                None => return // File stem empty or only filled with track number
            }
        }

        items.sort_by(|a, b| a.0.cmp(&b.0));

        if items[0].0 > 1 { return; } // Numbering does not start on 0 or 1

        let mut expected = &items[0].0 + 1;
        for item in &items[1..] {
            if item.0 != expected { return; } // Numbering is not monotonic
            expected += 1;
        }

        let mut common_separator_pattern = CommonSeparatorPattern::new();

        for item in &items {
            common_separator_pattern.add(&item.1);
        }

        let separator_pattern = match common_separator_pattern.determine() {
            Some(separator_pattern) => separator_pattern,
            None => return // No discernible tendency towards any separator, we don't guess
        };

        for track in &mut *release_tracks {
            let file_stem = track.transcodes.file_meta.path.file_stem().unwrap().to_string_lossy();

            let split_index = file_stem.find(|c: char| !c.is_ascii_digit()).unwrap();
            let track_number = file_stem[..split_index].parse::<u32>().unwrap();
            let remainder = &file_stem[split_index..];

            let title = separator_pattern.trim_separator_prefix(remainder);

            track.heuristic_audio_meta = Some(HeuristicAudioMeta::new(title, track_number));
        }
    }

    pub fn new(title: String, track_number: u32) -> HeuristicAudioMeta {
        HeuristicAudioMeta {
            title,
            track_number
        }
    }
}

impl CommonSeparatorPattern {
    /// Determines and adds another separator pattern
    pub fn add(&mut self, string: &str) {
        self.total += 1;

        if string.starts_with('.') {
            self.dot_space += 1;
        } else if string.starts_with(':') {
            self.colon_space += 1;
        } else if string.starts_with(char::is_whitespace) {
            if string.trim_start().starts_with('-') {
                self.space_dash_space += 1;
            } else {
                self.space += 1;
            }
        } else if string.starts_with('-') {
            self.space_dash_space += 1;
        } else {
            self.unseparated += 1;
        }
    }

    pub fn determine(&self) -> Option<SeparatorPattern> {
        let pattern_threshold = |threshold: usize| -> Option<SeparatorPattern> {
            if self.colon_space >= threshold {
                Some(SeparatorPattern::ColonSpace)
            } else if self.dot_space >= threshold {
                Some(SeparatorPattern::DotSpace)
            } else if self.space >= threshold {
                Some(SeparatorPattern::Space)
            } else if self.space_dash_space >= threshold {
                Some(SeparatorPattern::SpaceDashSpace)
            } else if self.unseparated >= threshold {
                Some(SeparatorPattern::Unseparated)
            } else {
                None
            }
        };

        if self.total <= 2 {
            // For 1-2 tracks we require all to have the same separator
            pattern_threshold(self.total)
        } else if self.total < 5 {
            // For up to five tracks we require all but one to have the same separator
            pattern_threshold(self.total - 1)
        } else {
            // For up to ten tracks, two can have a different separator,
            // and for every additional 10 tracks one can have a different separator
            pattern_threshold(self.total - 2 - self.total / 10)
        }
    }

    pub fn new() -> CommonSeparatorPattern {
        CommonSeparatorPattern {
            colon_space: 0,
            dot_space: 0,
            space: 0,
            space_dash_space: 0,
            total: 0,
            unseparated: 0
        }
    }
}

impl SeparatorPattern {
    /// Pass the part of the filename after the track number, and the function
    /// will trim the separator from the start, e.g. you pass " - Example Song" and
    /// it will return "Example Song".
    pub fn trim_separator_prefix(&self, remainder: &str) -> String {
        match self {
            SeparatorPattern::ColonSpace => {
                if let Some(colon_stripped) = remainder.strip_prefix(':') {
                    colon_stripped.trim_start().to_string()
                } else {
                    remainder.trim_start().to_string()
                }
            }
            SeparatorPattern::DotSpace => {
                if let Some(dot_stripped) = remainder.strip_prefix('.') {
                    dot_stripped.trim_start().to_string()
                } else {
                    remainder.trim_start().to_string()
                }
            }
            SeparatorPattern::Space => {
                remainder.trim_start().to_string()
            }
            SeparatorPattern::SpaceDashSpace => {
                if let Some(dash_stripped) = remainder.trim_start().strip_prefix('.') {
                    dash_stripped.trim_start().to_string()
                } else {
                    remainder.trim_start().to_string()
                }
            }
            SeparatorPattern::Unseparated => {
                remainder.trim_start().to_string()
            }
        }
    }
}
