// SPDX-FileCopyrightText: 2024 Tommaso Croce
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Lossless e compresso, se utilizzi esclusivamente prodotti Apple scegli questo invece di FLAC"),
        audio_format_average: String::from("Compressione media, appropriata se il tuo lettore non supporta formati migliori"),
        audio_format_flac: String::from("Lossless e compresso, scelta migliore per semplici fini di archiviazione"),
        audio_format_mp3: String::from("Compressione inefficiente, appropriata se è necessaria compatibilità con player più vecchi"),
        audio_format_opus_48: String::from("Compressione eccellente, qualità ridotta, buona scelta se lo spazio è limitato"),
        audio_format_opus_96: String::from("Compressione eccellente, qualità standard, buona scelta per ascolti offline"),
        audio_format_opus_128: String::from("Compressione eccellente, massima qualità, migliore scelta per ascolti offline"),
        audio_format_uncompressed: String::from("File grandi non compressi, appropriati solo per la produzione audio"),
        audio_player_widget_for_release: String::from(r#"Widget del lettore audio per l'uscita "{title}""#),
        audio_player_widget_for_track: String::from(r#"Widget del lettore audio per il brano "{title}""#),
        auto_generated_cover: String::from("Copertina generata automaticamente"),
        available_formats: String::from("Formati disponibili:"),
        buy: String::from("Acquista"),
        copied: String::from("Copiato"),
        copy: String::from("Copia"),
        copy_link: String::from("Copia link"),
        confirm: String::from("Conferma"),
        r#continue: String::from("Continua"),
        cover_image: String::from("Immagine di copertina"),
        default_unlock_text: String::from("Devi inserire un codice per sbloccare questi download. Chiedi ai gestori del sito come ottenerne uno."),
        dimmed: String::from("Abbassato"),
        download: String::from("Scarica"),
        downloads: String::from("Download"),
        downloads_permalink: String::from("downloads"),
        embed: String::from("Incorpora"),
        embed_entire_release: String::from("Incorpora l'intera uscita"),
        enter_code_here: String::from("Inserisci il codice qui"),
        external_link: String::from("Link esterno"),
        extras: String::from("Extra"),
        failed: String::from("Fallito"),
        feed: String::from("Feed"),
        fixed_price: String::from("Prezzo fisso:"),
        image_descriptions: String::from("Descrizioni delle immagini"),
        image_descriptions_guide: String::from("\
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
        image_descriptions_permalink: String::from("descrizioni-immagini"),
        listen: String::from("Ascolta"),
        loading: String::from("Caricamento"),
        m3u_playlist: untranslated!(m3u_playlist),
        made_or_arranged_payment: String::from("Ho effettuato o predisposto il pagamento"),
        missing_image_description_note: String::from("Descrizione immagine mancante<br>Clicca per saperne di più"),
        more: String::from("Scopri di più"),
        muted: String::from("Disattivato"),
        name_your_price: String::from("Indica un prezzo a tua scelta"),
        next_track: String::from("Brano successivo"),
        option: String::from("Opzione"),
        pause: String::from("Pausa"),
        payment_options: String::from("Opzioni di pagamento:"),
        play: String::from("Riproduci"),
        previous_track: untranslated!(previous_track),
        purchase_downloads: String::from("Acquista download"),
        purchase_permalink: String::from("acquista"),
        recommended_format: String::from("Formato consigliato"),
        rss_feed: String::from("Feed RSS"),
        this_site_was_created_with_faircamp: String::from("Questo sito è stato creato con {faircamp_link}"),
        unlisted: String::from("Non elencato"),
        unlock: String::from("Sblocca"),
        unlock_downloads: String::from("Sblocca download"),
        unlock_permalink: String::from("sblocca"),
        unlock_code_seems_incorrect: String::from("Il codice di sblocco sembra essere errato, controlla eventuali errori di battitura."),
        unlock_manual_instructions: String::from("\
Per sbloccare il download, apporta le modifiche descritte di seguito \
all'indirizzo nella barra degli indirizzi del tuo browser.\
<br><br>\
Prima di procedere, sii consapevole che codici o \
modifiche errate all'indirizzo ti porteranno a una pagina 404. In questo caso \
usa il pulsante Indietro e segui nuovamente attentamente le istruzioni.\
<br><br>\
Sostituisci la parte finale dell'indirizzo - /{unlock_permalink}/{page_hash}{index_suffix} - \
con /{downloads_permalink}/[il-tuo-codice-di-sblocco]{index_suffix} e poi premi Invio."),
        up_to_xxx: String::from("Fino a {xxx}"),
        visual_impairment: String::from("Disabilità visiva"),
        volume: untranslated!(volume),
        xxx_and_others: String::from(r#"{xxx} e <a href="{others_link}">altri</a>"#),
        xxx_or_more: String::from("{xxx} o più")
    }
}
