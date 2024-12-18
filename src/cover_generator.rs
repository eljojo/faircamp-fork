// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::f32::consts::TAU;

use indoc::formatdoc;

use crate::Release;

#[derive(Clone, Debug, Hash)]
pub enum CoverGenerator {
    BestRillen,
    GlassSplinters,
    LooneyTunes,
    ScratchyFaintRillen,
    SpaceTimeRupture
}

impl CoverGenerator {
    pub fn generate(
        &self,
        label: &str,
        release: &Release,
        max_tracks_in_release: usize
    ) -> String {
       match self {
            CoverGenerator::BestRillen => CoverGenerator::generate_best_rillen(label, release),
            CoverGenerator::GlassSplinters => CoverGenerator::generate_glass_splinters(label, release),
            CoverGenerator::LooneyTunes => CoverGenerator::generate_looney_tunes(label, release, max_tracks_in_release),
            CoverGenerator::ScratchyFaintRillen => CoverGenerator::generate_scratchy_faint_rillen(label, release),
            CoverGenerator::SpaceTimeRupture => CoverGenerator::generate_space_time_rupture(label, release)
        }
    }

    fn generate_best_rillen(label: &str, release: &Release) -> String {
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_track_duration = release.longest_track_duration();

        let mut track_offset = 0.0;
        let points = release.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.transcodes.borrow().source_meta;

                let altitude_width = radius / release.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds / longest_track_duration;

                let mut samples = Vec::new();
                let step = 2;

                let mut previous = None;

                let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

                for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                    let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                    let x_vector = ((track_compensation + peak_offset * track_arc_range) * TAU).sin();
                    let y_vector = ((track_compensation + peak_offset * track_arc_range) * TAU).cos();

                    let x = radius + ((release.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * 0.3 * altitude_width) * x_vector;
                    let y = radius + ((release.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * 0.3 * altitude_width) * y_vector;

                    if let Some((x_prev, y_prev)) = previous {
                        let stroke_width = peak * 0.24; // .06px is our ideal for waveforms
                        let sample = format!(r##"<line stroke="var(--fg-1)" stroke-opacity="{peak}" stroke-width="{stroke_width}px" x1="{x_prev}" x2="{x}" y1="{y_prev}" y2="{y}"/>"##);
                        samples.push(sample);
                    }

                    previous = Some((x, y));
                }

                track_offset += track_arc_range;

                samples.join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r##"
            <svg width="20em" height="20em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <title>{label}</title>
                {points}
            </svg>
        "##)
    }

    fn generate_glass_splinters(label: &str, release: &Release) -> String {
        let edge = 64.0;

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
        let points = release.tracks
            .iter()
            .map(|track| {
                let source_meta = &track.transcodes.borrow().source_meta;

                let track_arc_range = source_meta.duration_seconds / total_duration;

                let mut samples = Vec::new();
                let step = 4;

                for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                    let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32; // 0-1

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

                format!(r##"<path d="{d}" fill="none" stroke="var(--fg-1)" stroke-width=".06px"/>"##)

            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r#"
            <svg width="20em" height="20em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <title>{label}</title>
                {points}
            </svg>
        "#)
    }

    fn generate_looney_tunes(label: &str, release: &Release, max_tracks_in_release: usize) -> String {
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_track_duration = release.longest_track_duration();

        let mut track_offset = 0.0;
        let points = release.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.transcodes.borrow().source_meta;

                let altitude_range = 0.75 * release.tracks.len() as f32 / max_tracks_in_release as f32;
                let altitude_width = radius * altitude_range / release.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds / longest_track_duration;

                let mut samples = Vec::new();
                let step = 1;

                let mut previous = None;

                let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

                for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                    let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                    let arc_offset = (track_compensation + peak_offset * track_arc_range) * TAU;
                    let amplitude =
                        radius * 0.25 +
                        (max_tracks_in_release - 1 - track_index) as f32 * altitude_width +
                        (peak * 0.3 * altitude_width);

                    let x = radius + amplitude * arc_offset.sin();
                    let y = radius + amplitude * arc_offset.cos();

                    if let Some((x_prev, y_prev)) = previous {
                        let stroke_width = peak * 0.32;
                        let sample = format!(r##"<line stroke="var(--fg-1)" stroke-opacity="{peak}" stroke-width="{stroke_width}px" x1="{x_prev}" x2="{x}" y1="{y_prev}" y2="{y}"/>"##);
                        samples.push(sample);
                    }

                    previous = Some((x, y));
                }

                track_offset += track_arc_range;

                samples.join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r##"
            <svg width="20em" height="20em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <rect fill="transparent" height="64" width="64" x="0" y="0"/>
                <title>{label}</title>
                {points}
            </svg>
        "##)
    }

    fn generate_scratchy_faint_rillen(label: &str, release: &Release) -> String {
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_track_duration = release.longest_track_duration();

        let mut track_offset = 0.0;
        let points = release.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.transcodes.borrow().source_meta;

                let altitude_width = radius / release.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds / longest_track_duration;

                let mut samples = Vec::new();
                let step = 2;

                for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                    let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32; // 0-1

                    let x_vector = (peak_offset * track_arc_range * TAU).sin();
                    let y_vector = (peak_offset * track_arc_range * TAU).cos();

                    let x = radius + ((release.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * altitude_width) * x_vector;
                    let y = radius + ((release.tracks.len() - 1 - track_index) as f32 * altitude_width + peak * altitude_width) * y_vector;

                    let command = if peak_index == 0 { "M" } else { "L" };
                    let sample = format!("{command} {x} {y}");

                    samples.push(sample);
                }

                let d = samples.join(" ");

                track_offset += track_arc_range;

                format!(r##"<path d="{d}" fill="none" stroke="var(--fg-1)" stroke-width=".06px"/>"##)
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r#"
            <svg width="20em" height="20em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <title>{label}</title>
                {points}
            </svg>
        "#)
    }

    fn generate_space_time_rupture(label: &str, release: &Release) -> String {
        let edge = 64.0;

        let total_duration: f32 = release.tracks
            .iter()
            .map(|track| track.transcodes.borrow().source_meta.duration_seconds)
            .sum();

        let shortest_track_duration = release.shortest_track_duration();

        let longest_track_duration = release.longest_track_duration();

        let mut track_offset = 0.0;
        let points = release.tracks
            .iter()
            .map(|track| {
                let source_meta = &track.transcodes.borrow().source_meta;

                let altitude_factor = (source_meta.duration_seconds - shortest_track_duration) / (longest_track_duration - shortest_track_duration);
                let track_arc_range = source_meta.duration_seconds / total_duration;

                let mut samples = Vec::new();
                let step = 6;

                for (peak_index, peak) in source_meta.peaks.iter().step_by(step).enumerate() {
                    let peak_offset = peak_index as f32 / (source_meta.peaks.len() - 1) as f32 * step as f32; // 0-1

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

                format!(r##"<path d="{d}" fill="none" stroke="var(--fg-1)" stroke-width=".06px"/>"##)
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r#"
            <svg width="20em" height="20em" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <title>{label}</title>
                {points}
            </svg>
        "#)
    }
}