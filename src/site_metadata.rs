// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::mem;
use std::path::PathBuf;

use crate::SourceHash;

#[derive(Debug)]
pub struct SiteAsset {
    pub filename: String,
    pub hash: SourceHash,
    pub path: PathBuf
}

/// Represents site metadata - mostly arbitrary html that is injected into the
/// <head>…</head> of the page, which may contain dynamically interpolatable
/// references to filenames - in a parsed, tokenized form.
#[derive(Debug)]
pub struct SiteMetadata {
    tokens: Vec<Token>
}

#[derive(Debug)]
enum Token {
    Markup(String),
    ResolvedSiteAsset {
        filename: String,
        hash: String
    },
    /// At parse-time we store references to assets (like "{{custom.css}}") as
    /// unresolved markers that only indicate the filename that needs to be
    /// looked up later. When we validate all filename references, we replace
    /// UnresolvedSiteAsset tokens with ResolvedSiteAsset tokens, which
    /// contain the actually written filename in the build directory and a
    /// hash based on their content (used for cache invalidation).
    UnresolvedSiteAsset(String)
}

impl SiteAsset {
    pub fn new(path: PathBuf) -> SiteAsset {
        let filename = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let hash = SourceHash::new(&path);

        SiteAsset {
            filename,
            hash,
            path
        }
    }
}

impl SiteMetadata {
    /// Parses e.g. an input like "<link href="{{custom.css}}" rel="stylesheet">"
    /// into a sequence of tokens like Token::Markup("<link href=""),
    /// Token::UnresolvedSiteAsset("custom.css"), Token::Markup("" rel="stylesheet">") for
    /// later rendering, wrapped in a SiteMetadata struct.
    pub fn parse(input: &str) -> Result<SiteMetadata, String> {
        let mut tokens = Vec::new();

        let mut split_on_open = input.split("{{");

        let leading_markup = split_on_open.next().unwrap();

        if leading_markup.contains("}}") {
            return Err(String::from(r#""}}" not matched by a preceding "{{""#));
        }

        tokens.push(Token::Markup(leading_markup.to_string()));

        while let Some(in_between_open) = split_on_open.next() {
            let mut split_on_close = in_between_open.split("}}");

            let filename = split_on_close.next().unwrap();
            tokens.push(Token::UnresolvedSiteAsset(filename.to_string()));

            if let Some(trailing_markup) = split_on_close.next() {
                tokens.push(Token::Markup(trailing_markup.to_string()));
            }

            if split_on_close.next().is_some() {
                return Err(String::from(r#""}}" not matched by a preceding "{{""#));
            }
        }

        Ok(SiteMetadata { tokens })
    }

    pub fn render(&self, root_prefix: &str) -> String {
        self.tokens
            .iter()
            .map(|token| {
                match token {
                    Token::ResolvedSiteAsset { filename, hash } => {
                        format!("{root_prefix}{filename}?{hash}")
                    }
                    Token::Markup(markup) => markup.clone(),
                    Token::UnresolvedSiteAsset(_) => unreachable!()
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    /// Returns Ok(()) if all referenced filenames in the site metadata can be
    /// resolved, otherwise returns an Err(Vec<…>) containing all missing
    /// filenames.
    pub fn resolve_filename_references(&mut self, site_assets: &[SiteAsset]) -> Result<(), Vec<String>> {
        let mut missing_files = Vec::new();

        for token in self.tokens.iter_mut() {
            if let Token::UnresolvedSiteAsset(filename) = token {
                if let Some(site_asset) = site_assets
                    .iter()
                    .find(|site_asset| site_asset.filename == *filename) {
                    *token = Token::ResolvedSiteAsset {
                        filename: mem::take(filename),
                        hash: site_asset.hash.as_url_safe_base64(),
                    }
                } else {
                    missing_files.push(filename.to_string());
                }
            }
        }

        match missing_files.is_empty() {
            true => Ok(()),
            false => Err(missing_files)
        }
    }
}
