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
    manifest::Overrides,
    PaymentOption,
    Permalink,
    render,
    SourceFileSignature,
    TagMapping,
    Track,
    util
};

#[derive(Debug)]
pub struct Release {
    pub artists: Vec<Rc<RefCell<Artist>>>,
    pub artists_to_map: Vec<String>,
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download asset filenames.
    pub asset_basename: Option<String>,
    pub assets: Rc<RefCell<ReleaseAssets>>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseAssets {
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

#[derive(Clone, Debug)]
pub enum TrackNumbering {
    Disabled,
    Arabic,
    Hexadecimal,
    Roman
}

impl Release {
    pub fn new(
        artists_to_map: Vec<String>,
        assets: Rc<RefCell<ReleaseAssets>>,
        cover: Option<Rc<RefCell<Image>>>,
        manifest_overrides: &Overrides,
        permalink_option: Option<Permalink>,
        title: String,
        tracks: Vec<Track>
    ) -> Release {
        // TODO: Use/store multiple images (beyond just one cover) (think this through: why?)

        let runtime = tracks
            .iter()
            .map(|track| track.assets.borrow().source_meta.duration_seconds)
            .sum();

        let permalink = permalink_option.unwrap_or_else(|| Permalink::generate(&title));

        Release {
            artists: Vec::new(),
            artists_to_map,
            asset_basename: None,
            assets,
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
                if track.assets.borrow().get(*format).is_none() {
                    if format.lossless() && !track.assets.borrow().source_meta.lossless {
                        warn_discouraged!(
                            "Track {} comes from a lossy format, offering it in a lossless format is wasteful and misleading to those who will download it.",
                            &track.assets.borrow().source_file_signature.path.display()
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

                    track.transcode_as(
                        *format,
                        build,
                        AssetIntent::Deliverable,
                        &tag_mapping_option
                    );

                    track.assets.borrow().persist_to_cache(&build.cache_dir);
                }

                let mut download_track_assets_mut = track.assets.borrow_mut();
                let download_track_asset = download_track_assets_mut
                    .get_mut(*format)
                    .as_mut()
                    .unwrap();

                download_track_asset.unmark_stale();

                let track_filename = format!(
                    "{basename}{extension}",
                    basename = track.asset_basename.as_ref().unwrap(),
                    extension = format.extension()
                );

                let hash = build.hash(
                    &self.permalink.slug,
                    format.asset_dirname(),
                    &track_filename
                );

                // TODO: We should calculate this earlier and persist it so we can reuse it for copying
                // and for rendering the hrefs that point to it, however we need to figure out where 
                // (or on what) to store it - that's a bit tricky. (applies in a few places)
                let hash_dir = format_dir.join(hash);

                util::ensure_dir(&hash_dir);

                // TODO: Track might already have been copied (?) (if streaming format is identical)
                util::hard_link_or_copy(
                    build.cache_dir.join(&download_track_asset.filename),
                    hash_dir.join(track_filename)
                );

                // TODO: Track might already have been added (?) (if streaming format is identical)
                build.stats.add_track(download_track_asset.filesize_bytes);
            }

            let mut release_assets_mut = self.assets.borrow_mut();
            let cached_archive_asset = release_assets_mut.get_mut(*format);

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
                    let assets_ref = track.assets.borrow();
                    let download_track_asset = assets_ref.get(*format).as_ref().unwrap();

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

                    track.assets.borrow().persist_to_cache(&build.cache_dir);
                }

                if let Some(cover) = &mut self.cover {
                    let cover_mut = cover.borrow_mut();
                    let mut assets_mut = cover_mut.assets.borrow_mut();
                    let cover_asset = assets_mut.download_cover_asset(build, AssetIntent::Intermediate);

                    zip_writer.start_file("cover.jpg", options).unwrap();

                    let mut zip_inner_file = File::open(
                        &build.cache_dir.join(&cover_asset.filename)
                    ).unwrap();

                    zip_inner_file.read_to_end(&mut buffer).unwrap();
                    zip_writer.write_all(&*buffer).unwrap();
                    buffer.clear();

                    assets_mut.persist_to_cache(&build.cache_dir);
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

            let hash = build.hash(
                &self.permalink.slug,
                format.asset_dirname(),
                &archive_filename
            );

            let hash_dir = format_dir.join(hash);

            util::ensure_dir(&hash_dir);

            util::hard_link_or_copy(
                build.cache_dir.join(&download_archive_asset.filename),
                hash_dir.join(&archive_filename)
            );

            build.stats.add_archive(download_archive_asset.filesize_bytes);

            release_assets_mut.persist_to_cache(&build.cache_dir);
        }
    }

    pub fn write_files(&self, build: &mut Build, catalog: &Catalog) {
        match &self.download_option {
            DownloadOption::Codes(codes) => {
                let page_hash = build.hash_generic(&[&self.permalink.slug, "checkout"]);

                let checkout_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join("checkout")
                    .join(page_hash);

                let checkout_html = render::release::checkout::checkout_html(build, catalog, self);
                util::ensure_dir_and_write_index(&checkout_page_dir, &checkout_html);

                let download_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join("download");

                let download_html = render::release::download::download_html(build, catalog, self);

                for code in codes {
                    // TODO: We will need to limit the code character set to url safe characters.
                    //       Needs to be validated when reading the input directory.
                    let code_dir = download_dir.join(code);
                    util::ensure_dir_and_write_index(&code_dir, &download_html);
                }
            }
            DownloadOption::Disabled => (),
            DownloadOption::Free  => {
                let page_hash = build.hash_generic(&[&self.permalink.slug, "download"]);

                let download_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join("download")
                    .join(page_hash);

                let download_html = render::release::download::download_html(build, catalog, self);
                util::ensure_dir_and_write_index(&download_page_dir, &download_html);
            }
            DownloadOption::Paid(_currency, _ranges) => {
                if self.payment_options.is_empty() {
                    warn!(
                        "No payment options specified for release '{}', no purchase/download option will be displayed for this release.",
                        self.title
                    );
                } else {
                    let checkout_page_hash = build.hash_generic(&[&self.permalink.slug, "checkout"]);

                    let checkout_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join("checkout")
                        .join(checkout_page_hash);

                    let checkout_html = render::release::checkout::checkout_html(build, catalog, self);
                    util::ensure_dir_and_write_index(&checkout_page_dir, &checkout_html);

                    let download_page_hash = build.hash_generic(&[&self.permalink.slug, "download"]);

                    let download_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join("download")
                        .join(download_page_hash);

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

impl ReleaseAssets {
    pub fn deserialize_cached(path: &Path) -> Option<ReleaseAssets> {
        match fs::read(path) {
            Ok(bytes) => bincode::deserialize::<ReleaseAssets>(&bytes).ok(),
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

    pub fn new(source_file_signatures: Vec<SourceFileSignature>) -> ReleaseAssets {
        ReleaseAssets {
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

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), &serialized).unwrap();
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
