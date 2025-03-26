// SPDX-FileCopyrightText: 2024 Denys Nykula
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const UK: Translations = Translations {
    audio_format_alac: Reviewed("Стиснення без втрат. Якщо користуєтесь лише пристроями Apple, оберіть це замість FLAC"),
    audio_format_average: Reviewed("Середнє стиснення. Годиться, якщо ваш пристрій не підтримує кращих форматів"),
    audio_format_flac: Reviewed("Стиснення без втрат. Найкращий вибір для архівування"),
    audio_format_mp3: Reviewed("Слабке стиснення. Годиться, якщо треба сумісність зі старими пристроями"),
    audio_format_opus_48: Reviewed("Чудове стиснення, ощадлива якість. Хороший вибір в умовах обмеженого простору"),
    audio_format_opus_96: Reviewed("Чудове стиснення, звичайна якість. Хороший вибір для офлайн-прослуховування"),
    audio_format_opus_128: Reviewed("Чудове стиснення, найвища якість. Найкращий вибір для офлайн-прослуховування"),
    audio_format_uncompressed: Reviewed("Нестиснені важкі файли. Годиться лише для продюсування музики"),
    audio_player_widget_for_xxx: Unreviewed("Звуковий програвач «{title}»"),
    auto_generated_cover: Reviewed("Автоматично згенерована обкладинка"),
    available_formats: Reviewed("Доступні формати:"),
    buy: Reviewed("Придбати"),
    copied: Reviewed("Скопійовано"),
    copy: Reviewed("Копіювати"),
    copy_link: Reviewed("Копіювати посилання"),
    confirm: Reviewed("Підтвердити"),
    r#continue: Reviewed("Далі"),
    cover_image: Reviewed("Зображення обкладинки"),
    default_unlock_info: Reviewed("Щоб дістатися цих завантажень, вам потрібно ввести код. Запитайте в операторів сайту, як його отримати."),
    download: Reviewed("Завантажити"),
    download_code_seems_incorrect: Unreviewed("Код розблокування виглядає хибним. Перевірте, чи нема одруку."),
    downloads: Reviewed("Завантаження"),
    downloads_permalink: Reviewed("downloads"),
    embed: Reviewed("Вбудувати"),
    embed_entire_release: Reviewed("Вбудувати цілий випуск"),
    enter_code_here: Reviewed("Уведіть код сюди"),
    external_link: Reviewed("Зовнішнє посилання"),
    extras: Reviewed("Додатки"),
    failed: Reviewed("Помилка"),
    feed: Reviewed("Стрічка"),
    fixed_price: Reviewed("Фіксована ціна:"),
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
    listen: Reviewed("Прослухати"),
    loading: Reviewed("Завантаження"),
    made_or_arranged_payment: Reviewed("Платіж здійснено"),
    missing_image_description_note: Reviewed("Бракує опису картинки<br>Натисніть, щоб дізнатися більше"),
    more: Reviewed("Більше"),
    name_your_price: Reviewed("Назвіть свою ціну"),
    next_track: Reviewed("Наступна доріжка"),
    pause: Reviewed("Призупинити"),
    play: Reviewed("Відтворити"),
    previous_track: Reviewed("Попередня доріжка"),
    purchase_downloads: Reviewed("Купити завантаження"),
    purchase_permalink: Reviewed("purchase"),
    recommended_format: Reviewed("Рекомендований формат"),
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
    up_to_xxx: Reviewed("До {xxx}"),
    visual_impairment: Reviewed("Вади зору"),
    volume: Reviewed("Гучність"),
    xxx_and_others: Reviewed(r#"{xxx} та <a href="{others_link}">інші</a>"#),
    xxx_or_more: Reviewed("{xxx} чи більше"),
    ..Translations::UNTRANSLATED
};
