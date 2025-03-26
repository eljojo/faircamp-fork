// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::f32::consts::TAU;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use chrono::{DateTime, Utc};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde_derive::{Serialize, Deserialize};
use tiny_skia::{
    Color,
    LineCap,
    Paint,
    PathBuilder,
    Pixmap,
    Rect,
    Stroke,
    Transform
};

use crate::{Build, ImgAttributes, Release};
use crate::util::{uid, url_safe_base64};

#[derive(Clone, Debug, Hash)]
pub enum CoverGenerator {
    BestRillen,
    Blocks,
    GlassSplinters,
    LooneyTunes,
    ScratchyFaintRillen,
    SpaceTimeRupture
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProceduralCover {
    pub asset_120: ProceduralCoverAsset,
    pub asset_240: ProceduralCoverAsset,
    pub asset_480: ProceduralCoverAsset,
    pub asset_720: ProceduralCoverAsset,
    pub marked_stale: Option<DateTime<Utc>>,
    /// This is a hash computed from all aspects that are relevant for the generation
    /// of the procedural cover, allowing us to retrieve the right archives with only
    /// 64 bits of space used.
    pub signature: u64
}

/// A single, resized version of a procedural cover image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProceduralCoverAsset {
    pub filename: String,
    pub filesize_bytes: u64
}

#[derive(Clone, Debug)]
pub struct ProceduralCoverRc {
    pub procedural_cover: Rc<RefCell<ProceduralCover>>,
}

impl CoverGenerator {
    pub const ALL_GENERATORS: [&'static str; 6] = [
        "best_rillen",
        "blocks",
        "glass_splinters",
        "looney_tunes",
        "scratchy_faint_rillen",
        "space_time_rupture"
    ];

    pub fn from_manifest_key(key: &str) -> Option<CoverGenerator> {
        match key {
            "best_rillen" => Some(CoverGenerator::BestRillen),
            "blocks" => Some(CoverGenerator::Blocks),
            "glass_splinters" => Some(CoverGenerator::GlassSplinters),
            "looney_tunes" => Some(CoverGenerator::LooneyTunes),
            "scratchy_faint_rillen" => Some(CoverGenerator::ScratchyFaintRillen),
            "space_time_rupture" => Some(CoverGenerator::SpaceTimeRupture),
            _ => None
        }
    }

    pub fn generate(
        &self,
        build: &Build,
        max_tracks_in_release: usize,
        release: &Release,
        signature: u64
    ) -> ProceduralCover {
        let generator_name = self.name();

        let generate_for_size = |edge_size: u32| -> ProceduralCoverAsset {
            info_generating!("Procedural Cover for {} in the style '{}' at {} pixel edge size", release.title, generator_name, edge_size);

            let filename = format!("{}.png", uid());
            let output_path = build.cache_dir.join(&filename);

            match self {
                CoverGenerator::BestRillen => CoverGenerator::generate_best_rillen(edge_size, &output_path, release),
                CoverGenerator::Blocks => CoverGenerator::generate_blocks(edge_size, &output_path, release, signature),
                CoverGenerator::GlassSplinters => CoverGenerator::generate_glass_splinters(edge_size, &output_path, release),
                CoverGenerator::LooneyTunes => CoverGenerator::generate_looney_tunes(edge_size, &output_path, release, max_tracks_in_release),
                CoverGenerator::ScratchyFaintRillen => CoverGenerator::generate_scratchy_faint_rillen(edge_size, &output_path, release),
                CoverGenerator::SpaceTimeRupture => CoverGenerator::generate_space_time_rupture(edge_size, &output_path, release)
            }

            ProceduralCoverAsset::new(build, filename)
        };

        let asset_720 = generate_for_size(720);
        let asset_480 = generate_for_size(480);
        let asset_240 = generate_for_size(240);
        let asset_120 = generate_for_size(120);

        ProceduralCover {
            asset_120,
            asset_240,
            asset_480,
            asset_720,
            marked_stale: Some(build.build_begin),
            signature
        }
    }

