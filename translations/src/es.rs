// SPDX-FileCopyrightText: 2025 Patricio Maripani
// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const ES: Translations = Translations {
    audio_format_alac: Unreviewed("Sin pérdidas y comprimido, si usas exclusivamente productos Apple, elige esto en lugar de FLAC"),
    audio_format_average: Unreviewed("Compresión media, apropiado si tu reproductor no admite mejores formatos"),
    audio_format_flac: Unreviewed("Sin pérdidas y comprimido, mejor opción por archivar"),
    audio_format_mp3: Unreviewed("Compresión ineficiente, apropiado si necesitas compatibilidad con reproductores viejos"),
    audio_format_opus_48: Unreviewed("Excelente compresión, calidad frugal, buena opción si el espacio es limitado"),
    audio_format_opus_96: Unreviewed("Excelente compresión, calidad estandar, buena opción para escuchar sin conexión"),
    audio_format_opus_128: Unreviewed("Excelente compresión, mas alta calidad, mejor opción para escuchar sin conexión"),
    audio_format_uncompressed: Unreviewed("Archivos grandes sin comprimir, apropiado solo para la producción de audio"),
    audio_player_widget_for_xxx: Unreviewed(r#"Widget de reproductor de audio para "{title}""#),
    auto_generated_cover: Unreviewed("Imagen de tapa generada automáticamente"),
    available_formats: Unreviewed("Formatos disponibles:"),
    browse: Reviewed("Explorar"),
    buy: Reviewed("Comprar"),
    close: Unreviewed("Cerrar"),
    copied: Unreviewed("Copiado"),
    copy: Unreviewed("Copiar"),
    copy_link: Reviewed("Copiar enlace"),
    confirm: Unreviewed("Confirmar"),
    r#continue: Unreviewed("Continuar"),
    cover_image: Unreviewed("Imagen de tapa"),
    default_unlock_info: Unreviewed("Tienes que ingresar un código para desbloquear estas descargas. Pregunta a los operadores del sitio cómo obtener uno."),
    download: Reviewed("Descargar"),
    download_code_seems_incorrect: Unreviewed("El código de desbloqueo entrado parece ser incorrecto, por favor revise si hay errores tipográficos."),
    downloads: Unreviewed("Descargas"),
    downloads_permalink: Unreviewed("descargas"),
    embed: Unreviewed("Incrustar"),
    embed_entire_release: Unreviewed("Incrustar la grabación entera"),
    enter_code_here: Reviewed("Ingresa el código aquí"),
    external_link: Reviewed("Enlace externo"),
    extras: Unreviewed("Extras"),
    failed: Unreviewed("Falló"),
    feed: Unreviewed("Feed"),
    fixed_price: Unreviewed("Precio fijo:"),
    image_descriptions: Unreviewed("Descripciones de imágenes"),
    image_descriptions_guide: Unreviewed("\
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
    image_descriptions_permalink: Unreviewed("descripciones-de-imagenes"),
    javascript_is_disabled_listen_at_xxx: Reviewed("JavaScript está desabilitado - Escúchalo en {link}"),
    javascript_is_disabled_text: Reviewed("JavaScript está desabilitado - Algunas funcionalidades no están disponibles"),
    listen: Unreviewed("Escuchar"),
    loading: Reviewed("Cargando"),
    m3u_playlist: Unreviewed("Lista de reproducción M3U"),
    made_or_arranged_payment: Unreviewed("He hecho o arreglado el pago"),
    missing_image_description_note: Unreviewed("Falta una descripción de imagen<br>Haz click para aprender más"),
    more: Unreviewed("Más"),
    mute: Unreviewed("Silenciar"),
    name_your_price: Reviewed("Escoge tu precio"),
    next_track: Reviewed("Siguiente canción"),
    pause: Unreviewed("Pausar"),
    play: Unreviewed("Reproducir"),
    player_closed: Reviewed("Reproductor cerrado"),
    previous_track: Reviewed("Canción previa"),
    purchase_downloads: Unreviewed("Comprar descargas"),
    purchase_permalink: Unreviewed("comprar"),
    recommended_format: Unreviewed("Formato recomendado"),
    search: Unreviewed("Buscar"),
    showing_featured_items: Reviewed("Mostrando ítems destacados"),
    showing_xxx_results_for_xxx: Reviewed("Mostrando {count} resultados para '{query}'"),
    skip_to_main_content: Reviewed("Saltar al contenido principal"),
    unlisted: Unreviewed("No listado"),
    unlock: Unreviewed("Desbloquear"),
    unlock_downloads: Unreviewed("Desbloquear descargas"),
    unlock_manual_instructions: Unreviewed("\
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
    unlock_permalink: Unreviewed("desbloquear"),
    up_to_xxx: Unreviewed("Hasta {xxx}"),
    volume: Reviewed("Volumen"),
    xxx_and_others: Unreviewed(r#"{xxx} y <a href="{others_link}">otros</a>"#),
    xxx_minutes: Unreviewed("{xxx} minutos"),
    xxx_or_more: Unreviewed("{xxx} o más"),
    ..Translations::UNTRANSLATED
};
