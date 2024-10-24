// SPDX-FileCopyrightText: 2024 Denys Nykula
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Стиснення без втрат. Якщо користуєтесь лише пристроями Apple, оберіть це замість FLAC"),
        audio_format_average: String::from("Середнє стиснення. Годиться, якщо ваш пристрій не підтримує кращих форматів"),
        audio_format_flac: String::from("Стиснення без втрат. Найкращий вибір для архівування"),
        audio_format_mp3: String::from("Слабке стиснення. Годиться, якщо треба сумісність зі старими пристроями"),
        audio_format_opus_48: String::from("Чудове стиснення, ощадлива якість. Хороший вибір в умовах обмеженого простору"),
        audio_format_opus_96: String::from("Чудове стиснення, звичайна якість. Хороший вибір для офлайн-прослуховування"),
        audio_format_opus_128: String::from("Чудове стиснення, найвища якість. Найкращий вибір для офлайн-прослуховування"),
        audio_format_uncompressed: String::from("Нестиснені важкі файли. Годиться лише для продюсування музики"),
        audio_player_widget_for_release: String::from("Звуковий програвач випуску «{title}»"),
        audio_player_widget_for_track: String::from("Звуковий програвач доріжки «{title}»"),
        auto_generated_cover: String::from("Автоматично згенерована обкладинка"),
        available_formats: String::from("Доступні формати:"),
        buy: String::from("Придбати"),
        copied: String::from("Скопійовано"),
        copy: String::from("Копіювати"),
        copy_link: String::from("Копіювати посилання"),
        copy_link_to_track: String::from("Копіювати посилання на доріжку"),
        confirm: String::from("Підтвердити"),
        r#continue: String::from("Далі"),
        cover_image: String::from("Зображення обкладинки"),
        default_unlock_text: String::from("Щоб дістатися цих завантажень, вам потрібно ввести код. Запитайте в операторів сайту, як його отримати."),
        dimmed: String::from("Притишено"),
        download: String::from("Завантажити"),
        downloads: String::from("Завантаження"),
        downloads_permalink: String::from("downloads"),
        embed: String::from("Вбудувати"),
        embed_entire_release: String::from("Вбудувати цілий випуск"),
        enter_code_here: String::from("Уведіть код сюди"),
        external_link: String::from("Зовнішнє посилання"),
        extras: String::from("Додатки"),
        failed: String::from("Помилка"),
        feed: String::from("Стрічка"),
        fixed_price: String::from("Фіксована ціна:"),
        image_descriptions: String::from("Описи зображень"),
        image_descriptions_guide: String::from("\
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
        image_descriptions_permalink: String::from("image-descriptions"),
        listen: String::from("Прослухати"),
        loading: String::from("Завантаження"),
        made_or_arranged_payment: String::from("Платіж здійснено"),
        missing_image_description_note: String::from("Бракує опису картинки<br>Натисніть, щоб дізнатися більше"),
        more: String::from("Більше"),
        muted: String::from("Вимкнено"),
        name_your_price: String::from("Назвіть свою ціну"),
        next_track: String::from("Наступна доріжка"),
        option: String::from("Варіант"),
        pause: String::from("Призупинити"),
        pay_on_liberapay: String::from("Сплатити на liberapay:"),
        payment_options: String::from("Як сплатити:"),
        play: String::from("Відтворити"),
        previous_track: String::from("Попередня доріжка"),
        purchase_downloads: String::from("Купити завантаження"),
        purchase_permalink: String::from("purchase"),
        recommended_format: String::from("Рекомендований формат"),
        releases: String::from("Випуски"),
        rss_feed: String::from("RSS-стрічка"),
        this_site_was_created_with_faircamp: String::from("Сайт створено за допомогою {faircamp_link}"),
        top: String::from("Угору"),
        tracks: String::from("Доріжки"),
        unlisted: String::from("Поза списком"),
        unlock: String::from("Розблокувати"),
        unlock_downloads: String::from("Розблокувати завантаження"),
        unlock_permalink: String::from("unlock"),
        unlock_code_seems_incorrect: String::from("Код розблокування виглядає хибним. Перевірте, чи нема одруку."),
        unlock_manual_instructions: String::from("\
Щоб розблокувати завантаження, змініть адресу в браузері так, \
як описано внизу.\
<br><br>\
Перш ніж це зробити, врахуйте, що хибні коди чи додаткові \
зміни адреси приведуть вас на сторінку «Не знайдено». В такому \
разі натисніть кнопку «Назад» і уважніше виконайте інструкції.\
<br><br>\
Замініть кінець адреси — /{unlock_permalink}/{page_hash}{index_suffix} — \
на /{downloads_permalink}/[your-unlock-code]{index_suffix} і тоді натисніть Enter."),
        up_to_xxx: String::from("До {xxx}"),
        visual_impairment: String::from("Вади зору"),
        volume: String::from("Гучність"),
        xxx_and_others: String::from(r#"{xxx} та <a href="{others_link}">інші</a>"#),
        xxx_or_more: String::from("{xxx} чи більше")
    }
}
