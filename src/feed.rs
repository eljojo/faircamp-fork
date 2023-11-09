use std::fs;

use crate::{Build, Catalog};

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

                let pub_date = match release_ref.date {
                    Some(date) => format!(
                        "<pubDate>{}</pubDate>",
                        date.and_hms_opt(0, 0, 0).unwrap().and_utc().to_rfc2822()
                    ),
                    None => String::new()
                };

                format!(
                    include_str!("templates/feed/item.xml"),
                    description = format!("A release by {}", artists_list),
                    permalink = base_url.join(&release_ref.permalink.slug).unwrap(),
                    pub_date = pub_date,
                    title = release_ref.title,
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        
        // TODO: This should maybe be stripped off html (?) - practically speaking
        //       our markdown parser allows us to manually generate such output.
        let channel_description = catalog.text
            .as_deref()
            .unwrap_or("A faircamp-based music catalog"); 
        
        let channel_title = catalog.title();
        
        let channel_image = if catalog.feed_image.is_some() {
            format!(
                include_str!("templates/feed/image.xml"),
                base_url = base_url,
                image_url = base_url.join("feed.jpg").unwrap(),
                title = channel_title
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
            title = channel_title
        );
        
        fs::write(build.build_dir.join("feed.rss"), xml).unwrap();
    }
}
