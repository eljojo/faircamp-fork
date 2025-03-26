// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations};

pub const EN: Translations = Translations {
    audio_format_alac: Reviewed("Lossless and compressed, if you exclusively use Apple products choose this over FLAC"),
    audio_format_average: Reviewed("Average compression, appropriate if your player does not support better formats"),
    audio_format_flac: Reviewed("Lossless and compressed, best choice for archival"),
    audio_format_mp3: Reviewed("Inefficient compression, appropriate if compatibility with older players is needed"),
    audio_format_opus_48: Reviewed("Excellent compression, frugal quality, good choice if space is limited"),
    audio_format_opus_96: Reviewed("Excellent compression, standard quality, good choice for offline listening"),
    audio_format_opus_128: Reviewed("Excellent compression, highest quality, best choice for offline listening"),
    audio_format_uncompressed: Reviewed("Uncompressed large files, appropriate only for audio production"),
    audio_player_widget_for_xxx: Reviewed(r#"Audio player widget for "{title}""#),
    auto_generated_cover: Reviewed("Automatically generated cover"),
    available_formats: Reviewed("Available formats:"),
    browse: Reviewed("Browse"),
    buy: Reviewed("Buy"),
    close: Reviewed("Close"),
    copied: Reviewed("Copied"),
    copy: Reviewed("Copy"),
    copy_link: Reviewed("Copy link"),
    confirm: Reviewed("Confirm"),
    r#continue: Reviewed("Continue"),
    cover_image: Reviewed("Cover Image"),
    default_unlock_info: Reviewed("You need to enter a code to unlock these downloads. Ask the site operators for how to obtain one."),
    download: Reviewed("Download"),
    download_code_seems_incorrect: Reviewed("The download code seems to be incorrect, please check for typos."),
    downloads: Reviewed("Downloads"),
    downloads_permalink: Reviewed("downloads"),
    embed: Reviewed("Embed"),
    embed_entire_release: Reviewed("Embed the entire release"),
    enter_code_here: Reviewed("Enter code here"),
    external_link: Reviewed("External Link"),
    extras: Reviewed("Extras"),
    failed: Reviewed("Failed"),
    feed: Reviewed("Feed"),
    fixed_price: Reviewed("Fixed price:"),
    generic_rss: Reviewed("Generic RSS"),
    image_descriptions: Reviewed("Image Descriptions"),
    image_descriptions_guide: Reviewed("\
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
    image_descriptions_permalink: Reviewed("image-descriptions"),
    javascript_is_disabled_listen_at_xxx: Reviewed("JavaScript is disabled – Listen at {link}"),
    javascript_is_disabled_text: Reviewed("JavaScript is disabled – Some features are not available"),
    listen: Reviewed("Listen"),
    loading: Reviewed("Loading"),
    m3u_playlist: Reviewed("M3U Playlist"),
    made_or_arranged_payment: Reviewed("I have made or arranged the payment"),
    missing_image_description_note: Reviewed("Missing image description<br>Click to learn more"),
    more: Reviewed("More"),
    mute: Reviewed("Mute"),
    name_your_price: Reviewed("Name your price"),
    next_track: Reviewed("Next Track"),
    nothing_found_for_xxx: Reviewed("Nothing found for '{query}'"),
    pause: Reviewed("Pause"),
    play: Reviewed("Play"),
    playback_position: Reviewed("Playback position"),
    player_closed: Reviewed("Player closed"),
    player_open_playing_xxx: Reviewed("Player open, playing {title}"),
    player_open_with_xxx: Reviewed("Player open with {title}"),
    previous_track: Reviewed("Previous Track"),
    purchase_downloads: Reviewed("Purchase downloads"),
    purchase_permalink: Reviewed("purchase"),
    recommended_format: Reviewed("Recommended Format"),
    search: Reviewed("Search"),
    showing_featured_items: Reviewed("Showing featured items"),
    showing_xxx_results_for_xxx: Reviewed("Showing {count} results for '{query}'"),
    skip_to_main_content: Reviewed("Skip to main content"),
    subscribe: Reviewed("Subscribe"),
    subscribe_permalink: Reviewed("subscribe"),
    unlisted: Reviewed("Unlisted"),
    unlock: Reviewed("Unlock"),
    unlock_downloads: Reviewed("Unlock downloads"),
    unlock_manual_instructions: Reviewed("\
To unlock the download, please make the following \
changes to the address in your browser's address bar.\
<br><br>\
Before you try this please be aware that incorrect codes or \
address modifications will take you to a 404 page. In this case \
use the Back button and closely follow the instructions again.\
<br><br>\
Replace the final part of the address - /{unlock_permalink}/{page_hash}{index_suffix} - \
with /{downloads_permalink}/[your-download-code]{index_suffix} and then press Enter."),
    unlock_permalink: Reviewed("unlock"),
    unmute: Reviewed("Unmute"),
    up_to_xxx: Reviewed("Up to {xxx}"),
    visual_impairment: Reviewed("Visual Impairment"),
    volume: Reviewed("Volume"),
    xxx_and_others: Reviewed(r#"{xxx} and <a href="{others_link}">others</a>"#),
    xxx_hours: Reviewed("{xxx} hours"),
    xxx_minutes: Reviewed("{xxx} minutes"),
    xxx_or_more: Reviewed("{xxx} or more"),
    xxx_seconds: Reviewed("{xxx} seconds")
};
