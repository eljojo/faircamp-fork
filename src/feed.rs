use indoc::formatdoc;

pub fn generate() -> String {
    let items = &["TODO", "TODO"]
        .iter()
        .map(|_entry_todo|
            formatdoc!(
                r#"
                    <item>
                        <description>{item_description}</description>
                        <guid>{base_url}/{item_permalink}</guid>
                        <link>{base_url}/{item_permalink}</link>
                        <pubDate>{item_date}</pubDate>
                        <title>{item_title}</title>
                    </item>
                "#,
                base_url="TODO",
                item_date="TODO",
                item_description="TODO",
                item_permalink="TODO",
                item_title="TODO",
            )
        )
        .collect::<Vec<String>>()
        .join("\n");
    
    formatdoc!(
        r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
                <channel>
                    <atom:link href="{base_url}/feed.rss" rel="self" type="application/rss+xml"/>
                    <description>{description}</description>
                    <image>
                        <url>{base_url}/{feed_image}</url>
                        <title>{image_title}</title>
                        <link>{base_url}</link>
                    </image>
                    <language>{language}</language>
                    <lastBuildDate>{build_date}</lastBuildDate>
                    <link>{base_url}</link>
                    <title>{feed_title}</title>
                    
                    {items}
                </channel>
            </rss>
        "#,
        base_url="https://TODO",
        build_date="TODO",
        description="TODO",
        feed_image="TODO.png",
        feed_title="TODO",
        image_title="TODO",
        items=items,
        language="en"
    )
}