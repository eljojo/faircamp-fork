// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::Path;

use libvips::{VipsApp, VipsImage};
use libvips::ops::{self, Interesting, SmartcropOptions};

use crate::Build;
use crate::util;

use super::ResizeMode;

const CROP_OPTIONS: SmartcropOptions = SmartcropOptions { interesting: Interesting::Centre };

pub struct ImageInMemory {
    pub vips_image: VipsImage
}

pub struct ImageProcessor {
    pub vips_app: VipsApp
}

impl ImageInMemory {
    pub fn width(&self) -> u32 {
        self.vips_image.get_width() as u32
    }
}

impl ImageProcessor {
    pub fn new() -> ImageProcessor {
        let vips_app = VipsApp::new("faircamp", false).expect("Cannot initialize libvips");

        vips_app.concurrency_set(2);

        ImageProcessor { vips_app }
    }

    /// In the libvips implementation open_opaque and open_transparent are
    /// identical, only for the image crate implementation the differentation
    /// is necessary.
    pub fn open_opaque(&self, absolute_path: &Path) -> ImageInMemory {
        let vips_image = VipsImage::new_from_file(&absolute_path.to_string_lossy()).unwrap();

        ImageInMemory { vips_image }
    }

    // TODO: This was initially implemented to resize procedural covers (png format)
    //       to smaller sizes, but it turned out that generation at different sizes
    //       is both faster and visually more attractive. We're keeping around this
    //       code though because opening/resizing transparent images might be of
    //       interest at a future point.
    // /// In the libvips implementation open_opaque and open_transparent are
    // /// identical, only for the image crate implementation the differentation
    // /// is necessary.
    // pub fn open_transparent(&self, absolute_path: &Path) -> ImageInMemory {
    //     let vips_image = VipsImage::new_from_file(&absolute_path.to_string_lossy()).unwrap();

    //     ImageInMemory { vips_image }
    // }

    /// Resizing for opaque images, targeting jpeg as output format. Coincidentally
    /// this is for all user-supplied images.
    pub fn resize_opaque(
        &self,
        build: &Build,
        image_in_memory: &ImageInMemory,
        resize_mode: ResizeMode
    ) -> (String, (u32, u32)) {
        let image = &image_in_memory.vips_image;

        let height = image.get_height() as u32;
        let width = image.get_width() as u32;

        let save = |vips_image: &VipsImage| -> (String, (u32, u32)) {
            let options = ops::JpegsaveOptions {
                interlace: true,
                optimize_coding: true,
                q: 80,
                strip: true,
                ..ops::JpegsaveOptions::default()
            };

            let target_filename = format!("{}.jpg", util::uid());

            match ops::jpegsave_with_opts(
                vips_image,
                &build.cache_dir.join(&target_filename).to_string_lossy(),
                &options
            ) {
                Ok(_) => (),
                Err(_) => println!("error: {}", self.vips_app.error_buffer().unwrap())
            }

            let result_dimensions = (
                vips_image.get_width() as u32,
                vips_image.get_height() as u32
            );

            (target_filename, result_dimensions)
        };

        match resize_mode {
            ResizeMode::ContainInSquare { max_edge_size } => {
                let longer_edge = std::cmp::max(height, width);

                if longer_edge > max_edge_size {
                    let resized = ops::resize(image, max_edge_size as f64 / longer_edge as f64).unwrap();
                    save(&resized)
                } else {
                    save(image)
                }
            }
            ResizeMode::CoverSquare { edge_size } => {
                let smaller_edge = std::cmp::min(height, width);

                let resize = |vips_image: &VipsImage| -> (String, (u32, u32)) {
                    if smaller_edge <= edge_size {
                        save(vips_image)
                    } else {
                        let resized = ops::resize(vips_image, edge_size as f64 / smaller_edge as f64).unwrap();
                        save(&resized)
                    }
                };

                if height != width {
                    let cropped = ops::smartcrop_with_opts(
                        image,
                        smaller_edge as i32,
                        smaller_edge as i32,
                        &CROP_OPTIONS
                    ).unwrap();
                    resize(&cropped)
                } else {
                    resize(image)
                }
            }
            ResizeMode::CoverRectangle { max_aspect, max_width, min_aspect } => {
                let resize = |vips_image: &VipsImage| -> (String, (u32, u32)) {
                    let cropped_width = vips_image.get_width() as u32;
                    if cropped_width > max_width {
                        let resized = ops::resize(vips_image, max_width as f64 / cropped_width as f64).unwrap();
                        save(&resized)
                    } else {
                        save(vips_image)
                    }
                };

                let found_aspect = width as f32 / height as f32;

                if found_aspect < min_aspect {
                    // too tall, reduce height
                    let cropped = ops::smartcrop_with_opts(
                        image,
                        width as i32,
                        (width as f32 / min_aspect).floor() as i32,
                        &CROP_OPTIONS
                    ).unwrap();
                    resize(&cropped)
                } else if found_aspect > max_aspect {
                    // too wide, reduce width
                    let cropped = ops::smartcrop_with_opts(
                        image,
                        (max_aspect * height as f32).floor() as i32,
                        height as i32,
                        &CROP_OPTIONS
                    ).unwrap();
                    resize(&cropped)
                } else {
                    resize(image)
                }
            }
        }
    }

    // TODO: This was initially implemented to resize procedural covers (png format)
    //       to smaller sizes, but it turned out that generation at different sizes
    //       is both faster and visually more attractive. We're keeping around this
    //       code though because opening/resizing transparent images might be of
    //       interest at a future point.
    // /// Resizing for transparent images, targeting png as output format. Coincidentally
    // /// this is for all procedurally generated cover images.
    // pub fn resize_transparent(
    //     &self,
    //     build: &Build,
    //     image_in_memory: &ImageInMemory,
    //     source_edge_size: u32,
    //     target_edge_size: u32
    // ) -> String {
    //     let original = &image_in_memory.vips_image;

    //     let resized = ops::resize(original, target_edge_size as f64 / source_edge_size as f64).unwrap();

    //     let output_filename = format!("{}.png", util::uid());

    //     let result = ops::pngsave(&resized, &build.cache_dir.join(&output_filename).to_string_lossy());

    //     if let Err(_) = result {
    //         println!("error: {}", self.vips_app.error_buffer().unwrap());
    //     }

    //     output_filename
    // }
}
