// SPDX-FileCopyrightText: 2023 Harald Eilertsen
// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Tapsfritt og komprimert, velg dette over FLAC dersom du kun bruker Apple produkter"),
        audio_format_average: String::from("Middels komprimering, passende dersom avspilleren din ikke støtter bedre formater"),
        audio_format_flac: String::from("Tapsfritt og komprimert, det beste valget for arkivering"),
        audio_format_mp3: String::from("Lite effektiv komprimering, passende dersom kompatibilitet med eldre avspillere er nødvendig"),
        audio_format_opus_48: String::from("Utmerket kompresjon, begrenset kvalitet, et godt valg dersom det er begrenset lagringsplass"),
        audio_format_opus_96: String::from("Utmerket kompresjon, standard kvalitet, et godt valg for å lytte frakoblet"),
        audio_format_opus_128: String::from("Utmerket kompresjon, høyeste kvalitet, det beste valget for å lytte frakoblet"),
        audio_format_uncompressed: String::from("Ukomprimerte store filer, kun passende for lydproduksjon"),
        audio_player_widget_for_release: String::from(r#"Lydavspillerelement for utgivelsen "{title}""#),
        audio_player_widget_for_track: String::from(r#"Lydavspillerelement for sporet "{title}""#),
        auto_generated_cover: String::from("Automatisk generert omslag"),
        available_formats: String::from("Tilgjengelige formater:"),
        buy: untranslated!(buy),
        copied: String::from("Kopiert"),
        copy: String::from("Kopier"),
        copy_link: untranslated!(copy_link),
        copy_link_to_track: untranslated!(copy_link_to_track),
        confirm: String::from("Bekreft"),
        r#continue: String::from("Fortsett"),
        cover_image: String::from("Omslagsbilde"),
        default_unlock_text: String::from("Du må skrive inn en kode for å få lastet ned disse filene. Spør sidens administratorer for hvordan du kan få en."),
        dimmed: untranslated!(dimmed),
        download: untranslated!(download),
        downloads: String::from("Nedlastinger"),
        downloads_permalink: String::from("nedlastinger"),
        embed: String::from("Bygg inn"),
        embed_entire_release: String::from("Bygg inn hele utgivelsen"),
        enter_code_here: String::from("Skriv inn koden her"),
        external_link: untranslated!(external_link),
        extras: String::from("Ekstra"),
        failed: String::from("Feilet"),
        feed: String::from("Abonner"),
        fixed_price: String::from("Fast pris:"),
        image_descriptions: String::from("Bildebeskrivelser"),
        image_descriptions_guide: String::from("\
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
        image_descriptions_permalink: String::from("image-descriptions"),
        listen: untranslated!(listen),
        loading: untranslated!(loading),
        made_or_arranged_payment: String::from("Jeg har utført eller ordnet med betaling"),
        missing_image_description_note: String::from("Manglende bildebeskrivelse<br>Klikk for å lære mer"),
        more: String::from("Mer"),
        more_info: untranslated!(more_info),
        muted: untranslated!(muted),
        name_your_price: String::from("Velg din egen pris"),
        next_track: untranslated!(next_track),
        option: String::from("Valg"),
        pause: untranslated!(pause),
        pay_on_liberapay: String::from("Betal via Liberapay"),
        payment_options: String::from("Payment options:"),
        play: untranslated!(play),
        previous_track: untranslated!(previous_track),
        purchase_downloads: String::from("Kjøp nedlastinger"),
        purchase_permalink: String::from("kjop"),
        recommended_format: String::from("Anbefalt format"),
        releases: untranslated!(releases),
        rss_feed: String::from("RSS-Strøm"),
        this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
        top: untranslated!(top),
        tracks: untranslated!(tracks),
        unlisted: untranslated!(unlisted),
        unlock: String::from("Lås opp"),
        unlock_downloads: String::from("Lås opp nedlastinger"),
        unlock_permalink: String::from("las-opp"),
        unlock_code_seems_incorrect: String::from("Koden for å låse opp er ikke riktig, sjekk om du har noen skrivefeil."),
        unlock_manual_instructions: String::from("\
For å låse opp nedlastingen må du gjøre følgende endringer \
til addressen i nettleserens addressefelt.\
<br><br>\
Før du prøver på dette, så vær oppmerksom på at feil kode eller \
addresseendringer vil føre til en side som ikke finnes. Hvis det \
skjer, bruk tilbake-knappen og prøv å følge instruksjonene nøye igjen.\
<br><br>\
Erstatt den siste delen av addressen - /{unlock_permalink}/{page_hash}{index_suffix} - \
med /{downloads_permalink}/[your-unlock-code]{index_suffix} og trykk ENTER."),
        up_to_xxx: String::from("Opp til {xxx}"),
        visual_impairment: untranslated!(visual_impairment),
        xxx_or_more: String::from("{xxx} eller mer")
    }
}
