use std::fs;
use std::path::PathBuf;

use crate::Build;

#[derive(Debug)]
pub enum Favicon {
    Custom {
        absolute_path: PathBuf,
        extension: String,
    },
    Default
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

    pub fn header_tags(&self, root_prefix: &str) -> String {
        match self {
            Favicon::Custom { extension, .. } => {
                format!(r#"
                    <link href="{root_prefix}favicon.{extension}" rel="icon">
                "#)
            }
            Favicon::Default => {
                format!(r#"
                    <link href="{root_prefix}favicon.svg" rel="icon" type="image/svg+xml">
                    <link href="{root_prefix}favicon_light.png" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
                    <link href="{root_prefix}favicon_dark.png" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
                "#)
            }
        }
    }

    pub fn write(&self, build: &Build) {
        match self {
            Favicon::Custom { absolute_path, extension } => {
                fs::copy(
                    absolute_path,
                    build.build_dir.join(format!("favicon.{extension}"))
                ).unwrap();
            }
            Favicon::Default => {
                fs::write(
                    build.build_dir.join("favicon.svg"),
                    include_bytes!("assets/favicon.svg")
                ).unwrap();

                fs::write(
                    build.build_dir.join("favicon_dark.png"),
                    include_bytes!("assets/favicon_dark.png")
                ).unwrap();

                fs::write(
                    build.build_dir.join("favicon_light.png"),
                    include_bytes!("assets/favicon_light.png")
                ).unwrap();
            }
        }
    }
}
