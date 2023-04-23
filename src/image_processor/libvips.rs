use libvips::{VipsApp, VipsImage};
use libvips::ops::{self, Interesting, SmartcropOptions};
use std::path::PathBuf;

use crate::{Build, ResizeMode, util};

pub struct ImageProcessor {
    pub vips_app: VipsApp
}

impl ImageProcessor {
	pub fn new() -> ImageProcessor {
        let vips_app = VipsApp::new("faircamp", false).expect("Cannot initialize libvips");

        vips_app.concurrency_set(2);

        ImageProcessor { vips_app }
	}

	pub fn resize(
		&self,
	    build: &Build,
	    path: &PathBuf,
	    resize_mode: ResizeMode
	) -> (String, (u32, u32)) {
	    let image = VipsImage::new_from_file(&build.catalog_dir.join(path).to_string_lossy()).unwrap();

	    let height = image.get_height() as u32;
	    let width = image.get_width() as u32;

	    let transformed = match resize_mode {
	        ResizeMode::ContainInSquare { max_edge_size } => {
	            info_resizing!("{:?} to contain within a square <= {}px", path, max_edge_size);

	            let longer_edge = std::cmp::max(height, width);

	            if longer_edge > max_edge_size {
	                ops::resize(&image, max_edge_size as f64 / longer_edge as f64).unwrap()
	            } else {
	                image
	            }
	        }
	        ResizeMode::CoverSquare { edge_size } => {
	            info_resizing!("{:?} to cover a square <= {}px", path, edge_size);

	            let smaller_edge = std::cmp::min(height, width);

	            let cropped_square = if height != width {
	                ops::smartcrop_with_opts(
	                    &image,
	                    smaller_edge as i32,
	                    smaller_edge as i32,
	                    &SmartcropOptions { interesting: Interesting::Centre }
	                ).unwrap()
	            } else {
	                image
	            };

	            if smaller_edge <= edge_size {
	                cropped_square
	            } else {
	                ops::resize(&cropped_square, edge_size as f64 / smaller_edge as f64).unwrap()
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
	                ops::smartcrop_with_opts(
	                    &image,
	                    width as i32,
	                    (width as f32 / min_aspect).floor() as i32,
	                    &SmartcropOptions { interesting: Interesting::Centre }
	                ).unwrap()
	            } else if found_aspect > max_aspect {
	                // too wide, reduce width
	                ops::smartcrop_with_opts(
	                    &image,
	                    (max_aspect * height as f32).floor() as i32,
	                    height as i32,
	                    &SmartcropOptions { interesting: Interesting::Centre }
	                ).unwrap()
	            } else {
	                image
	            };

	            let cropped_width = cropped_rectangle.get_width() as u32;
	            if cropped_width > max_width {
	                ops::resize(&cropped_rectangle, max_width as f64 / cropped_width as f64).unwrap()
	            } else {
	                cropped_rectangle
	            }
	        }
	    };

	    let options = ops::JpegsaveOptions {
	        interlace: true,
	        optimize_coding: true,
	        q: 80,
	        strip: true,
	        ..ops::JpegsaveOptions::default()
	    };

	    let target_filename = format!("{}.jpg", util::uid());

	    let result_dimensions = (
	    	transformed.get_width() as u32,
	    	transformed.get_height() as u32
	    );

	    match ops::jpegsave_with_opts(
	        &transformed,
	        &build.cache_dir.join(&target_filename).to_string_lossy(),
	        &options
	    ) {
	        Ok(_) => (),
	        Err(_) => println!("error: {}", self.vips_app.error_buffer().unwrap())
	    }

	    (target_filename, result_dimensions)
	}
}