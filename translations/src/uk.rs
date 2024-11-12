// SPDX-FileCopyrightText: 2024 Denys Nykula
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const UK: Translations = Translations {
    audio_format_alac: reviewed!("Стиснення без втрат. Якщо користуєтесь лише пристроями Apple, оберіть це замість FLAC"),
    audio_format_average: reviewed!("Середнє стиснення. Годиться, якщо ваш пристрій не підтримує кращих форматів"),
    audio_format_flac: reviewed!("Стиснення без втрат. Найкращий вибір для архівування"),
    audio_format_mp3: reviewed!("Слабке стиснення. Годиться, якщо треба сумісність зі старими пристроями"),
    audio_format_opus_48: reviewed!("Чудове стиснення, ощадлива якість. Хороший вибір в умовах обмеженого простору"),
    audio_format_opus_96: reviewed!("Чудове стиснення, звичайна якість. Хороший вибір для офлайн-прослуховування"),
    audio_format_opus_128: reviewed!("Чудове стиснення, найвища якість. Найкращий вибір для офлайн-прослуховування"),
    audio_format_uncompressed: reviewed!("Нестиснені важкі файли. Годиться лише для продюсування музики"),
    audio_player_widget_for_xxx: unreviewed!("Звуковий програвач «{title}»"),
    auto_generated_cover: reviewed!("Автоматично згенерована обкладинка"),
    available_formats: reviewed!("Доступні формати:"),
    browse: untranslated!(browse),
    buy: reviewed!("Придбати"),
    close: untranslated!(close),
    copied: reviewed!("Скопійовано"),
    copy: reviewed!("Копіювати"),
    copy_link: reviewed!("Копіювати посилання"),
    confirm: reviewed!("Підтвердити"),
    r#continue: reviewed!("Далі"),
    cover_image: reviewed!("Зображення обкладинки"),
    default_unlock_text: reviewed!("Щоб дістатися цих завантажень, вам потрібно ввести код. Запитайте в операторів сайту, як його отримати."),
    dimmed: reviewed!("Притишено"),
    download: reviewed!("Завантажити"),
    downloads: reviewed!("Завантаження"),
    downloads_permalink: reviewed!("downloads"),
    embed: reviewed!("Вбудувати"),
    embed_entire_release: reviewed!("Вбудувати цілий випуск"),
    enter_code_here: reviewed!("Уведіть код сюди"),
    external_link: reviewed!("Зовнішнє посилання"),
    extras: reviewed!("Додатки"),
    failed: reviewed!("Помилка"),
    feed: reviewed!("Стрічка"),
    fixed_price: reviewed!("Фіксована ціна:"),
    image_descriptions: reviewed!("Описи зображень"),
    image_descriptions_guide: reviewed!("\
Мільйони людей використовують веб за допомогою програм \
читання екрану, бо не можуть бачити чи бачать нечітко. \
Зображення без текстових описів для них недоступні, \
тож нам слід докладати зусиль, аби надавати для них \
описи зображень.<br><br>\
\
Щоб дізнатись, як додавати описи зображень, \
прочитайте README faircamp. Це нескладно і є \
добрим вчинком.<br><br>\
\
Ось кілька порад, як писати хороші описи зображень:<br>\
- Принаймні якийсь опис краще, ніж жодного опису. Не хвилюйтесь, що робите щось неправильно.<br>\
- Будьте лаконічними. Пишіть стільки, скільки необхідно, водночас намагаючись висловлюватись якнайкоротше.<br>\
- Не інтерпретуйте. Описуйте, що зображено й що слід знати для розуміння. Не аналізуйте докладніше.<br>\
- Використовуйте кольори, де в цьому є сенс. Багато хто втратили зір пізно в житті, кольори тішать їхню пам'ять."),
    image_descriptions_permalink: reviewed!("image-descriptions"),
    listen: reviewed!("Прослухати"),
    loading: reviewed!("Завантаження"),
    m3u_playlist: untranslated!(m3u_playlist),
    made_or_arranged_payment: reviewed!("Платіж здійснено"),
    missing_image_description_note: reviewed!("Бракує опису картинки<br>Натисніть, щоб дізнатися більше"),
    more: reviewed!("Більше"),
    mute: untranslated!(mute),
    muted: reviewed!("Вимкнено"),
    name_your_price: reviewed!("Назвіть свою ціну"),
    next_track: reviewed!("Наступна доріжка"),
    nothing_found_for_xxx: untranslated!(nothing_found_for_xxx),
    pause: reviewed!("Призупинити"),
    play: reviewed!("Відтворити"),
    playback_position: untranslated!(playback_position),
    player_closed: untranslated!(player_closed),
    player_open_playing_xxx: untranslated!(player_open_playing_xxx),
    previous_track: reviewed!("Попередня доріжка"),
    purchase_downloads: reviewed!("Купити завантаження"),
    purchase_permalink: reviewed!("purchase"),
    recommended_format: reviewed!("Рекомендований формат"),
    rss_feed: reviewed!("RSS-стрічка"),
    search: untranslated!(search),
    showing_featured_items: untranslated!(showing_featured_items),
    showing_xxx_results_for_xxx: untranslated!(showing_xxx_results_for_xxx),
    this_site_was_created_with_faircamp: reviewed!("Сайт створено за допомогою {faircamp_link}"),
    unlisted: reviewed!("Поза списком"),
    unlock: reviewed!("Розблокувати"),
    unlock_downloads: reviewed!("Розблокувати завантаження"),
    unlock_permalink: reviewed!("unlock"),
    unlock_code_seems_incorrect: reviewed!("Код розблокування виглядає хибним. Перевірте, чи нема одруку."),
    unlock_manual_instructions: reviewed!("\
Щоб розблокувати завантаження, змініть адресу в браузері так, \
як описано внизу.\
<br><br>\
Перш ніж це зробити, врахуйте, що хибні коди чи додаткові \
зміни адреси приведуть вас на сторінку «Не знайдено». В такому \
разі натисніть кнопку «Назад» і уважніше виконайте інструкції.\
<br><br>\
Замініть кінець адреси — /{unlock_permalink}/{page_hash}{index_suffix} — \
на /{downloads_permalink}/[your-unlock-code]{index_suffix} і тоді натисніть Enter."),
    unmute: untranslated!(unmute),
    up_to_xxx: reviewed!("До {xxx}"),
    visual_impairment: reviewed!("Вади зору"),
    volume: reviewed!("Гучність"),
    xxx_and_others: reviewed!(r#"{xxx} та <a href="{others_link}">інші</a>"#),
    xxx_minutes: untranslated!(xxx_minutes),
    xxx_or_more: reviewed!("{xxx} чи більше")
};
