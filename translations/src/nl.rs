// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const NL: Translations = Translations {
    audio_format_alac: unreviewed!("Verliesloos en gecomprimeerd, als je uitsluitend Apple-producten gebruikt, kies dan voor FLAC"),
    audio_format_average: unreviewed!("Gemiddelde compressie, geschikt als uw speler geen betere formaten ondersteunt"),
    audio_format_flac: unreviewed!("Verliesloos en gecomprimeerd, beste keuze voor archivering"),
    audio_format_mp3: unreviewed!("Inefficiënte compressie, geschikt als compatibiliteit met oudere spelers nodig is"),
    audio_format_opus_48: unreviewed!("Uitstekende compressie, zuinige kwaliteit, goede keuze als de ruimte beperkt is"),
    audio_format_opus_96: unreviewed!("Uitstekende compressie, standaard kwaliteit, goede keuze voor offline luisteren"),
    audio_format_opus_128: unreviewed!("Uitstekende compressie, hoogste kwaliteit, beste keuze voor offline luisteren"),
    audio_format_uncompressed: unreviewed!("Ongecomprimeerde grote bestanden, alleen geschikt voor audioproductie"),
    audio_player_widget_for_xxx: unreviewed!(r#"Audiospelerwidget voor "{title}""#),
    auto_generated_cover: unreviewed!("Automatisch gegenereerde omslagafbeelding"),
    available_formats: unreviewed!("Beschikbare formaten:"),
    buy: untranslated!(buy),
    close: unreviewed!("Sluiten"),
    copied: unreviewed!("Gekopieerd"),
    copy: unreviewed!("Kopiëren"),
    copy_link: untranslated!(copy_link),
    confirm: unreviewed!("Bevestigen"),
    r#continue: unreviewed!("Voortzetten"),
    cover_image: unreviewed!("Omslagafbeelding"),
    default_unlock_text: unreviewed!("Jij moet een code invullen om deze downloads te ontsluiten. Vraag de websitebeheerder hoe je er een kan verkrijgen."),
    dimmed: untranslated!(dimmed),
    download: untranslated!(download),
    downloads: unreviewed!("Downloads"),
    downloads_permalink: unreviewed!("downloads"),
    embed: unreviewed!("Inzetten"),
    embed_entire_release: unreviewed!("De heele release inzetten"),
    enter_code_here: unreviewed!("Vul hier het code in"),
    external_link: untranslated!(external_link),
    extras: unreviewed!("Extra's"),
    failed: unreviewed!("Mislukt"),
    feed: unreviewed!("Feed"),
    fixed_price: unreviewed!("Vaste prijs:"),
    image_descriptions: unreviewed!("Afbeeldingsomschrijving"),
    image_descriptions_guide: unreviewed!("\
Miljoenen mensen surfen op internet met behulp van schermlezers \
omdat ze niet (of niet goed genoeg) kunnen zien. Afbeeldingen \
zonder tekstuele beschrijvingen zijn voor hen ontoegankelijk, \
en daarom moeten we de moeite nemen om er beeldbeschrijvingen \
voor te bieden.<br><br>\
\
Raadpleeg de Faircamp README voor het toevoegen van \
afbeeldingsbeschrijvingen, het is eenvoudig en een \
vriendelijke daad.<br><br>\
\
Hier zijn een paar tips voor het schrijven van goede afbeeldingsomschrijvingen:<br>\
- Elke beschrijving is beter dan geen beschrijving, maak je geen zorgen dat je het verkeerd doet.<br>\
- Maak het beknopt. Schrijf zoveel als nodig is, maar houd het tegelijkertijd zo kort mogelijk.<br>\
- Niet interpreteren. Beschrijf wat er is en relevant voor het begrip ervan, analyseer niet verder dan dat.<br>\
- Je kunt kleuren gebruiken waar dat zinvol is; veel mensen zijn pas later hun gezichtsvermogen kwijtgeraakt en begrijpen en waarderen kleuren."),
    image_descriptions_permalink: unreviewed!("afbeeldingsomschrijvingen"),
    listen: untranslated!(listen),
    loading: untranslated!(loading),
    m3u_playlist: untranslated!(m3u_playlist),
    made_or_arranged_payment: unreviewed!("Ik heb de betaling gedaan of geregeld"),
    missing_image_description_note: unreviewed!("Ontbrekende afbeeldingsomschrijving<br>Klik om meer te leren"),
    more: unreviewed!("Meer"),
    mute: untranslated!(mute),
    muted: untranslated!(muted),
    name_your_price: unreviewed!("Noem je prijs"),
    next_track: untranslated!(next_track),
    pause: untranslated!(pause),
    play: untranslated!(play),
    playback_position: untranslated!(playback_position),
    player_closed: untranslated!(player_closed),
    player_open_playing_xxx: untranslated!(player_open_playing_xxx),
    previous_track: untranslated!(previous_track),
    purchase_downloads: unreviewed!("Downloads kopen"),
    purchase_permalink: unreviewed!("kopen"),
    recommended_format: unreviewed!("Aanbevolen Formaat"),
    rss_feed: unreviewed!("RSS Feed"),
    search: untranslated!(search),
    this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
    unlisted: untranslated!(unlisted),
    unlock: unreviewed!("Ontsluiten"),
    unlock_downloads: unreviewed!("Downloads ontsluiten"),
    unlock_permalink: unreviewed!("ontsluiten"),
    unlock_code_seems_incorrect: unreviewed!("De ontgrendelingscode lijkt onjuist te zijn. Controleer op typefouten."),
    unlock_manual_instructions: unreviewed!("\
Om de download te ontgrendelen, breng je de hieronder beschreven \
wijzigingen aan in het adres in de adresbalk van jouw browser.\
<br><br>\
Voordat je dat doet houd er rekening mee dat verkeerde codes \
of adreswijzigingen u naar een 404-pagina leiden. In het geval \
gebruik de terugknop en volg de voorschriften nauwkeurig opnieuw.\
<br><br>\
Vervang het laatste deel van het adres - /{unlock_permalink}/{page_hash}{index_suffix} - \
met /{downloads_permalink}/[your-unlock-code]{index_suffix} en druk vervolgens op Enter."),
    unmute: untranslated!(unmute),
    up_to_xxx: unreviewed!("Tot {xxx}"),
    visual_impairment: untranslated!(visual_impairment),
    volume: untranslated!(volume),
    xxx_and_others: unreviewed!(r#"{xxx} en <a href="{others_link}">meer</a>"#),
    xxx_minutes: untranslated!(xxx_minutes),
    xxx_or_more: unreviewed!("{xxx} of meer")
};
