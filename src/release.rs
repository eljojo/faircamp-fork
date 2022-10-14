use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
    rc::Rc
};
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    Artist,
    Asset,
    AssetIntent,
    AudioFormat,
    Build,
    CacheManifest,
    Catalog,
    DownloadOption,
    Image,
    ImageFormat,
    manifest::Overrides,
    PaymentOption,
    Permalink,
    render,
    SourceFileSignature,
    TagMapping,
    Track,
    util
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedReleaseAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub flac: Option<Asset>,
    pub mp3: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub opus_48: Option<Asset>,
    pub opus_96: Option<Asset>,
    pub opus_128: Option<Asset>,
    pub source_file_signatures: Vec<SourceFileSignature>,
    pub uid: String,
    pub wav: Option<Asset>
}

#[derive(Debug)]
pub struct Release {
    pub artists: Vec<Rc<RefCell<Artist>>>,
    pub artists_to_map: Vec<String>,
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download asset filenames.
    pub asset_basename: Option<String>,
    pub cached_assets: CachedReleaseAssets,
    pub cover: Option<Rc<RefCell<Image>>>,
    pub download_formats: Vec<AudioFormat>,
    pub download_option: DownloadOption,
    pub embedding: bool,
    pub payment_options: Vec<PaymentOption>,
    pub permalink: Permalink,
    pub rewrite_tags: bool,
    pub runtime: u32,
    pub streaming_format: AudioFormat,
    pub text: Option<String>,
    pub title: String,
    pub track_numbering: TrackNumbering,
    pub tracks: Vec<Track>
}

#[derive(Clone, Debug)]
pub enum TrackNumbering {
    Disabled,
    Arabic,
    Hexadecimal,
    Roman
}

impl CachedReleaseAssets {
    pub fn deserialize(path: &Path) -> Option<CachedReleaseAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<CachedReleaseAssets>(&bytes).ok(),
            Err(_) => None
        }
    }

    pub fn get(&self, format: AudioFormat) -> &Option<Asset> {
        match format {
            AudioFormat::Aac => &self.aac,
            AudioFormat::Aiff => &self.aiff,
            AudioFormat::Flac => &self.flac,
            AudioFormat::Mp3VbrV0 => &self.mp3,
            AudioFormat::OggVorbis => &self.ogg_vorbis,
            AudioFormat::Opus48Kbps => &self.opus_48,
            AudioFormat::Opus96Kbps => &self.opus_96,
            AudioFormat::Opus128Kbps => &self.opus_128,
            AudioFormat::Wav => &self.wav
        }
    }

    pub fn get_mut(&mut self, format: AudioFormat) -> &mut Option<Asset> {
        match format {
            AudioFormat::Aac => &mut self.aac,
            AudioFormat::Aiff => &mut self.aiff,
            AudioFormat::Flac => &mut self.flac,
            AudioFormat::Mp3VbrV0 => &mut self.mp3,
            AudioFormat::OggVorbis => &mut self.ogg_vorbis,
            AudioFormat::Opus48Kbps => &mut self.opus_48,
            AudioFormat::Opus96Kbps => &mut self.opus_96,
            AudioFormat::Opus128Kbps => &mut self.opus_128,
            AudioFormat::Wav => &mut self.wav
        }
    }

    pub fn manifest_path(&self, cache_dir: &Path) -> PathBuf {
        let filename = format!("{}.bincode", self.uid);
        cache_dir.join(CacheManifest::MANIFEST_RELEASES_DIR).join(filename)
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for format in AudioFormat::ALL_FORMATS {
            if let Some(asset) = self.get_mut(format) {
                asset.mark_stale(timestamp);
            }
        }
    }

    pub fn new(source_file_signatures: Vec<SourceFileSignature>) -> CachedReleaseAssets {
        CachedReleaseAssets {
            aac: None,
            aiff: None,
            flac: None,
            mp3: None,
            ogg_vorbis: None,
            opus_48: None,
            opus_96: None,
            opus_128: None,
            source_file_signatures,
            uid: util::uid(),
            wav: None
        }
    }

    pub fn persist(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), &serialized).unwrap();
    }
}

impl Release {
    pub fn new(
        artists_to_map: Vec<String>,
        cached_assets: CachedReleaseAssets,
        cover: Option<Rc<RefCell<Image>>>,
        manifest_overrides: &Overrides,
        permalink_option: Option<Permalink>,
        title: String,
        tracks: Vec<Track>
    ) -> Release {
        // TODO: Use/store multiple images (beyond just one cover) (think this through: why?)

        let runtime = tracks
            .iter()
            .map(|track| track.cached_assets.source_meta.duration_seconds)
            .sum();

        let permalink = permalink_option.unwrap_or_else(|| Permalink::generate(&title));

        Release {
            artists: Vec::new(),
            artists_to_map,
            asset_basename: None,
            cached_assets,
            cover,
            download_formats: manifest_overrides.download_formats.clone(),
            download_option: manifest_overrides.download_option.clone(),
            embedding: manifest_overrides.embedding,
            payment_options: manifest_overrides.payment_options.clone(),
            permalink,
            rewrite_tags: manifest_overrides.rewrite_tags,
            runtime,
            streaming_format: manifest_overrides.streaming_format.clone(),
            text: manifest_overrides.release_text.clone(),
            title,
            track_numbering: manifest_overrides.release_track_numbering.clone(),
            tracks
        }
    }

