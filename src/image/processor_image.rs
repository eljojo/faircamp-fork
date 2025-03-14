// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::File;
use std::path::Path;

use ::image::{DynamicImage, ImageFormat};
use ::image::imageops::FilterType;

use crate::Build;
use crate::util;

use super::ResizeMode;

pub struct ImageInMemory {
    dynamic_image: DynamicImage
}

pub struct ImageProcessor;

impl ImageInMemory {
    pub fn width(&self) -> u32 {
        self.dynamic_image.width()
    }
}

impl ImageProcessor {
    pub fn new() -> ImageProcessor {
        ImageProcessor
    }

    /// Open an image and drop the alpha channel right away. This is used
    /// when we process images with jpeg as target format, i.e. for all
    /// user-supplied images.
    pub fn open_opaque(&self, absolute_path: &Path) -> ImageInMemory {
        let unknown_representation = image::open(absolute_path).unwrap();

        // Since image 0.25.0, alpha channels must be manually dropped before
        // saving to a format that does not support alpha channels. As we
        // export exclusively to jpeg formats in the
        // open_opaque/resize_opaque chain, we always drop any present alpha
        // channels right after we open any image for further processing.
        let dynamic_image = DynamicImage::ImageRgb8(unknown_representation.into_rgb8());

        ImageInMemory { dynamic_image }
    }

    // TODO: This was initially implemented to resize procedural covers (png format)
    //       to smaller sizes, but it turned out that generation at different sizes
    //       is both faster and visually more attractive. We're keeping around this
    //       code though because opening/resizing transparent images might be of
    //       interest at a future point.
    // /// Open an image and keep the alpha channel. This is used when we process
    // /// procedural cover images with transparent png as target format.
    // pub fn open_transparent(&self, absolute_path: &Path) -> ImageInMemory {
    //     let dynamic_image = image::open(absolute_path).unwrap();

    //     ImageInMemory { dynamic_image }
    // }

    /// Resizing for opaque images, targeting jpeg as output format. Coincidentally
    /// this is for all user-supplied images.
    pub fn resize_opaque(
        &self,
        build: &Build,
        image_in_memory: &ImageInMemory,
        resize_mode: ResizeMode
    ) -> (String, (u32, u32)) {
        let original = &image_in_memory.dynamic_image;

        let height = original.height();
        let width = original.width();

        let save = |dynamic_image: &DynamicImage| -> (String, (u32, u32)) {
            let result_dimensions = (dynamic_image.width(), dynamic_image.height());

            let output_filename = format!("{}.jpg", util::uid());
            let output_path = build.cache_dir.join(&output_filename);

            let mut output_file = File::create(output_path).unwrap();

            dynamic_image.write_to(&mut output_file, ImageFormat::Jpeg).unwrap();

            (output_filename, result_dimensions)
        };

        match resize_mode {
            ResizeMode::ContainInSquare { max_edge_size } => {
                let longer_edge = std::cmp::max(height, width);

                if longer_edge > max_edge_size {
                    let resize_factor = max_edge_size as f32 / longer_edge as f32;
                    let new_width = (width as f32 * resize_factor) as u32;
                    let new_height = (height as f32 * resize_factor) as u32;
                    let resized = original.resize(new_width, new_height, FilterType::Lanczos3);
                    save(&resized)
                } else {
                    save(original)
                }
            }
            ResizeMode::CoverSquare { edge_size } => {
                let smaller_edge = std::cmp::min(height, width);

                let resize = |dynamic_image: &DynamicImage| -> (String, (u32, u32)) {
                    if smaller_edge <= edge_size {
                        save(dynamic_image)
                    } else {
                        let resize_factor = edge_size as f32 / smaller_edge as f32;
                        let new_width = (width as f32 * resize_factor) as u32;
                        let new_height = (height as f32 * resize_factor) as u32;
                        let resized = dynamic_image.resize(new_width, new_height, FilterType::Lanczos3);
                        save(&resized)
                    }
                };

                if height == width {
                    resize(original)
                } else {
                    let cropped = if height > width {
                        let y = (height - width) / 2;
                        original.crop_imm(0, y, width, width)
                    } else {
                        let x = (width - height) / 2;
                        original.crop_imm(x, 0, height, height)
                    };

                    resize(&cropped)
                }
            }
            ResizeMode::CoverRectangle { max_aspect, max_width, min_aspect } => {
                let resize = |dynamic_image: &DynamicImage| -> (String, (u32, u32)) {
                    let cropped_width = dynamic_image.width();
                    if cropped_width > max_width {
                        let resize_factor = max_width as f32 / cropped_width as f32;
                        let new_width = (width as f32 * resize_factor) as u32;
                        let new_height = (height as f32 * resize_factor) as u32;
                        let resized = dynamic_image.resize(new_width, new_height, FilterType::Lanczos3);
                        save(&resized)
                    } else {
                        save(dynamic_image)
                    }
                };

                let found_aspect = width as f32 / height as f32;

                if found_aspect < min_aspect {
                    // too tall, reduce height
                    let new_height = (width as f32 / min_aspect).floor() as u32;
                    let y = (height - new_height) / 2;
                    let cropped = original.crop_imm(0, y, width, new_height);
                    resize(&cropped)
                } else if found_aspect > max_aspect {
                    // too wide, reduce width
                    let new_width = (max_aspect * height as f32).floor() as u32;
                    let x = (width - new_width) / 2;
                    let cropped = original.crop_imm(x, 0, new_width, height);
                    resize(&cropped)
                } else {
                    resize(original)
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
    //     _source_edge_size: u32, // Only used in the libvips implementation of this function
    //     target_edge_size: u32
    // ) -> String {
    //     let original = &image_in_memory.dynamic_image;
    //     let resized = original.resize(target_edge_size, target_edge_size, FilterType::Lanczos3);

    //     let output_filename = format!("{}.png", util::uid());
    //     let output_path = build.cache_dir.join(&output_filename);

    //     let mut output_file = File::create(output_path).unwrap();

    //     resized.write_to(&mut output_file, ImageFormat::Png).unwrap();

    //     output_filename
    // }
}
