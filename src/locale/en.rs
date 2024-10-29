// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Lossless and compressed, if you exclusively use Apple products choose this over FLAC"),
        audio_format_average: String::from("Average compression, appropriate if your player does not support better formats"),
        audio_format_flac: String::from("Lossless and compressed, best choice for archival"),
        audio_format_mp3: String::from("Inefficent compression, appropriate if compatibility with older players is needed"),
        audio_format_opus_48: String::from("Excellent compression, frugal quality, good choice if space is limited"),
        audio_format_opus_96: String::from("Excellent compression, standard quality, good choice for offline listening"),
        audio_format_opus_128: String::from("Excellent compression, highest quality, best choice for offline listening"),
        audio_format_uncompressed: String::from("Uncompressed large files, appropriate only for audio production"),
        audio_player_widget_for_release: String::from(r#"Audio player widget for the release "{title}""#),
        audio_player_widget_for_track: String::from(r#"Audio player widget for the track "{title}""#),
        auto_generated_cover: String::from("Automatically generated cover"),
        available_formats: String::from("Available formats:"),
        buy: String::from("Buy"),
        copied: String::from("Copied"),
        copy: String::from("Copy"),
        copy_link: String::from("Copy link"),
        confirm: String::from("Confirm"),
        r#continue: String::from("Continue"),
        cover_image: String::from("Cover Image"),
        default_unlock_text: String::from("You need to enter a code to unlock these downloads. Ask the site operators for how to obtain one."),
        dimmed: String::from("Dimmed"),
        download: String::from("Download"),
        downloads: String::from("Downloads"),
        downloads_permalink: String::from("downloads"),
        embed: String::from("Embed"),
        embed_entire_release: String::from("Embed the entire release"),
        enter_code_here: String::from("Enter code here"),
        external_link: String::from("External Link"),
        extras: String::from("Extras"),
        failed: String::from("Failed"),
        feed: String::from("Feed"),
        fixed_price: String::from("Fixed price:"),
        image_descriptions: String::from("Image Descriptions"),
        image_descriptions_guide: String::from("\
Millions of people browse the web using screen-readers \
because they can not see (or not well enough). Images \
without textual descriptions are inaccessible to them, \
and this is why we should make the effort to provide \
image descriptions for them.<br><br>\
\
Consult the faircamp README for how to add image \
descriptions, it's simple and an act of \
kindness.<br><br>\
\
Here are some tips for writing good image descriptions:<br>\
- Any description is better than having no description, don't worry about doing it wrong.<br>\
- Make it concise. Write as much as needed, but at the same time keep it as short as possible.<br>\
- Don't interpret. Describe what is there and relevant for its understanding, don't analyze beyond that.<br>\
- You can use colors where it makes sense - many people only lost their sight later on and understand and appreciate colors."),
        image_descriptions_permalink: String::from("image-descriptions"),
        listen: String::from("Listen"),
        loading: String::from("Loading"),
        m3u_playlist: String::from("M3U Playlist"),
        made_or_arranged_payment: String::from("I have made or arranged the payment"),
        missing_image_description_note: String::from("Missing image description<br>Click to learn more"),
        more: String::from("More"),
        muted: String::from("Muted"),
        name_your_price: String::from("Name your price"),
        next_track: String::from("Next Track"),
        option: String::from("Option"),
        pause: String::from("Pause"),
        payment_options: String::from("Payment options:"),
        play: String::from("Play"),
        previous_track: String::from("Previous Track"),
        purchase_downloads: String::from("Purchase downloads"),
        purchase_permalink: String::from("purchase"),
        recommended_format: String::from("Recommended Format"),
        rss_feed: String::from("RSS Feed"),
        this_site_was_created_with_faircamp: String::from("This site was created with {faircamp_link}"),
        unlisted: String::from("Unlisted"),
        unlock: String::from("Unlock"),
        unlock_downloads: String::from("Unlock downloads"),
        unlock_permalink: String::from("unlock"),
        unlock_code_seems_incorrect: String::from("The unlock code seems to be incorrect, please check for typos."),
        unlock_manual_instructions: String::from("\
To unlock the download, please make the below described \
changes to the address in your browser's adress bar.\
<br><br>\
Before you embark on this please be aware that wrong codes or \
address modifications take you to a 404 page. In this case \
use the Back button and closely follow the instructions again.\
<br><br>\
Replace the final part of the address - /{unlock_permalink}/{page_hash}{index_suffix} - \
with /{downloads_permalink}/[your-unlock-code]{index_suffix} and then press Enter."),
        up_to_xxx: String::from("Up to {xxx}"),
        visual_impairment: String::from("Visual Impairment"),
        volume: String::from("Volume"),
        xxx_and_others: String::from(r#"{xxx} and <a href="{others_link}">others</a>"#),
        xxx_or_more: String::from("{xxx} or more")
    }
}
