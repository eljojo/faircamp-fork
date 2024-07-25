// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Verlustfrei komprimiert, wenn du nur Apple Produkte verwendest wähle dies hier statt FLAC"),
        audio_format_average: String::from("Durchschnittliche Komprimierung, sinnvoll wenn dein Player keine besseren Formate unterstützt"),
        audio_format_flac: String::from("Verlustfrei komprimiert, beste Wahl für Archivierung"),
        audio_format_mp3: String::from("Ineffiziente Komprimierung, sinnvoll wenn Kompatibilität mit älteren Playern benötigt wird"),
        audio_format_opus_48: String::from("Exzellente Komprimierung, genügsame Qualität, gute Wahl bei limitiertem Speicherplatz"),
        audio_format_opus_96: String::from("Exzellente Komprimierung, Standard Qualität, gute Wahl zum offline hören"),
        audio_format_opus_128: String::from("Exzellente Komprimierung, höchste Qualität, beste Wahl zum offline hören"),
        audio_format_uncompressed: String::from("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
        audio_player_widget_for_release: String::from(r#"Audio Player Widget für den Release "{title}""#),
        audio_player_widget_for_track: String::from(r#"Audio Player Widget für den Track "{title}""#),
        auto_generated_cover: String::from("Automatisch generiertes Cover"),
        available_formats: String::from("Verfügbare Formate:"),
        buy: String::from("Kaufen"),
        copied: String::from("Kopiert"),
        copy: String::from("Kopieren"),
        copy_link: String::from("Link kopieren"),
        copy_link_to_track: String::from("Link zu Track kopieren"),
        confirm: String::from("Bestätigen"),
        r#continue: String::from("Fortfahren"),
        cover_image: String::from("Cover Bild"),
        default_unlock_text: String::from("Du musst einen Code eingeben um diese Downloads freizuschalten. Frag bei den Seitenbetreiber*innen nach wie du einen bekommst."),
        dimmed: String::from("Gedimmt"),
        download: String::from("Downloaden"),
        downloads: String::from("Downloads"),
        downloads_permalink: String::from("downloads"),
        embed: String::from("Einbetten"),
        embed_entire_release: String::from("Den gesamten Release einbetten"),
        enter_code_here: String::from("Code hier eingeben"),
        external_link: String::from("Externer Link"),
        extras: String::from("Extras"),
        failed: String::from("Fehler"),
        feed: String::from("Feed"),
        fixed_price: String::from("Fixer Preis:"),
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
        loading: String::from("Lädt"),
        made_or_arranged_payment: String::from("Ich habe die Bezahlung durchgeführt oder arrangiert"),
        missing_image_description_note: String::from("Fehlende Bildbeschreibung<br>Klick für mehr Info"),
        more: String::from("Mehr"),
        muted: String::from("Stummgeschaltet"),
        name_your_price: String::from("Nenne einen Preis"),
        next_track: String::from("Nächster Track"),
        option: String::from("Option"),
        pause: String::from("Pausieren"),
        pay_on_liberapay: String::from("Auf Liberapay bezahlen:"),
        payment_options: String::from("Zahlungsoptionen:"),
        play: String::from("Abspielen"),
        previous_track: String::from("Vorheriger Track"),
        purchase_downloads: String::from("Downloads bezahlen"),
        purchase_permalink: String::from("bezahlen"),
        recommended_format: String::from("Empfohlenes Format"),
        rss_feed: String::from("RSS Feed"),
        unlisted: String::from("Ungelistet"),
        unlock: String::from("Freischalten"),
        unlock_downloads: String::from("Downloads freischalten"),
        unlock_permalink: String::from("freischalten"),
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
/{unlock_permalink}/{page_hash}{index_suffix} - \
mit /{downloads_permalink}/[dein-freischaltecode]{index_suffix} and drücke dann Enter."),
        up_to_xxx: String::from("Bis zu {xxx}"),
        visual_impairment: String::from("Visuelle Beeinträchtigung"),
        xxx_or_more: String::from("{xxx} oder mehr")
    }
}
