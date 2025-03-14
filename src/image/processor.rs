// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(not(any(feature = "image", feature = "libvips")))]
compile_error!(r#"An image processing feature needs to be enabled, re-run your last command with either "--features image" added (pick this if you're unsure which to pick) or "--features libvips" (pick this if you know exactly what you're doing)"#);

#[cfg(all(feature = "image", feature = "libvips"))]
compile_error!(r#"Only one image processing feature can be enabled, remove either "--features image" or "--features libvips" from your last command"#);

#[cfg_attr(feature = "image", path = "processor_image.rs")]
#[cfg_attr(feature = "libvips", path = "processor_libvips.rs")]
mod implementation;

pub use implementation::{ImageInMemory, ImageProcessor};

pub enum ResizeMode {
    /// Resize such that the longer edge of the image does not exceed the maximum edge size.
    ContainInSquare { max_edge_size: u32 },
    /// Perform a square crop, then resize to a maximum edge size.
    CoverSquare { edge_size: u32 },
    /// Perform a crop to a rectangle with a minimum aspect ratio if needed, then resize to a maximum width.
    /// Aspect ratio is width / height, e.g. 16/9 = 1.7777777
    CoverRectangle { max_aspect: f32, max_width: u32, min_aspect: f32 }
}
