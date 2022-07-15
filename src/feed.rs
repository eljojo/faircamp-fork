use std::fs;

use crate::{
    build::Build,
    catalog::Catalog,
    ffmpeg::{self, MediaFormat},
    image_format::ImageFormat
};

pub fn generate(build: &Build, catalog: &Catalog) {
    if let Some(base_url) = &build.base_url { 
        let channel_items = catalog.releases
            .iter()
            .map(|release| {
                let artists_list = release.artists
                    .iter()
                    .map(|artist| artist.name.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ");
                
                format!(
                    include_str!("templates/feed/item.xml"),
                    description=format!("A release by {}", artists_list),
                    permalink=base_url.join(&release.permalink.get()).unwrap(),
                    title=release.title,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        
        // TODO: This should maybe be stripped off html (?) - practically speaking
        //       our markdown parser allows us to manually generate such output.
        let channel_description = catalog.text
            .as_ref()
            .map(|string| string.as_str())
            .unwrap_or("A faircamp-based music catalog"); 
        
        let channel_title = catalog.title();
        
        let channel_image = match &catalog.feed_image {
            Some(feed_image) => {
                // TODO: Go through asset cache with this
                ffmpeg::transcode(
                    &build.catalog_dir.join(feed_image),
                    &build.build_dir.join("feed.jpg"),
                    MediaFormat::Image(&ImageFormat::Jpeg)
                ).unwrap();
                
                format!(
                    include_str!("templates/feed/image.xml"),
                    base_url=base_url,
                    image_url=base_url.join("feed.jpg").unwrap(),
                    title=channel_title
                )
            },
            None => String::new()
        };
        
        let xml = format!(
            include_str!("templates/feed/channel.xml"),
            base_url=base_url,
            build_date=build.build_begin.to_rfc2822(),
            description=channel_description,
            feed_url=base_url.join("feed.rss").unwrap(),
            image=channel_image,
            items=channel_items,
            language=build.localization.language,
            title=channel_title
        );
        
        fs::write(build.build_dir.join("feed.rss"), xml).unwrap();
    }
}
