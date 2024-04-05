use libvips::{VipsApp, VipsImage};
use libvips::ops::{self, Interesting, SmartcropOptions};
use std::path::Path;

use crate::{Build, ResizeMode, util};

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

	pub fn open(&self, build: &Build, path: &Path) -> ImageInMemory {
		let vips_image = VipsImage::new_from_file(&build.catalog_dir.join(path).to_string_lossy()).unwrap();

		ImageInMemory { vips_image }
	}

	pub fn resize(
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
		        &vips_image,
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
	                let resized = ops::resize(&image, max_edge_size as f64 / longer_edge as f64).unwrap();
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
		                let resized = ops::resize(&vips_image, edge_size as f64 / smaller_edge as f64).unwrap();
		                save(&resized)
		            }
	            };

	            if height != width {
	                let cropped = ops::smartcrop_with_opts(
	                    &image,
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
		                let resized = ops::resize(&vips_image, max_width as f64 / cropped_width as f64).unwrap();
		                save(&resized)
		            } else {
		                save(vips_image)
		            }
	            };

	            let found_aspect = width as f32 / height as f32;

	            if found_aspect < min_aspect {
	                // too tall, reduce height
	                let cropped = ops::smartcrop_with_opts(
	                    &image,
	                    width as i32,
	                    (width as f32 / min_aspect).floor() as i32,
	                    &CROP_OPTIONS
	                ).unwrap();
	                resize(&cropped)
	            } else if found_aspect > max_aspect {
	                // too wide, reduce width
	                let cropped = ops::smartcrop_with_opts(
	                    &image,
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
}