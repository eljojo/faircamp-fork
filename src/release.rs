use chrono::{DateTime, NaiveDate, Utc};
use indoc::formatdoc;
use serde_derive::{Serialize, Deserialize};
use std::{
    cell::RefCell,
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
    rc::Rc
};
use std::f32::consts::TAU;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{
    Artist,
    Asset,
    AssetIntent,
    AudioFormat,
    Build,
    Cache,
    Catalog,
    DownloadOption,
    Image,
    manifest::Overrides,
    PaymentOption,
    Permalink,
    render,
    SourceFileSignature,
    TagMapping,
    Theme,
    Track,
    util
};

#[derive(Debug)]
pub struct Release {
    /// Generated when we gathered all artist and title metadata.
    /// Used to compute the download asset filenames.
    pub asset_basename: Option<String>,
    pub assets: Rc<RefCell<ReleaseAssets>>,
    pub cover: Option<Rc<RefCell<Image>>>,
    pub date: Option<NaiveDate>,
    pub download_formats: Vec<AudioFormat>,
    pub download_option: DownloadOption,
    pub embedding: bool,
    /// The artists that are the principal authors of a release ("Album Artist" in tag lingo)
    pub main_artists: Vec<Rc<RefCell<Artist>>>,
    /// The order in which we encounter artists and releases when reading the
    /// catalog is arbitrary, hence when we read a release, we might not yet
    /// have read metadata that tells us to which artist(s) it needs to be
    /// mapped. `main_artists_to_map` is an intermediate, name-based mapping
    /// we store until the entire catalog is read. After that point, we
    /// use it to build the final mapping in `main_artists`, then dispose of it.
    pub main_artists_to_map: Vec<String>,
    pub payment_options: Vec<PaymentOption>,
    pub permalink: Permalink,
    pub rewrite_tags: bool,
    pub runtime: u32,
    pub streaming_format: AudioFormat,
    /// Artists that appear on the release as collaborators, features, etc.
    pub support_artists: Vec<Rc<RefCell<Artist>>>,
    /// See `main_artists_to_map` for what this does
    pub support_artists_to_map: Vec<String>,
    pub text: Option<String>,
    pub title: String,
    pub track_numbering: TrackNumbering,
    pub tracks: Vec<Track>
}

