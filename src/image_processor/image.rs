use ::image::ImageOutputFormat;
use ::image::imageops::FilterType;
use std::fs::File;
use std::path::PathBuf;

use crate::{Build, ResizeMode, util};

pub struct ImageProcessor;

impl ImageProcessor {
	pub fn new() -> ImageProcessor {
		ImageProcessor
	}

	pub fn resize(
		&self,
	    build: &Build,
	    path: &PathBuf,
	    resize_mode: ResizeMode
	) -> (String, (u32, u32)) {
	    let original = image::open(build.catalog_dir.join(path)).unwrap();

	    let height = original.height();
	    let width = original.width();

	    let transformed = match resize_mode {
	        ResizeMode::ContainInSquare { max_edge_size } => {
	            info_resizing!("{:?} to contain within a square <= {}px", path, max_edge_size);

	            let longer_edge = std::cmp::max(height, width);

	            if longer_edge > max_edge_size {
	                let resize_factor = max_edge_size as f32 / longer_edge as f32;
	                let new_width = (width as f32 * resize_factor) as u32;
	                let new_height = (height as f32 * resize_factor) as u32;
	                original.resize(new_width, new_height, FilterType::Lanczos3)
	            } else {
	                original
	            }
	        }
	        ResizeMode::CoverSquare { edge_size } => {
	            info_resizing!("{:?} to cover a square <= {}px", path, edge_size);

	            let smaller_edge = std::cmp::min(height, width);

	            #[allow(clippy::comparison_chain)]
	            let cropped_square = if height > width {
	                let y = (height - width) / 2;
	                original.crop_imm(0, y, width, width)
	            } else if height < width {
	                let x = (width - height) / 2;
	                original.crop_imm(x, 0, height, height)
	            } else {
	                original
	            };

	            if smaller_edge <= edge_size {
	                cropped_square
	            } else {
	                let resize_factor = edge_size as f32 / smaller_edge as f32;
	                let new_width = (width as f32 * resize_factor) as u32;
	                let new_height = (height as f32 * resize_factor) as u32;
	                cropped_square.resize(new_width, new_height, FilterType::Lanczos3)
	            }
	        }
	        ResizeMode::CoverRectangle { max_aspect, max_width, min_aspect } => {
	            // TODO: These messages are probably rather confusing to the person running faircamp
	            //       Maybe reconsider how they should be worded, or what exactly is reported, e.g.
	            //       only reporting (once!) that the image is resized to be used as an artist image.
	            //       It's not important to the reader what exact resizing to which sizes is done.
	            //       (this is rather debug level info)
	            info_resizing!("{:?} to cover a {}-{} aspect ratio rectangle <= {}px", path, min_aspect, max_aspect, max_width);

	            let found_aspect = width as f32 / height as f32;
	            let cropped_rectangle = if found_aspect < min_aspect {
	                // too tall, reduce height
	                let new_height = (width as f32 / min_aspect).floor() as u32;
	                let y = (height - new_height) / 2;
	                original.crop_imm(0, y, width, new_height)
	            } else if found_aspect > max_aspect {
	                // too wide, reduce width
	                let new_width = (max_aspect * height as f32).floor() as u32;
	                let x = (width - new_width) / 2;
	                original.crop_imm(x, 0, new_width, height)
	            } else {
	                original
	            };

	            let cropped_width = cropped_rectangle.width();
	            if cropped_width > max_width {
	                let resize_factor = max_width as f32 / cropped_width as f32;
	                let new_width = (width as f32 * resize_factor) as u32;
	                let new_height = (height as f32 * resize_factor) as u32;
	                cropped_rectangle.resize(new_width, new_height, FilterType::Lanczos3)
	            } else {
	                cropped_rectangle
	            }
	        }
	    };

	    let result_dimensions = (transformed.width(), transformed.height());

	    let output_filename = format!("{}.jpg", util::uid());
	    let output_path = build.cache_dir.join(&output_filename);

	    let mut output_file = File::create(output_path).unwrap();

	    transformed.write_to(&mut output_file, ImageOutputFormat::Jpeg(80)).unwrap();

	    (output_filename, result_dimensions)
	}
}