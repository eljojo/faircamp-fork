use chrono::{DateTime, NaiveDate, Utc};
use indoc::formatdoc;
use sanitize_filename::sanitize;
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
    rc::Rc
};
use std::f32::consts::TAU;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    Artist,
    Asset,
    AssetIntent,
    Build,
    Cache,
    Catalog,
    DownloadFormat,
    DownloadOption,
    HtmlAndStripped,
    Image,
    manifest::Overrides,
    PaymentOption,
    Permalink,
    render,
    SourceFileSignature,
    StreamingQuality,
    TagMapping,
    Theme,
    Track,
    util
};

/// Downloadable zip archives for a release, including cover and tracks
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArchiveAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub alac: Option<Asset>,
    pub cover_source_file_signature: Option<SourceFileSignature>,
    pub extra_source_file_signatures: Vec<SourceFileSignature>,
    pub flac: Option<Asset>,
    pub mp3_v0: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub opus_48: Option<Asset>,
    pub opus_96: Option<Asset>,
    pub opus_128: Option<Asset>,
    pub track_source_file_signatures: Vec<SourceFileSignature>,
    pub uid: String,
    pub wav: Option<Asset>
}

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadGranularity {
    AllOptions,
    EntireRelease,
    SingleFiles
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Extra {
    pub sanitized_filename: String,
    pub source_file_signature: SourceFileSignature
}

#[derive(Debug)]
pub struct Release {
    pub archive_assets: Rc<RefCell<ArchiveAssets>>,
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download asset filenames.
    pub asset_basename: Option<String>,
    pub cover: Option<Rc<RefCell<Image>>>,
    pub date: Option<NaiveDate>,
    pub download_formats: Vec<DownloadFormat>,
    pub download_granularity: DownloadGranularity,
    pub download_option: DownloadOption,
    pub embedding: bool,
    /// Additional files that are included in the download archive,
    /// such as additional images, liner notes, etc.
    pub extras: Vec<Extra>,
    /// Whether additional files in the release directory (besides audio files,
    /// cover image and manifest(s)) should be included in the downloads. 
    pub include_extras: bool,
    /// The artists that are the principal authors of a release ("Album Artist" in tag lingo)
    pub main_artists: Vec<Rc<RefCell<Artist>>>,
    /// The order in which we encounter artists and releases when reading the
    /// catalog is arbitrary, hence when we read a release, we might not yet
    /// have read metadata that tells us to which artist(s) it needs to be
    /// mapped. `main_artists_to_map` is an intermediate, name-based mapping
    /// we store until the entire catalog is read. After that point, we
    /// use it to build the final mapping in `main_artists`, then dispose of it.
    pub main_artists_to_map: Vec<String>,
    pub payment_options: Vec<PaymentOption>,
    pub permalink: Permalink,
    pub rewrite_tags: bool,
    /// Relative path of the release directory in the catalog directory.
    /// This is used to augment permalink conflict errors with additional
    /// info for resolving the conflict.
    pub source_dir: PathBuf,
    pub streaming_quality: StreamingQuality,
    /// Artists that appear on the release as collaborators, features, etc.
    pub support_artists: Vec<Rc<RefCell<Artist>>>,
    /// See `main_artists_to_map` for what this does
    pub support_artists_to_map: Vec<String>,
    pub text: Option<HtmlAndStripped>,
    pub title: String,
    pub track_numbering: TrackNumbering,
    /// The order of tracks (and derived from this the track numbers) are
    /// authoritative, i.e. when the release is constructed, tracks are
    /// passed in the order that has been determined by track number metadata
    /// and/or alphabetical sorting of filenames as a fallback. When the release
    /// input files include both files with track number metadata and without,
    /// and/or when the track numbers don't start at 1 and/or don't monotonically
    /// increase in steps of 1 some unexpected or random track ordering and numbering
    /// might happen, but this is somewhat impossible to avoid.
    pub tracks: Vec<Track>
}

#[derive(Clone, Debug)]
pub enum TrackNumbering {
    Disabled,
    Arabic,
    Hexadecimal,
    Roman
}

impl Extra {
    pub fn new(source_file_signature: SourceFileSignature) -> Extra {
        let sanitized_filename = sanitize(
            source_file_signature.path.file_name().unwrap().to_string_lossy()
        );

        Extra {
            sanitized_filename,
            source_file_signature
        }
    }
}

impl Release {
    pub fn generate_cover_looney_tunes(&self, theme: &Theme, max_tracks_in_release: usize) -> String {
        // TODO: This is too simplistic, text also has text_h and text_s
        // currently (but theming may change quite a bit so no rush). Also
        // unfortunately generated covers don't interactively repaint when
        // using the --theming-widget, but that's probably to be accepted.
        let text_l = theme.base.text_l;
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_track_duration = self.longest_track_duration();

        let mut track_offset = 0.0;
        let points = self.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.assets.borrow().source_meta;

                let altitude_range = 0.75 * self.tracks.len() as f32 / max_tracks_in_release as f32;
                let altitude_width = radius * altitude_range / self.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds / longest_track_duration;

                if let Some(peaks) = &source_meta.peaks {
                    let mut samples = Vec::new();
                    let step = 1;

                    let mut previous = None;

                    let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

                    for (peak_index, peak) in peaks.iter().step_by(step).enumerate() {
                        let peak_offset = peak_index as f32 / (peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                        let arc_offset = (track_compensation + peak_offset * track_arc_range) * TAU;
                        let amplitude = 
                            radius * 0.25 +
                            (max_tracks_in_release - 1 - track_index) as f32 * altitude_width +
                            (peak * 0.3 * altitude_width);

                        let x = radius + amplitude * arc_offset.sin();
                        let y = radius + amplitude * arc_offset.cos();

                        if let Some((x_prev, y_prev)) = previous {
                            let stroke = format!("hsla(0, 0%, {text_l}%, {peak})");
                            let stroke_width = peak * 0.32;
                            let sample = format!(r##"<line stroke="{stroke}" stroke-width="{stroke_width}px" x1="{x_prev}" x2="{x}" y1="{y_prev}" y2="{y}"/>"##);
                            samples.push(sample);
                        }

                        previous = Some((x, y));
                    }

                    track_offset += track_arc_range;

                    samples.join("\n")
                } else {
                    let cx = radius + (edge / 3.0) * (track_offset * TAU).sin();
                    let cy = radius + (edge / 3.0) * (track_offset * TAU).cos();

                    track_offset += track_arc_range;

                    format!(r##"<circle cx="{cx}" cy="{cy}" fill="#ffffff" r="1"/>"##)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r##"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <rect fill="none" height="63.96" stroke="hsl(0, 0% {text_l}%)" stroke-width=".12px" width="63.96" x="0.02" y="0.02"/>
                {points}
            </svg>
        "##)
    }

    pub fn generate_cover_space_time_rupture(&self, theme: &Theme) -> String {
        // TODO: This is too simplistic, text also has text_h and text_s
        // currently (but theming may change quite a bit so no rush). Also
        // unfortunately generated covers don't interactively repaint when
        // using the --theming-widget, but that's probably to be accepted.
        let text_l = theme.base.text_l;
        let edge = 64.0;

        let total_duration: f32 = self.tracks
            .iter()
            .map(|track| track.assets.borrow().source_meta.duration_seconds)
            .sum();

        let shortest_track_duration = self.shortest_track_duration();

        let longest_track_duration = self.longest_track_duration();

        let mut track_offset = 0.0;
        let points = self.tracks
            .iter()
            .enumerate()
            .map(|(_track_index, track)| {
                let source_meta = &track.assets.borrow().source_meta;

                let altitude_factor = (source_meta.duration_seconds - shortest_track_duration) / (longest_track_duration - shortest_track_duration);
                let track_arc_range = source_meta.duration_seconds / total_duration;

                let fill_or_stroke = format!("hsl(0, 0%, {text_l}%)");
                if let Some(peaks) = &source_meta.peaks {
                    let mut samples = Vec::new();
                    let step = 6;

                    for (peak_index, peak) in peaks.iter().step_by(step).enumerate() {
                        let peak_offset = peak_index as f32 / (peaks.len() - 1) as f32 * step as f32; // 0-1

                        let x_vector = ((track_offset + peak_offset * track_arc_range) * TAU).sin();
                        let y_vector = ((track_offset + peak_offset * track_arc_range + 0.25) * TAU).sin(); // TODO: Use cos (also elsewhere)

                        let x = (edge / 2.0) + ((edge / 6.0) + (edge / 6.0) * altitude_factor + (1.0 - peak) * edge / 12.0) * x_vector;
                        let y = (edge / 2.0) + ((edge / 6.0) + (edge / 6.0) * altitude_factor + (1.0 - peak) * edge / 12.0) * y_vector;

                        let command = if peak_index == 0 { "M" } else { "L" };
                        let sample = format!("{command} {x} {y}");

                        samples.push(sample);
                    }

                    let d = samples.join(" ");

                    track_offset += track_arc_range;

                    format!(r##"<path d="{d}" fill="none" stroke="{fill_or_stroke}" stroke-width=".06px"/>"##)
                } else {
                    let cx = (edge / 2.0) + (edge / 3.0) * (track_offset * TAU).sin();
                    let cy = (edge / 2.0) + (edge / 3.0) * ((track_offset + 0.25) * TAU).sin();

                    track_offset += track_arc_range;

                    format!(r##"<circle cx="{cx}" cy="{cy}" fill="{fill_or_stroke}" r="1"/>"##)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                {points}
            </svg>
        "#)
    }

    pub fn longest_track_duration(&self) -> f32 {
        let mut longest_track_duration = 0.0;
        for track in &self.tracks {
            let duration_seconds = &track.assets.borrow().source_meta.duration_seconds;
            if *duration_seconds > longest_track_duration {
                longest_track_duration = *duration_seconds;
            }
        }
        longest_track_duration
    }

    pub fn generate_cover_best_rillen(&self, theme: &Theme) -> String {
        // TODO: This is too simplistic, text also has text_h and text_s
        // currently (but theming may change quite a bit so no rush). Also
        // unfortunately generated covers don't interactively repaint when
        // using the --theming-widget, but that's probably to be accepted.
        let text_l = theme.base.text_l;
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_track_duration = self.longest_track_duration();

        let mut track_offset = 0.0;
        let points = self.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.assets.borrow().source_meta;

                let altitude_width = radius / self.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds / longest_track_duration;

                if let Some(peaks) = &source_meta.peaks {
                    let mut samples = Vec::new();
                    let step = 2;

                    let mut previous = None;

                    let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

                    for (peak_index, peak) in peaks.iter().step_by(step).enumerate() {
                        let peak_offset = peak_index as f32 / (peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                        let x_vector = ((track_compensation + peak_offset * track_arc_range) * TAU).sin();
                        let y_vector = ((track_compensation + peak_offset * track_arc_range) * TAU).cos();

                        let x = radius + ((self.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * 0.3 * altitude_width) * x_vector;
                        let y = radius + ((self.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * 0.3 * altitude_width) * y_vector;

                        if let Some((x_prev, y_prev)) = previous {
                            let stroke = format!("hsla(0, 0%, {text_l}%, {peak})");
                            let stroke_width = peak * 0.24; // .06px is our ideal for waveforms
                            let sample = format!(r##"<line stroke="{stroke}" stroke-width="{stroke_width}px" x1="{x_prev}" x2="{x}" y1="{y_prev}" y2="{y}"/>"##);
                            samples.push(sample);
                        }

                        previous = Some((x, y));
                    }

                    track_offset += track_arc_range;

                    samples.join("\n")
                } else {
                    let cx = radius + (edge / 3.0) * (track_offset * TAU).sin();
                    let cy = radius + (edge / 3.0) * (track_offset * TAU).cos();

                    track_offset += track_arc_range;

                    let fill = format!("hsl(0, 0%, {text_l}%)");
                    format!(r##"<circle cx="{cx}" cy="{cy}" fill="{fill}" r="1"/>"##)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r##"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                {points}
            </svg>
        "##)
    }

    // "scratchy faint rillen"
    pub fn generate_cover_scratchy_faint_rillen(&self, theme: &Theme) -> String {
        // TODO: This is too simplistic, text also has text_h and text_s
        // currently (but theming may change quite a bit so no rush). Also
        // unfortunately generated covers don't interactively repaint when
        // using the --theming-widget, but that's probably to be accepted.
        let text_l = theme.base.text_l;
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_track_duration = self.longest_track_duration();

        let mut track_offset = 0.0;
        let points = self.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.assets.borrow().source_meta;

                let altitude_width = radius / self.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds / longest_track_duration;

                let stroke_or_fill = format!("hsl(0, 0%, {text_l}%)");

                if let Some(peaks) = &source_meta.peaks {
                    let mut samples = Vec::new();
                    let step = 2;

                    for (peak_index, peak) in peaks.iter().step_by(step).enumerate() {
                        let peak_offset = peak_index as f32 / (peaks.len() - 1) as f32 * step as f32; // 0-1

                        let x_vector = (peak_offset * track_arc_range * TAU).sin();
                        let y_vector = (peak_offset * track_arc_range * TAU).cos();

                        let x = radius + ((self.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * altitude_width) * x_vector;
                        let y = radius + ((self.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * altitude_width) * y_vector;

                        let command = if peak_index == 0 { "M" } else { "L" };
                        let sample = format!("{command} {x} {y}");

                        samples.push(sample);
                    }

                    let d = samples.join(" ");

                    track_offset += track_arc_range;

                    format!(r##"<path d="{d}" fill="none" stroke="{stroke_or_fill}" stroke-width=".06px"/>"##)
                } else {
                    let cx = radius + (edge / 3.0) * (track_offset * TAU).sin();
                    let cy = radius + (edge / 3.0) * (track_offset * TAU).cos();

                    track_offset += track_arc_range;

                    format!(r##"<circle cx="{cx}" cy="{cy}" fill="{stroke_or_fill}" r="1"/>"##)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                {points}
            </svg>
        "#)
    }

    pub fn generate_cover_glass_splinters(&self, theme: &Theme) -> String {
        // TODO: This is too simplistic, text also has text_h and text_s
        // currently (but theming may change quite a bit so no rush). Also
        // unfortunately generated covers don't interactively repaint when
        // using the --theming-widget, but that's probably to be accepted.
        let text_l = theme.base.text_l;
        let edge = 64.0;

        let total_duration: f32 = self.tracks
            .iter()
            .map(|track| track.assets.borrow().source_meta.duration_seconds)
            .sum();

        let shortest_track_duration = self.shortest_track_duration();

        let mut gap_arc = 0.02;

        let min_gap_arc = (shortest_track_duration / total_duration) / 2.0;
        if min_gap_arc < gap_arc {
            gap_arc = min_gap_arc;
        }

        let stroke_or_fill = format!("hsl(0, 0%, {text_l}%)");

        let mut track_offset = 0.0;
        let points = self.tracks
            .iter()
            .enumerate()
            .map(|(_track_index, track)| {
                let source_meta = &track.assets.borrow().source_meta;

                let track_arc_range = source_meta.duration_seconds / total_duration;

                if let Some(peaks) = &source_meta.peaks {
                    let mut samples = Vec::new();
                    let step = 4;

                    for (peak_index, peak) in peaks.iter().step_by(step).enumerate() {
                        let peak_offset = peak_index as f32 / (peaks.len() - 1) as f32 * step as f32; // 0-1

                        let x_vector = ((track_offset + peak_offset * (track_arc_range - gap_arc)) * TAU).sin();
                        let y_vector = ((track_offset + peak_offset * (track_arc_range - gap_arc) + 0.25) * TAU).sin(); // TODO: Use cos (also elsewhere)

                        let x = (edge / 2.0) + (edge / 6.0 + (1.0 - peak) * edge / 3.5) * x_vector;
                        let y = (edge / 2.0) + (edge / 6.0 + (1.0 - peak) * edge / 3.5) * y_vector;

                        let command = if peak_index == 0 { "M" } else { "L" };
                        let sample = format!("{command} {x} {y}");

                        samples.push(sample);
                    }

                    let d = samples.join(" ");

                    track_offset += track_arc_range;

                    format!(r##"<path d="{d}" fill="none" stroke="{stroke_or_fill}" stroke-width=".06px"/>"##)

                } else {
                    let cx = (edge / 2.0) + (edge / 3.0) * (track_offset * TAU).sin();
                    let cy = (edge / 2.0) + (edge / 3.0) * ((track_offset + 0.25) * TAU).sin();

                    track_offset += track_arc_range;

                    format!(r##"<circle cx="{cx}" cy="{cy}" fill="#ffffff" r="1"/>"##)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                {points}
            </svg>
        "#)
    }

    pub fn new(
        archive_assets: Rc<RefCell<ArchiveAssets>>,
        cover: Option<Rc<RefCell<Image>>>,
        date: Option<NaiveDate>,
        extras: Vec<Extra>,
        main_artists_to_map: Vec<String>,
        manifest_overrides: &Overrides,
        permalink_option: Option<Permalink>,
        source_dir: PathBuf,
        support_artists_to_map: Vec<String>,
        title: String,
        tracks: Vec<Track>
    ) -> Release {
        let permalink = permalink_option.unwrap_or_else(|| Permalink::generate(&title));

        let mut download_option = manifest_overrides.download_option.clone();

        if let DownloadOption::Codes { unlock_text, .. } = &mut download_option {
            if let Some(custom_unlock_text) = &manifest_overrides.unlock_text {
                unlock_text.replace(custom_unlock_text.clone());
            }
        }

        Release {
            archive_assets,
            asset_basename: None,
            cover,
            date,
            download_formats: manifest_overrides.download_formats.clone(),
            download_granularity: manifest_overrides.download_granularity.clone(),
            download_option,
            embedding: manifest_overrides.embedding,
            extras,
            include_extras: manifest_overrides.include_extras,
            main_artists: Vec::new(),
            main_artists_to_map,
            payment_options: manifest_overrides.payment_options.clone(),
            permalink,
            rewrite_tags: manifest_overrides.rewrite_tags,
            source_dir,
            streaming_quality: manifest_overrides.streaming_quality,
            support_artists: Vec::new(),
            support_artists_to_map,
            text: manifest_overrides.release_text.clone(),
            title,
            track_numbering: manifest_overrides.release_track_numbering.clone(),
            tracks
        }
    }

    fn shortest_track_duration(&self) -> f32 {
        let mut shortest_track_duration = f32::INFINITY;
        for track in &self.tracks {
            let duration_seconds = &track.assets.borrow().source_meta.duration_seconds;
            if *duration_seconds < shortest_track_duration {
                shortest_track_duration = *duration_seconds;
            }
        }
        shortest_track_duration
    }

    pub fn write_downloadable_files(&mut self, build: &mut Build) {
        let mut tag_mapping_option = if self.rewrite_tags {
            Some(TagMapping {
                album: Some(self.title.clone()),
                album_artist: if self.main_artists.is_empty() {
                    None
                } else {
                    Some(
                        self.main_artists
                            .iter()
                            .map(|artist| artist.borrow().name.clone())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                },
                artist: None,
                title: None,
                track: None
            })
        } else {
            None
        };

        let release_dir = build.build_dir.join(&self.permalink.slug);

        for download_format in &self.download_formats {
            let format_dir = release_dir.join(download_format.as_audio_format().asset_dirname());

            util::ensure_dir(&format_dir);

            let mut archive_assets_mut = self.archive_assets.borrow_mut();
            let cached_archive_asset = archive_assets_mut.get_mut(*download_format);

            for (track_index, track) in self.tracks.iter_mut().enumerate() {
                // Transcode track to required format if needed and not yet available
                if !(self.download_granularity == DownloadGranularity::EntireRelease && cached_archive_asset.is_some()) &&
                    track.assets.borrow().get(download_format.as_audio_format()).is_none() {
                    if download_format.is_lossless() && !track.assets.borrow().source_meta.lossless {
                        warn_discouraged!(
                            "Track {} comes from a lossy format, offering it in a lossless format is wasteful and misleading to those who will download it.",
                            &track.assets.borrow().source_file_signature.path.display()
                        );
                    }

                    if let Some(tag_mapping) = &mut tag_mapping_option {
                        tag_mapping.artist = if track.artists.is_empty() {
                            None
                        } else {
                             Some(
                                track.artists
                                .iter()
                                .map(|artist| artist.borrow().name.clone())
                                .collect::<Vec<String>>()
                                .join(", ")
                            )
                        };
                        tag_mapping.title = Some(track.title.clone());

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
                        tag_mapping.track = Some(track_index + 1);
                    }

                    let asset_intent = if self.download_granularity == DownloadGranularity::EntireRelease {
                        AssetIntent::Intermediate
                    } else {
                        AssetIntent::Deliverable
                    };

                    track.transcode_as(
                        download_format.as_audio_format(),
                        build,
                        asset_intent,
                        &tag_mapping_option
                    );

                    track.assets.borrow().persist_to_cache(&build.cache_dir);
                }

                // If single track downloads are enabled copy transcoded track to build
                if self.download_granularity != DownloadGranularity::EntireRelease {
                    let mut download_track_assets_mut = track.assets.borrow_mut();
                    let download_track_asset = download_track_assets_mut
                        .get_mut(download_format.as_audio_format())
                        .as_mut()
                        .unwrap();

                    download_track_asset.unmark_stale();

                    let track_filename = format!(
                        "{basename}{extension}",
                        basename = track.asset_basename.as_ref().unwrap(),
                        extension = download_format.as_audio_format().extension()
                    );

                    let hash = build.hash(
                        &self.permalink.slug,
                        download_format.as_audio_format().asset_dirname(),
                        &track_filename
                    );

                    // TODO: We should calculate this earlier and persist it so we can reuse it for copying
                    // and for rendering the hrefs that point to it, however we need to figure out where 
                    // (or on what) to store it - that's a bit tricky. (applies in a few places)
                    let hash_dir = format_dir.join(hash);

                    util::ensure_dir(&hash_dir);

                    let target_path = hash_dir.join(&track_filename);

                    // The track asset might already have been copied to the build directory
                    // if the download format is identical to one of the streaming formats.
                    // So we only copy and add it to the stats if that hasn't yet happened.
                    if !target_path.exists() {
                        util::hard_link_or_copy(
                            build.cache_dir.join(&download_track_asset.filename),
                            target_path
                        );

                        build.stats.add_track(download_track_asset.filesize_bytes);
                    }
                }
            }

            // If entire release downloads are enabled create the zip (if not available yet) and copy it to build
            if self.download_granularity != DownloadGranularity::SingleFiles {
                // Create zip for required format if not yet available
                if cached_archive_asset.is_none() {
                    let cached_archive_filename = format!("{}.zip", util::uid());

                    info_zipping!(
                        "Creating download archive for release '{}' ({})",
                        self.title,
                        download_format.as_audio_format()
                    );

                    let zip_file = File::create(
                        build.cache_dir.join(&cached_archive_filename)
                    ).unwrap();
                    let mut zip_writer = ZipWriter::new(zip_file);
                    let options = FileOptions::default()
                        .compression_method(CompressionMethod::Deflated)
                        .unix_permissions(0o755);

                    let mut buffer = Vec::new();

                    for track in self.tracks.iter_mut() {
                        let assets_ref = track.assets.borrow();
                        let download_track_asset = assets_ref.get(download_format.as_audio_format()).as_ref().unwrap();

                        let filename = format!(
                            "{basename}{extension}",
                            basename = track.asset_basename.as_ref().unwrap(),
                            extension = download_format.as_audio_format().extension()
                        );

                        zip_writer.start_file(&filename, options).unwrap();

                        let mut zip_inner_file = File::open(
                            &build.cache_dir.join(&download_track_asset.filename)
                        ).unwrap();

                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&buffer).unwrap();
                        buffer.clear();

                        track.assets.borrow().persist_to_cache(&build.cache_dir);
                    }

                    if let Some(cover) = &mut self.cover {
                        let cover_mut = cover.borrow_mut();
                        let mut assets_mut = cover_mut.assets.borrow_mut();
                        let cover_assets = assets_mut.cover_asset(build, AssetIntent::Intermediate);

                        zip_writer.start_file("cover.jpg", options).unwrap();

                        let mut zip_inner_file = File::open(
                            &build.cache_dir.join(&cover_assets.largest().filename)
                        ).unwrap();

                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&buffer).unwrap();
                        buffer.clear();

                        assets_mut.persist_to_cache(&build.cache_dir);
                    }

                    for extra in &self.extras {
                        zip_writer.start_file(&extra.sanitized_filename, options).unwrap();

                        let mut zip_inner_file = File::open(
                            &build.catalog_dir.join(&extra.source_file_signature.path)
                        ).unwrap();

                        zip_inner_file.read_to_end(&mut buffer).unwrap();
                        zip_writer.write_all(&buffer).unwrap();
                        buffer.clear();
                    }

                    match zip_writer.finish() {
                        Ok(_) => cached_archive_asset.replace(Asset::new(build, cached_archive_filename, AssetIntent::Deliverable)),
                        Err(err) => panic!("{}", err)
                    };
                }

                // Now we copy the zip to the build
                let download_archive_asset = cached_archive_asset.as_mut().unwrap();

                download_archive_asset.unmark_stale();

                let archive_filename = format!(
                    "{basename}.zip",
                    basename = self.asset_basename.as_ref().unwrap()
                );

                let hash = build.hash(
                    &self.permalink.slug,
                    download_format.as_audio_format().asset_dirname(),
                    &archive_filename
                );

                let hash_dir = format_dir.join(hash);

                util::ensure_dir(&hash_dir);

                util::hard_link_or_copy(
                    build.cache_dir.join(&download_archive_asset.filename),
                    hash_dir.join(&archive_filename)
                );

                build.stats.add_archive(download_archive_asset.filesize_bytes);

                archive_assets_mut.persist_to_cache(&build.cache_dir);
            }
        }

        // Write extras for discrete download access (outside of archives/zips)
        if !self.extras.is_empty() && self.download_granularity != DownloadGranularity::EntireRelease {
            for extra in &self.extras {
                let extras_dir = release_dir.join("extras");

                util::ensure_dir(&extras_dir);

                let hash = build.hash(
                    &self.permalink.slug,
                    "extras",
                    &extra.sanitized_filename
                );

                // TODO: We should calculate this earlier and persist it so we can reuse it for copying
                // and for rendering the hrefs that point to it, however we need to figure out where 
                // (or on what) to store it - that's a bit tricky. (applies in a few places)
                let hash_dir = extras_dir.join(hash);

                util::ensure_dir(&hash_dir);

                let target_path = hash_dir.join(&extra.sanitized_filename);

                util::hard_link_or_copy(
                    build.catalog_dir.join(&extra.source_file_signature.path),
                    target_path
                );

                build.stats.add_extra(extra.source_file_signature.size);
            }
        }
    }

    pub fn write_files(&self, build: &mut Build, catalog: &Catalog) {
        match &self.download_option {
            DownloadOption::Codes { codes, .. } => {
                let t_unlock_permalink = &build.locale.translations.unlock_permalink;
                let page_hash = build.hash_generic(&[&self.permalink.slug, t_unlock_permalink]);

                let unlock_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_unlock_permalink)
                    .join(page_hash);

                // TODO: Split up checkout_html into unlock_html + purchase_html - different pages!
                let unlock_html = render::release::checkout::checkout_html(build, catalog, self);
                util::ensure_dir_and_write_index(&unlock_page_dir, &unlock_html);

                let t_downloads_permalink = &build.locale.translations.downloads_permalink;
                let download_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_downloads_permalink);

                let download_html = render::release::download::download_html(build, catalog, self);

                for code in codes {
                    // TODO: We will need to limit the code character set to url safe characters.
                    //       Needs to be validated when reading the manifests.
                    let code_dir = download_dir.join(code);
                    util::ensure_dir_and_write_index(&code_dir, &download_html);
                }
            }
            DownloadOption::Disabled => (),
            DownloadOption::Free  => {
                let t_downloads_permalink = &build.locale.translations.downloads_permalink;
                let page_hash = build.hash_generic(&[&self.permalink.slug, t_downloads_permalink]);

                let download_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_downloads_permalink)
                    .join(page_hash);

                let download_html = render::release::download::download_html(build, catalog, self);
                util::ensure_dir_and_write_index(&download_page_dir, &download_html);
            }
            DownloadOption::Paid(_currency, _ranges) => {
                if self.payment_options.is_empty() {
                    warn!(
                        "No payment options specified for release '{}', no purchase/download option will be displayed for this release.",
                        self.title
                    );
                } else {
                    let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                    let purchase_page_hash = build.hash_generic(&[&self.permalink.slug, t_purchase_permalink]);

                    let purchase_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_purchase_permalink)
                        .join(purchase_page_hash);

                    let purchase_html = render::release::checkout::checkout_html(build, catalog, self);
                    util::ensure_dir_and_write_index(&purchase_page_dir, &purchase_html);

                    let t_downloads_permalink = &build.locale.translations.downloads_permalink;
                    let download_page_hash = build.hash_generic(&[&self.permalink.slug, t_downloads_permalink]);

                    let download_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_downloads_permalink)
                        .join(download_page_hash);

                    let download_html = render::release::download::download_html(build, catalog, self);
                    util::ensure_dir_and_write_index(&download_page_dir, &download_html);
                }
            }
        }
        
        if let Some(cover) = &self.cover {
            if cover.borrow().description.is_none() {
                warn_discouraged!("The cover image for release '{}' is missing an image description.", self.title);
                build.missing_image_descriptions = true;
            }
        }

        let release_dir = build.build_dir.join(&self.permalink.slug);
        let release_html = render::release::release_html(build, catalog, self);
        util::ensure_dir_and_write_index(&release_dir, &release_html);

        if self.embedding  {
            if let Some(base_url) = &build.base_url {
                let embed_choices_dir = release_dir.join("embed");
                let embed_choices_html = render::release::embed::embed_choices_html(build, catalog, self, base_url);
                util::ensure_dir_and_write_index(&embed_choices_dir, &embed_choices_html);

                let embed_release_dir = embed_choices_dir.join("all");
                let embed_release_html = render::release::embed::embed_release_html(build, catalog, self, base_url);
                util::ensure_dir_and_write_index(&embed_release_dir, &embed_release_html);

                for (index, track) in self.tracks.iter().enumerate() {
                    let track_number = index + 1;
                    let embed_track_dir = embed_choices_dir.join(track_number.to_string());
                    let embed_track_html = render::release::embed::embed_track_html(build, catalog, self, track, track_number, base_url);
                    util::ensure_dir_and_write_index(&embed_track_dir, &embed_track_html);
                }
            }
        }
    }
}

impl ArchiveAssets {
    pub fn deserialize_cached(path: &Path) -> Option<ArchiveAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<ArchiveAssets>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn get(&self, download_format: DownloadFormat) -> &Option<Asset> {
        match download_format {
            DownloadFormat::Aac => &self.aac,
            DownloadFormat::Aiff => &self.aiff,
            DownloadFormat::Alac => &self.alac,
            DownloadFormat::Flac => &self.flac,
            DownloadFormat::Mp3VbrV0 => &self.mp3_v0,
            DownloadFormat::OggVorbis => &self.ogg_vorbis,
            DownloadFormat::Opus48Kbps => &self.opus_48,
            DownloadFormat::Opus96Kbps => &self.opus_96,
            DownloadFormat::Opus128Kbps => &self.opus_128,
            DownloadFormat::Wav => &self.wav
        }
    }

    pub fn get_mut(&mut self, download_format: DownloadFormat) -> &mut Option<Asset> {
        match download_format {
            DownloadFormat::Aac => &mut self.aac,
            DownloadFormat::Aiff => &mut self.aiff,
            DownloadFormat::Alac => &mut self.alac,
            DownloadFormat::Flac => &mut self.flac,
            DownloadFormat::Mp3VbrV0 => &mut self.mp3_v0,
            DownloadFormat::OggVorbis => &mut self.ogg_vorbis,
            DownloadFormat::Opus48Kbps => &mut self.opus_48,
            DownloadFormat::Opus96Kbps => &mut self.opus_96,
            DownloadFormat::Opus128Kbps => &mut self.opus_128,
            DownloadFormat::Wav => &mut self.wav
        }
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(Cache::ARCHIVE_MANIFESTS_DIR).join(filename)
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for download_format in DownloadFormat::ALL_DOWNLOAD_FORMATS {
            if let Some(asset) = self.get_mut(download_format) {
                asset.mark_stale(timestamp);
            }
        }
    }

    pub fn new(
        cover_source_file_signature: Option<SourceFileSignature>,
        track_source_file_signatures: Vec<SourceFileSignature>,
        extra_source_file_signatures: Vec<SourceFileSignature>
    ) -> ArchiveAssets {
        ArchiveAssets {
            aac: None,
            aiff: None,
            alac: None,
            cover_source_file_signature,
            extra_source_file_signatures,
            flac: None,
            mp3_v0: None,
            ogg_vorbis: None,
            opus_48: None,
            opus_96: None,
            opus_128: None,
            track_source_file_signatures,
            uid: util::uid(),
            wav: None
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}

impl TrackNumbering {
    pub fn format(&self, number: usize) -> String {
        match self {
            TrackNumbering::Disabled => String::from(""),
            TrackNumbering::Arabic => format!("{:02}", number),
            TrackNumbering::Hexadecimal => format!("0x{:02X}", number),
            TrackNumbering::Roman => Self::to_roman(number)
        }
    }
    
    pub fn from_manifest_key(key: &str) -> Option<TrackNumbering> {
        match key {
            "disabled" => Some(TrackNumbering::Disabled),
            "arabic" => Some(TrackNumbering::Arabic),
            "hexadecimal" => Some(TrackNumbering::Hexadecimal),
            "roman" => Some(TrackNumbering::Roman),
            _ =>  None
        }
    }
    
    fn to_roman(number: usize) -> String {
        // TODO: Implement to at least ~256 (or more) using proper algorithm
        let roman = match number {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            10 => "X",
            11 => "XI",
            12 => "XII",
            13 => "XIII",
            14 => "XIV",
            15 => "XV",
            16 => "XVI",
            17 => "XVII",
            18 => "XVIII",
            19 => "XIX",
            20 => "XX",
            21 => "XXI",
            22 => "XXII",
            23 => "XXIII",
            24 => "XXIV",
            25 => "XXV",
            26 => "XXVI",
            27 => "XXVII",
            28 => "XXVIII",
            29 => "XXIX",
            30 => "XXX",
            31 => "XXXI",
            32 => "XXXII",
            33 => "XXXIII",
            34 => "XXXIV",
            35 => "XXXV",
            36 => "XXXVI",
            37 => "XXXVII",
            38 => "XXXVIII",
            39 => "XXXIX",
            40 => "XL",
            _ => unimplemented!()
        };
        
        roman.to_string()
    }
}