    pub fn write_downloadable_files(&mut self, build: &mut Build) {
        let mut tag_mapping_option = if self.rewrite_tags {
            Some(TagMapping {
                album: Some(self.title.clone()),
                album_artist: if self.artists.is_empty() {
                    None
                } else {
                    Some(
                        self.artists
                        .iter()
                        .map(|artist| artist.borrow().name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                    )
                },
                artist: None,
                title: None,
            })
        } else {
            None
        };

        let release_dir = build.build_dir.join(&self.permalink.slug);

        for format in &self.download_formats {
            let format_dir = release_dir.join(format.asset_dirname());

            util::ensure_dir(&format_dir);

            for track in self.tracks.iter_mut() {
                let cached_track_asset = track.cached_assets.get_mut(*format);

                if cached_track_asset.is_none() {
                    if format.lossless() && !track.cached_assets.source_meta.lossless {
                        warn_discouraged!(
                            "Track {} comes from a lossy format, offering it in a lossless format is wasteful and misleading to those who will download it.",
                            &track.source_file.display()
                        );
                    }

                    if let Some(tag_mapping) = &mut tag_mapping_option {
                        tag_mapping.artist = if track.artists.is_empty() {
                            None
                        } else {
                             Some(
                                track.artists
                                .iter()
                                .map(|artist| artist.borrow().name.clone())
                                .collect::<Vec<String>>()
                                .join(", ")
                            )
                        };
                        tag_mapping.title = Some(track.title.clone());
                    }

                    track.get_or_transcode_as(
                        *format,
                        build,
                        AssetIntent::Deliverable,
                        &tag_mapping_option
                    );

                    track.cached_assets.persist(&build.cache_dir);
                }

                let download_track_asset = track.cached_assets.get_mut(*format).as_mut().unwrap();

                download_track_asset.unmark_stale();

                let track_filename = format!(
                    "{basename}{extension}",
                    basename = track.asset_basename.as_ref().unwrap(),
                    extension = format.extension()
                );

                // TODO: Track might already have been copied (?) (if streaming format is identical)
                fs::copy(
                    build.cache_dir.join(&download_track_asset.filename),
                    format_dir.join(track_filename)
                ).unwrap();

                // TODO: Track might already have been added (?) (if streaming format is identical)
                build.stats.add_track(download_track_asset.filesize_bytes);
            }

            let cached_archive_asset = self.cached_assets.get_mut(*format);
            if cached_archive_asset.is_none() {
                let cached_archive_filename = format!("{}.zip", util::uid());

                info_zipping!("Creating download archive for release '{}' ({})", self.title, &format);

                let zip_file = File::create(
                    build.cache_dir.join(&cached_archive_filename)
                ).unwrap();
                let mut zip_writer = ZipWriter::new(zip_file);
                let options = FileOptions::default()
                    .compression_method(CompressionMethod::Deflated)
                    .unix_permissions(0o755);

                let mut buffer = Vec::new();

                for track in self.tracks.iter_mut() {
                    let download_track_asset = track.get_as(*format).as_ref().unwrap();

                    let filename = format!(
                        "{basename}{extension}",
                        basename = track.asset_basename.as_ref().unwrap(),
                        extension = format.extension()
                    );

                    zip_writer.start_file(&filename, options).unwrap();

                    let mut zip_inner_file = File::open(
                        &build.cache_dir.join(&download_track_asset.filename)
                    ).unwrap();

                    zip_inner_file.read_to_end(&mut buffer).unwrap();
                    zip_writer.write_all(&*buffer).unwrap();
                    buffer.clear();

                    track.cached_assets.persist(&build.cache_dir);
                }

                if let Some(cover) = &mut self.cover {
                    let mut cover_mut = cover.borrow_mut();
                    let cover_asset = cover_mut.get_or_transcode_as(ImageFormat::Cover, build, AssetIntent::Intermediate);

                    zip_writer.start_file("cover.jpg", options).unwrap();

                    let mut zip_inner_file = File::open(
                        &build.cache_dir.join(&cover_asset.filename)
                    ).unwrap();

                    zip_inner_file.read_to_end(&mut buffer).unwrap();
                    zip_writer.write_all(&*buffer).unwrap();
                    buffer.clear();

                    cover_mut.cached_assets.persist(&build.cache_dir);
                }

                match zip_writer.finish() {
                    Ok(_) => cached_archive_asset.replace(Asset::new(build, cached_archive_filename, AssetIntent::Deliverable)),
                    Err(err) => panic!("{}", err)
                };
            }

            let download_archive_asset = cached_archive_asset.as_mut().unwrap();

            download_archive_asset.unmark_stale();

            let archive_filename = format!(
                "{basename}.zip",
                basename = self.asset_basename.as_ref().unwrap()
            );

            fs::copy(
                build.cache_dir.join(&download_archive_asset.filename),
                format_dir.join(&archive_filename)
            ).unwrap();

            build.stats.add_archive(download_archive_asset.filesize_bytes);

            self.cached_assets.persist(&build.cache_dir);
        }
    }

    pub fn write_files(&self, build: &mut Build, catalog: &Catalog) {
        match &self.download_option {
            DownloadOption::Disabled => (),
            DownloadOption::Free { download_page_uid }  => {
                let download_page_dir = build.build_dir.join("download").join(&self.permalink.slug).join(download_page_uid);
                let download_html = render::release::download::download_html(build, catalog, self);
                util::ensure_dir_and_write_index(&download_page_dir, &download_html);
            }
            DownloadOption::Paid { checkout_page_uid, download_page_uid, .. } => {
                if self.payment_options.is_empty() {
                    warn!(
                        "No payment options specified for release '{}', no purchase/download option will be displayed for this release.",
                        self.title
                    );
                } else {
                    let checkout_page_dir = build.build_dir.join("checkout").join(&self.permalink.slug).join(checkout_page_uid);
                    let checkout_html = render::release::checkout::checkout_html(build, catalog, self, download_page_uid);
                    util::ensure_dir_and_write_index(&checkout_page_dir, &checkout_html);

                    let download_page_dir = build.build_dir.join("download").join(&self.permalink.slug).join(download_page_uid);
                    let download_html = render::release::download::download_html(build, catalog, self);
                    util::ensure_dir_and_write_index(&download_page_dir, &download_html);
                }

            }
        }
        
        if let Some(cover) = &self.cover {
            if cover.borrow().description.is_none() {
                warn_discouraged!("The cover image for release '{}' is missing an image description.", self.title);
                build.missing_image_descriptions = true;
            }
        }

        let release_dir = build.build_dir.join(&self.permalink.slug);
        let release_html = render::release::release_html(build, catalog, self);
        util::ensure_dir_and_write_index(&release_dir, &release_html);

        if self.embedding  {
            if let Some(base_url) = &build.base_url {
                let embed_choices_dir = release_dir.join("embed");
                let embed_choices_html = render::release::embed::embed_choices_html(build, catalog, self, &base_url);
                util::ensure_dir_and_write_index(&embed_choices_dir, &embed_choices_html);

                let embed_release_dir = embed_choices_dir.join("all");
                let embed_release_html = render::release::embed::embed_release_html(build, catalog, self, &base_url);
                util::ensure_dir_and_write_index(&embed_release_dir, &embed_release_html);

                for (index, track) in self.tracks.iter().enumerate() {
                    let track_number = index + 1;
                    let embed_track_dir = embed_choices_dir.join(track_number.to_string());
                    let embed_track_html = render::release::embed::embed_track_html(build, catalog, self, track, track_number, &base_url);
                    util::ensure_dir_and_write_index(&embed_track_dir, &embed_track_html);
                }
            }
        }
    }
}

impl TrackNumbering {
    pub fn format(&self, number: usize) -> String {
        match self {
            TrackNumbering::Disabled => String::from(""),
            TrackNumbering::Arabic => format!("{:02}", number),
            TrackNumbering::Hexadecimal => format!("0x{:02X}", number),
            TrackNumbering::Roman => Self::to_roman(number)
        }
    }
    
