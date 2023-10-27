use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_description_aac: String::from("Calidad de codificación promedio – apropiado si tu reproductor no admite mejores formatos"),
        audio_format_description_aiff: String::from("Archivos grandes sin comprimir – apropiado solo para la producción de audio"),
        audio_format_description_flac: String::from("Sin pérdidas y comprimido – mejor opción por archivar"),
        audio_format_description_mp3_vbr: String::from("Calidad de codificación inferior – apropiado si necesitas compatibilidad con reproductores viejos"),
        audio_format_description_ogg_vorbis: String::from("Calidad de codificación promedio – apropiado si tu reproductor no admite mejores formatos"),
        // TODO: Both hints "for streaming" below address the wrong
        // question somehow: The person reading this wants to download,
        // streaming choice is only relevant to someone who would stream
        // to an audience themselves?
        audio_format_description_opus_48: String::from("La mejor calidad de codificación a 48Kbps – mejor opción para la transmisión de alta demanda"),
        audio_format_description_opus_96: String::from("La mejor calidad de codificación a 96Kbps – mejor opción para la transmisión"),
        audio_format_description_opus_128: String::from("La mejor calidad de codificación a 128Kbps – mejor opción para escuchar sin conexión"),
        audio_format_description_wav: String::from("Archivos grandes sin comprimir – apropiado solo para la producción de audio"),
        audio_player_widget_for_release: String::from(r#"Widget de reproductor de audio para la grabación "{title}""#),
        audio_player_widget_for_track: String::from(r#"Widget de reproductor de audio para la pista "{title}""#),
        auto_generated_cover: String::from("Imagen de tapa generada automáticamente"),
        available_formats: String::from("Formatos disponibles:"),
        close: String::from("Cerrar"),
        copied: String::from("Copiado"),
        copy: String::from("Copiar"),
        confirm: String::from("Confirmar"),
        r#continue: String::from("Continuar"),
        cover_image: String::from("Imagen de tapa"),
        default_unlock_text: String::from("Tienes que ingresar un código para desbloquear estas descargas. Pregunta a los operadores del sitio cómo obtener uno."),
        downloads: String::from("Descargas"),
        downloads_permalink: String::from("descargas"),
        download_choice_hints: String::from(r##"Descargas de una sola pista o en otros formatos están disponibles abajo. ¿No sabes qué elegir? Ver los <a href="#hints">consejos</a> abajo."##),
        embed: String::from("Incrustar"),
        embed_entire_release: String::from("Incrustar la grabación entera"),
        enter_code_here: String::from("Entra código aquí"),
        entire_release: String::from("Grabación entera"),
        extra_material: String::from("Material Extra"),
        failed: String::from("Falló"),
        feed: String::from("Feed"),
        fixed_price: String::from("Precio fijo:"),
        format_guide: String::from("Guía de formatos:"),
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
        made_or_arranged_payment: String::from("He hecho o arreglado el pago"),
        missing_image_description_note: String::from("Falta una descripción de imagen<br>Haz click para aprender más"),
        name_your_price: String::from("Nombra tu precio"),
        option: String::from("Opción"),
        pay_on_liberapay: String::from("Pagar en liberapay:"),
        payment_options: String::from("Opciones de pago:"),
        purchase_downloads: String::from("Comprar descargas"),
        purchase_permalink: String::from("comprar"),
        recommended_format: String::from("Formato recomendado"),
        rss_feed: String::from("Feed RSS"),
        share: String::from("Compartir"),
        share_not_available_navigator_clipboard: String::from("No está disponible en tu navegador (navigator.clipboard no está disponible)"),
        share_not_available_requires_javascript: String::from("No está disponible en tu navegador (necesita JavaScript)"),
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
Reemplaza el parte final de la direccion - /desbloquear/{page_hash}{index_suffix} - \
con /descargas/[tu-código-de-desbloqueo]{index_suffix} y presiona Enter."),
        up_to_xxx: String::from("Hasta {xxx}"),
        xxx_or_more: String::from("{xxx} o más")
    }
}