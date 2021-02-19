use std::fs;

use crate::{
    build::Build,
    catalog::Catalog,
    message
};

pub fn generate(build: &Build, catalog: &Catalog) {
    if let Some(base_url) = &build.base_url {        
        let items = catalog.releases
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
                    permalink=base_url.join(&release.slug).unwrap(),
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
        
        let channel_title = catalog.title
            .as_ref()
            .map(|string| string.as_str())
            .unwrap_or("Catalog");
        
        let xml = format!(
            include_str!("templates/feed/channel.xml"),
            base_url=base_url,
            build_date=build.build_begin.to_rfc2822(),
            channel_description=channel_description,
            channel_title=channel_title,
            feed_image="TODO.png",
            feed_image_title="TODO",
            feed_url=base_url.join("feed.rss").unwrap(),
            items=items,
            language="en"
        );
        
        fs::write(build.build_dir.join("feed.rss"), xml).unwrap();
    } else {
        message::warning(&format!("No base_url specified, skipping RSS feed generation"));
    }
}