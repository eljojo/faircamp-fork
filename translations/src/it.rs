// SPDX-FileCopyrightText: 2024 Tommaso Croce
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const IT: Translations = Translations {
    audio_format_alac: reviewed!("Lossless e compresso, se utilizzi esclusivamente prodotti Apple scegli questo invece di FLAC"),
    audio_format_average: reviewed!("Compressione media, appropriata se il tuo lettore non supporta formati migliori"),
    audio_format_flac: reviewed!("Lossless e compresso, scelta migliore per semplici fini di archiviazione"),
    audio_format_mp3: reviewed!("Compressione inefficiente, appropriata se è necessaria compatibilità con player più vecchi"),
    audio_format_opus_48: reviewed!("Compressione eccellente, qualità ridotta, buona scelta se lo spazio è limitato"),
    audio_format_opus_96: reviewed!("Compressione eccellente, qualità standard, buona scelta per ascolti offline"),
    audio_format_opus_128: reviewed!("Compressione eccellente, massima qualità, migliore scelta per ascolti offline"),
    audio_format_uncompressed: reviewed!("File grandi non compressi, appropriati solo per la produzione audio"),
    audio_player_widget_for_xxx: reviewed!(r#"Widget del lettore audio per "{title}""#),
    auto_generated_cover: reviewed!("Copertina generata automaticamente"),
    available_formats: reviewed!("Formati disponibili:"),
    buy: reviewed!("Acquista"),
    copied: reviewed!("Copiato"),
    copy: reviewed!("Copia"),
    copy_link: reviewed!("Copia link"),
    confirm: reviewed!("Conferma"),
    r#continue: reviewed!("Continua"),
    cover_image: reviewed!("Immagine di copertina"),
    default_unlock_text: reviewed!("Devi inserire un codice per sbloccare questi download. Chiedi ai gestori del sito come ottenerne uno."),
    dimmed: reviewed!("Abbassato"),
    download: reviewed!("Scarica"),
    downloads: reviewed!("Download"),
    downloads_permalink: reviewed!("downloads"),
    embed: reviewed!("Incorpora"),
    embed_entire_release: reviewed!("Incorpora l'intera uscita"),
    enter_code_here: reviewed!("Inserisci il codice qui"),
    external_link: reviewed!("Link esterno"),
    extras: reviewed!("Extra"),
    failed: reviewed!("Fallito"),
    feed: reviewed!("Feed"),
    fixed_price: reviewed!("Prezzo fisso:"),
    image_descriptions: reviewed!("Descrizioni delle immagini"),
    image_descriptions_guide: reviewed!("\
Milioni di persone navigano in rete utilizzando lettori di schermo \
perché sono non-vedenti (o ipovedenti). Tutte le immagini \
senza descrizioni testuali sono inaccessibili a loro, \
ed è per questo che dovremmo sforzarci di fornire sempre \
descrizioni per le immagini che pubblichiamo.<br><br>\
\
Consulta il README di Faircamp per sapere come aggiungere descrizioni alle immagini, \
è facile ed è un gesto di gentilezza.<br><br>\
\
Ecco alcuni consigli per scrivere buone descrizioni per le immagini:<br>\
- Una qualsiasi descrizione è meglio di nessuna descrizione, non preoccuparti di sbagliare.<br>\
- Sii conciso. Scrivi quanto necessario, ma allo stesso tempo mantieni il testo il più breve possibile.<br>\
- Non dare un'interpretazione. Descrivi semplicemente ciò che è presente e rilevante per la sua comprensione, non analizzare oltre.<br>\
- Puoi usare i colori dove ha senso - molte persone hanno perso la vista più tardi nella vita e comprendono e apprezzano i colori."),
    image_descriptions_permalink: reviewed!("descrizioni-immagini"),
    listen: reviewed!("Ascolta"),
    loading: reviewed!("Caricamento"),
    m3u_playlist: reviewed!("Elenco di riproduzione M3U"),
    made_or_arranged_payment: reviewed!("Ho effettuato o predisposto il pagamento"),
    missing_image_description_note: reviewed!("Descrizione immagine mancante<br>Clicca per saperne di più"),
    more: reviewed!("Scopri di più"),
    muted: reviewed!("Disattivato"),
    name_your_price: reviewed!("Indica un prezzo a tua scelta"),
    next_track: reviewed!("Brano successivo"),
    pause: reviewed!("Pausa"),
    play: reviewed!("Riproduci"),
    previous_track: reviewed!("Traccia Precedente"),
    purchase_downloads: reviewed!("Acquista download"),
    purchase_permalink: reviewed!("acquista"),
    recommended_format: reviewed!("Formato consigliato"),
    rss_feed: reviewed!("Feed RSS"),
    this_site_was_created_with_faircamp: reviewed!("Questo sito è stato creato con {faircamp_link}"),
    unlisted: reviewed!("Non elencato"),
    unlock: reviewed!("Sblocca"),
    unlock_downloads: reviewed!("Sblocca download"),
    unlock_permalink: reviewed!("sblocca"),
    unlock_code_seems_incorrect: reviewed!("Il codice di sblocco sembra essere errato, controlla eventuali errori di battitura."),
    unlock_manual_instructions: reviewed!("\
Per sbloccare il download, apporta le modifiche descritte di seguito \
all'indirizzo nella barra degli indirizzi del tuo browser.\
<br><br>\
Prima di procedere, sii consapevole che codici o \
modifiche errate all'indirizzo ti porteranno a una pagina 404. In questo caso \
usa il pulsante Indietro e segui nuovamente attentamente le istruzioni.\
<br><br>\
Sostituisci la parte finale dell'indirizzo - /{unlock_permalink}/{page_hash}{index_suffix} - \
con /{downloads_permalink}/[il-tuo-codice-di-sblocco]{index_suffix} e poi premi Invio."),
    up_to_xxx: reviewed!("Fino a {xxx}"),
    visual_impairment: reviewed!("Disabilità visiva"),
    volume: reviewed!("Volume"),
    xxx_and_others: reviewed!(r#"{xxx} e <a href="{others_link}">altri</a>"#),
    xxx_or_more: reviewed!("{xxx} o più")
};
