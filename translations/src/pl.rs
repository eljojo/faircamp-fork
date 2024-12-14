// SPDX-FileCopyrightText: 2024 Damian Szetela
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const PL: Translations = Translations {
    audio_format_alac: Reviewed("Bezstratne i skompresowane, jesli używasz tylko produtów Apple, to wybierz ten format zamiast FLAC"),
    audio_format_average: Reviewed("Średnia kompresja, stosowne jeśli twój odtwarzacz nie wspiera lepszych formatów"),
    audio_format_flac: Reviewed("Bezstratne i skompresowane, najlepszy wybór w celach archiwalnych"),
    audio_format_mp3: Reviewed("Niedoskonała kompresja, stosowne jeśli kompatybilność ze starszymi odtwarzaczami jest potrzebna"),
    audio_format_opus_48: Reviewed("Świetna kompresja, średnia jakość, dobry wybór jeśli miejsce jest limitowane"),
    audio_format_opus_96: Reviewed("Świetna kompresja, standardowa jakość, dobry wybór do odsłuchu offline"),
    audio_format_opus_128: Reviewed("Świetna kompresja, najwyższa jakość, najlepszy wybór do odsłuchu offline"),
    audio_format_uncompressed: Reviewed("Nieskompresowane duże pliki, stosowne tylko do produkcji audio"),
    audio_player_widget_for_xxx: Unreviewed(r#"Odtwarzacz audio dla "{title}""#),
    auto_generated_cover: Reviewed("Automatycznie wygenerowana okładka"),
    available_formats: Reviewed("Dostępne czczionki:"),
    close: Reviewed("Zamknij"),
    copied: Reviewed("Skopiowane"),
    copy: Reviewed("Kopiuj"),
    confirm: Reviewed("Potwierdź"),
    r#continue: Reviewed("Kontynuuj"),
    cover_image: Reviewed("Okładka"),
    default_unlock_info: Reviewed("Musisz podać kod aby odblokować pobieranie. Spytaj administratora strony jak dostać jeden."),
    download_code_seems_incorrect: Unreviewed("Kod do odblokowania nie jest prawidłowy, sprawdź literówki."),
    downloads: Reviewed("Pobieranie"),
    downloads_permalink: Reviewed("pobieranie"),
    embed: Reviewed("Osadź"),
    embed_entire_release: Reviewed("Osadź całe wydanie"),
    enter_code_here: Reviewed("Wpisz kod tutaj"),
    extras: Reviewed("Ekstra"),
    failed: Reviewed("Niepowodzenie"),
    feed: Reviewed("Żródło RSS"),
    fixed_price: Reviewed("Stała cena:"),
    image_descriptions: Reviewed("Opisy obrazka"),
    image_descriptions_guide: Reviewed("\
Miliony ludzi przeglądają sieć przez czytniki dla niewidomych \
ponieważ mają problem ze wzrokiem. Obrazki \
bez opisu tekstowego są dla nich niedostępne, \
i właśnie dlatego powinniśmy podjąć wysiłek by dodać \
opisy obrazków dla nich.<br><br>\
\
Przeczytaj plik README by dowiedzieć się jak dodać \
opisy, to proste i jest to akt \
życzliwości.<br><br>\
\
Tutaj są rady jak pisać dobre opisy obrazków:<br>\
- Jakikolwiek opis jest lepszy niż brak opisu, nie martw się że zrobisz go źle.<br>\
- Uczyń go zwięzłym. Napisz tyle ile potrzeba, ale zarazem najkrócej jak się da.<br>\
- Nie interpretuj. Opisz co tam jest co jest niezbędne do zrozumienia, nie analizuj ponad to.<br>\
- Używaj kolorów jeśli to ma sens - wiele ludzi straciło wzrok później i rozumieją i doceniają kolory."),
    image_descriptions_permalink: Reviewed("opisy-obrazkow"),
    made_or_arranged_payment: Reviewed("Zrobiłem albo ustawiłem zapłatę"),
    missing_image_description_note: Reviewed("Brakujący opis obrazka<br>Kliknij by dowiedzieć się więcej"),
    name_your_price: Reviewed("Ustal swoją cenę"),
    purchase_downloads: Reviewed("Kup pobranie"),
    purchase_permalink: Reviewed("zakup"),
    recommended_format: Reviewed("Polecany Format"),
    rss_feed: Reviewed("Źródło RSS"),
    unlock: Reviewed("Odblokuj"),
    unlock_downloads: Reviewed("Odblokuj pobieranie"),
    unlock_manual_instructions: Reviewed("\
Aby odblokować pobieranie, proszę dokonać poniższych \
zmian w pasku adresu przeglądarki.\
<br><br>\
Zanim to rozpoczniesz bądź świadomy że zły kod albo \
modyfikacja adresu zabierze Cię na stronę 404 page. W takim wypadku \
użyj guzika Wstecz i precyzyjnie podążaj za instrukcjami.\
<br><br>\
Zamień ostatnią część adresu - /{unlock_permalink}/{page_hash}{index_suffix} - \
na /{downloads_permalink}/[twój-kod-odblokowujący]{index_suffix} i naciśnij Enter."),
    unlock_permalink: Reviewed("odblokuj"),
    up_to_xxx: Reviewed("Aż do {xxx}"),
    xxx_or_more: Reviewed("{xxx} i więcej"),
    ..Translations::UNTRANSLATED
};
