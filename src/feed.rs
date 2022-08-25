use std::fs;

use crate::{Build, Catalog, ImageFormat};

pub fn generate(build: &Build, catalog: &Catalog) {
    if let Some(base_url) = &build.base_url { 
        let channel_items = catalog.releases
            .iter()
            .map(|release| {
                let artists_list = release.artists
                    .iter()
                    .map(|artist| artist.borrow().name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                
                format!(
                    include_str!("templates/feed/item.xml"),
                    description=format!("A release by {}", artists_list),
                    permalink=base_url.join(&release.permalink.slug).unwrap(),
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
            Some(channel_image) => {
                format!(
                    include_str!("templates/feed/image.xml"),
                    base_url=base_url,
                    image_url=base_url.join(&channel_image.borrow().get_as(&ImageFormat::Feed).as_ref().unwrap().filename).unwrap(),
                    title=channel_title
                )
            }
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
