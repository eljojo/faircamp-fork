// SPDX-FileCopyrightText: 2023 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::PathBuf;

use indoc::formatdoc;

use crate::Build;
use crate::util::url_safe_hash_base64;

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

    pub fn header_tags(&self, build: &Build, root_prefix: &str) -> String {
        match self {
            Favicon::Custom { extension, .. } => {
                let favicon_custom_hash = build.asset_hashes.favicon_custom.as_ref().unwrap();
                format!(r#"<link href="{root_prefix}favicon.{extension}?{favicon_custom_hash}" rel="icon">"#)
            }
            Favicon::Default => {
                let favicon_svg_hash = build.asset_hashes.favicon_svg.as_ref().unwrap();
                let favicon_dark_png_hash = build.asset_hashes.favicon_dark_png.as_ref().unwrap();
                let favicon_light_png_hash = build.asset_hashes.favicon_light_png.as_ref().unwrap();

                formatdoc!(r#"
                    <link href="{root_prefix}favicon.svg?{favicon_svg_hash}" rel="icon" type="image/svg+xml">
                    <link href="{root_prefix}favicon_light.png?{favicon_light_png_hash}" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                    <link href="{root_prefix}favicon_dark.png?{favicon_dark_png_hash}" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                "#)
            }
            Favicon::None => String::new()
        }
    }

    pub fn write(&self, build: &mut Build) {
        match self {
            Favicon::Custom { absolute_path, extension } => {
                let custom = fs::read(absolute_path).unwrap();
                build.asset_hashes.favicon_custom = Some(url_safe_hash_base64(&custom));
                fs::write(build.build_dir.join(format!("favicon.{extension}")), custom).unwrap();
            }
            Favicon::Default => {
                let svg = include_bytes!("assets/favicon.svg");
                build.asset_hashes.favicon_svg = Some(url_safe_hash_base64(&svg));
                fs::write(build.build_dir.join("favicon.svg"), svg).unwrap();

                let dark_png = include_bytes!("assets/favicon_dark.png");
                build.asset_hashes.favicon_dark_png = Some(url_safe_hash_base64(&dark_png));
                fs::write(build.build_dir.join("favicon_dark.png"), dark_png).unwrap();

                let light_png = include_bytes!("assets/favicon_light.png");
                build.asset_hashes.favicon_light_png = Some(url_safe_hash_base64(&light_png));
                fs::write(build.build_dir.join("favicon_light.png"), light_png).unwrap();
            }
            Favicon::None => ()
        }
    }
}
