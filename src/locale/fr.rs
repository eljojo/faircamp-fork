// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Florian Antoine
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Sans perte et compressé, si tu utilises exclusivement des produits Apple, choisis ceci plutôt que FLAC"),
        audio_format_average: String::from("Compression moyenne, approprié si votre lecteur ne supporte pas de meilleurs formats"),
        audio_format_flac: String::from("Sans perte et compressé, idéal pour l'archivage"),
        audio_format_mp3: String::from("Compression inefficace, approprié si la compatibilité avec des lecteur plus anciens est nécessaire"),
        audio_format_opus_48: String::from("Excellente compression, qualité économe, bon choix si l'espace est limité"),
        audio_format_opus_96: String::from("Excellente compression, qualité standard, bon choix pour l'écoute hors ligne"),
        audio_format_opus_128: String::from("Excellente compression, qualité supérieure, meilleur choix pour l'écoute hors ligne"),
        audio_format_uncompressed: String::from("Fichiers volumineux non compressés, approprié uniquement pour la production audio"),
        audio_player_widget_for_release: String::from(r#"Widget de lecteur audio pour l'album "{title}""#),
        audio_player_widget_for_track: String::from(r#"Widget de lecteur audio pour la piste "{title}""#),
        auto_generated_cover: String::from("Image de couverture générée automatiquement"),
        available_formats: String::from("Formats disponibles:"),
        buy: untranslated!(buy),
        copied: String::from("Copié"),
        copy: String::from("Copier"),
        copy_link: untranslated!(copy_link),
        confirm: String::from("Confirmer"),
        r#continue: String::from("Continuer"),
        cover_image: String::from("Image de couverture"),
        default_unlock_text: String::from("Vous devez entrer un code pour déverrouiller ces téléchargements. Demandez au gestionnaire du site comment en obtenir un."),
        dimmed: untranslated!(dimmed),
        download: untranslated!(download),
        downloads: String::from("Téléchargements"),
        downloads_permalink: String::from("telechargements"),
        embed: String::from("Intégrer"),
        embed_entire_release: String::from("Intégrer tout l'album"),
        enter_code_here: String::from("Entrer le code ici"),
        external_link: untranslated!(external_link),
        extras: String::from("Suppléments"),
        failed: String::from("Échec"),
        feed: String::from("Flux RSS"),
        fixed_price: String::from("Prix fixe:"),
        image_descriptions: String::from("Descriptions des images"),
        image_descriptions_guide: String::from("\
De millions des personnes naviguent sur le Web à l'aide de lecteurs \
d'écran parce qu'elles ne voient pas (ou pas assez bien). \
Les images sans descriptions textuelles leur sont inaccessibles, \
et c'est pourquoi nous devrions faire l'effort de leur fournir \
des desciptions d'images. \
<br><br>\
Consultez le faircamp README pour savoir comment ajouter \
des descriptions d'images, c'est simple et c'est un acte de \
gentillesse.\
<br><br>\
Voici quelques conseils pour rédiger de bonnes descriptions d'images:<br>\
- N'importe quelle description vaut mieux que de ne pas avoir de description, ne vous inquiétez pas si vous vous trompez.<br>\
- Soyez concis. Écrivez autant que nécessaire, mais en même temps soyez aussi bref que possible.<br>\
- N'interprétez pas. Décrivez ce qui est là et pertinent pour la compréhension, n'analysez pas au-delà.<br>\
- Vous pouvez utiliser des couleurs là où cela a du sens - beaucoup de gens n'ont perdu la vue que plus tard et comprennent et apprécient les coleurs."),
        image_descriptions_permalink: String::from("descriptions-des-images"),
        listen: untranslated!(listen),
        loading: untranslated!(loading),
        m3u_playlist: untranslated!(m3u_playlist),
        made_or_arranged_payment: String::from("J'ai effectué ou organisé le paiement"),
        missing_image_description_note: String::from("Description de l'image manquante<br>Cliquez pour en savoir plus"),
        more: String::from("Plus"),
        muted: untranslated!(muted),
        name_your_price: String::from("Choisis ton prix"),
        next_track: untranslated!(next_track),
        option: String::from("Option"),
        pause: untranslated!(pause),
        pay_on_liberapay: String::from("Acheter avec liberapay:"),
        payment_options: String::from("Options d'achat:"),
        play: untranslated!(play),
        previous_track: untranslated!(previous_track),
        purchase_downloads: String::from("Acheter des téléchargements"),
        purchase_permalink: String::from("acheter"),
        recommended_format: String::from("Format recommandé"),
        rss_feed: String::from("Flux RSS"),
        this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
        unlisted: untranslated!(unlisted),
        unlock: String::from("Déverouiller"),
        unlock_downloads: String::from("Déverouiller les téléchargements"),
        unlock_permalink: String::from("deverouiller"),
        unlock_code_seems_incorrect: String::from("Le code de déverrouillage semble être incorrect, veuillez vérifier les fautes de frappe."),
        unlock_manual_instructions: String::from("\
Pour déverrouiller le téléchargement, veuillez effectuer les modifications \
décrites ci-dessous dans la barre d'adresse de votre navigateur. \
<br><br>\
Avant de commencer, sachez que des codes ou des modifications d'adresse erronés \
vous amènent à une page 404. Dans ce cas, utilisez le bouton Retour et suivez à \
nouveau attentivement les instructions. \
<br><br>\
Remplacez la dernière partie de l'adresse - /{unlock_permalink}/{page_hash}{index_suffix} - \
avec /{downloads_permalink}/[votre-code-de-deverrouillage]{index_suffix} et appuyez sur Entrée."),
        up_to_xxx: String::from("Jusqu'à {xxx}"),
        visual_impairment: untranslated!(visual_impairment),
        volume: untranslated!(volume),
        xxx_and_others: String::from(r#"{xxx} et <a href="{others_link}">plus</a>"#),
        xxx_or_more: String::from("{xxx} ou plus")
    }
}