    pub fn from_manifest_key(key: &str) -> Option<TrackNumbering> {
        match key {
            "disabled" => Some(TrackNumbering::Disabled),
            "arabic" => Some(TrackNumbering::Arabic),
            "hexadecimal" => Some(TrackNumbering::Hexadecimal),
            "roman" => Some(TrackNumbering::Roman),
            _ =>  None
        }
    }
    
    fn to_roman(number: usize) -> String {
        // TODO: Implement to at least ~256 (or more) using proper algorithm
        let roman = match number {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            10 => "X",
            11 => "XI",
            12 => "XII",
            13 => "XIII",
            14 => "XIV",
            15 => "XV",
            16 => "XVI",
            17 => "XVII",
            18 => "XVIII",
            19 => "XIX",
            20 => "XX",
            21 => "XXI",
            22 => "XXII",
            23 => "XXIII",
            24 => "XXIV",
            25 => "XXV",
            26 => "XXVI",
            27 => "XXVII",
            28 => "XXVIII",
            29 => "XXIX",
            30 => "XXX",
            31 => "XXXI",
            32 => "XXXII",
            33 => "XXXIII",
            34 => "XXXIV",
            35 => "XXXV",
            36 => "XXXVI",
            37 => "XXXVII",
            38 => "XXXVIII",
            39 => "XXXIX",
            40 => "XL",
            _ => unimplemented!()
        };
        
        format!("{}", roman)
    }
}
