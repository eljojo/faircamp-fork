// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2024 sknob
// SPDX-FileCopyrightText: 2023 Florian Antoine
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const FR: Translations = Translations {
    audio_format_alac: Reviewed("Sans perte et compacté. Préférer à FLAC si vous n’utilisez que des produits Apple."),
    audio_format_average: Reviewed("Compression moyenne. Adapté aux lecteurs qui ne prennent pas en charge de meilleurs formats."),
    audio_format_flac: Reviewed("Sans perte et compacté. Idéal pour l’archivage."),
    audio_format_mp3: Reviewed("Compression inefficace. Adapté si la compatibilité avec d’anciens lecteurs est nécessaire."),
    audio_format_opus_48: Reviewed("Excellente compression, qualité acceptable. Bon choix en cas d’espace limité."),
    audio_format_opus_96: Reviewed("Excellente compression, qualité standard. Bon choix pour l’écoute hors ligne."),
    audio_format_opus_128: Reviewed("Excellente compression, qualité élevée. Meilleur choix pour l’écoute hors ligne."),
    audio_format_uncompressed: Reviewed("Fichiers volumineux non compactés. Réservé à la création audio."),
    audio_player_widget_for_xxx: Unreviewed(r#"Widget de lecteur audio pour "{title}""#),
    auto_generated_cover: Reviewed("Image de couverture générée automatiquement"),
    available_formats: Reviewed("Formats disponibles:"),
    close: Reviewed("Fermer"),
    copied: Reviewed("Copié"),
    copy: Reviewed("Copier"),
    confirm: Reviewed("Confirmer"),
    r#continue: Reviewed("Continuer"),
    cover_image: Reviewed("Image de couverture"),
    default_unlock_text: Reviewed("Vous devez entrer un code pour déverrouiller ces téléchargements. Demandez au gestionnaire du site comment en obtenir un."),
    downloads: Reviewed("Téléchargements"),
    downloads_permalink: Reviewed("telechargements"),
    embed: Reviewed("Intégrer"),
    embed_entire_release: Reviewed("Intégrer tout l'album"),
    enter_code_here: Reviewed("Entrer le code ici"),
    extras: Reviewed("Suppléments"),
    failed: Reviewed("Échec"),
    feed: Reviewed("Flux RSS"),
    fixed_price: Reviewed("Prix fixe:"),
    image_descriptions: Reviewed("Descriptions des images"),
    image_descriptions_guide: Reviewed("\
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
    image_descriptions_permalink: Reviewed("descriptions-des-images"),
    made_or_arranged_payment: Reviewed("J'ai effectué ou organisé le paiement"),
    missing_image_description_note: Reviewed("Description de l'image manquante<br>Cliquez pour en savoir plus"),
    more: Reviewed("Plus"),
    name_your_price: Reviewed("Choisis ton prix"),
    purchase_downloads: Reviewed("Acheter des téléchargements"),
    purchase_permalink: Reviewed("acheter"),
    recommended_format: Reviewed("Format recommandé"),
    rss_feed: Reviewed("Flux RSS"),
    search: Reviewed("Chercher"),
    unlock: Reviewed("Déverouiller"),
    unlock_downloads: Reviewed("Déverouiller les téléchargements"),
    unlock_permalink: Reviewed("deverouiller"),
    unlock_code_seems_incorrect: Reviewed("Le code de déverrouillage semble être incorrect, veuillez vérifier les fautes de frappe."),
    unlock_manual_instructions: Reviewed("\
Pour déverrouiller le téléchargement, veuillez effectuer les modifications \
décrites ci-dessous dans la barre d'adresse de votre navigateur. \
<br><br>\
Avant de commencer, sachez que des codes ou des modifications d'adresse erronés \
vous amènent à une page 404. Dans ce cas, utilisez le bouton Retour et suivez à \
nouveau attentivement les instructions. \
<br><br>\
Remplacez la dernière partie de l'adresse - /{unlock_permalink}/{page_hash}{index_suffix} - \
avec /{downloads_permalink}/[votre-code-de-deverrouillage]{index_suffix} et appuyez sur Entrée."),
    up_to_xxx: Reviewed("Jusqu'à {xxx}"),
    xxx_and_others: Reviewed(r#"{xxx} et <a href="{others_link}">plus</a>"#),
    xxx_or_more: Reviewed("{xxx} ou plus"),
    ..Translations::UNTRANSLATED
};
