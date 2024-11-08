// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const EN: Translations = Translations {
    audio_format_alac: reviewed!("Lossless and compressed, if you exclusively use Apple products choose this over FLAC"),
    audio_format_average: reviewed!("Average compression, appropriate if your player does not support better formats"),
    audio_format_flac: reviewed!("Lossless and compressed, best choice for archival"),
    audio_format_mp3: reviewed!("Inefficent compression, appropriate if compatibility with older players is needed"),
    audio_format_opus_48: reviewed!("Excellent compression, frugal quality, good choice if space is limited"),
    audio_format_opus_96: reviewed!("Excellent compression, standard quality, good choice for offline listening"),
    audio_format_opus_128: reviewed!("Excellent compression, highest quality, best choice for offline listening"),
    audio_format_uncompressed: reviewed!("Uncompressed large files, appropriate only for audio production"),
    audio_player_widget_for_xxx: reviewed!(r#"Audio player widget for "{title}""#),
    auto_generated_cover: reviewed!("Automatically generated cover"),
    available_formats: reviewed!("Available formats:"),
    buy: reviewed!("Buy"),
    close: reviewed!("Close"),
    copied: reviewed!("Copied"),
    copy: reviewed!("Copy"),
    copy_link: reviewed!("Copy link"),
    confirm: reviewed!("Confirm"),
    r#continue: reviewed!("Continue"),
    cover_image: reviewed!("Cover Image"),
    default_unlock_text: reviewed!("You need to enter a code to unlock these downloads. Ask the site operators for how to obtain one."),
    dimmed: reviewed!("Dimmed"),
    download: reviewed!("Download"),
    downloads: reviewed!("Downloads"),
    downloads_permalink: reviewed!("downloads"),
    embed: reviewed!("Embed"),
    embed_entire_release: reviewed!("Embed the entire release"),
    enter_code_here: reviewed!("Enter code here"),
    external_link: reviewed!("External Link"),
    extras: reviewed!("Extras"),
    failed: reviewed!("Failed"),
    feed: reviewed!("Feed"),
    fixed_price: reviewed!("Fixed price:"),
    image_descriptions: reviewed!("Image Descriptions"),
    image_descriptions_guide: reviewed!("\
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
    image_descriptions_permalink: reviewed!("image-descriptions"),
    listen: reviewed!("Listen"),
    loading: reviewed!("Loading"),
    m3u_playlist: reviewed!("M3U Playlist"),
    made_or_arranged_payment: reviewed!("I have made or arranged the payment"),
    missing_image_description_note: reviewed!("Missing image description<br>Click to learn more"),
    more: reviewed!("More"),
    mute: reviewed!("Mute"),
    muted: reviewed!("Muted"),
    name_your_price: reviewed!("Name your price"),
    next_track: reviewed!("Next Track"),
    pause: reviewed!("Pause"),
    play: reviewed!("Play"),
    playback_position: reviewed!("Playback position"),
    previous_track: reviewed!("Previous Track"),
    purchase_downloads: reviewed!("Purchase downloads"),
    purchase_permalink: reviewed!("purchase"),
    recommended_format: reviewed!("Recommended Format"),
    rss_feed: reviewed!("RSS Feed"),
    search: reviewed!("Search"),
    this_site_was_created_with_faircamp: reviewed!("This site was created with {faircamp_link}"),
    unlisted: reviewed!("Unlisted"),
    unlock: reviewed!("Unlock"),
    unlock_downloads: reviewed!("Unlock downloads"),
    unlock_permalink: reviewed!("unlock"),
    unlock_code_seems_incorrect: reviewed!("The unlock code seems to be incorrect, please check for typos."),
    unlock_manual_instructions: reviewed!("\
To unlock the download, please make the below described \
changes to the address in your browser's adress bar.\
<br><br>\
Before you embark on this please be aware that wrong codes or \
address modifications take you to a 404 page. In this case \
use the Back button and closely follow the instructions again.\
<br><br>\
Replace the final part of the address - /{unlock_permalink}/{page_hash}{index_suffix} - \
with /{downloads_permalink}/[your-unlock-code]{index_suffix} and then press Enter."),
    unmute: reviewed!("Unmute"),
    up_to_xxx: reviewed!("Up to {xxx}"),
    visual_impairment: reviewed!("Visual Impairment"),
    volume: reviewed!("Volume"),
    xxx_and_others: reviewed!(r#"{xxx} and <a href="{others_link}">others</a>"#),
    xxx_minutes: reviewed!("{xxx} minutes"),
    xxx_or_more: reviewed!("{xxx} or more")
};
