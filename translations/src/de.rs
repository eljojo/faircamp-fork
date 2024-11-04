// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const DE: Translations = Translations {
    audio_format_alac: reviewed!("Verlustfrei komprimiert, wenn du nur Apple Produkte verwendest wähle dies hier statt FLAC"),
    audio_format_average: reviewed!("Durchschnittliche Komprimierung, sinnvoll wenn dein Player keine besseren Formate unterstützt"),
    audio_format_flac: reviewed!("Verlustfrei komprimiert, beste Wahl für Archivierung"),
    audio_format_mp3: reviewed!("Ineffiziente Komprimierung, sinnvoll wenn Kompatibilität mit älteren Playern benötigt wird"),
    audio_format_opus_48: reviewed!("Exzellente Komprimierung, genügsame Qualität, gute Wahl bei limitiertem Speicherplatz"),
    audio_format_opus_96: reviewed!("Exzellente Komprimierung, Standard Qualität, gute Wahl zum offline hören"),
    audio_format_opus_128: reviewed!("Exzellente Komprimierung, höchste Qualität, beste Wahl zum offline hören"),
    audio_format_uncompressed: reviewed!("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
    audio_player_widget_for_xxx: reviewed!(r#"Audio Player Widget für "{title}""#),
    auto_generated_cover: reviewed!("Automatisch generiertes Cover"),
    available_formats: reviewed!("Verfügbare Formate:"),
    buy: reviewed!("Kaufen"),
    copied: reviewed!("Kopiert"),
    copy: reviewed!("Kopieren"),
    copy_link: reviewed!("Link kopieren"),
    confirm: reviewed!("Bestätigen"),
    r#continue: reviewed!("Fortfahren"),
    cover_image: reviewed!("Cover Bild"),
    default_unlock_text: reviewed!("Du musst einen Code eingeben um diese Downloads freizuschalten. Frag bei den Seitenbetreiber*innen nach wie du einen bekommst."),
    dimmed: reviewed!("Gedimmt"),
    download: reviewed!("Downloaden"),
    downloads: reviewed!("Downloads"),
    downloads_permalink: reviewed!("downloads"),
    embed: reviewed!("Einbetten"),
    embed_entire_release: reviewed!("Den gesamten Release einbetten"),
    enter_code_here: reviewed!("Code hier eingeben"),
    external_link: reviewed!("Externer Link"),
    extras: reviewed!("Extras"),
    failed: reviewed!("Fehler"),
    feed: reviewed!("Feed"),
    fixed_price: reviewed!("Fixer Preis:"),
    image_descriptions: reviewed!("Bildbeschreibungen"),
    image_descriptions_guide: reviewed!("\
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
    image_descriptions_permalink: reviewed!("bildbeschreibungen"),
    listen: reviewed!("Anhören"),
    loading: reviewed!("Lädt"),
    m3u_playlist: reviewed!("M3U Playlist"),
    made_or_arranged_payment: reviewed!("Ich habe die Bezahlung durchgeführt oder arrangiert"),
    missing_image_description_note: reviewed!("Fehlende Bildbeschreibung<br>Klick für mehr Info"),
    more: reviewed!("Mehr"),
    muted: reviewed!("Stummgeschaltet"),
    name_your_price: reviewed!("Nenne einen Preis"),
    next_track: reviewed!("Nächster Track"),
    pause: reviewed!("Pausieren"),
    play: reviewed!("Abspielen"),
    previous_track: reviewed!("Vorheriger Track"),
    purchase_downloads: reviewed!("Downloads bezahlen"),
    purchase_permalink: reviewed!("bezahlen"),
    recommended_format: reviewed!("Empfohlenes Format"),
    rss_feed: reviewed!("RSS Feed"),
    this_site_was_created_with_faircamp: reviewed!("Diese Seite wurde mit {faircamp_link} erstellt"),
    unlisted: reviewed!("Ungelistet"),
    unlock: reviewed!("Freischalten"),
    unlock_downloads: reviewed!("Downloads freischalten"),
    unlock_permalink: reviewed!("freischalten"),
    unlock_code_seems_incorrect: reviewed!("Der Freischaltecode scheint nicht korrekt zu sein, überprüfe ihn bitte auf Tippfehler."),
    unlock_manual_instructions: reviewed!("\
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
    up_to_xxx: reviewed!("Bis zu {xxx}"),
    visual_impairment: reviewed!("Visuelle Beeinträchtigung"),
    volume: reviewed!("Lautstärke"),
    xxx_and_others: reviewed!(r#"{xxx} und <a href="{others_link}">Weitere</a>"#),
    xxx_or_more: reviewed!("{xxx} oder mehr")
};
