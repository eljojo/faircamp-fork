use chrono::{DateTime, Duration, Utc};
use libvips::{ops::{self, Interesting, SmartcropOptions}, VipsImage};
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc
};

use crate::{
    Asset,
    AssetIntent,
    Build,
    Cache,
    CacheOptimization,
    SourceFileSignature,
    util
};

const BACKGROUND_MAX_EDGE_SIZE: i32 = 1280;
const FEED_MAX_EDGE_SIZE: i32 = 920;

/// A single, resized version of the artist image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAsset {
    pub filename: String,
    pub filesize_bytes: u64,
    pub format: String,
    pub height: i32,
    pub width: i32
}

/// Represents multiple, differently sized versions of an artist image, for
/// display on different screen sizes. (Numbers refer to maximum width)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArtistAssets {
    pub fixed_max_320: ArtistAsset,
    pub fixed_max_480: Option<ArtistAsset>,
    pub fixed_max_640: Option<ArtistAsset>,
    pub fluid_max_640: ArtistAsset,
    pub fluid_max_960: Option<ArtistAsset>,
    pub fluid_max_1280: Option<ArtistAsset>,
    pub marked_stale: Option<DateTime<Utc>>
}

/// A single, resized version of the cover image.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverAsset {
    /// Represents both height and width (covers have a square aspect ratio)
    pub edge_size: i32,
    pub filename: String,
    pub filesize_bytes: u64
}

/// Represents multiple, differently sized versions of a cover image, for
/// display on different screen sizes and for inclusion in the release
/// archive. (Numbers refer to the square edge size, both height and width)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoverAssets {
    pub marked_stale: Option<DateTime<Utc>>,
    pub max_160: CoverAsset,
    pub max_320: Option<CoverAsset>,
    pub max_480: Option<CoverAsset>,
    pub max_800: Option<CoverAsset>,
    pub max_1280: Option<CoverAsset>
}

/// Associates image assets with an image description
#[derive(Debug)]
pub struct Image {
    pub assets: Rc<RefCell<ImageAssets>>,
    pub description: Option<String>
}

/// Represents a source image file in the catalog and all its generated
/// (compressed/resized) derived versions.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageAssets {
    pub artist: Option<ArtistAssets>,
    pub background: Option<Asset>,
    pub cover: Option<CoverAssets>,
    pub feed: Option<Asset>,
    pub source_file_signature: SourceFileSignature,
    pub uid: String
}

pub struct ImgAttributes {
    pub src: String,
    pub srcset: String
}

enum ResizeMode {
    /// Resize such that the longer edge of the image does not exceed the maximum edge size.
    ContainInSquare { max_edge_size: i32 },
    /// Perform a square crop, then resize to a maximum edge size.
    CoverSquare { edge_size: i32 },
    /// Perform a crop to a rectangle with a minimum aspect ratio if needed, then resize to a maximum width.
    /// Aspect ratio is width / height, e.g. 16/9 = 1.7777777
    CoverRectangle { max_aspect: f32, max_width: i32, min_aspect: f32 }
}

fn resize(
    build: &Build,
    path: &PathBuf,
    resize_mode: ResizeMode
) -> (String, (i32, i32)) {
    let image = VipsImage::new_from_file(&build.catalog_dir.join(path).to_string_lossy()).unwrap();

    let height = image.get_height();
    let width = image.get_width();

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
                    smaller_edge,
                    smaller_edge,
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
                    width,
                    (width as f32 / min_aspect).floor() as i32,
                    &SmartcropOptions { interesting: Interesting::Centre }
                ).unwrap()
            } else if found_aspect > max_aspect {
                // too wide, reduce width
                ops::smartcrop_with_opts(
                    &image,
                    (max_aspect * height as f32).floor() as i32,
                    height,
                    &SmartcropOptions { interesting: Interesting::Centre }
                ).unwrap()
            } else {
                image
            };

            let cropped_width = cropped_rectangle.get_width();
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

    let result_dimensions = (transformed.get_width(), transformed.get_height());

    match ops::jpegsave_with_opts(
        &transformed,
        &build.cache_dir.join(&target_filename).to_string_lossy(),
        &options
    ) {
        Ok(_) => (),
        Err(_) => println!("error: {}", build.libvips_app.error_buffer().unwrap())
    }

    (target_filename, result_dimensions)
}

