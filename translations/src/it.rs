// SPDX-FileCopyrightText: 2024 Tommaso Croce
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations};

pub const IT: Translations = Translations {
    audio_format_alac: Reviewed("Lossless e compresso, se utilizzi esclusivamente prodotti Apple scegli questo invece di FLAC"),
    audio_format_average: Reviewed("Compressione media, appropriata se il tuo lettore non supporta formati migliori"),
    audio_format_flac: Reviewed("Lossless e compresso, scelta migliore per semplici fini di archiviazione"),
    audio_format_mp3: Reviewed("Compressione inefficiente, appropriata se è necessaria compatibilità con player più vecchi"),
    audio_format_opus_48: Reviewed("Compressione eccellente, qualità ridotta, buona scelta se lo spazio è limitato"),
    audio_format_opus_96: Reviewed("Compressione eccellente, qualità standard, buona scelta per ascolti offline"),
    audio_format_opus_128: Reviewed("Compressione eccellente, massima qualità, migliore scelta per ascolti offline"),
    audio_format_uncompressed: Reviewed("File grandi non compressi, appropriati solo per la produzione audio"),
    audio_player_widget_for_xxx: Reviewed(r#"Widget del lettore audio per "{title}""#),
    auto_generated_cover: Reviewed("Copertina generata automaticamente"),
    available_formats: Reviewed("Formati disponibili:"),
    browse: Reviewed("Esplora"),
    buy: Reviewed("Acquista"),
    close: Reviewed("Chiudi"),
    copied: Reviewed("Copiato"),
    copy: Reviewed("Copia"),
    copy_link: Reviewed("Copia link"),
    confirm: Reviewed("Conferma"),
    r#continue: Reviewed("Continua"),
    cover_image: Reviewed("Immagine di copertina"),
    default_unlock_info: Reviewed("Devi inserire un codice per sbloccare questi download. Chiedi ai gestori del sito come ottenerne uno."),
    download: Reviewed("Scarica"),
    download_code_seems_incorrect: Reviewed("Il codice di sblocco sembra essere errato, controlla eventuali errori di battitura."),
    downloads: Reviewed("Download"),
    downloads_permalink: Reviewed("downloads"),
    embed: Reviewed("Incorpora"),
    embed_entire_release: Reviewed("Incorpora l'intera uscita"),
    enter_code_here: Reviewed("Inserisci il codice qui"),
    external_link: Reviewed("Link esterno"),
    extras: Reviewed("Extra"),
    failed: Reviewed("Fallito"),
    feed: Reviewed("Feed"),
    fixed_price: Reviewed("Prezzo fisso:"),
    image_descriptions: Reviewed("Descrizioni delle immagini"),
    image_descriptions_guide: Reviewed("\
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
    image_descriptions_permalink: Reviewed("descrizioni-immagini"),
    javascript_is_disabled_listen_at_xxx: Reviewed("JavaScript è disabilitato – Ascolta su {link}"),
    javascript_is_disabled_text: Reviewed("JavaScript è disabilitato – Alcune funzionalità non sono disponibili."),
    listen: Reviewed("Ascolta"),
    loading: Reviewed("Caricamento"),
    m3u_playlist: Reviewed("Elenco di riproduzione M3U"),
    made_or_arranged_payment: Reviewed("Ho effettuato o predisposto il pagamento"),
    missing_image_description_note: Reviewed("Descrizione immagine mancante<br>Clicca per saperne di più"),
    more: Reviewed("Scopri di più"),
    mute: Reviewed("Disattiva audio"),
    name_your_price: Reviewed("Indica un prezzo a tua scelta"),
    next_track: Reviewed("Brano successivo"),
    nothing_found_for_xxx: Reviewed("Nessun risultato trovato per '{query}'"),
    pause: Reviewed("Pausa"),
    play: Reviewed("Riproduci"),
    playback_position: Reviewed("Posizione di riproduzione"),
    player_closed: Reviewed("Lettore audio chiuso"),
    player_open_playing_xxx: Reviewed("Lettore audio aperto, in riproduzione {title}"),
    previous_track: Reviewed("Traccia Precedente"),
    purchase_downloads: Reviewed("Acquista download"),
    purchase_permalink: Reviewed("acquista"),
    recommended_format: Reviewed("Formato consigliato"),
    search: Reviewed("Cerca"),
    showing_featured_items: Reviewed("Mostrando gli elementi in evidenza"),
    showing_xxx_results_for_xxx: Reviewed("Mostrando {count} risultati per '{query}'"),
    skip_to_main_content: Reviewed("Salta al contenuto principale"),
    unlisted: Reviewed("Non elencato"),
    unlock: Reviewed("Sblocca"),
    unlock_downloads: Reviewed("Sblocca download"),
    unlock_manual_instructions: Reviewed("\
Per sbloccare il download, apporta le modifiche descritte di seguito \
all'indirizzo nella barra degli indirizzi del tuo browser.\
<br><br>\
Prima di procedere, sii consapevole che codici o \
modifiche errate all'indirizzo ti porteranno a una pagina 404. In questo caso \
usa il pulsante Indietro e segui nuovamente attentamente le istruzioni.\
<br><br>\
Sostituisci la parte finale dell'indirizzo - /{unlock_permalink}/{page_hash}{index_suffix} - \
con /{downloads_permalink}/[il-tuo-codice-di-sblocco]{index_suffix} e poi premi Invio."),
    unlock_permalink: Reviewed("sblocca"),
    unmute: Reviewed("Riattiva audio"),
    up_to_xxx: Reviewed("Fino a {xxx}"),
    visual_impairment: Reviewed("Disabilità visiva"),
    volume: Reviewed("Volume"),
    xxx_and_others: Reviewed(r#"{xxx} e <a href="{others_link}">altri</a>"#),
    xxx_hours: Reviewed("{xxx} ore"),
    xxx_minutes: Reviewed("{xxx} minuti"),
    xxx_or_more: Reviewed("{xxx} o più"),
    xxx_seconds: Reviewed("{xxx} secondi"),
    ..Translations::UNTRANSLATED
};
