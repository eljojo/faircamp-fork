// SPDX-FileCopyrightText: 2024 Miró Allard
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const SV: Translations = Translations {
    audio_format_alac: Reviewed("Förlustfri komprimering, om du uteslutande använder Apple-produkter, välj detta format till förmån för FLAC"),
    audio_format_average: Reviewed("Genomsnittlig komprimering, lämpligt om din spelare inte stödjer bättre format"),
    audio_format_flac: Reviewed("Förlustfri komprimering, bäst val för arkivering"),
    audio_format_mp3: Reviewed("Ineffektiv komprimering, lämpligt om kompatibilitet med äldre spelare avkrävs"),
    audio_format_opus_48: Reviewed("Utmärkt komprimering, sparsam kvalitet, bra val vid lite lagringsutrymme"),
    audio_format_opus_96: Reviewed("Utmärkt komprimering, standardkvalitet, bra val för lyssnande offline"),
    audio_format_opus_128: Reviewed("Utmärkt komprimering, högst kvalitet, bästa valet för lyssnande offline"),
    audio_format_uncompressed: Reviewed("Okomprimerade stora filer, lämpligt endast för ljudproduktion"),
    audio_player_widget_for_xxx: Unreviewed(r#"Ljudspelare för "{title}""#),
    auto_generated_cover: Reviewed("Automatiskt genererat omslag"),
    available_formats: Reviewed("Tillgängliga format:"),
    buy: Reviewed("Köp"),
    copied: Reviewed("Kopierad"),
    copy: Reviewed("Kopiera"),
    copy_link: Reviewed("Kopiera länk"),
    confirm: Reviewed("Bekräfta"),
    r#continue: Reviewed("Fortsätt"),
    cover_image: Reviewed("Omslagsbild"),
    default_unlock_text: Reviewed("Du behöver ange en kod för att ha tillgång till nedladdningar. Fråga sidutvecklarna om hur en sådan erhålles."),
    download: Reviewed("Ladda ned"),
    downloads: Reviewed("Nedladdningar"),
    downloads_permalink: Reviewed("nedladdningar"),
    embed: Reviewed("Bädda in"),
    embed_entire_release: Reviewed("Bädda in hela musiksläppet"),
    enter_code_here: Reviewed("Ange kod här"),
    external_link: Reviewed("Extern länk"),
    extras: Reviewed("Extramaterial"),
    failed: Reviewed("Misslyckade"),
    feed: Reviewed("Flöde"),
    fixed_price: Reviewed("Fast pris:"),
    image_descriptions: Reviewed("Bildbeskrivning"),
    image_descriptions_guide: Reviewed("\
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
    image_descriptions_permalink: Reviewed("bildbeskrivningar"),
    listen: Reviewed("Lyssna"),
    loading: Reviewed("Laddar"),
    made_or_arranged_payment: Reviewed("Jag har genomfört eller påbörjat betalningen"),
    missing_image_description_note: Reviewed("Bildbeskrivning saknas<br>Klicka för att lära dig mer"),
    more: Reviewed("Mer"),
    name_your_price: Reviewed("Ange eget pris"),
    next_track: Reviewed("Nästa spår"),
    pause: Reviewed("Pausa"),
    play: Reviewed("Spela"),
    purchase_downloads: Reviewed("Köp nedladdningar"),
    purchase_permalink: Reviewed("betala"),
    recommended_format: Reviewed("Rekommenderat format"),
    rss_feed: Reviewed("RSS-flöde"),
    this_site_was_created_with_faircamp: Reviewed("Denna hemsida skapades med {faircamp_link}"),
    unlisted: Reviewed("Olistade"),
    unlock: Reviewed("Lås upp"),
    unlock_downloads: Reviewed("Lås upp nedladdningar"),
    unlock_permalink: Reviewed("las-upp"),
    unlock_code_seems_incorrect: Reviewed("Upplåsningskoden verkar vara felaktig, var vänlig kontrollera stavfel."),
    unlock_manual_instructions: Reviewed("\
För att låsa upp nedladdningen, vänligen ändra på \
din webbläsares adressfält med hjälp av nedanstående instruktioner.\
<br><br>\
Var medveten om att felaktiga koder \
eller ändringar i adressfältet kommer att ta dig till en 404-sida. \
Om detta sker, använd bakåtknappen och följ noggrant instruktionerna igen.\
<br><br>\
Ersätt den sista delen av adressen - /{unlock_permalink}/{page_hash}{index_suffix} - \
med /{downloads_permalink}/[din-upplåsningskod]{index_suffix} och tryck sedan på Retur-tangenten."),
    up_to_xxx: Reviewed("Upp till {xxx}"),
    visual_impairment: Reviewed("Synnedsättning"),
    xxx_and_others: Reviewed(r#"{xxx} med <a href="{others_link}">flera</a>"#),
    xxx_or_more: Reviewed("{xxx} eller mer"),
    ..Translations::UNTRANSLATED
};
