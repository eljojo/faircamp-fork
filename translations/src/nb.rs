// SPDX-FileCopyrightText: 2023 Harald Eilertsen
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const NB: Translations = Translations {
    audio_format_alac: reviewed!("Tapsfritt og komprimert, velg dette over FLAC dersom du kun bruker Apple produkter"),
    audio_format_average: reviewed!("Middels komprimering, passende dersom avspilleren din ikke støtter bedre formater"),
    audio_format_flac: reviewed!("Tapsfritt og komprimert, det beste valget for arkivering"),
    audio_format_mp3: reviewed!("Lite effektiv komprimering, passende dersom kompatibilitet med eldre avspillere er nødvendig"),
    audio_format_opus_48: reviewed!("Utmerket kompresjon, begrenset kvalitet, et godt valg dersom det er begrenset lagringsplass"),
    audio_format_opus_96: reviewed!("Utmerket kompresjon, standard kvalitet, et godt valg for å lytte frakoblet"),
    audio_format_opus_128: reviewed!("Utmerket kompresjon, høyeste kvalitet, det beste valget for å lytte frakoblet"),
    audio_format_uncompressed: reviewed!("Ukomprimerte store filer, kun passende for lydproduksjon"),
    audio_player_widget_for_xxx: unreviewed!(r#"Lydavspillerelement for "{title}""#),
    auto_generated_cover: reviewed!("Automatisk generert omslag"),
    available_formats: reviewed!("Tilgjengelige formater:"),
    buy: untranslated!(buy),
    copied: reviewed!("Kopiert"),
    copy: reviewed!("Kopier"),
    copy_link: untranslated!(copy_link),
    confirm: reviewed!("Bekreft"),
    r#continue: reviewed!("Fortsett"),
    cover_image: reviewed!("Omslagsbilde"),
    default_unlock_text: reviewed!("Du må skrive inn en kode for å få lastet ned disse filene. Spør sidens administratorer for hvordan du kan få en."),
    dimmed: untranslated!(dimmed),
    download: untranslated!(download),
    downloads: reviewed!("Nedlastinger"),
    downloads_permalink: reviewed!("nedlastinger"),
    embed: reviewed!("Bygg inn"),
    embed_entire_release: reviewed!("Bygg inn hele utgivelsen"),
    enter_code_here: reviewed!("Skriv inn koden her"),
    external_link: untranslated!(external_link),
    extras: reviewed!("Ekstra"),
    failed: reviewed!("Feilet"),
    feed: reviewed!("Abonner"),
    fixed_price: reviewed!("Fast pris:"),
    image_descriptions: reviewed!("Bildebeskrivelser"),
    image_descriptions_guide: reviewed!("\
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
    image_descriptions_permalink: reviewed!("image-descriptions"),
    listen: untranslated!(listen),
    loading: untranslated!(loading),
    m3u_playlist: untranslated!(m3u_playlist),
    made_or_arranged_payment: reviewed!("Jeg har utført eller ordnet med betaling"),
    missing_image_description_note: reviewed!("Manglende bildebeskrivelse<br>Klikk for å lære mer"),
    more: reviewed!("Mer"),
    muted: untranslated!(muted),
    name_your_price: reviewed!("Velg din egen pris"),
    next_track: untranslated!(next_track),
    pause: untranslated!(pause),
    play: untranslated!(play),
    previous_track: untranslated!(previous_track),
    purchase_downloads: reviewed!("Kjøp nedlastinger"),
    purchase_permalink: reviewed!("kjop"),
    recommended_format: reviewed!("Anbefalt format"),
    rss_feed: reviewed!("RSS-Strøm"),
    this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
    unlisted: untranslated!(unlisted),
    unlock: reviewed!("Lås opp"),
    unlock_downloads: reviewed!("Lås opp nedlastinger"),
    unlock_permalink: reviewed!("las-opp"),
    unlock_code_seems_incorrect: reviewed!("Koden for å låse opp er ikke riktig, sjekk om du har noen skrivefeil."),
    unlock_manual_instructions: reviewed!("\
For å låse opp nedlastingen må du gjøre følgende endringer \
til addressen i nettleserens addressefelt.\
<br><br>\
Før du prøver på dette, så vær oppmerksom på at feil kode eller \
addresseendringer vil føre til en side som ikke finnes. Hvis det \
skjer, bruk tilbake-knappen og prøv å følge instruksjonene nøye igjen.\
<br><br>\
Erstatt den siste delen av addressen - /{unlock_permalink}/{page_hash}{index_suffix} - \
med /{downloads_permalink}/[your-unlock-code]{index_suffix} og trykk ENTER."),
    up_to_xxx: reviewed!("Opp til {xxx}"),
    visual_impairment: untranslated!(visual_impairment),
    volume: untranslated!(volume),
    xxx_and_others: reviewed!(r#"{xxx} og <a href="{others_link}">mer</a>"#),
    xxx_or_more: reviewed!("{xxx} eller mer")
};
