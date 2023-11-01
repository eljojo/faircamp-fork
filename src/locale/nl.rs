use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_average: String::from("Gemiddelde compressie, geschikt als uw speler geen betere formaten ondersteunt"),
        audio_format_flac: String::from("Verliesloos en gecomprimeerd, beste keuze voor archivering"),
        audio_format_mp3: String::from("Inefficiënte compressie, geschikt als compatibiliteit met oudere spelers nodig is"),
        audio_format_opus_48: String::from("Uitstekende compressie, zuinige kwaliteit, goede keuze als de ruimte beperkt is"),
        audio_format_opus_96: String::from("Uitstekende compressie, standaard kwaliteit, goede keuze voor offline luisteren"),
        audio_format_opus_128: String::from("Uitstekende compressie, hoogste kwaliteit, beste keuze voor offline luisteren"),
        audio_format_uncompressed: String::from("Ongecomprimeerde grote bestanden, alleen geschikt voor audioproductie"),
        audio_player_widget_for_release: String::from(r#"Audiospelerwidget voor de release "{title}""#),
        audio_player_widget_for_track: String::from(r#"Audiospelerwidget voor de track "{title}""#),
        auto_generated_cover: String::from("Automatisch gegenereerde omslagafbeelding"),
        available_formats: String::from("Beschikbare formaten:"),
        close: String::from("Sluiten"),
        copied: String::from("Gekopieerd"),
        copy: String::from("Kopiëren"),
        confirm: String::from("Bevestigen"),
        r#continue: String::from("Voortzetten"),
        cover_image: String::from("Omslagafbeelding"),
        default_unlock_text: String::from("Jij moet een code invullen om deze downloads te ontsluiten. Vraag de websitebeheerder hoe je er een kan verkrijgen."),
        downloads: String::from("Downloads"),
        downloads_permalink: String::from("downloads"),
        embed: String::from("Inzetten"),
        embed_entire_release: String::from("De heele release inzetten"),
        enter_code_here: String::from("Vul hier het code in"),
        extras: String::from("Extra's"),
        failed: String::from("Mislukt"),
        feed: String::from("Feed"),
        fixed_price: String::from("Vaste prijs:"),
        image_descriptions: String::from("Afbeeldingsomschrijving"),
        image_descriptions_guide: String::from("\
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
        image_descriptions_permalink: String::from("afbeeldingsomschrijvingen"),
        made_or_arranged_payment: String::from("Ik heb de betaling gedaan of geregeld"),
        missing_image_description_note: String::from("Ontbrekende afbeeldingsomschrijving<br>Klik om meer te leren"),
        name_your_price: String::from("Noem je prijs"),
        option: String::from("Optie"),
        pay_on_liberapay: String::from("Op liberapay betaalen:"),
        payment_options: String::from("Betalingsmogelijkheden:"),
        purchase_downloads: String::from("Downloads kopen"),
        purchase_permalink: String::from("kopen"),
        recommended_format: String::from("Aanbevolen Formaat"),
        rss_feed: String::from("RSS Feed"),
        share: String::from("Deelen"),
        share_not_available_navigator_clipboard: String::from("Niet beschikbaar in je browser (navigator.clipboard is niet ondersteund)"),
        share_not_available_requires_javascript: String::from("Niet beschikbaar in je browser (vereist JavaScript)"),
        unlock: String::from("Ontsluiten"),
        unlock_downloads: String::from("Downloads ontsluiten"),
        unlock_permalink: String::from("ontsluiten"),
        unlock_code_seems_incorrect: String::from("De ontgrendelingscode lijkt onjuist te zijn. Controleer op typefouten."),
        unlock_manual_instructions: String::from("\
Om de download te ontgrendelen, breng je de hieronder beschreven \
wijzigingen aan in het adres in de adresbalk van jouw browser.\
<br><br>\
Voordat je dat doet houd er rekening mee dat verkeerde codes \
of adreswijzigingen u naar een 404-pagina leiden. In het geval \
gebruik de terugknop en volg de voorschriften nauwkeurig opnieuw.\
<br><br>\
Vervang het laatste deel van het adres - /ontsluiten/{page_hash}{index_suffix} - \
met /downloads/[your-unlock-code]{index_suffix} en druk vervolgens op Enter."),
        up_to_xxx: String::from("Tot {xxx}"),
        xxx_or_more: String::from("{xxx} of meer")
    }
}