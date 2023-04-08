use super::Translations;

pub fn translations() -> Translations {
    Translations {
        any_amount: String::from("Choix libre"),
        audio_format_description_aac: String::from("Qualité d'encodage moyenne – approprié si votre lecteur ne supporte pas de meilleurs formats"),
        audio_format_description_aiff: String::from("Fichiers volumineux non compressés – approprié uniquement pour la production audio"),
        audio_format_description_flac: String::from("Sans perte et compressé – meilleur choix pour l'archivage"),
        audio_format_description_mp3_vbr: String::from("Qualité d'encodage inférieur – approprié si la compatibilité avec des joueurs plus âgés est nécessaire"),
        audio_format_description_ogg_vorbis: String::from("Qualité d'encodage moyenne – approprié si votre lecteur ne supporte pas de meilleurs formats"),
        // TODO: Both hints "for streaming" below address the wrong
        // question somehow: The person reading this wants to download,
        // streaming choice is only relevant to someone who would stream
        // to an audience themselves?
        audio_format_description_opus_48: String::from("Meilleur qualité d'encodage á 48Kbps – meilleur choix pour streaming à forte demande"),
        audio_format_description_opus_96: String::from("Meilleur qualité d'encodage á 96Kbps – meilleur choix pour streaming"),
        audio_format_description_opus_128: String::from("Meilleur qualité d'encodage á 128Kbps – meilleur choix pour l'écoute hors ligne"),
        audio_format_description_wav: String::from("Fichiers volumineux non compressés – approprié uniquement pour la production audio"),
        audio_player_widget_for_release: String::from(r#"Widget de lecteur audio pour l'album "{title}""#),
        audio_player_widget_for_track: String::from(r#"Widget de lecteur audio pour le piste "{title}""#),
        available_formats: String::from("Formats disponibles:"),
        close: String::from("Fermer"),
        copied: String::from("Copié"),
        copy: String::from("Copier"),
        confirm: String::from("Confirmer"),
        r#continue: String::from("Continuer"),
        cover_image: String::from("Image de couverture"),
        default_unlock_text: String::from("Vous devez entrer un code pour déverrouiller ces téléchargements. Demandez aux opérateurs du site comment en obtenir un."),
        downloads: String::from("Téléchargements"),
        downloads_permalink: String::from("telechargements"),
        download_choice_hints: String::from(r##"Téléchargements de piste unique ou téléchargements en autres formats sont disponibles dessous. Vous ne savez pas quel format choisir? Consultez les <a href="#hints">conseils</a> ci-dessous."##),
        embed: String::from("Intégrer"),
        embed_entire_release: String::from("Intégrer tout l'album"),
        enter_code_here: String::from("Entrer code ici"),
        entire_release: String::from("Tout l'album"),
        failed: String::from("Échoué"),
        feed: String::from("Flux RSS"),
        fixed_price: String::from("Prix fixe:"),
        format_guide: String::from("Guide des formats:"),
        image_descriptions: String::from("Descriptions des images"),
        image_descriptions_guide: String::from("\
De millions des personnes naviguent sur le Web à l'aide de lecteurs \
d'écran parce qu'elles ne voient pas (ou pas assez bien). \
Les images sans descriptions textuelles leur sont inaccessibles, \
et c'est pourquoi nous devrions faire l'effort de leur fournir \
des desciptions d'images. \
<br><br>\
Consultez le faircamp README pour savoir comment ajouter \
des descriptions d'images, c'est simple et un acte de \
gentillesse.\
<br><br>\
Voici quelques conseils pour rédiger de bonnes descriptions d'images:<br>\
- N'importe quelle description vaut mieux que de ne pas avoir de description, ne vous inquiétez pas si vous vous trompez.<br>\
- Soyez concis. Écrivez autant que nécessaire, mais en même temps soyez aussi bref que possible.<br>\
- N'interprétez pas. Décrivez ce qui est là et pertinent pour sa compréhension, n'analysez pas au-delà.<br>\
- Vous pouvez utiliser des coleurs là où cela a du sens - beaucoup de gens n'ont perdu la vue que plus tard et comprennent et apprécient les coleurs."),
        image_descriptions_permalink: String::from("descriptions-des-images"),
        made_or_arranged_payment: String::from("J'ai effectué ou organisé le paiement"),
        missing_image_description_note: String::from("Description de l'image manquante<br>Cliquez pour en savoir plus"),
        name_your_price: String::from("Choisis ton prix"),
        option: String::from("Option"),
        pay_on_liberapay: String::from("Acheter en liberapay:"),
        payment_options: String::from("Options d'achat:"),
        purchase_downloads: String::from("Acheter des téléchargements"),
        purchase_permalink: String::from("acheter"),
        recommended_format: String::from("Format recommandé"),
        rss_feed: String::from("Flux RSS"),
        share: String::from("Partager"),
        share_not_available_navigator_clipboard: String::from("Non disponible dans ton navigateur (navigator.clipboard n'est pas supportée)"),
        share_not_available_requires_javascript: String::from("Non disponible dans ton navigateur (nécessite JavaScript)"),
        unlock: String::from("Dégager"),
        unlock_downloads: String::from("Dégager les téléchargements"),
        unlock_permalink: String::from("degager"),
        unlock_code_seems_incorrect: String::from("Le code de déverrouillage semble être incorrect, veuillez vérifier les fautes de frappe."),
        unlock_manual_instructions: String::from("\
Pour déverrouiller le téléchargement, veuillez effectuer les modifications \
décrites ci-dessous à l'adresse dans la barre d'adresse de votre navigateur. \
<br><br>\
Avant de commencer, sachez que des codes ou des modifications d'adresse erronés \
vous amènent à une page 404. Dans ce cas, utilisez le bouton Retour et suivez à \
nouveau attentivement les instructions. \
<br><br>\
Remplacez la dernière partie de l'adresse - /degager/{page_hash}{index_suffix} - \
avec /telechargements/[votre-code-de-deverrouillage]{index_suffix} et appuyez sur Entrée."),
        up_to_xxx: String::from("Jusqu'à {xxx}"),
        xxx_or_more: String::from("{xxx} ou plus")
    }
}