impl ArtistAssets {
    pub fn all(&self) -> Vec<&ArtistAsset> {
        let mut result = Vec::with_capacity(4);

        result.push(&self.fixed_max_320);
        if let Some(asset) = &self.fixed_max_480 { result.push(asset); }
        if let Some(asset) = &self.fixed_max_640 { result.push(asset); }
        result.push(&self.fluid_max_640);
        if let Some(asset) = &self.fluid_max_960 { result.push(asset); }
        if let Some(asset) = &self.fluid_max_1280 { result.push(asset); }

        result
    }

    pub fn img_attributes_fixed(&self, permalink: &str, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fixed_max_320);
        if let Some(asset) = &self.fixed_max_480 { assets.push(asset); }
        if let Some(asset) = &self.fixed_max_640 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, permalink, prefix)
    }

    pub fn img_attributes_fluid(&self, permalink: &str, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.fluid_max_640);
        if let Some(asset) = &self.fluid_max_960 { assets.push(asset); }
        if let Some(asset) = &self.fluid_max_1280 { assets.push(asset); }

        ImgAttributes::new_for_artist(assets, permalink, prefix)
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    pub fn obsolete(&self, build: &Build) -> bool {
        match &self.marked_stale {
            Some(marked_stale) => {
                match &build.cache_optimization {
                    CacheOptimization::Default | 
                    CacheOptimization::Delayed => 
                        build.build_begin.signed_duration_since(*marked_stale) > Duration::hours(24),
                    CacheOptimization::Immediate |
                    CacheOptimization::Manual |
                    CacheOptimization::Wipe => true
                }
            },
            None => false
        }
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}

impl CoverAssets {
    pub fn all(&self) -> Vec<&CoverAsset> {
        let mut result = Vec::with_capacity(4);

        result.push(&self.max_160);
        if let Some(asset) = &self.max_320 { result.push(asset); }
        if let Some(asset) = &self.max_480 { result.push(asset); }
        if let Some(asset) = &self.max_800 { result.push(asset); }
        if let Some(asset) = &self.max_1280 { result.push(asset); }

        result
    }

