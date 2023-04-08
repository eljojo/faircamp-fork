use super::Translations;

pub fn translations() -> Translations {
    Translations {
        any_amount: String::from("Beliebiger Betrag"),
        audio_format_description_aac: String::from("Durchschnittliche Komprimierungsqualität – sinnvoll wenn dein Player keine besseren Formate unterstützt"),
        audio_format_description_aiff: String::from("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
        audio_format_description_flac: String::from("Verlustfrei komprimiert – Beste Wahl für Archivierung"),
        audio_format_description_mp3_vbr: String::from("Unterdurchschnittliche Komprimierungsqualität – Sinnvoll wenn Kompatibilität mit älteren Playern benötigt wird"),
        audio_format_description_ogg_vorbis: String::from("Durchschnittliche Komprimierungsqualität – Sinnvoll wenn dein Player keine besseren Formate unterstützt"),
        audio_format_description_opus_48: String::from("Bestverfügbare Komprimierung bei 48Kbps – Beste Wahl für Streaming mit vielen Zuhörern"),
        audio_format_description_opus_96: String::from("Bestverfügbare Komprimierung bei 96Kbps – Beste Wahl für Streaming"),
        audio_format_description_opus_128: String::from("Bestverfügbare Komprimierung bei 128Kbps – Beste Wahl zum offline hören"),
        audio_format_description_wav: String::from("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
        audio_player_widget_for_release: String::from(r#"Audio Player Widget für den Release "{title}""#),
        audio_player_widget_for_track: String::from(r#"Audio Player Widget für den Track "{title}""#),
        available_formats: String::from("Verfügbare Formate:"),
        buy: String::from("Kaufen"),
        buy_release: String::from("Release kaufen"),
        close: String::from("Schließen"),
        copied: String::from("Kopiert"),
        copy: String::from("Kopieren"),
        confirm: String::from("Bestätigen"),
        r#continue: String::from("Fortfahren"),
        cover_image: String::from("Cover Bild"),
        default_unlock_text: String::from("\
Downloads für diesen Release sind verfügbar in dem du einen \
Freischaltecode eingibst. Wenn du noch keinen hast, musst du \
ihn von den Künstler*innen/Leuten die diese Seite betreiben \
organisieren - kontaktiere diese oder schau ob du Informationen \
dazu auf der Release Seite selbst findest. Downloadcodes werden \
manchmal als Perks bei Crowdfunding Kampagnen oder Abos vergeben, \
schau auch dort falls dir welche bekannt sind!"),
        download: String::from("Download"),
        download_choice_hints: String::from(r##"Einzelne Tracks oder Downloads in anderen Formaten sind unten verfügbar. Nicht sicher welches Format du nehmen sollst? Folge den <a href="#hints">Tipps</a> unten."##),
        download_with_code: String::from("Download mit Code"),
        embed: String::from("Einbetten"),
        embed_entire_release: String::from("Den gesamten Release einbetten"),
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
        image_descriptions_permalink: String::from("bildbeschreibungen"),
        made_or_arranged_payment: String::from("Ich habe die Bezahlung durchgeführt oder arrangiert"),
        missing_image_description_note: String::from("Fehlende Bildbeschreibung<br>Klick für mehr Info"),
        name_your_price: String::from("Nenne einen Preis"),
        option: String::from("Option"),
        pay_on_liberapay: String::from("Auf Liberapay bezahlen:"),
        payment_options: String::from("Zahlungsoptionen:"),
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
Ersetze den letzten Abschnitt der Adresse - \
/checkout/{page_hash}{index_suffix} - \
mit /download/[dein-freischaltecode]{index_suffix} and drücke dann Enter."),
        up_to_xxx: String::from("Bis zu {xxx}"),
        xxx_or_more: String::from("{xxx} oder mehr")
    }
}