// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Sin pérdidas y comprimido, si usas exclusivamente productos Apple, elige esto en lugar de FLAC"),
        audio_format_average: String::from("Compresión media, apropiado si tu reproductor no admite mejores formatos"),
        audio_format_flac: String::from("Sin pérdidas y comprimido, mejor opción por archivar"),
        audio_format_mp3: String::from("Compresión ineficiente, apropiado si necesitas compatibilidad con reproductores viejos"),
        audio_format_opus_48: String::from("Excelente compresión, calidad frugal, buena opción si el espacio es limitado"),
        audio_format_opus_96: String::from("Excelente compresión, calidad estandar, buena opción para escuchar sin conexión"),
        audio_format_opus_128: String::from("Excelente compresión, mas alta calidad, mejor opción para escuchar sin conexión"),
        audio_format_uncompressed: String::from("Archivos grandes sin comprimir, apropiado solo para la producción de audio"),
        audio_player_widget_for_release: String::from(r#"Widget de reproductor de audio para la grabación "{title}""#),
        audio_player_widget_for_track: String::from(r#"Widget de reproductor de audio para la pista "{title}""#),
        auto_generated_cover: String::from("Imagen de tapa generada automáticamente"),
        available_formats: String::from("Formatos disponibles:"),
        buy: untranslated!(buy),
        copied: String::from("Copiado"),
        copy: String::from("Copiar"),
        copy_link: untranslated!(copy_link),
        copy_link_to_track: untranslated!(copy_link_to_track),
        confirm: String::from("Confirmar"),
        r#continue: String::from("Continuar"),
        cover_image: String::from("Imagen de tapa"),
        default_unlock_text: String::from("Tienes que ingresar un código para desbloquear estas descargas. Pregunta a los operadores del sitio cómo obtener uno."),
        dimmed: untranslated!(dimmed),
        download: untranslated!(download),
        downloads: String::from("Descargas"),
        downloads_permalink: String::from("descargas"),
        embed: String::from("Incrustar"),
        embed_entire_release: String::from("Incrustar la grabación entera"),
        enter_code_here: String::from("Entra código aquí"),
        external_link: untranslated!(external_link),
        extras: String::from("Extras"),
        failed: String::from("Falló"),
        feed: String::from("Feed"),
        fixed_price: String::from("Precio fijo:"),
        image_descriptions: String::from("Descripciones de imágenes"),
        image_descriptions_guide: String::from("\
Millones de personas navegan el web usando lectores de pantalla \
porque no pueden ver, o no pueden ver lo suficientemente bien. \
Imágenes sin descripciones textuales no están asequibles a ellas, \
y por eso debemos hacer el esfuerzo de proporcionar \
descripciones de imágenes a ellas. \
<br><br>\
Consultar el README de faircamp por aprender como añadir \
descripciones de imágenes, está facil y un acto de bondad. \
<br><br>\
Aquí hay algunos consejos para escribir buenas descripciones de imágenes:<br>\
- Cualquier descripción está mejor que no tener uno, no te preocupes por hacerlo mal.<br>\
- Sea concisa. Escriba tanto como sea necesario, pero también manténgalo lo más corto posible.<br>\
- No interpretas. Describa que hay en el imagen pero no analices más allá de eso.<br>\
- Puedes usar colores cuando tiene sentido - mucha gente solo perdan la vista mas tarde y entienden y aprecian los colores."),
        image_descriptions_permalink: String::from("descripciones-de-imagenes"),
        listen: untranslated!(listen),
        loading: untranslated!(loading),
        made_or_arranged_payment: String::from("He hecho o arreglado el pago"),
        missing_image_description_note: String::from("Falta una descripción de imagen<br>Haz click para aprender más"),
        more: String::from("Más"),
        more_info: untranslated!(more_info),
        muted: untranslated!(muted),
        name_your_price: String::from("Nombra tu precio"),
        next_track: untranslated!(next_track),
        option: String::from("Opción"),
        pause: untranslated!(pause),
        pay_on_liberapay: String::from("Pagar en liberapay:"),
        payment_options: String::from("Opciones de pago:"),
        play: untranslated!(play),
        purchase_downloads: String::from("Comprar descargas"),
        purchase_permalink: String::from("comprar"),
        recommended_format: String::from("Formato recomendado"),
        releases: untranslated!(releases),
        rss_feed: String::from("Feed RSS"),
        this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
        top: untranslated!(top),
        tracks: untranslated!(tracks),
        unlisted: untranslated!(unlisted),
        unlock: String::from("Desbloquear"),
        unlock_downloads: String::from("Desbloquear descargas"),
        unlock_permalink: String::from("desbloquear"),
        unlock_code_seems_incorrect: String::from("El código de desbloqueo entrado parece ser incorrecto, por favor revise si hay errores tipográficos."),
        unlock_manual_instructions: String::from("\
Para desbloquear la descarga, por favor haz los cambios \
descritos abajo en la barra de direcciones del navegador. \
<br><br>\
Antes de empezarlo por favor ten en cuenta que un código \
o modificaciones incorrectas te llevan a una pagina 404. \
En este caso usar el botón de retroceso y sigue los \
instrucciones de nuevo. \
<br><br>\
Reemplaza el parte final de la direccion - /{unlock_permalink}/{page_hash}{index_suffix} - \
con /{downloads_permalink}/[tu-código-de-desbloqueo]{index_suffix} y presiona Enter."),
        up_to_xxx: String::from("Hasta {xxx}"),
        visual_impairment: untranslated!(visual_impairment),
        xxx_and_others: String::from(r#"{xxx} y <a href="{others_link}">otros</a>"#),
        xxx_or_more: String::from("{xxx} o más")
    }
}
