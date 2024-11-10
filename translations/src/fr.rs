// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2024 sknob
// SPDX-FileCopyrightText: 2023 Florian Antoine
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const FR: Translations = Translations {
    audio_format_alac: reviewed!("Sans perte et compacté. Préférer à FLAC si vous n’utilisez que des produits Apple."),
    audio_format_average: reviewed!("Compression moyenne. Adapté aux lecteurs qui ne prennent pas en charge de meilleurs formats."),
    audio_format_flac: reviewed!("Sans perte et compacté. Idéal pour l’archivage."),
    audio_format_mp3: reviewed!("Compression inefficace. Adapté si la compatibilité avec d’anciens lecteurs est nécessaire."),
    audio_format_opus_48: reviewed!("Excellente compression, qualité acceptable. Bon choix en cas d’espace limité."),
    audio_format_opus_96: reviewed!("Excellente compression, qualité standard. Bon choix pour l’écoute hors ligne."),
    audio_format_opus_128: reviewed!("Excellente compression, qualité élevée. Meilleur choix pour l’écoute hors ligne."),
    audio_format_uncompressed: reviewed!("Fichiers volumineux non compactés. Réservé à la création audio."),
    audio_player_widget_for_xxx: unreviewed!(r#"Widget de lecteur audio pour "{title}""#),
    auto_generated_cover: reviewed!("Image de couverture générée automatiquement"),
    available_formats: reviewed!("Formats disponibles:"),
    buy: untranslated!(buy),
    close: reviewed!("Fermer"),
    copied: reviewed!("Copié"),
    copy: reviewed!("Copier"),
    copy_link: untranslated!(copy_link),
    confirm: reviewed!("Confirmer"),
    r#continue: reviewed!("Continuer"),
    cover_image: reviewed!("Image de couverture"),
    default_unlock_text: reviewed!("Vous devez entrer un code pour déverrouiller ces téléchargements. Demandez au gestionnaire du site comment en obtenir un."),
    dimmed: untranslated!(dimmed),
    download: untranslated!(download),
    downloads: reviewed!("Téléchargements"),
    downloads_permalink: reviewed!("telechargements"),
    embed: reviewed!("Intégrer"),
    embed_entire_release: reviewed!("Intégrer tout l'album"),
    enter_code_here: reviewed!("Entrer le code ici"),
    external_link: untranslated!(external_link),
    extras: reviewed!("Suppléments"),
    failed: reviewed!("Échec"),
    feed: reviewed!("Flux RSS"),
    fixed_price: reviewed!("Prix fixe:"),
    image_descriptions: reviewed!("Descriptions des images"),
    image_descriptions_guide: reviewed!("\
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
    image_descriptions_permalink: reviewed!("descriptions-des-images"),
    listen: untranslated!(listen),
    loading: untranslated!(loading),
    m3u_playlist: untranslated!(m3u_playlist),
    made_or_arranged_payment: reviewed!("J'ai effectué ou organisé le paiement"),
    missing_image_description_note: reviewed!("Description de l'image manquante<br>Cliquez pour en savoir plus"),
    more: reviewed!("Plus"),
    mute: untranslated!(mute),
    muted: untranslated!(muted),
    name_your_price: reviewed!("Choisis ton prix"),
    next_track: untranslated!(next_track),
    pause: untranslated!(pause),
    play: untranslated!(play),
    playback_position: untranslated!(playback_position),
    player_closed: untranslated!(player_closed),
    player_open_playing_xxx: untranslated!(player_open_playing_xxx),
    previous_track: untranslated!(previous_track),
    purchase_downloads: reviewed!("Acheter des téléchargements"),
    purchase_permalink: reviewed!("acheter"),
    recommended_format: reviewed!("Format recommandé"),
    rss_feed: reviewed!("Flux RSS"),
    search: reviewed!("Chercher"),
    this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
    unlisted: untranslated!(unlisted),
    unlock: reviewed!("Déverouiller"),
    unlock_downloads: reviewed!("Déverouiller les téléchargements"),
    unlock_permalink: reviewed!("deverouiller"),
    unlock_code_seems_incorrect: reviewed!("Le code de déverrouillage semble être incorrect, veuillez vérifier les fautes de frappe."),
    unlock_manual_instructions: reviewed!("\
Pour déverrouiller le téléchargement, veuillez effectuer les modifications \
décrites ci-dessous dans la barre d'adresse de votre navigateur. \
<br><br>\
Avant de commencer, sachez que des codes ou des modifications d'adresse erronés \
vous amènent à une page 404. Dans ce cas, utilisez le bouton Retour et suivez à \
nouveau attentivement les instructions. \
<br><br>\
Remplacez la dernière partie de l'adresse - /{unlock_permalink}/{page_hash}{index_suffix} - \
avec /{downloads_permalink}/[votre-code-de-deverrouillage]{index_suffix} et appuyez sur Entrée."),
    unmute: untranslated!(unmute),
    up_to_xxx: reviewed!("Jusqu'à {xxx}"),
    visual_impairment: untranslated!(visual_impairment),
    volume: untranslated!(volume),
    xxx_and_others: reviewed!(r#"{xxx} et <a href="{others_link}">plus</a>"#),
    xxx_minutes: untranslated!(xxx_minutes),
    xxx_or_more: reviewed!("{xxx} ou plus")
};
