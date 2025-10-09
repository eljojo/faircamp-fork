// SPDX-FileCopyrightText: 2024-2025 Denys Nykula
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations};

pub const UK: Translations = Translations {
    audio_format_alac: Reviewed("Стиснення без втрат. Якщо користуєтесь лише пристроями Apple, оберіть це замість FLAC"),
    audio_format_average: Reviewed("Середнє стиснення. Годиться, якщо ваш пристрій не підтримує кращих форматів"),
    audio_format_flac: Reviewed("Стиснення без втрат. Найкращий вибір для архівування"),
    audio_format_mp3: Reviewed("Слабке стиснення. Годиться, якщо треба сумісність зі старими пристроями"),
    audio_format_opus_48: Reviewed("Чудове стиснення, ощадлива якість. Хороший вибір в умовах обмеженого простору"),
    audio_format_opus_96: Reviewed("Чудове стиснення, звичайна якість. Хороший вибір для офлайн-прослуховування"),
    audio_format_opus_128: Reviewed("Чудове стиснення, найвища якість. Найкращий вибір для офлайн-прослуховування"),
    audio_format_uncompressed: Reviewed("Нестиснені важкі файли. Годиться лише для продюсування музики"),
    audio_player_widget_for_xxx: Reviewed("Звуковий програвач «{title}»"),
    auto_generated_cover: Reviewed("Автоматично згенерована обкладинка"),
    available_formats: Reviewed("Доступні формати:"),
    browse: Reviewed("Огляд"),
    buy: Reviewed("Придбати"),
    close: Reviewed("Закрити"),
    copied: Reviewed("Скопійовано"),
    copy: Reviewed("Копіювати"),
    copy_link: Reviewed("Копіювати посилання"),
    confirm: Reviewed("Підтвердити"),
    r#continue: Reviewed("Далі"),
    cover_image: Reviewed("Зображення обкладинки"),
    default_unlock_info: Reviewed("Щоб дістатися цих завантажень, вам потрібно ввести код. Запитайте в операторів сайту, як його отримати."),
    download: Reviewed("Завантажити"),
    download_code_seems_incorrect: Reviewed("Код розблокування виглядає хибним. Перевірте, чи нема одруку."),
    downloads: Reviewed("Завантаження"),
    downloads_permalink: Reviewed("downloads"),
    embed: Reviewed("Вбудувати"),
    embed_entire_release: Reviewed("Вбудувати цілий випуск"),
    enter_code_here: Reviewed("Уведіть код сюди"),
    external_link: Reviewed("Зовнішнє посилання"),
    extras: Reviewed("Додатки"),
    failed: Reviewed("Помилка"),
    feed: Reviewed("Стрічка"),
    generic_rss: Reviewed("Звичайна RSS"),
    image_descriptions: Reviewed("Описи зображень"),
    image_descriptions_guide: Reviewed("\
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
    image_descriptions_permalink: Reviewed("image-descriptions"),
    javascript_is_disabled_listen_at_xxx: Reviewed("JavaScript вимкнено. Слухати: {link}"),
    javascript_is_disabled_text: Reviewed("JavaScript вимкнено — деякі можливості недоступні"),
    listen: Reviewed("Прослухати"),
    loading: Reviewed("Завантаження"),
    m3u_playlist: Reviewed("M3U-добірка"),
    made_or_arranged_payment: Reviewed("Платіж здійснено"),
    missing_image_description_note: Reviewed("Бракує опису картинки<br>Натисніть, щоб дізнатися більше"),
    more: Reviewed("Більше"),
    mute: Reviewed("Вимкнути звук"),
    name_your_price: Reviewed("Назвіть свою ціну"),
    next_track: Reviewed("Наступна доріжка"),
    nothing_found_for_xxx: Reviewed("«{query}» не знайдено'"),
    pause: Reviewed("Призупинити"),
    play: Reviewed("Відтворити"),
    playback_position: Reviewed("Зараз відтворюється"),
    player_closed: Reviewed("Програвач закрито"),
    player_open_playing_xxx: Reviewed("Програвач відтворює {title}"),
    player_open_with_xxx: Reviewed("У програвачі відкрито {title}"),
    previous_track: Reviewed("Попередня доріжка"),
    price: Reviewed("Ціна:"),
    purchase_downloads: Reviewed("Купити завантаження"),
    purchase_permalink: Reviewed("purchase"),
    recommended_format: Reviewed("Рекомендований формат"),
    search: Reviewed("Пошук"),
    showing_featured_items: Reviewed("Показано обрані елементи"),
    showing_xxx_results_for_xxx: Reviewed("Результати ({count}) пошуку «{query}»"),
    skip_to_main_content: Reviewed("Перейти до головного вмісту"),
    subscribe: Reviewed("Підписатись"),
    subscribe_permalink: Reviewed("subscribe"),
    unlisted: Reviewed("Поза списком"),
    unlock: Reviewed("Розблокувати"),
    unlock_downloads: Reviewed("Розблокувати завантаження"),
    unlock_manual_instructions: Reviewed("\
Щоб розблокувати завантаження, змініть адресу в браузері так, \
як описано внизу.\
<br><br>\
Перш ніж це зробити, врахуйте, що хибні коди чи додаткові \
зміни адреси приведуть вас на сторінку «Не знайдено». В такому \
разі натисніть кнопку «Назад» і уважніше виконайте інструкції.\
<br><br>\
Замініть кінець адреси — /{unlock_permalink}/{page_hash}{index_suffix} — \
на /{downloads_permalink}/[your-unlock-code]{index_suffix} і тоді натисніть Enter."),
    unlock_permalink: Reviewed("unlock"),
    unmute: Reviewed("Увімкнути звук"),
    up_to_xxx: Reviewed("До {xxx}"),
    visual_impairment: Reviewed("Вади зору"),
    volume: Reviewed("Гучність"),
    xxx_and_others: Reviewed(r#"{xxx} та <a href="{others_link}">інші</a>"#),
    xxx_hours: Reviewed("{xxx} год"),
    xxx_minutes: Reviewed("{xxx} хв"),
    xxx_or_more: Reviewed("{xxx} чи більше"),
    xxx_seconds: Reviewed("{xxx} с"),
    ..Translations::UNTRANSLATED
};