    pub fn img_attributes_up_to_320(&self, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_320 {
            Some(max_320) => vec![&self.max_160, max_320],
            None => vec![&self.max_160]
        };

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn img_attributes_up_to_480(&self, prefix: &str) -> ImgAttributes {
        let assets = match &self.max_320 {
            Some(max_320) => match &self.max_480 {
                Some(max_480) => vec![&self.max_160, max_320, max_480],
                None => vec![&self.max_160, max_320]
            }
            None => vec![&self.max_160]
        };

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn img_attributes_up_to_1280(&self, prefix: &str) -> ImgAttributes {
        let mut assets = Vec::with_capacity(4);

        assets.push(&self.max_160);
        if let Some(asset) = &self.max_320 { assets.push(asset); }
        if let Some(asset) = &self.max_480 { assets.push(asset); }
        if let Some(asset) = &self.max_800 { assets.push(asset); }
        if let Some(asset) = &self.max_1280 { assets.push(asset); }

        ImgAttributes::new_for_cover(assets, prefix)
    }

    pub fn largest(&self) -> &CoverAsset {
        if let Some(max_1280) = &self.max_1280 {
            max_1280
        } else if let Some(max_800) = &self.max_800 {
            max_800
        } else if let Some(max_480) = &self.max_480 {
            max_480
        } else if let Some(max_320) = &self.max_320 {
            max_320
        } else {
            &self.max_160
        }
    }

    pub fn mark_stale(&mut self, timestamp: &DateTime<Utc>) {
        if self.marked_stale.is_none() {
            self.marked_stale = Some(*timestamp);
        }
    }

    pub fn obsolete(&self, build: &Build) -> bool {
        match &self.marked_stale {
            Some(marked_stale) => {
                match &build.cache_optimization {
                    CacheOptimization::Default | 
                    CacheOptimization::Delayed => 
                        build.build_begin.signed_duration_since(*marked_stale) > Duration::hours(24),
                    CacheOptimization::Immediate |
                    CacheOptimization::Manual |
                    CacheOptimization::Wipe => true
                }
            },
            None => false
        }
    }

    pub fn unmark_stale(&mut self) {
        self.marked_stale = None;
    }
}

impl Image {
    pub fn new(assets: Rc<RefCell<ImageAssets>>, description: Option<String>) -> Image {
        Image {
            assets,
            description
        }
    }
}

impl ImageAssets {
    pub fn artist_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut ArtistAssets {
        if let Some(assets) = self.artist.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                assets.unmark_stale();
            }
        } else {
            // Compute fixed sizes.
            // Viewport width < 30rem (480px at 16px font-size) = 100vw/40vw = 2.5
            // Viewport width > 60rem (960px at 16px font-size) = 27rem/12rem = 2.25
            // We therefore approximate it for both by limiting the aspect to 2.25.-2.5

            let (fixed_filename_320, fixed_dimensions_320) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CoverRectangle {
                    max_aspect: 2.5,
                    max_width: 320,
                    min_aspect: 2.25
                }
            );

            let fixed_metadata_320 = fs::metadata(build.cache_dir.join(&fixed_filename_320)).unwrap();

            let fixed_max_320 = ArtistAsset {
                filename: fixed_filename_320,
                filesize_bytes: fixed_metadata_320.len(),
                format: String::from("fixed"),
                height: fixed_dimensions_320.1,
                width: fixed_dimensions_320.0
            };

            let mut fixed_max_480 = None;
            let mut fixed_max_640 = None;

            if fixed_dimensions_320.0 == 320 {
                let (fixed_filename_480, fixed_dimensions_480) = resize(
                    build,
                    &self.source_file_signature.path,
                    ResizeMode::CoverRectangle {
                        max_aspect: 2.5,
                        max_width: 480,
                        min_aspect: 2.25
                    }
                );

                let fixed_metadata_480 = fs::metadata(build.cache_dir.join(&fixed_filename_480)).unwrap();

                fixed_max_480 = Some(ArtistAsset {
                    filename: fixed_filename_480,
                    filesize_bytes: fixed_metadata_480.len(),
                    format: String::from("fixed"),
                    height: fixed_dimensions_480.1,
                    width: fixed_dimensions_480.0
                });

                if fixed_dimensions_480.0 == 480 {
                    let (fixed_filename_640, fixed_dimensions_640) = resize(
                        build,
                        &self.source_file_signature.path,
                        ResizeMode::CoverRectangle {
                            max_aspect: 2.5,
                            max_width: 640,
                            min_aspect: 2.25
                        }
                    );

                    let fixed_metadata_640 = fs::metadata(build.cache_dir.join(&fixed_filename_640)).unwrap();

                    fixed_max_640 = Some(ArtistAsset {
                        filename: fixed_filename_640,
                        filesize_bytes: fixed_metadata_640.len(),
                        format: String::from("fixed"),
                        height: fixed_dimensions_640.1,
                        width: fixed_dimensions_640.0
                    });
                }
            }

            // Compute fluid sizes
            // Viewport width @ 30rem (480px at 16px font-size) = 100vw=30rem/12rem = 2.5
            // Viewport width @ 60rem (960px at 16px font-size) = 100vw=960px/12rem = 5
            // We therefore approximate it for both by limiting the aspect to 2.5-5

            let (fluid_filename_640, fluid_dimensions_640) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CoverRectangle {
                    max_aspect: 5.0,
                    max_width: 640,
                    min_aspect: 2.5
                }
            );

            let fluid_metadata_640 = fs::metadata(build.cache_dir.join(&fluid_filename_640)).unwrap();

            let fluid_max_640 = ArtistAsset {
                filename: fluid_filename_640,
                filesize_bytes: fluid_metadata_640.len(),
                format: String::from("fluid"),
                height: fluid_dimensions_640.1,
                width: fluid_dimensions_640.0
            };

