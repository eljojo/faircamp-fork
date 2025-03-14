// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Build, Catalog};
use crate::util::html_escape_inside_attribute;

pub struct OpenGraphImage {
    pub height: u32,
    pub url: String,
    pub width: u32
}

/// For the Open Graph specification see https://ogp.me/
pub struct OpenGraphMeta {
    description: Option<String>,
    image: Option<OpenGraphImage>,
    image_alt: Option<String>,
    title: String,
    url: String
}

impl OpenGraphMeta {
    pub fn description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn image(&mut self, image: OpenGraphImage) {
        self.image = Some(image);
    }

    pub fn image_alt(&mut self, description: &str) {
        self.image_alt = Some(description.to_string());
    }

    pub fn new(title: String, url: String) -> OpenGraphMeta {
        OpenGraphMeta {
            description: None,
            image: None,
            image_alt: None,
            title,
            url
        }
    }

    pub fn tags(&self, build: &Build, catalog: &Catalog) -> String {
        let mut tags = Vec::new();

        if let Some(description) = &self.description {
            let description_escaped = html_escape_inside_attribute(description);
            tags.push(format!(r#"<meta property="og:description" content="{description_escaped}"/>"#));
        }

        if let Some(image) = &self.image {
            tags.push(format!(r#"<meta property="og:image" content="{}"/>"#, image.url));
            tags.push(format!(r#"<meta property="og:image:height" content="{}"/>"#, image.height));
            tags.push(format!(r#"<meta property="og:image:width" content="{}"/>"#, image.width));
        }

        if let Some(image_alt) = &self.image_alt {
            let image_alt_escaped = html_escape_inside_attribute(image_alt);
            tags.push(format!(r#"<meta property="og:image:alt" content="{image_alt_escaped}"/>"#));
        }

        tags.push(format!(r#"<meta property="og:locale" content="{}"/>"#, &build.locale.language));

        let site_name_escaped = html_escape_inside_attribute(&catalog.title());
        tags.push(format!(r#"<meta property="og:site_name" content="{site_name_escaped}"/>"#));

        let title_escaped = html_escape_inside_attribute(&self.title);
        tags.push(format!(r#"<meta property="og:title" content="{title_escaped}"/>"#));

        tags.push(String::from(r#"<meta property="og:type" content="website"/>"#));
        tags.push(format!(r#"<meta property="og:url" content="{}"/>"#, self.url));

        tags.join("\n")
    }
}
