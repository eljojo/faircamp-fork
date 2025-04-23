// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::PathBuf;

use indoc::formatdoc;

use crate::{AssetHashes, Build};
use crate::util::url_safe_hash_base64;

const FAVICON_DARK_PNG: &[u8; 952] = include_bytes!("assets/favicon_dark.png");
const FAVICON_DARK_PNG_FILENAME: &str = "favicon_dark.png";

const FAVICON_LIGHT_PNG: &[u8; 945] = include_bytes!("assets/favicon_light.png");
const FAVICON_LIGHT_PNG_FILENAME: &str = "favicon_light.png";

#[cfg(not(target_os = "windows"))]
const FAVICON_SVG: &[u8; 1105] = include_bytes!("assets/favicon.svg");

#[cfg(target_os = "windows")]
const FAVICON_SVG: &[u8; 1109] = include_bytes!("assets/favicon.svg");

const FAVICON_SVG_FILENAME: &str = "favicon.svg";

#[derive(Debug)]
pub enum Favicon {
    Custom {
        absolute_path: PathBuf,
        extension: String,
    },
    Default,
    None
}

impl Favicon {
    pub fn custom(absolute_path: PathBuf) -> Result<Favicon, String> {
        match absolute_path.extension() {
            Some(extension) => {
                if extension == "ico" || extension == "png" {
                    let favicon = Favicon::Custom {
                        extension: extension.to_str().unwrap().to_string(),
                        absolute_path
                    };

                    Ok(favicon)
                } else {
                    Err(format!("Favicon file extension {:?} not supported (only .ico/.png is supported)", extension))
                }
            }
            None => Err(String::from("Custom favicon file needs to have a file extension"))
        }
    }

    pub fn header_tags(&self, build: &Build, root_prefix: &str) -> Option<String> {
        match self {
            Favicon::Custom { extension, .. } => {
                let favicon_custom_hash = build.asset_hashes.favicon_custom.as_ref().unwrap();

                Some(format!(r#"<link href="{root_prefix}favicon.{extension}?{favicon_custom_hash}" rel="icon">"#))
            }
            Favicon::Default => {
                let favicon_dark_png_hash = AssetHashes::FAVICON_DARK_PNG;
                let favicon_light_png_hash = AssetHashes::FAVICON_LIGHT_PNG;
                let favicon_svg_hash = AssetHashes::FAVICON_SVG;

                Some(formatdoc!(r#"
                    <link href="{root_prefix}{FAVICON_SVG_FILENAME}?{favicon_svg_hash}" rel="icon" type="image/svg+xml">
                    <link href="{root_prefix}{FAVICON_LIGHT_PNG_FILENAME}.png?{favicon_light_png_hash}" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                    <link href="{root_prefix}{FAVICON_DARK_PNG_FILENAME}.png?{favicon_dark_png_hash}" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                "#))
            }
            Favicon::None => None
        }
    }

    pub fn write(&self, build: &mut Build) {
        match self {
            Favicon::Custom { absolute_path, extension } => {
                let custom = fs::read(absolute_path).unwrap();
                build.asset_hashes.favicon_custom = Some(url_safe_hash_base64(&custom));
                let target_filename = format!("favicon.{extension}");

                fs::write(
                    build.build_dir.join(&target_filename),
                    custom
                ).unwrap();

                build.reserve_filename(target_filename);
            }
            Favicon::Default => {
                fs::write(
                    build.build_dir.join(FAVICON_DARK_PNG_FILENAME),
                    FAVICON_DARK_PNG
                ).unwrap();

                fs::write(
                    build.build_dir.join(FAVICON_LIGHT_PNG_FILENAME),
                    FAVICON_LIGHT_PNG
                ).unwrap();

                fs::write(
                    build.build_dir.join(FAVICON_SVG_FILENAME),
                    FAVICON_SVG
                ).unwrap();

                build.reserve_filename(FAVICON_DARK_PNG_FILENAME);
                build.reserve_filename(FAVICON_LIGHT_PNG_FILENAME);
                build.reserve_filename(FAVICON_SVG_FILENAME);
            }
            Favicon::None => ()
        }
    }
}