            let mut fluid_max_960 = None;
            let mut fluid_max_1280 = None;

            if fluid_dimensions_640.0 == 640 {
                let (fluid_filename_960, fluid_dimensions_960) = resize(
                    build,
                    &self.source_file_signature.path,
                    ResizeMode::CoverRectangle {
                        max_aspect: 5.0,
                        max_width: 960,
                        min_aspect: 2.5
                    }
                );

                let fluid_metadata_960 = fs::metadata(build.cache_dir.join(&fluid_filename_960)).unwrap();

                fluid_max_960 = Some(ArtistAsset {
                    filename: fluid_filename_960,
                    filesize_bytes: fluid_metadata_960.len(),
                    format: String::from("fluid"),
                    height: fluid_dimensions_960.1,
                    width: fluid_dimensions_960.0
                });

                if fluid_dimensions_960.0 == 960 {
                    let (fluid_filename_1280, fluid_dimensions_1280) = resize(
                        build,
                        &self.source_file_signature.path,
                        ResizeMode::CoverRectangle {
                            max_aspect: 5.0,
                            max_width: 1280,
                            min_aspect: 2.5
                        }
                    );

                    let fluid_metadata_1280 = fs::metadata(build.cache_dir.join(&fluid_filename_1280)).unwrap();

                    fluid_max_1280 = Some(ArtistAsset {
                        filename: fluid_filename_1280,
                        filesize_bytes: fluid_metadata_1280.len(),
                        format: String::from("fluid"),
                        height: fluid_dimensions_1280.1,
                        width: fluid_dimensions_1280.0
                    });
                }
            }

            let artist_assets = ArtistAssets {
                fixed_max_320,
                fixed_max_480,
                fixed_max_640,
                fluid_max_640,
                fluid_max_960,
                fluid_max_1280,
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                }
            };

            self.artist.replace(artist_assets);
        }

        self.artist.as_mut().unwrap()
    }

    pub fn background_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut Asset {
        if let Some(asset) = self.background.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let (filename, _dimensions) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::ContainInSquare { max_edge_size: BACKGROUND_MAX_EDGE_SIZE }
            );

            self.background.replace(Asset::new(build, filename, asset_intent));
        }

        self.background.as_mut().unwrap()
    }

    pub fn cover_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut CoverAssets {
        if let Some(assets) = self.cover.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                assets.unmark_stale();
            }
        } else {
            let (filename_160, dimensions_160) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::CoverSquare { edge_size: 160 }
            );

            let metadata_160 = fs::metadata(build.cache_dir.join(&filename_160)).unwrap();

            let max_160 = CoverAsset {
                edge_size: dimensions_160.0,
                filename: filename_160,
                filesize_bytes: metadata_160.len()
            };

            let mut max_320 = None;
            let mut max_480 = None;
            let mut max_800 = None;
            let mut max_1280 = None;

            if dimensions_160.0 == 160 {
                let (filename_320, dimensions_320) = resize(
                    build,
                    &self.source_file_signature.path,
                    ResizeMode::CoverSquare { edge_size: 320 }
                );

                let metadata_320 = fs::metadata(build.cache_dir.join(&filename_320)).unwrap();

                max_320 = Some(CoverAsset {
                    edge_size: dimensions_320.0,
                    filename: filename_320,
                    filesize_bytes: metadata_320.len()
                });

                if dimensions_320.0 == 320 {
                    let (filename_480, dimensions_480) = resize(
                        build,
                        &self.source_file_signature.path,
                        ResizeMode::CoverSquare { edge_size: 480 }
                    );

                    let metadata_480 = fs::metadata(build.cache_dir.join(&filename_480)).unwrap();

                    max_480 = Some(CoverAsset {
                        edge_size: dimensions_480.0,
                        filename: filename_480,
                        filesize_bytes: metadata_480.len()
                    });

                    if dimensions_480.0 == 480 {
                        let (filename_800, dimensions_800) = resize(
                            build,
                            &self.source_file_signature.path,
                            ResizeMode::CoverSquare { edge_size: 800 }
                        );

                        let metadata_800 = fs::metadata(build.cache_dir.join(&filename_800)).unwrap();

                        max_800 = Some(CoverAsset {
                            edge_size: dimensions_800.0,
                            filename: filename_800,
                            filesize_bytes: metadata_800.len()
                        });

                        if dimensions_800.0 == 800 {
                            let (filename_1280, dimensions_1280) = resize(
                                build,
                                &self.source_file_signature.path,
                                ResizeMode::CoverSquare { edge_size: 1280 }
                            );

                            let metadata_1280 = fs::metadata(build.cache_dir.join(&filename_1280)).unwrap();

                            max_1280 = Some(CoverAsset {
                                edge_size: dimensions_1280.0,
                                filename: filename_1280,
                                filesize_bytes: metadata_1280.len()
                            });
                        }
                    }
                }
            }

            let cover_assets = CoverAssets {
                marked_stale: match asset_intent {
                    AssetIntent::Deliverable => None,
                    AssetIntent::Intermediate => Some(build.build_begin)
                },
                max_160,
                max_320,
                max_480,
                max_800,
                max_1280
            };

            self.cover.replace(cover_assets);
        }

        self.cover.as_mut().unwrap()
    }

    pub fn deserialize_cached(path: &Path) -> Option<ImageAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<ImageAssets>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn feed_asset(
        &mut self,
        build: &Build,
        asset_intent: AssetIntent
    ) -> &mut Asset {
        if let Some(asset) = self.feed.as_mut() {
            if asset_intent == AssetIntent::Deliverable {
                asset.unmark_stale();
            }
        } else {
            let (filename, _dimensions) = resize(
                build,
                &self.source_file_signature.path,
                ResizeMode::ContainInSquare { max_edge_size: FEED_MAX_EDGE_SIZE }
            );

            self.feed.replace(Asset::new(build, filename, asset_intent));
        }

        self.feed.as_mut().unwrap()
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(Cache::MANIFEST_IMAGES_DIR).join(filename)
    }
    
    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        if let Some(asset) = self.artist.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.background.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.cover.as_mut() { asset.mark_stale(timestamp); }
        if let Some(asset) = self.feed.as_mut() { asset.mark_stale(timestamp); }
    }
    
    pub fn new(source_file_signature: SourceFileSignature) -> ImageAssets {
        ImageAssets {
            artist: None,
            background: None,
            cover: None,
            feed: None,
            source_file_signature,
            uid: util::uid()
        }
    }
    
    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
    }
}

