use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_description_aac: String::from("Average encoding quality – appropriate if your player does not support better formats"),
        audio_format_description_aiff: String::from("Uncompressed large files – appropriate only for audio production"),
        audio_format_description_flac: String::from("Lossless and compressed – best choice for archival"),
        audio_format_description_mp3_vbr: String::from("Inferior encoding quality – appropriate if compatibility with older players is needed"),
        audio_format_description_ogg_vorbis: String::from("Average encoding quality – appropriate if your player does not support better formats"),
        // TODO: Both hints "for streaming" below address the wrong
        // question somehow: The person reading this wants to download,
        // streaming choice is only relevant to someone who would stream
        // to an audience themselves?
        audio_format_description_opus_48: String::from("State-of-the-art encoding quality at 48Kbps – best choice for high-demand streaming"),
        audio_format_description_opus_96: String::from("State-of-the-art encoding quality at 96Kbps – best choice for streaming"),
        audio_format_description_opus_128: String::from("State-of-the-art encoding quality at 128Kbps – best choice for offline listening"),
        audio_format_description_wav: String::from("Uncompressed large files – appropriate only for audio production"),
        audio_player_widget_for_release: String::from(r#"Audio player widget for the release "{title}""#),
        audio_player_widget_for_track: String::from(r#"Audio player widget for the track "{title}""#),
        auto_generated_cover: String::from("Automatically generated cover"),
        available_formats: String::from("Available formats:"),
        close: String::from("Close"),
        copied: String::from("Copied"),
        copy: String::from("Copy"),
        confirm: String::from("Confirm"),
        r#continue: String::from("Continue"),
        cover_image: String::from("Cover Image"),
        default_unlock_text: String::from("You need to enter a code to unlock these downloads. Ask the site operators for how to obtain one."),
        downloads: String::from("Downloads"),
        downloads_permalink: String::from("downloads"),
        download_choice_hints: String::from(r##"Single track downloads or downloads in other formats are available below. Not sure what format to pick? See the <a href="#hints">hints</a> below."##),
        embed: String::from("Embed"),
        embed_entire_release: String::from("Embed the entire release"),
        enter_code_here: String::from("Enter code here"),
        entire_release: String::from("Entire Release"),
        failed: String::from("Failed"),
        feed: String::from("Feed"),
        fixed_price: String::from("Fixed price:"),
        format_guide: String::from("Format Guide:"),
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
        made_or_arranged_payment: String::from("I have made or arranged the payment"),
        missing_image_description_note: String::from("Missing image description<br>Click to learn more"),
        name_your_price: String::from("Name your price"),
        option: String::from("Option"),
        pay_on_liberapay: String::from("Pay on liberapay:"),
        payment_options: String::from("Payment options:"),
        purchase_downloads: String::from("Purchase downloads"),
        purchase_permalink: String::from("purchase"),
        recommended_format: String::from("Recommended Format"),
        rss_feed: String::from("RSS Feed"),
        share: String::from("Share"),
        share_not_available_navigator_clipboard: String::from("Not available in your browser (navigator.clipboard is not supported)"),
        share_not_available_requires_javascript: String::from("Not available in your browser (requires JavaScript)"),
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
Replace the final part of the address - /unlock/{page_hash}{index_suffix} - \
with /downloads/[your-unlock-code]{index_suffix} and then press Enter."),
        up_to_xxx: String::from("Up to {xxx}"),
        xxx_or_more: String::from("{xxx} or more")
    }
}