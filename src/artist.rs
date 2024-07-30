// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::{
    Build,
    Cache,
    Catalog,
    DescribedImage,
    HtmlAndStripped,
    Overrides,
    Permalink,
    ReleaseRc,
    Theme
};
use crate::markdown;

#[derive(Debug)]
pub struct Artist {
    pub aliases: Vec<String>,
    pub copy_link: bool,
    pub image: Option<DescribedImage>,
    pub name: String,
    pub permalink: Permalink,
    pub releases: Vec<ReleaseRc>,
    pub text: Option<HtmlAndStripped>,
    pub theme: Theme,
    pub unlisted: bool
}

#[derive(Clone, Debug)]
pub struct ArtistRc {
    artist: Rc<RefCell<Artist>>,
}

impl Artist {
    pub fn new(copy_link: bool, name: &str, theme: Theme) -> Artist {
        let permalink = Permalink::generate(name);
        
        Artist {
            aliases: Vec::new(),
            copy_link,
            image: None,
            name: name.to_string(),
            permalink,
            releases: Vec::new(),
            text: None,
            theme,
            unlisted: false
        }
    }

    pub fn public_releases(&self) -> impl Iterator<Item = &ReleaseRc> {
        self.releases
            .iter()
            .filter(|release| !release.borrow().unlisted)
    }

    pub fn read_manifest(
        build: &Build,
        cache: &mut Cache,
        catalog: &mut Catalog,
        dir: &Path,
        overrides: &Overrides
    ) {
        let manifest_path = dir.join("_artist.eno");

        let content = match fs::read_to_string(&manifest_path) {
            Ok(content) => content,
            Err(err) => {
                error!("Could not read manifest {} ({})", manifest_path.display(), err);
                return
            }
        };

        let document = match enolib::parse(&content) {
            Ok(document) => document,
            Err(err) => {
                error!("Syntax error in {}:{} ({})", manifest_path.display(), err.line, err);
                return
            }
        };

        let name = 'get_name: {
            match document.optional_field("name") {
                Ok(Some(field)) => {
                    match field.optional_value() {
                        Ok(Some(value)) => break 'get_name value,
                        Ok(None) => (),
                        Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
                    }
                }
                Ok(None) => (),
                Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
            }

            dir.file_name().unwrap().to_string_lossy().to_string()
        };

        let artist = catalog.create_artist(overrides.copy_link, &name, overrides.theme.clone());
        let mut artist_mut = artist.borrow_mut();

        match document.optional_field("aliases") {
            Ok(Some(field)) => match field.items() {
                Ok(items) => {
                    for item in items {
                        match item.optional_value() {
                            Ok(Some(alias)) => artist_mut.aliases.push(alias),
                            _ => ()
                        }
                    }
                }
                Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
            }
            Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line),
            _ => ()
        }

        match document.optional_field("image") {
            Ok(Some(field)) => match field.required_attribute("file") {
                Ok(attribute) => match attribute.required_value::<String>() {
                    Ok(path_relative_to_manifest) => {
                        let absolute_path = manifest_path.parent().unwrap().join(&path_relative_to_manifest);
                        if absolute_path.exists() {
                            // TODO: Print errors, refactor
                            let description = match field.required_attribute("description") {
                                Ok(attribute) => match attribute.optional_value() {
                                    Some(Ok(description)) => Some(description),
                                    _ => None
                                }
                                _ => None
                            };

                            let path_relative_to_catalog = absolute_path.strip_prefix(&build.catalog_dir).unwrap();
                            let image = cache.get_or_create_image(build, path_relative_to_catalog);

                            artist_mut.image = Some(DescribedImage::new(description, image));
                        } else {
                            error!("Ignoring invalid image.file setting value '{}' in {}:{} (The referenced file was not found)", path_relative_to_manifest, manifest_path.display(), attribute.line_number)
                        }
                    }
                    Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
                }
                Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
            }
            Ok(None) => (),
            Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
        }

        match document.optional_field("permalink") {
            Ok(Some(field)) => match field.optional_value() {
                Ok(Some(slug)) => {
                    match Permalink::new(&slug) {
                        Ok(permalink) => artist_mut.permalink = permalink,
                        Err(err) => error!("Ignoring invalid permalink value '{}' in {}:{} ({})", slug, manifest_path.display(), field.line_number, err)
                    }
                }
                Ok(None) => (),
                Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
            }
            Ok(None) => (),
            Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
        }

        match document.optional_embed("text") {
            Ok(Some(embed)) => match embed.optional_value::<String>() {
                Some(Ok(text_markdown)) => {
                    artist_mut.text = Some(markdown::to_html_and_stripped(&text_markdown));
                }
                _ => ()
            }
            Ok(None) => (),
            Err(err) => error!("{} {}:{}", err.message, manifest_path.display(), err.line)
        }
    }
}

impl ArtistRc {
    pub fn borrow(&self) -> Ref<'_, Artist> {
        self.artist.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Artist> {
        self.artist.borrow_mut()
    }

    pub fn new(artist: Artist) -> ArtistRc {
        ArtistRc {
            artist: Rc::new(RefCell::new(artist))
        }
    }

    pub fn ptr_eq(a: &ArtistRc, b: &ArtistRc) -> bool {
        Rc::ptr_eq(&a.artist, &b.artist)
    }
}