    fn generate_best_rillen(
        edge_size: u32,
        file_path: &Path,
        release: &Release
    ) {
        let longest_track_duration = release.longest_track_duration();

        let edge_center = edge_size as f32 / 2.0;
        let radius = edge_size as f32 / 3.0;

        let stroke_lightness = release.theme.procedural_cover_stroke_lightness();
        let fill_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 0.025).unwrap();
        let mut stroke_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 1.0).unwrap();

        let mut paint = Paint::default();
        paint.anti_alias = true;

        let mut pixmap = Pixmap::new(edge_size, edge_size).unwrap();
        pixmap.fill(fill_color);

        let mut stroke = Stroke::default();
        stroke.line_cap = LineCap::Round;

        for (track_index, track) in release.tracks.iter().enumerate() {
            let source_meta = &track.transcodes.borrow().source_meta;

            let amplitude_width = radius / release.tracks.len() as f32;
            let track_arc_range = source_meta.duration_seconds / longest_track_duration;

            let step = 2;

            let mut previous = None;

            let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

            for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                let x_vector = ((track_compensation + peak_offset * track_arc_range) * TAU).sin();
                let y_vector = ((track_compensation + peak_offset * track_arc_range) * TAU).cos();

                let x = edge_center + ((release.tracks.len() - 1 - track_index) as f32 * amplitude_width + peak * 0.3 * amplitude_width) * x_vector;
                let y = edge_center + ((release.tracks.len() - 1 - track_index) as f32 * amplitude_width + peak * 0.3 * amplitude_width) * y_vector;

                if let Some((x_prev, y_prev)) = previous {
                    let path = {
                        let mut path_builder = PathBuilder::new();
                        path_builder.move_to(x_prev, y_prev);
                        path_builder.line_to(x, y);
                        path_builder.finish().unwrap()
                    };

                    stroke_color.set_alpha(*peak);
                    paint.set_color(stroke_color);

                    stroke.width = peak * (edge_size as f32 / 400.0);

                    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                }

                previous = Some((x, y));
            }
        }

        pixmap.save_png(file_path).unwrap();
    }

    fn generate_blocks(
        edge_size: u32,
        file_path: &Path,
        release: &Release,
        signature: u64
    ) {
        let stroke_lightness = release.theme.procedural_cover_stroke_lightness();
        let mut fill_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 0.025).unwrap();

        let mut paint = Paint::default();
        paint.anti_alias = true;

        let mut pixmap = Pixmap::new(edge_size, edge_size).unwrap();
        pixmap.fill(fill_color);

        let mut rng = ChaCha8Rng::seed_from_u64(signature);

        let squares = 6;
        let square_edge_size = edge_size as f32 / squares as f32;

        for horizontal_index in 0..squares {
            for vertical_index in 0..squares {
                let alpha = rng.random_range(0.0..1.0);
                fill_color.set_alpha(alpha);
                paint.set_color(fill_color);

                let rect = Rect::from_xywh(
                    horizontal_index as f32 * square_edge_size,
                    vertical_index as f32 * square_edge_size,
                    square_edge_size,
                    square_edge_size
                ).unwrap();

                pixmap.fill_rect(
                    rect,
                    &paint,
                    Transform::identity(),
                    None
                );
            }
        }

        pixmap.save_png(file_path).unwrap();
    }

    fn generate_glass_splinters(
        edge_size: u32,
        file_path: &Path,
        release: &Release
    ) {
        let edge_center = edge_size as f32 / 2.0;

        let stroke_lightness = release.theme.procedural_cover_stroke_lightness();
        let fill_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 0.025).unwrap();
        let stroke_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 1.0).unwrap();

        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint.set_color(stroke_color);

        let mut pixmap = Pixmap::new(edge_size, edge_size).unwrap();
        pixmap.fill(fill_color);

        let mut stroke = Stroke::default();
        stroke.line_cap = LineCap::Round;
        stroke.width = edge_size as f32 / 400.0;

        let total_duration: f32 = release.tracks
            .iter()
            .map(|track| track.transcodes.borrow().source_meta.duration_seconds)
            .sum();

        let shortest_track_duration = release.shortest_track_duration();

        let mut gap_arc = 0.02;

        let min_gap_arc = (shortest_track_duration / total_duration) / 2.0;
        if min_gap_arc < gap_arc {
            gap_arc = min_gap_arc;
        }

        let mut track_offset = 0.0;
        for track in &release.tracks {
            let mut path_builder = PathBuilder::new();

            let source_meta = &track.transcodes.borrow().source_meta;

            let track_arc_range = source_meta.duration_seconds / total_duration;

            let step = 4;

            for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32; // 0-1

                let x_vector = ((track_offset + peak_offset * (track_arc_range - gap_arc)) * TAU).sin();
                let y_vector = ((track_offset + peak_offset * (track_arc_range - gap_arc) + 0.25) * TAU).sin(); // TODO: Use cos (also elsewhere)

                let x = edge_center + (edge_size as f32 / 6.0 + (1.0 - peak) * edge_size as f32 / 3.5) * x_vector;
                let y = edge_center + (edge_size as f32 / 6.0 + (1.0 - peak) * edge_size as f32 / 3.5) * y_vector;

                if peak_index == 0 {
                    path_builder.move_to(x, y);
                } else {
                    path_builder.line_to(x, y);
                };
            }

            let path = path_builder.finish().unwrap();

            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

            track_offset += track_arc_range;
        }

        pixmap.save_png(file_path).unwrap();
    }

    fn generate_looney_tunes(
        edge_size: u32,
        file_path: &Path,
        release: &Release,
        max_tracks_in_release: usize
    ) {
        let longest_track_duration = release.longest_track_duration();

        let edge_center = edge_size as f32 / 2.0;
        let radius = edge_size as f32 / 3.0;

        let stroke_lightness = release.theme.procedural_cover_stroke_lightness();
        let fill_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 0.025).unwrap();
        let stroke_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 1.0).unwrap();

        let mut pixmap = Pixmap::new(edge_size, edge_size).unwrap();
        pixmap.fill(fill_color);

        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint.set_color(stroke_color);

        let mut stroke = Stroke::default();
        stroke.line_cap = LineCap::Round;

        for (track_index, track) in release.tracks.iter().enumerate() {
            let source_meta = &track.transcodes.borrow().source_meta;

            let amplitude_range = 0.75 * release.tracks.len() as f32 / max_tracks_in_release as f32;
            let amplitude_width = radius * amplitude_range / release.tracks.len() as f32;
            let track_arc_range = source_meta.duration_seconds / longest_track_duration;

            let step = 1;

            let mut previous = None;

            let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

            for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                let arc_offset = (track_compensation + peak_offset * track_arc_range) * TAU;
                let amplitude =
                    radius * 0.25 +
                    (max_tracks_in_release - 1 - track_index) as f32 * amplitude_width +
                    (peak * 0.3 * amplitude_width);

                let x = edge_center + amplitude * arc_offset.sin();
                let y = edge_center + amplitude * arc_offset.cos();

                if let Some((x_prev, y_prev)) = previous {
                    let path = {
                        let mut path_builder = PathBuilder::new();
                        path_builder.move_to(x_prev, y_prev);
                        path_builder.line_to(x, y);
                        path_builder.finish().unwrap()
                    };

                    stroke.width = peak * (edge_size as f32 / 400.0);

                    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                }

                previous = Some((x, y));
            }
        }

        pixmap.save_png(file_path).unwrap();
    }

    fn generate_scratchy_faint_rillen(
        edge_size: u32,
        file_path: &Path,
        release: &Release
    ) {
        let edge_center = edge_size as f32 / 2.0;
        let radius = edge_size as f32 / 3.0;

        let stroke_lightness = release.theme.procedural_cover_stroke_lightness();
        let fill_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 0.025).unwrap();
        let stroke_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 1.0).unwrap();

        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint.set_color(stroke_color);

        let mut pixmap = Pixmap::new(edge_size, edge_size).unwrap();
        pixmap.fill(fill_color);

        let mut stroke = Stroke::default();
        stroke.line_cap = LineCap::Round;
        stroke.width = edge_size as f32 / 400.0;

        let longest_track_duration = release.longest_track_duration();

        for (track_index, track) in release.tracks.iter().enumerate() {
            let mut path_builder = PathBuilder::new();

            let source_meta = &track.transcodes.borrow().source_meta;

            let amplitude_width = radius / release.tracks.len() as f32;
            let track_arc_range = source_meta.duration_seconds / longest_track_duration;

            let step = 2;

            for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32; // 0-1

                let x_vector = (peak_offset * track_arc_range * TAU).sin();
                let y_vector = (peak_offset * track_arc_range * TAU).cos();

                let x = edge_center + ((release.tracks.len() - 1 - track_index) as f32 * amplitude_width + peak * amplitude_width) * x_vector;
                let y = edge_center + ((release.tracks.len() - 1 - track_index) as f32 * amplitude_width + peak * amplitude_width) * y_vector;

                if peak_index == 0 {
                    path_builder.move_to(x, y);
                } else {
                    path_builder.line_to(x, y);
                };
            }

            let path = path_builder.finish().unwrap();

            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }


        pixmap.save_png(file_path).unwrap();
    }

    fn generate_space_time_rupture(
        edge_size: u32,
        file_path: &Path,
        release: &Release
    ) {
        let edge_center = edge_size as f32 / 2.0;

        let stroke_lightness = release.theme.procedural_cover_stroke_lightness();
        let fill_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 0.025).unwrap();
        let stroke_color = Color::from_rgba(stroke_lightness, stroke_lightness, stroke_lightness, 1.0).unwrap();

        let mut paint = Paint::default();
        paint.anti_alias = true;
        paint.set_color(stroke_color);

        let mut pixmap = Pixmap::new(edge_size, edge_size).unwrap();
        pixmap.fill(fill_color);

        let mut stroke = Stroke::default();
        stroke.line_cap = LineCap::Round;
        stroke.width = edge_size as f32 / 400.0;

        let total_duration: f32 = release.tracks
            .iter()
            .map(|track| track.transcodes.borrow().source_meta.duration_seconds)
            .sum();

        let shortest_track_duration = release.shortest_track_duration();

        let longest_track_duration = release.longest_track_duration();

        let mut track_offset = 0.0;
        for track in &release.tracks {
            let mut path_builder = PathBuilder::new();

            let source_meta = &track.transcodes.borrow().source_meta;

            let amplitude_factor = if shortest_track_duration == longest_track_duration {
                0.0
            } else {
                (source_meta.duration_seconds - shortest_track_duration) / (longest_track_duration - shortest_track_duration)
            };
            let track_arc_range = source_meta.duration_seconds / total_duration;

            let step = 6;

            for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32; // 0-1

                let x_vector = ((track_offset + peak_offset * track_arc_range) * TAU).sin();
                let y_vector = ((track_offset + peak_offset * track_arc_range + 0.25) * TAU).sin(); // TODO: Use cos (also elsewhere)


                let x = edge_center + ((edge_size as f32 / 6.0) + (edge_size as f32 / 6.0) * amplitude_factor + (1.0 - peak) * edge_size as f32 / 12.0) * x_vector;
                let y = edge_center + ((edge_size as f32 / 6.0) + (edge_size as f32 / 6.0) * amplitude_factor + (1.0 - peak) * edge_size as f32 / 12.0) * y_vector;

                if peak_index == 0 {
                    path_builder.move_to(x, y);
                } else {
                    path_builder.line_to(x, y);
                };
            }

            let path = path_builder.finish().unwrap();

            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);

            track_offset += track_arc_range;
        }

        pixmap.save_png(file_path).unwrap();
    }

    pub fn name(&self) -> &str {
        match self {
            CoverGenerator::BestRillen => "Beste Rillen",
            CoverGenerator::Blocks => "Blocks",
            CoverGenerator::GlassSplinters => "Glass Splinters",
            CoverGenerator::LooneyTunes => "Looney Tunes",
            CoverGenerator::ScratchyFaintRillen => "Scratchy Faint Rillen",
            CoverGenerator::SpaceTimeRupture => "Space Time Rupture"
        }
    }
}

