// SPDX-FileCopyrightText: 2023 Harald Eilertsen
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const NB: Translations = Translations {
    audio_format_alac: Reviewed("Tapsfritt og komprimert, velg dette over FLAC dersom du kun bruker Apple produkter"),
    audio_format_average: Reviewed("Middels komprimering, passende dersom avspilleren din ikke støtter bedre formater"),
    audio_format_flac: Reviewed("Tapsfritt og komprimert, det beste valget for arkivering"),
    audio_format_mp3: Reviewed("Lite effektiv komprimering, passende dersom kompatibilitet med eldre avspillere er nødvendig"),
    audio_format_opus_48: Reviewed("Utmerket kompresjon, begrenset kvalitet, et godt valg dersom det er begrenset lagringsplass"),
    audio_format_opus_96: Reviewed("Utmerket kompresjon, standard kvalitet, et godt valg for å lytte frakoblet"),
    audio_format_opus_128: Reviewed("Utmerket kompresjon, høyeste kvalitet, det beste valget for å lytte frakoblet"),
    audio_format_uncompressed: Reviewed("Ukomprimerte store filer, kun passende for lydproduksjon"),
    audio_player_widget_for_xxx: Unreviewed(r#"Lydavspillerelement for "{title}""#),
    auto_generated_cover: Reviewed("Automatisk generert omslag"),
    available_formats: Reviewed("Tilgjengelige formater:"),
    close: Reviewed("Lukk"),
    copied: Reviewed("Kopiert"),
    copy: Reviewed("Kopier"),
    confirm: Reviewed("Bekreft"),
    r#continue: Reviewed("Fortsett"),
    cover_image: Reviewed("Omslagsbilde"),
    default_unlock_text: Reviewed("Du må skrive inn en kode for å få lastet ned disse filene. Spør sidens administratorer for hvordan du kan få en."),
    downloads: Reviewed("Nedlastinger"),
    downloads_permalink: Reviewed("nedlastinger"),
    embed: Reviewed("Bygg inn"),
    embed_entire_release: Reviewed("Bygg inn hele utgivelsen"),
    enter_code_here: Reviewed("Skriv inn koden her"),
    extras: Reviewed("Ekstra"),
    failed: Reviewed("Feilet"),
    feed: Reviewed("Abonner"),
    fixed_price: Reviewed("Fast pris:"),
    image_descriptions: Reviewed("Bildebeskrivelser"),
    image_descriptions_guide: Reviewed("\
Millioner av mennesker leser weben ved hjelp av skjermlesere \
på grunn av at de ikke kan se (eller ikke se godt nok). \
Bilder uten en beskrivende tekst er utilgjengelige for dem, \
og dette er grunnen til at vi burde gjøre en innsats for å \
beskrive bildene for dem.<br><br>\
\
Se faircamp sin README filr for hvordan du legger til \
bildebeskrivelser. Det er enkelt og gjør verden bedre \
for blinde og svaksynte.<br><br>\
\
Her er noen tips for å lage gode bildebeskrivelser:<br>\
- Noe beskrivelse er bedre enn ingen beskrivelse, ikke vær bekymret for om du gjør det feil.<br>\
- Hold den knapp. Skriv så mye som trengs, men hold den samtidig så kort som mulig.<br>\
- Ikke tolk. Beskriv hva som vises og som er relevant for forståelsen. Ikke gi noen videre analyse utover det.<br>\
- Du kan beskrive farger der hvor det gir mening - mange har mistet synet i løpet av livet, og forstår og setter pris på farger."),
    image_descriptions_permalink: Reviewed("image-descriptions"),
    made_or_arranged_payment: Reviewed("Jeg har utført eller ordnet med betaling"),
    missing_image_description_note: Reviewed("Manglende bildebeskrivelse<br>Klikk for å lære mer"),
    more: Reviewed("Mer"),
    name_your_price: Reviewed("Velg din egen pris"),
    purchase_downloads: Reviewed("Kjøp nedlastinger"),
    purchase_permalink: Reviewed("kjop"),
    recommended_format: Reviewed("Anbefalt format"),
    rss_feed: Reviewed("RSS-Strøm"),
    unlock: Reviewed("Lås opp"),
    unlock_downloads: Reviewed("Lås opp nedlastinger"),
    unlock_permalink: Reviewed("las-opp"),
    unlock_code_seems_incorrect: Reviewed("Koden for å låse opp er ikke riktig, sjekk om du har noen skrivefeil."),
    unlock_manual_instructions: Reviewed("\
For å låse opp nedlastingen må du gjøre følgende endringer \
til addressen i nettleserens addressefelt.\
<br><br>\
Før du prøver på dette, så vær oppmerksom på at feil kode eller \
addresseendringer vil føre til en side som ikke finnes. Hvis det \
skjer, bruk tilbake-knappen og prøv å følge instruksjonene nøye igjen.\
<br><br>\
Erstatt den siste delen av addressen - /{unlock_permalink}/{page_hash}{index_suffix} - \
med /{downloads_permalink}/[your-unlock-code]{index_suffix} og trykk ENTER."),
    up_to_xxx: Reviewed("Opp til {xxx}"),
    xxx_and_others: Reviewed(r#"{xxx} og <a href="{others_link}">mer</a>"#),
    xxx_or_more: Reviewed("{xxx} eller mer"),
    ..Translations::UNTRANSLATED
};
