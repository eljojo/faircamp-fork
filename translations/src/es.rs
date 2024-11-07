// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const ES: Translations = Translations {
    audio_format_alac: unreviewed!("Sin pérdidas y comprimido, si usas exclusivamente productos Apple, elige esto en lugar de FLAC"),
    audio_format_average: unreviewed!("Compresión media, apropiado si tu reproductor no admite mejores formatos"),
    audio_format_flac: unreviewed!("Sin pérdidas y comprimido, mejor opción por archivar"),
    audio_format_mp3: unreviewed!("Compresión ineficiente, apropiado si necesitas compatibilidad con reproductores viejos"),
    audio_format_opus_48: unreviewed!("Excelente compresión, calidad frugal, buena opción si el espacio es limitado"),
    audio_format_opus_96: unreviewed!("Excelente compresión, calidad estandar, buena opción para escuchar sin conexión"),
    audio_format_opus_128: unreviewed!("Excelente compresión, mas alta calidad, mejor opción para escuchar sin conexión"),
    audio_format_uncompressed: unreviewed!("Archivos grandes sin comprimir, apropiado solo para la producción de audio"),
    audio_player_widget_for_xxx: unreviewed!(r#"Widget de reproductor de audio para "{title}""#),
    auto_generated_cover: unreviewed!("Imagen de tapa generada automáticamente"),
    available_formats: unreviewed!("Formatos disponibles:"),
    buy: reviewed!("Comprar"),
    close: unreviewed!("Cerrar"),
    copied: unreviewed!("Copiado"),
    copy: unreviewed!("Copiar"),
    copy_link: reviewed!("Copiar enlace"),
    confirm: unreviewed!("Confirmar"),
    r#continue: unreviewed!("Continuar"),
    cover_image: unreviewed!("Imagen de tapa"),
    default_unlock_text: unreviewed!("Tienes que ingresar un código para desbloquear estas descargas. Pregunta a los operadores del sitio cómo obtener uno."),
    dimmed: unreviewed!("Atenuado"),
    download: reviewed!("Descargar"),
    downloads: unreviewed!("Descargas"),
    downloads_permalink: unreviewed!("descargas"),
    embed: unreviewed!("Incrustar"),
    embed_entire_release: unreviewed!("Incrustar la grabación entera"),
    enter_code_here: unreviewed!("Entra código aquí"),
    external_link: reviewed!("Enlace externo"),
    extras: unreviewed!("Extras"),
    failed: unreviewed!("Falló"),
    feed: unreviewed!("Feed"),
    fixed_price: unreviewed!("Precio fijo:"),
    image_descriptions: unreviewed!("Descripciones de imágenes"),
    image_descriptions_guide: unreviewed!("\
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
    image_descriptions_permalink: unreviewed!("descripciones-de-imagenes"),
    listen: unreviewed!("Escuchar"),
    loading: reviewed!("Cargando"),
    m3u_playlist: unreviewed!("Lista de reproducción M3U"),
    made_or_arranged_payment: unreviewed!("He hecho o arreglado el pago"),
    missing_image_description_note: unreviewed!("Falta una descripción de imagen<br>Haz click para aprender más"),
    more: unreviewed!("Más"),
    mute: unreviewed!("Silenciar"),
    muted: unreviewed!("Silenciado"),
    name_your_price: unreviewed!("Nombra tu precio"),
    next_track: untranslated!(next_track),
    pause: unreviewed!("Pausar"),
    play: unreviewed!("Reproducir"),
    previous_track: untranslated!(previous_track),
    purchase_downloads: unreviewed!("Comprar descargas"),
    purchase_permalink: unreviewed!("comprar"),
    recommended_format: unreviewed!("Formato recomendado"),
    rss_feed: unreviewed!("Feed RSS"),
    search: unreviewed!("Buscar"),
    this_site_was_created_with_faircamp: reviewed!("Este sitio fue creado con {faircamp_link}"),
    unlisted: unreviewed!("No listado"),
    unlock: unreviewed!("Desbloquear"),
    unlock_downloads: unreviewed!("Desbloquear descargas"),
    unlock_permalink: unreviewed!("desbloquear"),
    unlock_code_seems_incorrect: unreviewed!("El código de desbloqueo entrado parece ser incorrecto, por favor revise si hay errores tipográficos."),
    unlock_manual_instructions: unreviewed!("\
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
    unmute: untranslated!(unmute),
    up_to_xxx: unreviewed!("Hasta {xxx}"),
    visual_impairment: untranslated!(visual_impairment),
    volume: reviewed!("Volumen"),
    xxx_and_others: unreviewed!(r#"{xxx} y <a href="{others_link}">otros</a>"#),
    xxx_minutes: unreviewed!("{xxx} minutos"),
    xxx_or_more: unreviewed!("{xxx} o más")
};