impl ImgAttributes {
    /// Assets MUST be passed in ascending size
    pub fn new_for_artist(
        assets_ascending_by_size: Vec<&ArtistAsset>,
        permalink: &str,
        prefix: &str
    ) -> ImgAttributes {
        let mut src = String::new();
        let mut srcset = Vec::new();

        let mut asset_peek_iter = assets_ascending_by_size.iter().peekable();

        while let Some(asset) = asset_peek_iter.next() {
            srcset.push(format!("{}{}_{}_{}x{}.jpg {}w", prefix, permalink, asset.format, asset.width, asset.height, asset.width));

            if asset_peek_iter.peek().is_none() {
                src = format!("{}{}_{}_{}x{}.jpg", prefix, permalink, asset.format, asset.width, asset.height);
            }
        }

        ImgAttributes {
            src,
            srcset: srcset.join(",")
        }
    }

    /// Assets MUST be passed in ascending size
    pub fn new_for_cover(assets_ascending_by_size: Vec<&CoverAsset>, prefix: &str) -> ImgAttributes {
        let mut src = String::new();
        let mut srcset = Vec::new();

        let mut asset_peek_iter = assets_ascending_by_size.iter().peekable();

        while let Some(asset) = asset_peek_iter.next() {
            srcset.push(format!("{}cover_{}.jpg {}w", prefix, asset.edge_size, asset.edge_size));

            if asset_peek_iter.peek().is_none() {
                src = format!("{prefix}cover_{edge_size}.jpg", edge_size = asset.edge_size);
            }
        }

        ImgAttributes {
            src,
            srcset: srcset.join(",")
        }
    }
}