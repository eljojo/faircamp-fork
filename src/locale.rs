pub struct Locale {
    /// Language code such as "en", "de" etc.
    pub language: String,
    pub strings: Translations,
    pub writing_direction: WritingDirection
}

/// A key-value mapping for every translatable string found in the interface.
/// Used at build time to interpolate text in the right language.
/// Translations whose fields are not public are instead accessed through
/// a method of the same name - these are translations that need to be called
/// as a function because they interpolate some parameter into the translation.
pub struct Translations {
    pub audio_format_description_aac: String,
    pub audio_format_description_aiff: String,
    pub audio_format_description_flac: String,
    pub audio_format_description_mp3_vbr: String,
    pub audio_format_description_ogg_vorbis: String,
    pub audio_format_description_opus_48: String,
    pub audio_format_description_opus_96: String,
    pub audio_format_description_opus_128: String,
    pub audio_format_description_wav: String,
    pub audio_player_widget_for_release: String,
    pub audio_player_widget_for_track: String,
    pub buy: String,
    pub buy_release: String,
    pub close: String,
    pub copied: String,
    pub copy: String,
    pub confirm: String,
    pub r#continue: String,
    pub cover_image: String,
    pub default_unlock_text: String,
    pub download: String,
    pub download_choice_hints: String,
    pub download_release: String,
    pub download_with_unlock_code: String,
    pub embed: String,
    pub embed_entire_release: String,
    pub embed_release: String,
    pub enter_code: String,
    pub enter_code_here: String,
    pub entire_release: String,
    pub failed: String,
    pub feed: String,
    pub format_guide: String,
    pub image_descriptions: String,
    pub image_descriptions_guide: String,
    pub made_or_arranged_payment: String,
    pub missing_image_description_note: String,
    pub name_your_price: String,
    pub option: String,
    pub pay_on_liberapay: String,
    pub recommended_format: String,
    pub rss_feed: String,
    pub share: String,
    pub share_not_available_navigator_clipboard: String,
    pub share_not_available_requires_javascript: String,
    pub unlock: String,
    pub unlock_code_seems_incorrect: String,
    unlock_manual_instructions: String,
    up_to_xxx: String,
    xxx_or_more: String
}

pub enum WritingDirection {
    Ltr,
    Rtl
}

/// TODO: In-code provision of multiple locales is mostly there for
/// prototyping right now. Final implementation will probably go back to just
/// the default en locale in code, with other locales being loaded through
/// some other, probably runtime based mechanism.
impl Locale {
    #[allow(dead_code)]
    pub fn de() -> Locale {
        Locale {
            language: String::from("de"),
            strings: Translations::de(),
            writing_direction: WritingDirection::Ltr
        }
    }

    pub fn en() -> Locale {
        Locale {
            language: String::from("en"),
            strings: Translations::en(),
            writing_direction: WritingDirection::Ltr
        }
    }

    pub fn keys() -> Locale {
        Locale {
            language: String::from("en"),
            strings: Translations::keys(),
            writing_direction: WritingDirection::Ltr
        }
    }
}

