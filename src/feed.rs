use chrono::Utc;
use url::Url;

use crate::catalog::Catalog;

pub fn generate(base_url: &Url, catalog: &Catalog) -> String {
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
                date="TODO",
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
    
    format!(
        include_str!("templates/feed/channel.xml"),
        base_url=base_url,
        build_date=Utc::now().to_rfc2822(),
        channel_description=channel_description,
        channel_title=channel_title,
        feed_image="TODO.png",
        feed_image_title="TODO",
        feed_url=base_url.join("feed.rss").unwrap(),
        items=items,
        language="en"
    )
}