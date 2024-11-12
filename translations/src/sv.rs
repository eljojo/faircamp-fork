// SPDX-FileCopyrightText: 2024 Miró Allard
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const SV: Translations = Translations {
    audio_format_alac: reviewed!("Förlustfri komprimering, om du uteslutande använder Apple-produkter, välj detta format till förmån för FLAC"),
    audio_format_average: reviewed!("Genomsnittlig komprimering, lämpligt om din spelare inte stödjer bättre format"),
    audio_format_flac: reviewed!("Förlustfri komprimering, bäst val för arkivering"),
    audio_format_mp3: reviewed!("Ineffektiv komprimering, lämpligt om kompatibilitet med äldre spelare avkrävs"),
    audio_format_opus_48: reviewed!("Utmärkt komprimering, sparsam kvalitet, bra val vid lite lagringsutrymme"),
    audio_format_opus_96: reviewed!("Utmärkt komprimering, standardkvalitet, bra val för lyssnande offline"),
    audio_format_opus_128: reviewed!("Utmärkt komprimering, högst kvalitet, bästa valet för lyssnande offline"),
    audio_format_uncompressed: reviewed!("Okomprimerade stora filer, lämpligt endast för ljudproduktion"),
    audio_player_widget_for_xxx: unreviewed!(r#"Ljudspelare för "{title}""#),
    auto_generated_cover: reviewed!("Automatiskt genererat omslag"),
    available_formats: reviewed!("Tillgängliga format:"),
    browse: untranslated!(browse),
    buy: reviewed!("Köp"),
    close: untranslated!(close),
    copied: reviewed!("Kopierad"),
    copy: reviewed!("Kopiera"),
    copy_link: reviewed!("Kopiera länk"),
    confirm: reviewed!("Bekräfta"),
    r#continue: reviewed!("Fortsätt"),
    cover_image: reviewed!("Omslagsbild"),
    default_unlock_text: reviewed!("Du behöver ange en kod för att ha tillgång till nedladdningar. Fråga sidutvecklarna om hur en sådan erhålles."),
    dimmed: reviewed!("Nedtonad"),
    download: reviewed!("Ladda ned"),
    downloads: reviewed!("Nedladdningar"),
    downloads_permalink: reviewed!("nedladdningar"),
    embed: reviewed!("Bädda in"),
    embed_entire_release: reviewed!("Bädda in hela musiksläppet"),
    enter_code_here: reviewed!("Ange kod här"),
    external_link: reviewed!("Extern länk"),
    extras: reviewed!("Extramaterial"),
    failed: reviewed!("Misslyckade"),
    feed: reviewed!("Flöde"),
    fixed_price: reviewed!("Fast pris:"),
    image_descriptions: reviewed!("Bildbeskrivning"),
    image_descriptions_guide: reviewed!("\
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
    image_descriptions_permalink: reviewed!("bildbeskrivningar"),
    listen: reviewed!("Lyssna"),
    loading: reviewed!("Laddar"),
    m3u_playlist: untranslated!(m3u_playlist),
    made_or_arranged_payment: reviewed!("Jag har genomfört eller påbörjat betalningen"),
    missing_image_description_note: reviewed!("Bildbeskrivning saknas<br>Klicka för att lära dig mer"),
    more: reviewed!("Mer"),
    mute: untranslated!(mute),
    muted: reviewed!("Tyst"),
    name_your_price: reviewed!("Ange eget pris"),
    next_track: reviewed!("Nästa spår"),
    nothing_found_for_xxx: untranslated!(nothing_found_for_xxx),
    pause: reviewed!("Pausa"),
    play: reviewed!("Spela"),
    playback_position: untranslated!(playback_position),
    player_closed: untranslated!(player_closed),
    player_open_playing_xxx: untranslated!(player_open_playing_xxx),
    previous_track: untranslated!(previous_track),
    purchase_downloads: reviewed!("Köp nedladdningar"),
    purchase_permalink: reviewed!("betala"),
    recommended_format: reviewed!("Rekommenderat format"),
    rss_feed: reviewed!("RSS-flöde"),
    search: untranslated!(search),
    showing_featured_items: untranslated!(showing_featured_items),
    showing_xxx_results_for_xxx: untranslated!(showing_xxx_results_for_xxx),
    this_site_was_created_with_faircamp: reviewed!("Denna hemsida skapades med {faircamp_link}"),
    unlisted: reviewed!("Olistade"),
    unlock: reviewed!("Lås upp"),
    unlock_downloads: reviewed!("Lås upp nedladdningar"),
    unlock_permalink: reviewed!("las-upp"),
    unlock_code_seems_incorrect: reviewed!("Upplåsningskoden verkar vara felaktig, var vänlig kontrollera stavfel."),
    unlock_manual_instructions: reviewed!("\
För att låsa upp nedladdningen, vänligen ändra på \
din webbläsares adressfält med hjälp av nedanstående instruktioner.\
<br><br>\
Var medveten om att felaktiga koder \
eller ändringar i adressfältet kommer att ta dig till en 404-sida. \
Om detta sker, använd bakåtknappen och följ noggrant instruktionerna igen.\
<br><br>\
Ersätt den sista delen av adressen - /{unlock_permalink}/{page_hash}{index_suffix} - \
med /{downloads_permalink}/[din-upplåsningskod]{index_suffix} och tryck sedan på Retur-tangenten."),
    unmute: untranslated!(unmute),
    up_to_xxx: reviewed!("Upp till {xxx}"),
    visual_impairment: reviewed!("Synnedsättning"),
    volume: untranslated!(volume),
    xxx_and_others: reviewed!(r#"{xxx} med <a href="{others_link}">flera</a>"#),
    xxx_minutes: untranslated!(xxx_minutes),
    xxx_or_more: reviewed!("{xxx} eller mer")
};