impl ProceduralCover {
    pub const FILENAME_120: &str = "cover_120.png";
    pub const FILENAME_240: &str = "cover_240.png";
    pub const FILENAME_480: &str = "cover_480.png";
    pub const FILENAME_720: &str = "cover_720.png";

    /// Increase version on each change to the data layout of [ProceduralCover].
    /// This automatically informs the cache not to try to deserialize
    /// manifests that hold old, incompatible data.
    pub const CACHE_SERIALIZATION_KEY: &'static str = "procedural_cover1";

    // pub fn new(filename: String) -> ProceduralCover {
    //     ProceduralCover {
    //         filename,
    //         hash: "TODO"
    //     }
    // }

    pub fn deserialize_cached(path: &Path) -> Option<ProceduralCover> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<ProceduralCover>(&bytes).ok(),
            Err(_) => None
        }
    }

    /// Returns the statically assigned filename for the 120px variant,
    /// including a query string for cache invalidation.
    pub fn filename_120(&self) -> String {
        let filename = ProceduralCover::FILENAME_120;
        let hash = url_safe_base64(self.signature);
        format!("{filename}?{hash}")
    }

    /// Returns the statically assigned filename for the 480px variant,
    /// including a query string for cache invalidation.
    pub fn filename_480(&self) -> String {
        let filename = ProceduralCover::FILENAME_480;
        let hash = url_safe_base64(self.signature);
        format!("{filename}?{hash}")
    }

    /// Returns the statically assigned filename for the 720px variant,
    /// including a query string for cache invalidation.
    pub fn filename_720(&self) -> String {
        let filename = ProceduralCover::FILENAME_720;
        let hash = url_safe_base64(self.signature);
        format!("{filename}?{hash}")
    }

    /// Returns src and srcset attributes for usage of the procedural cover
    /// through an img tag inside html markup.
    pub fn img_attributes_all_sizes(&self, prefix: &str) -> ImgAttributes {
        let hash = url_safe_base64(self.signature);

        let src_120 = format!("{prefix}{filename}?{hash}", filename = ProceduralCover::FILENAME_120);
        let src_240 = format!("{prefix}{filename}?{hash}", filename = ProceduralCover::FILENAME_240);
        let src_480 = format!("{prefix}{filename}?{hash}", filename = ProceduralCover::FILENAME_480);
        let src_720 = format!("{prefix}{filename}?{hash}", filename = ProceduralCover::FILENAME_720);

        let srcset = format!("{src_120} 120w,{src_240} 240w,{src_480} 480w,{src_720} 720w");

        ImgAttributes::new(src_720, srcset)
    }

    pub fn is_stale(&self) -> bool {
        self.marked_stale.is_some()
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let manifest_filename = format!("{}.{}.bincode", url_safe_base64(self.signature), ProceduralCover::CACHE_SERIALIZATION_KEY);
        cache_dir.join(manifest_filename)
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let manifest_path = self.manifest_path(cache_dir);
        let serialized = bincode::serialize(self).unwrap();
        fs::write(manifest_path, serialized).unwrap();
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}

impl ProceduralCoverAsset {
    pub fn new(build: &Build, filename: String) -> ProceduralCoverAsset {
        let metadata = fs::metadata(build.cache_dir.join(&filename)).unwrap();

        ProceduralCoverAsset {
            filename,
            filesize_bytes: metadata.len()
        }
    }
}

impl ProceduralCoverRc {
    pub fn borrow(&self) -> Ref<'_, ProceduralCover> {
        self.procedural_cover.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, ProceduralCover> {
        self.procedural_cover.borrow_mut()
    }

    pub fn new(procedural_cover: ProceduralCover) -> ProceduralCoverRc {
        ProceduralCoverRc {
            procedural_cover: Rc::new(RefCell::new(procedural_cover))
        }
    }
}