impl Translations {
    #[allow(dead_code)]
    pub fn de() -> Translations {
        Translations {
            audio_format_description_aac: String::from("Durchschnittliche Komprimierungsqualität – sinnvoll wenn dein Player keine besseren Formate unterstützt"),
            audio_format_description_aiff: String::from("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
            audio_format_description_flac: String::from("Verlustfrei komprimiert – Beste Wahl für Archivierung"),
            audio_format_description_mp3_vbr: String::from("Unterdurchschnittliche Komprimierungsqualität – Sinnvoll wenn Kompatibilität mit älteren Playern benötigt wird"),
            audio_format_description_ogg_vorbis: String::from("Durchschnittliche Komprimierungsqualität – Sinnvoll wenn dein Player keine besseren Formate unterstützt"),
            audio_format_description_opus_48: String::from("Bestverfügbare Komprimierung bei 48Kbps – Beste Wahl für Streaming mit vielen Zuhörern"),
            audio_format_description_opus_96: String::from("Bestverfügbare Komprimierung bei 96Kbps – Beste Wahl für Streaming"),
            audio_format_description_opus_128: String::from("Bestverfügbare Komprimierung bei 128Kbps – Beste Wahl zum offline hören"),
            audio_format_description_wav: String::from("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
            audio_player_widget_for_release: String::from("Audio Player Widget für alle Tracks von einem Release"),
            audio_player_widget_for_track: String::from("Audio Player Widget für einen Track"),
            buy: String::from("Kaufen"),
            buy_release: String::from("Release kaufen"),
            close: String::from("Schließen"),
            copied: String::from("Kopiert"),
            copy: String::from("Kopieren"),
            confirm: String::from("Bestätigen"),
            r#continue: String::from("Fortfahren"),
            cover_image: String::from("Cover Bild"),
            default_unlock_text: String::from("\
Downloads für diesen Release sind verfügbar indem man einen \
Freischaltecode. Wenn du nicht bereits einen hast, musst du \
ihn von den Künstlerinnen/Leuten die diese Seite betreiben \
organisieren - kontaktiere sie oder schau ob du Informationen \
dazu auf der Release Seite selbst findest. Downloadcodes werden \
manchmal als Perks bei Crowdfunding Kampagnen oder Abos vergeben, \
schau auch dort falls dir welche bekannt sind!"),
            download: String::from("Download"),
            download_choice_hints: String::from(r##"Einzelne Tracks oder Downloads in anderen Formaten sind unten verfügbar. Nicht sicher welches Format du nehmen sollst? Folge den <a href="#hints">Tipps</a> unten."##),
            download_release: String::from("Release downloaden"),
            download_with_unlock_code: String::from("Download mit Freischaltecode"),
            embed: String::from("Einbetten"),
            embed_entire_release: String::from("Den gesamten Release einbetten"),
            embed_release: String::from("Release einbetten"),
            enter_code: String::from("Code eingeben"),
            enter_code_here: String::from("Code hier eingeben"),
            entire_release: String::from("Gesamter Release"),
            failed: String::from("Fehler"),
            feed: String::from("Feed"),
            format_guide: String::from("Format Hilfe:"),
            image_descriptions: String::from("Bildbeschreibungen"),
            image_descriptions_guide: String::from("\
Millionen Menschen bewegen sich mit Screen Readern \
durch das Netz, da sie nicht (oder nicht ausreichend \
gut) sehen können. Bilder ohne Textbeschreibungen sind \
für sie unzugänglich, deshalb sollten wir uns darum \
kümmern für sie Bildbeschreibungen zu schreiben.<br><br>\
\
Das Faircamp README beschreibt wie Bildbeschreibungen \
hinzugefügt werden können - es ist einfach und ermöglicht \
vielen Menschen Teilhabe, die ihnen sonst oft verwehrt bleibt.<br><br>\
\
Hier ein paar Tipps zum Schreiben guter Bildbeschreibungen:<br>\
- Jede Beschreibung ist besser als keine Beschreibung, lass dich nicht von der Angst abhalten du könntest etwas falsch machen<br>\
- Halte dich kurz. Schreib soviel wie nötig, aber gleichzeitig nicht mehr als nötig.<br>\
- Beschreib was da ist und wichtig fürs Verständnis, aber analysiere und interpretiere darüber hinaus nicht.<br>\
- Du kannst Farbbeschreibungen verwenden wo es Sinn macht - viele Menschen verlieren ihre Sehkraft erst spät im Leben und verstehen und schätzen Farben."),
            made_or_arranged_payment: String::from("Ich habe die Bezahlung durchgeführt oder arrangiert"),
            missing_image_description_note: String::from("Fehlende Bildbeschreibung.<br>Klick für mehr Info"),
            name_your_price: String::from("Nenne einen Preis"),
            option: String::from("Option"),
            pay_on_liberapay: String::from("Auf Liberapay bezahlen:"),
            recommended_format: String::from("Empfohlenes Format"),
            rss_feed: String::from("RSS Feed"),
            share: String::from("Teilen"),
            share_not_available_navigator_clipboard: String::from("In deinem Browser nicht verfügbar (navigator.clipboard wird nicht unterstützt)"),
            share_not_available_requires_javascript: String::from("In deinem Browser nicht verfügbar (benötigt JavaScript)"),
            unlock: String::from("Freischalten"),
            unlock_code_seems_incorrect: String::from("Der Freischaltecode scheint nicht korrekt zu sein, überprüfe ihn bitte auf Tippfehler."),
            unlock_manual_instructions: String::from("\
Um die Downloads freizuschalten, führe bitte die unten beschriebenen \
Änderungen in der Adressleiste deines Browsers durch.\
<br><br>\
Bevor du damit beginnst, sei dir bewusst dass falsche Codes oder \
Fehler bei der Adressänderung dich zu einer 404 Seite führen. \
Falls das passiert, benutze den Zurück Button deines Browsers \
und folge den Instruktionen erneut und ganz genau.\
<br><br>\
Ersetze den letzten Abschnitt der Adresse der in etwa so aussieht - \
/checkout/[zufällige-buchstaben-und-ziffern]{index_suffix} - \
mit /download/[dein-freischaltecode]{index_suffix} and dann drücke Enter."),
            up_to_xxx: String::from("Bis zu {xxx}"),
            xxx_or_more: String::from("{xxx} oder mehr")
        }
    }

    pub fn en() -> Translations {
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
            audio_player_widget_for_release: String::from("Audio player widget for all tracks of a release"),
            audio_player_widget_for_track: String::from("Audio player widget for one track"),
            buy: String::from("Buy"),
            buy_release: String::from("Buy Release"),
            close: String::from("Close"),
            copied: String::from("Copied"),
            copy: String::from("Copy"),
            confirm: String::from("Confirm"),
            r#continue: String::from("Continue"),
            cover_image: String::from("Cover Image"),
            default_unlock_text: String::from("\
Downloads for this release are available by entering an unlock \
code. If you don't already have a code you need to obtain one \
from the artists/people who run this site - get in touch with \
them or see if there's any information on the release page \
itself. Download codes may sometimes be offered as perks on \
crowdfunding campaigns or subscriptions, so also check these \
if you know of any!"),
            download: String::from("Download"),
            download_choice_hints: String::from(r##"Single track downloads or downloads in other formats are available below. Not sure what format to pick? See the <a href="#hints">hints</a> below."##),
            download_release: String::from("Download Release"),
            download_with_unlock_code: String::from("Download with unlock code"),
            embed: String::from("Embed"),
            embed_entire_release: String::from("Embed the entire release"),
            embed_release: String::from("Embed Release"),
            enter_code: String::from("Enter Code"),
            enter_code_here: String::from("Enter code here"),
            entire_release: String::from("Entire Release"),
            failed: String::from("Failed"),
            feed: String::from("Feed"),
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
            made_or_arranged_payment: String::from("I have made or arranged the payment"),
            missing_image_description_note: String::from("Missing image description.<br>Click to learn more"),
            name_your_price: String::from("Name your price"),
            option: String::from("Option"),
            pay_on_liberapay: String::from("Pay on liberapay:"),
            recommended_format: String::from("Recommended Format"),
            rss_feed: String::from("RSS Feed"),
            share: String::from("Share"),
            share_not_available_navigator_clipboard: String::from("Not available in your browser (navigator.clipboard is not supported)"),
            share_not_available_requires_javascript: String::from("Not available in your browser (requires JavaScript)"),
            unlock: String::from("Unlock"),
            unlock_code_seems_incorrect: String::from("The unlock code seems to be incorrect, please check for typos."),
            unlock_manual_instructions: String::from("\
To unlock the download, please make the below described \
changes to the address in your browser's adress bar.\
<br><br>\
Before you embark on this please be aware that wrong codes or \
address modifications take you to a 404 page. In this case \
use the Back button and closely follow the instructions again.\
<br><br>\
Replace the final part of the address that \
looks like this - /checkout/[some-random-letters]{index_suffix} - \
with /download/[your-unlock-code]{index_suffix} and then press Enter."),
            up_to_xxx: String::from("Up to {xxx}"),
            xxx_or_more: String::from("{xxx} or more")
        }
    }

    pub fn keys() -> Translations {
        Translations {
            audio_format_description_aac: String::from("audio_format_description_aac"),
            audio_format_description_aiff: String::from("audio_format_description_aiff"),
            audio_format_description_flac: String::from("audio_format_description_flac"),
            audio_format_description_mp3_vbr: String::from("audio_format_description_mp3_vbr"),
            audio_format_description_ogg_vorbis: String::from("audio_format_description_ogg_vorbis"),
            audio_format_description_opus_48: String::from("audio_format_description_opus_48"),
            audio_format_description_opus_96: String::from("audio_format_description_opus_96"),
            audio_format_description_opus_128: String::from("audio_format_description_opus_128"),
            audio_format_description_wav: String::from("audio_format_description_wav"),
            audio_player_widget_for_release: String::from("audio_player_widget_for_release"),
            audio_player_widget_for_track: String::from("audio_player_widget_for_track"),
            buy: String::from("buy"),
            buy_release: String::from("buy_release"),
            close: String::from("close"),
            copied: String::from("copied"),
            copy: String::from("copy"),
            confirm: String::from("confirm"),
            r#continue: String::from("continue"),
            cover_image: String::from("cover_image"),
            default_unlock_text: String::from("default_unlock_text"),
            download: String::from("download"),
            download_choice_hints: String::from("download_choice_hints"),
            download_release: String::from("download_release"),
            download_with_unlock_code: String::from("download_with_unlock_code"),
            embed: String::from("embed"),
            embed_entire_release: String::from("embed_entire_release"),
            embed_release: String::from("embed_release"),
            enter_code: String::from("enter_code"),
            enter_code_here: String::from("enter_code_here"),
            entire_release: String::from("entire_release"),
            failed: String::from("failed"),
            feed: String::from("feed"),
            format_guide: String::from("format_guide"),
            image_descriptions: String::from("image_descriptions"),
            image_descriptions_guide: String::from("image_descriptions_guide"),
            made_or_arranged_payment: String::from("made_or_arranged_payment"),
            missing_image_description_note: String::from("missing_image_description_note"),
            name_your_price: String::from("name_your_price"),
            option: String::from("option"),
            pay_on_liberapay: String::from("pay_on_liberapay"),
            recommended_format: String::from("recommended_format"),
            rss_feed: String::from("rss_feed"),
            share: String::from("share"),
            share_not_available_navigator_clipboard: String::from("share_not_available_navigator_clipboard"),
            share_not_available_requires_javascript: String::from("share_not_available_requires_javascript"),
            unlock: String::from("unlock"),
            unlock_code_seems_incorrect: String::from("unlock_code_seems_incorrect"),
            unlock_manual_instructions: String::from("unlock_manual_instructions"),
            up_to_xxx: String::from("up_to_xxx"),
            xxx_or_more: String::from("xxx_or_more")
        }
    }

    pub fn unlock_manual_instructions(&self, index_suffix: &str) -> String {
        self.unlock_manual_instructions.replace("{index_suffix}", index_suffix)
    }

    pub fn up_to_xxx(&self, xxx: &str) -> String {
        self.up_to_xxx.replace("{xxx}", xxx)
    }

    pub fn xxx_or_more(&self, xxx: &str) -> String {
        self.xxx_or_more.replace("{xxx}", xxx)
    }
}