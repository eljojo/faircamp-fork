// SPDX-FileCopyrightText: 2024 Miró Allard
// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Förlustfri komprimering, om du uteslutande använder Apple-produkter, välj detta format till förmån för FLAC"),
        audio_format_average: String::from("Genomsnittlig komprimering, lämpligt om din spelare inte stödjer bättre format"),
        audio_format_flac: String::from("Förlustfri komprimering, bäst val för arkivering"),
        audio_format_mp3: String::from("Ineffektiv komprimering, lämpligt om kompatibilitet med äldre spelare avkrävs"),
        audio_format_opus_48: String::from("Utmärkt komprimering, sparsam kvalitet, bra val vid lite lagringsutrymme"),
        audio_format_opus_96: String::from("Utmärkt komprimering, standardkvalitet, bra val för lyssnande offline"),
        audio_format_opus_128: String::from("Utmärkt komprimering, högst kvalitet, bästa valet för lyssnande offline"),
        audio_format_uncompressed: String::from("Okomprimerade stora filer, lämpligt endast för ljudproduktion"),
        audio_player_widget_for_release: String::from(r#"Ljudspelare för musiksläppet "{title}""#),
        audio_player_widget_for_track: String::from(r#"Ljudspelare för spåret "{title}""#),
        auto_generated_cover: String::from("Automatiskt genererat omslag"),
        available_formats: String::from("Tillgängliga format:"),
        buy: String::from("Köp"),
        copied: String::from("Kopierad"),
        copy: String::from("Kopiera"),
        copy_link: String::from("Kopiera länk"),
        copy_link_to_track: String::from("Kopiera länk till spår"),
        confirm: String::from("Bekräfta"),
        r#continue: String::from("Fortsätt"),
        cover_image: String::from("Omslagsbild"),
        default_unlock_text: String::from("Du behöver ange en kod för att ha tillgång till nedladdningar. Fråga sidutvecklarna om hur en sådan erhålles."),
        dimmed: String::from("Nedtonad"),
        download: String::from("Ladda ned"),
        downloads: String::from("Nedladdningar"),
        downloads_permalink: String::from("nedladdningar"),
        embed: String::from("Bädda in"),
        embed_entire_release: String::from("Bädda in hela musiksläppet"),
        enter_code_here: String::from("Ange kod här"),
        external_link: String::from("Extern länk"),
        extras: String::from("Extramaterial"),
        failed: String::from("Misslyckade"),
        feed: String::from("Flöde"),
        fixed_price: String::from("Fast pris:"),
        image_descriptions: String::from("Bildbeskrivning"),
        image_descriptions_guide: String::from("\
Miljontals människor navigerar internet med hjälp av skärmläsare \
på grund av synnedsättningar. Bilder utan bildbeskrivningar är \
otillgängliga för dem. Det är därför viktigt att ange bildbeskrivningar \
så att alla kan ta åt sig av bildernas innehåll.<br><br>\
\
Konsultera README-filen för information om hur en anger \
bildbeskrivningar - det är inte svårt och bidrar till att \
skapa ett bättre internet för alla.<br><br> \
\
Här är några tips för hur en skriver bra bildbeskrivningar:<br>\
- Det är alltid bättre med någon än ingen - oroa dig inte för att göra fel.<br>\
- Håll dig kortfattad. Försök att enbart beskriva det väsentliga.<br>\
- Tolka inte bildens innehåll. Beskriv det som är relevant för att förstå bilden utan personlig analys.<br>\
- Du kan beskriva färger om så är lämpligt - många personer får synfel först senare i livet och förstår och uppskattar färg."),
        image_descriptions_permalink: String::from("bildbeskrivningar"),
        listen: String::from("Lyssna"),
        loading: String::from("Laddar"),
        made_or_arranged_payment: String::from("Jag har genomfört eller påbörjat betalningen"),
        missing_image_description_note: String::from("Bildbeskrivning saknas<br>Klicka för att lära dig mer"),
        more: String::from("Mer"),
        muted: String::from("Tyst"),
        name_your_price: String::from("Ange eget pris"),
        next_track: String::from("Nästa spår"),
        option: String::from("Val"),
        pause: String::from("Pausa"),
        pay_on_liberapay: String::from("Betala med liberapay:"),
        payment_options: String::from("Betalningsalternativ:"),
        play: String::from("Spela"),
        previous_track: untranslated!(previous_track),
        purchase_downloads: String::from("Köp nedladdningar"),
        purchase_permalink: String::from("betala"),
        recommended_format: String::from("Rekommenderat format"),
        releases: String::from("Musiksläpp"),
        rss_feed: String::from("RSS-flöde"),
        this_site_was_created_with_faircamp: String::from("Denna hemsida skapades med {faircamp_link}"),
        tracks: String::from("Spår"),
        unlisted: String::from("Olistade"),
        unlock: String::from("Lås upp"),
        unlock_downloads: String::from("Lås upp nedladdningar"),
        unlock_permalink: String::from("las-upp"),
        unlock_code_seems_incorrect: String::from("Upplåsningskoden verkar vara felaktig, var vänlig kontrollera stavfel."),
        unlock_manual_instructions: String::from("\
För att låsa upp nedladdningen, vänligen ändra på \
din webbläsares adressfält med hjälp av nedanstående instruktioner.\
<br><br>\
Var medveten om att felaktiga koder \
eller ändringar i adressfältet kommer att ta dig till en 404-sida. \
Om detta sker, använd bakåtknappen och följ noggrant instruktionerna igen.\
<br><br>\
Ersätt den sista delen av adressen - /{unlock_permalink}/{page_hash}{index_suffix} - \
med /{downloads_permalink}/[din-upplåsningskod]{index_suffix} och tryck sedan på Retur-tangenten."),
        up_to_xxx: String::from("Upp till {xxx}"),
        visual_impairment: String::from("Synnedsättning"),
        volume: untranslated!(volume),
        xxx_and_others: String::from(r#"{xxx} med <a href="{others_link}">flera</a>"#),
        xxx_or_more: String::from("{xxx} eller mer")
    }
}
