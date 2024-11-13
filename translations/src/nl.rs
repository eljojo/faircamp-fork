// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Translations, Unreviewed};

pub const NL: Translations = Translations {
    audio_format_alac: Unreviewed("Verliesloos en gecomprimeerd, als je uitsluitend Apple-producten gebruikt, kies dan voor FLAC"),
    audio_format_average: Unreviewed("Gemiddelde compressie, geschikt als uw speler geen betere formaten ondersteunt"),
    audio_format_flac: Unreviewed("Verliesloos en gecomprimeerd, beste keuze voor archivering"),
    audio_format_mp3: Unreviewed("Inefficiënte compressie, geschikt als compatibiliteit met oudere spelers nodig is"),
    audio_format_opus_48: Unreviewed("Uitstekende compressie, zuinige kwaliteit, goede keuze als de ruimte beperkt is"),
    audio_format_opus_96: Unreviewed("Uitstekende compressie, standaard kwaliteit, goede keuze voor offline luisteren"),
    audio_format_opus_128: Unreviewed("Uitstekende compressie, hoogste kwaliteit, beste keuze voor offline luisteren"),
    audio_format_uncompressed: Unreviewed("Ongecomprimeerde grote bestanden, alleen geschikt voor audioproductie"),
    audio_player_widget_for_xxx: Unreviewed(r#"Audiospelerwidget voor "{title}""#),
    auto_generated_cover: Unreviewed("Automatisch gegenereerde omslagafbeelding"),
    available_formats: Unreviewed("Beschikbare formaten:"),
    close: Unreviewed("Sluiten"),
    copied: Unreviewed("Gekopieerd"),
    copy: Unreviewed("Kopiëren"),
    confirm: Unreviewed("Bevestigen"),
    r#continue: Unreviewed("Voortzetten"),
    cover_image: Unreviewed("Omslagafbeelding"),
    default_unlock_text: Unreviewed("Jij moet een code invullen om deze downloads te ontsluiten. Vraag de websitebeheerder hoe je er een kan verkrijgen."),
    downloads: Unreviewed("Downloads"),
    downloads_permalink: Unreviewed("downloads"),
    embed: Unreviewed("Inzetten"),
    embed_entire_release: Unreviewed("De heele release inzetten"),
    enter_code_here: Unreviewed("Vul hier het code in"),
    extras: Unreviewed("Extra's"),
    failed: Unreviewed("Mislukt"),
    feed: Unreviewed("Feed"),
    fixed_price: Unreviewed("Vaste prijs:"),
    image_descriptions: Unreviewed("Afbeeldingsomschrijving"),
    image_descriptions_guide: Unreviewed("\
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
    image_descriptions_permalink: Unreviewed("afbeeldingsomschrijvingen"),
    made_or_arranged_payment: Unreviewed("Ik heb de betaling gedaan of geregeld"),
    missing_image_description_note: Unreviewed("Ontbrekende afbeeldingsomschrijving<br>Klik om meer te leren"),
    more: Unreviewed("Meer"),
    name_your_price: Unreviewed("Noem je prijs"),
    purchase_downloads: Unreviewed("Downloads kopen"),
    purchase_permalink: Unreviewed("kopen"),
    recommended_format: Unreviewed("Aanbevolen Formaat"),
    rss_feed: Unreviewed("RSS Feed"),
    unlock: Unreviewed("Ontsluiten"),
    unlock_downloads: Unreviewed("Downloads ontsluiten"),
    unlock_permalink: Unreviewed("ontsluiten"),
    unlock_code_seems_incorrect: Unreviewed("De ontgrendelingscode lijkt onjuist te zijn. Controleer op typefouten."),
    unlock_manual_instructions: Unreviewed("\
Om de download te ontgrendelen, breng je de hieronder beschreven \
wijzigingen aan in het adres in de adresbalk van jouw browser.\
<br><br>\
Voordat je dat doet houd er rekening mee dat verkeerde codes \
of adreswijzigingen u naar een 404-pagina leiden. In het geval \
gebruik de terugknop en volg de voorschriften nauwkeurig opnieuw.\
<br><br>\
Vervang het laatste deel van het adres - /{unlock_permalink}/{page_hash}{index_suffix} - \
met /{downloads_permalink}/[your-unlock-code]{index_suffix} en druk vervolgens op Enter."),
    up_to_xxx: Unreviewed("Tot {xxx}"),
    xxx_and_others: Unreviewed(r#"{xxx} en <a href="{others_link}">meer</a>"#),
    xxx_or_more: Unreviewed("{xxx} of meer"),
    ..Translations::UNTRANSLATED
};
