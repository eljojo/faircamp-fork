/// RSS 2.0 Specification for reference:
/// https://www.rssboard.org/rss-specification

use std::fs;

use crate::{
    Build,
    Catalog,
    util::html_escape_outside_attribute
};

pub fn generate(build: &Build, catalog: &Catalog) {
    if let Some(base_url) = &build.base_url { 
        let channel_items = catalog.releases
            .iter()
            .map(|release| {
                let release_ref = release.borrow();

                let main_artists = release_ref.main_artists
                    .iter()
                    .map(|artist| artist.borrow().name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                
                let artists_list = if catalog.show_support_artists && !release_ref.support_artists.is_empty() {
                    let support_artists = release_ref.support_artists
                        .iter()
                        .map(|artist| artist.borrow().name.clone())
                        .collect::<Vec<String>>()
                        .join(", ");

                    format!("{main_artists}, {support_artists}")
                } else {
                    main_artists
                };

                format!(
                    include_str!("templates/feed/item.xml"),
                    description = format!("A release by {}", html_escape_outside_attribute(&artists_list)), // TODO: Translate
                    permalink = base_url.join(&release_ref.permalink.slug).unwrap(),
                    title = html_escape_outside_attribute(&release_ref.title),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        
        let channel_description = catalog.text
            .as_ref()
            .map(|html_and_stripped| html_escape_outside_attribute(html_and_stripped.stripped.as_str()))
            .unwrap_or(String::from("A faircamp-based music catalog")); // TODO: Translate
        
        let channel_title = catalog.title();
        
        let channel_image = if catalog.home_image.is_some() {
            format!(
                include_str!("templates/feed/image.xml"),
                base_url = base_url,
                image_url = base_url.join("feed.jpg").unwrap(),
                title = html_escape_outside_attribute(&channel_title)
            )
        } else {
            String::new()
        };
        
        let xml = format!(
            include_str!("templates/feed/channel.xml"),
            base_url = base_url,
            build_date = build.build_begin.to_rfc2822(),
            description = channel_description,
            feed_url = base_url.join("feed.rss").unwrap(),
            image = channel_image,
            items = channel_items,
            language = build.locale.language,
            title = html_escape_outside_attribute(&channel_title)
        );
        
        fs::write(build.build_dir.join("feed.rss"), xml).unwrap();
    }
}