/// These are the downloadable zip archives for a release,
/// not the stand-alone transcoded tracks!
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseAssets {
    pub aac: Option<Asset>,
    pub aiff: Option<Asset>,
    pub cover_source_file_signature: Option<SourceFileSignature>,
    pub flac: Option<Asset>,
    pub mp3: Option<Asset>,
    pub ogg_vorbis: Option<Asset>,
    pub opus_48: Option<Asset>,
    pub opus_96: Option<Asset>,
    pub opus_128: Option<Asset>,
    pub track_source_file_signatures: Vec<SourceFileSignature>,
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
    pub fn generate_cover(&self, theme: &Theme, max_tracks_in_release: usize) -> String {
        // TODO: This is too simplistic, text also has text_h and text_s
        // currently (but theming may change quite a bit so no rush). Also
        // unfortunately generated covers don't interactively repaint when
        // using the --theming-widget, but that's probably to be accepted.
        let text_l = theme.base.text_l;
        let edge = 64.0;
        let radius = edge / 2.0;

        let longest_duration = self.tracks
            .iter()
            .map(|track| track.assets.borrow().source_meta.duration_seconds)
            .max()
            .unwrap();

        let mut track_offset = 0.0;
        let points = self.tracks
            .iter()
            .enumerate()
            .map(|(track_index, track)| {
                let source_meta = &track.assets.borrow().source_meta;

                let altitude_range = 0.75 * self.tracks.len() as f32 / max_tracks_in_release as f32;
                let altitude_width = radius * altitude_range / self.tracks.len() as f32;
                let track_arc_range = source_meta.duration_seconds as f32 / longest_duration as f32;

                if let Some(peaks) = &source_meta.peaks {
                    let mut samples = Vec::new();
                    let step = 1;

                    let mut previous = None;

                    let track_compensation = 0.25 + (1.0 - track_arc_range) / 2.0;

                    for (peak_index, peak) in peaks.iter().step_by(step).enumerate() {
                        let peak_offset = peak_index as f32 / (peaks.len() - 1) as f32 * step as f32 * -1.0; // 0-1

                        let arc_offset = (track_compensation + peak_offset * track_arc_range) * TAU;
                        let amplitude = 
                            radius * 0.25 +
                            (max_tracks_in_release - 1 - track_index) as f32 * altitude_width +
                            (peak * 0.3 * altitude_width);

                        let x = radius + amplitude * arc_offset.sin();
                        let y = radius + amplitude * arc_offset.cos();

                        if let Some((x_prev, y_prev)) = previous {
                            let stroke = format!("hsla(0, 0%, {text_l}%, {peak})");
                            let stroke_width = peak * 0.32;
                            let sample = format!(r##"<line stroke="{stroke}" stroke-width="{stroke_width}px" x1="{x_prev}" x2="{x}" y1="{y_prev}" y2="{y}"/>"##);
                            samples.push(sample);
                        }

                        previous = Some((x, y));
                    }

                    track_offset += track_arc_range;

                    samples.join("\n")
                } else {
                    let cx = radius + (edge / 3.0) * (track_offset * TAU).sin();
                    let cy = radius + (edge / 3.0) * (track_offset * TAU).cos();

                    track_offset += track_arc_range;

                    format!(r##"<circle cx="{cx}" cy="{cy}" fill="#ffffff" r="1"/>"##)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatdoc!(r##"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <rect fill="none" height="63.96" stroke="hsl(0, 0% {text_l}%)" stroke-width=".12px" width="63.96" x="0.02" y="0.02"/>
                {points}
            </svg>
        "##)
    }

    pub fn new(
        assets: Rc<RefCell<ReleaseAssets>>,
        cover: Option<Rc<RefCell<Image>>>,
        date: Option<NaiveDate>,
        main_artists_to_map: Vec<String>,
        manifest_overrides: &Overrides,
        permalink_option: Option<Permalink>,
        support_artists_to_map: Vec<String>,
        title: String,
        tracks: Vec<Track>
    ) -> Release {
        let runtime = tracks
            .iter()
            .map(|track| track.assets.borrow().source_meta.duration_seconds)
            .sum();

        let permalink = permalink_option.unwrap_or_else(|| Permalink::generate(&title));

        let mut download_option = manifest_overrides.download_option.clone();

        if let DownloadOption::Codes { unlock_text, .. } = &mut download_option {
            if let Some(custom_unlock_text) = &manifest_overrides.unlock_text {
                unlock_text.replace(custom_unlock_text.clone());
            }
        }

        Release {
            asset_basename: None,
            assets,
            cover,
            date,
            download_formats: manifest_overrides.download_formats.clone(),
            download_option,
            embedding: manifest_overrides.embedding,
            main_artists: Vec::new(),
            main_artists_to_map,
            payment_options: manifest_overrides.payment_options.clone(),
            permalink,
            rewrite_tags: manifest_overrides.rewrite_tags,
            runtime,
            streaming_format: manifest_overrides.streaming_format,
            support_artists: Vec::new(),
            support_artists_to_map,
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
                album_artist: if self.main_artists.is_empty() {
                    None
                } else {
                    Some(
                        self.main_artists
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
                    zip_writer.write_all(&buffer).unwrap();
                    buffer.clear();

                    track.assets.borrow().persist_to_cache(&build.cache_dir);
                }

                if let Some(cover) = &mut self.cover {
                    let cover_mut = cover.borrow_mut();
                    let mut assets_mut = cover_mut.assets.borrow_mut();
                    let cover_assets = assets_mut.cover_asset(build, AssetIntent::Intermediate);

                    zip_writer.start_file("cover.jpg", options).unwrap();

                    let mut zip_inner_file = File::open(
                        &build.cache_dir.join(&cover_assets.largest().filename)
                    ).unwrap();

                    zip_inner_file.read_to_end(&mut buffer).unwrap();
                    zip_writer.write_all(&buffer).unwrap();
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
            DownloadOption::Codes { codes, .. } => {
                let t_unlock_permalink = &build.locale.translations.unlock_permalink;
                let page_hash = build.hash_generic(&[&self.permalink.slug, t_unlock_permalink]);

                let unlock_page_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_unlock_permalink)
                    .join(page_hash);

                // TODO: Split up checkout_html into unlock_html + purchase_html - different pages!
                let unlock_html = render::release::checkout::checkout_html(build, catalog, self);
                util::ensure_dir_and_write_index(&unlock_page_dir, &unlock_html);

                let t_downloads_permalink = &build.locale.translations.downloads_permalink;
                let download_dir = build.build_dir
                    .join(&self.permalink.slug)
                    .join(t_downloads_permalink);

                let download_html = render::release::download::download_html(build, catalog, self);

                for code in codes {
                    // TODO: We will need to limit the code character set to url safe characters.
                    //       Needs to be validated when reading the manifests.
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
                    let t_purchase_permalink = &build.locale.translations.purchase_permalink;
                    let purchase_page_hash = build.hash_generic(&[&self.permalink.slug, t_purchase_permalink]);

                    let purchase_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_purchase_permalink)
                        .join(purchase_page_hash);

                    let purchase_html = render::release::checkout::checkout_html(build, catalog, self);
                    util::ensure_dir_and_write_index(&purchase_page_dir, &purchase_html);

                    let t_downloads_permalink = &build.locale.translations.downloads_permalink;
                    let download_page_hash = build.hash_generic(&[&self.permalink.slug, t_downloads_permalink]);

                    let download_page_dir = build.build_dir
                        .join(&self.permalink.slug)
                        .join(t_downloads_permalink)
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
                let embed_choices_html = render::release::embed::embed_choices_html(build, catalog, self, base_url);
                util::ensure_dir_and_write_index(&embed_choices_dir, &embed_choices_html);

                let embed_release_dir = embed_choices_dir.join("all");
                let embed_release_html = render::release::embed::embed_release_html(build, catalog, self, base_url);
                util::ensure_dir_and_write_index(&embed_release_dir, &embed_release_html);

                for (index, track) in self.tracks.iter().enumerate() {
                    let track_number = index + 1;
                    let embed_track_dir = embed_choices_dir.join(track_number.to_string());
                    let embed_track_html = render::release::embed::embed_track_html(build, catalog, self, track, track_number, base_url);
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
        cache_dir.join(Cache::MANIFEST_RELEASES_DIR).join(filename)
    }

    pub fn mark_all_stale(&mut self, timestamp: &DateTime<Utc>) {
        for format in AudioFormat::ALL_FORMATS {
            if let Some(asset) = self.get_mut(format) {
                asset.mark_stale(timestamp);
            }
        }
    }

    pub fn new(
        cover_source_file_signature: Option<SourceFileSignature>,
        track_source_file_signatures: Vec<SourceFileSignature>
    ) -> ReleaseAssets {
        ReleaseAssets {
            aac: None,
            aiff: None,
            cover_source_file_signature,
            flac: None,
            mp3: None,
            ogg_vorbis: None,
            opus_48: None,
            opus_96: None,
            opus_128: None,
            track_source_file_signatures,
            uid: util::uid(),
            wav: None
        }
    }

    pub fn persist_to_cache(&self, cache_dir: &Path) {
        let serialized = bincode::serialize(self).unwrap();
        fs::write(self.manifest_path(cache_dir), serialized).unwrap();
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
        
        roman.to_string()
    }
}
