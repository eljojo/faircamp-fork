#[cfg_attr(feature = "image", path = "image_processor/image.rs")]
#[cfg_attr(feature = "libvips", path = "image_processor/libvips.rs")]
mod implementation;

pub use implementation::ImageProcessor;

pub enum ResizeMode {
    /// Resize such that the longer edge of the image does not exceed the maximum edge size.
    ContainInSquare { max_edge_size: u32 },
    /// Perform a square crop, then resize to a maximum edge size.
    CoverSquare { edge_size: u32 },
    /// Perform a crop to a rectangle with a minimum aspect ratio if needed, then resize to a maximum width.
    /// Aspect ratio is width / height, e.g. 16/9 = 1.7777777
    CoverRectangle { max_aspect: f32, max_width: u32, min_aspect: f32 }
}