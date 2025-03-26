// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations};

pub const DE: Translations = Translations {
    audio_format_alac: Reviewed("Verlustfrei komprimiert, wenn du nur Apple Produkte verwendest wähle dies hier statt FLAC"),
    audio_format_average: Reviewed("Durchschnittliche Komprimierung, sinnvoll wenn dein Player keine besseren Formate unterstützt"),
    audio_format_flac: Reviewed("Verlustfrei komprimiert, beste Wahl für Archivierung"),
    audio_format_mp3: Reviewed("Ineffiziente Komprimierung, sinnvoll wenn Kompatibilität mit älteren Playern benötigt wird"),
    audio_format_opus_48: Reviewed("Exzellente Komprimierung, genügsame Qualität, gute Wahl bei limitiertem Speicherplatz"),
    audio_format_opus_96: Reviewed("Exzellente Komprimierung, Standard Qualität, gute Wahl zum offline hören"),
    audio_format_opus_128: Reviewed("Exzellente Komprimierung, höchste Qualität, beste Wahl zum offline hören"),
    audio_format_uncompressed: Reviewed("Unkomprimierte, große Dateien – Nur für Audio Produktion sinnvoll"),
    audio_player_widget_for_xxx: Reviewed(r#"Audio Player Widget für "{title}""#),
    auto_generated_cover: Reviewed("Automatisch generiertes Cover"),
    available_formats: Reviewed("Verfügbare Formate:"),
    browse: Reviewed("Stöbern"),
    buy: Reviewed("Kaufen"),
    close: Reviewed("Schließen"),
    copied: Reviewed("Kopiert"),
    copy: Reviewed("Kopieren"),
    copy_link: Reviewed("Link kopieren"),
    confirm: Reviewed("Bestätigen"),
    r#continue: Reviewed("Fortfahren"),
    cover_image: Reviewed("Cover Bild"),
    default_unlock_info: Reviewed("Du musst einen Code eingeben um diese Downloads freizuschalten. Frag bei den Seitenbetreiber*innen nach wie du einen bekommst."),
    download: Reviewed("Downloaden"),
    download_code_seems_incorrect: Reviewed("Der Downloadcode scheint nicht korrekt zu sein, überprüfe ihn bitte auf Tippfehler."),
    downloads: Reviewed("Downloads"),
    downloads_permalink: Reviewed("downloads"),
    embed: Reviewed("Einbetten"),
    embed_entire_release: Reviewed("Den gesamten Release einbetten"),
    enter_code_here: Reviewed("Code hier eingeben"),
    external_link: Reviewed("Externer Link"),
    extras: Reviewed("Extras"),
    failed: Reviewed("Fehler"),
    feed: Reviewed("Feed"),
    fixed_price: Reviewed("Fixer Preis:"),
    generic_rss: Reviewed("Generisches RSS"),
    image_descriptions: Reviewed("Bildbeschreibungen"),
    image_descriptions_guide: Reviewed("\
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
    image_descriptions_permalink: Reviewed("bildbeschreibungen"),
    javascript_is_disabled_listen_at_xxx: Reviewed("JavaScript ist deaktiviert – Anhören auf {link}"),
    javascript_is_disabled_text: Reviewed("JavaScript ist deaktiviert – Manche Features sind nicht verfügbar"),
    listen: Reviewed("Anhören"),
    loading: Reviewed("Lädt"),
    m3u_playlist: Reviewed("M3U Playlist"),
    made_or_arranged_payment: Reviewed("Ich habe die Bezahlung durchgeführt oder arrangiert"),
    missing_image_description_note: Reviewed("Fehlende Bildbeschreibung<br>Klick für mehr Info"),
    more: Reviewed("Mehr"),
    mute: Reviewed("Stummschalten"),
    name_your_price: Reviewed("Nenne einen Preis"),
    next_track: Reviewed("Nächster Track"),
    nothing_found_for_xxx: Reviewed("Nichts für '{query}' gefunden"),
    pause: Reviewed("Pausieren"),
    play: Reviewed("Abspielen"),
    playback_position: Reviewed("Wiedergabeposition"),
    player_closed: Reviewed("Player geschlossen"),
    player_open_playing_xxx: Reviewed("Player offen, spielt {title}"),
    player_open_with_xxx: Reviewed("Player offen mit {title}"),
    previous_track: Reviewed("Vorheriger Track"),
    purchase_downloads: Reviewed("Downloads bezahlen"),
    purchase_permalink: Reviewed("bezahlen"),
    recommended_format: Reviewed("Empfohlenes Format"),
    search: Reviewed("Suchen"),
    showing_featured_items: Reviewed("Gefeaturete Einträge werden angezeigt"),
    showing_xxx_results_for_xxx: Reviewed("{count} Ergebnisse für '{query}' werden angezeigt"),
    skip_to_main_content: Reviewed("Zum Hauptinhalt springen"),
    subscribe: Reviewed("Abonnieren"),
    subscribe_permalink: Reviewed("abonnieren"),
    unlisted: Reviewed("Ungelistet"),
    unlock: Reviewed("Freischalten"),
    unlock_downloads: Reviewed("Downloads freischalten"),
    unlock_manual_instructions: Reviewed("\
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
mit /{downloads_permalink}/[dein-downloadcode]{index_suffix} und drücke dann Enter."),
    unlock_permalink: Reviewed("freischalten"),
    unmute: Reviewed("Lautschalten"),
    up_to_xxx: Reviewed("Bis zu {xxx}"),
    visual_impairment: Reviewed("Visuelle Beeinträchtigung"),
    volume: Reviewed("Lautstärke"),
    xxx_and_others: Reviewed(r#"{xxx} und <a href="{others_link}">Weitere</a>"#),
    xxx_hours: Reviewed("{xxx} Stunden"),
    xxx_minutes: Reviewed("{xxx} Minuten"),
    xxx_or_more: Reviewed("{xxx} oder mehr"),
    xxx_seconds: Reviewed("{xxx} Sekunden")
};
