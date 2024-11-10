// SPDX-FileCopyrightText: 2024 Damian Szetela
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub const PL: Translations = Translations {
    audio_format_alac: reviewed!("Bezstratne i skompresowane, jesli używasz tylko produtów Apple, to wybierz ten format zamiast FLAC"),
    audio_format_average: reviewed!("Średnia kompresja, stosowne jeśli twój odtwarzacz nie wspiera lepszych formatów"),
    audio_format_flac: reviewed!("Bezstratne i skompresowane, najlepszy wybór w celach archiwalnych"),
    audio_format_mp3: reviewed!("Niedoskonała kompresja, stosowne jeśli kompatybilność ze starszymi odtwarzaczami jest potrzebna"),
    audio_format_opus_48: reviewed!("Świetna kompresja, średnia jakość, dobry wybór jeśli miejsce jest limitowane"),
    audio_format_opus_96: reviewed!("Świetna kompresja, standardowa jakość, dobry wybór do odsłuchu offline"),
    audio_format_opus_128: reviewed!("Świetna kompresja, najwyższa jakość, najlepszy wybór do odsłuchu offline"),
    audio_format_uncompressed: reviewed!("Nieskompresowane duże pliki, stosowne tylko do produkcji audio"),
    audio_player_widget_for_xxx: unreviewed!(r#"Odtwarzacz audio dla "{title}""#),
    auto_generated_cover: reviewed!("Automatycznie wygenerowana okładka"),
    available_formats: reviewed!("Dostępne czczionki:"),
    buy: untranslated!(buy),
    close: reviewed!("Zamknij"),
    copied: reviewed!("Skopiowane"),
    copy: reviewed!("Kopiuj"),
    copy_link: untranslated!(copy_link),
    confirm: reviewed!("Potwierdź"),
    r#continue: reviewed!("Kontynuuj"),
    cover_image: reviewed!("Okładka"),
    default_unlock_text: reviewed!("Musisz podać kod aby odblokować pobieranie. Spytaj administratora strony jak dostać jeden."),
    dimmed: untranslated!(dimmed),
    download: untranslated!(download),
    downloads: reviewed!("Pobieranie"),
    downloads_permalink: reviewed!("pobieranie"),
    embed: reviewed!("Osadź"),
    embed_entire_release: reviewed!("Osadź całe wydanie"),
    enter_code_here: reviewed!("Wpisz kod tutaj"),
    external_link: untranslated!(external_link),
    extras: reviewed!("Ekstra"),
    failed: reviewed!("Niepowodzenie"),
    feed: reviewed!("Żródło RSS"),
    fixed_price: reviewed!("Stała cena:"),
    image_descriptions: reviewed!("Opisy obrazka"),
    image_descriptions_guide: reviewed!("\
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
    image_descriptions_permalink: reviewed!("opisy-obrazkow"),
    listen: untranslated!(listen),
    loading: untranslated!(loading),
    m3u_playlist: untranslated!(m3u_playlist),
    made_or_arranged_payment: reviewed!("Zrobiłem albo ustawiłem zapłatę"),
    missing_image_description_note: reviewed!("Brakujący opis obrazka<br>Kliknij by dowiedzieć się więcej"),
    more: untranslated!(more),
    mute: untranslated!(mute),
    muted: untranslated!(muted),
    name_your_price: reviewed!("Ustal swoją cenę"),
    next_track: untranslated!(next_track),
    pause: untranslated!(pause),
    play: untranslated!(play),
    playback_position: untranslated!(playback_position),
    player_closed: untranslated!(player_closed),
    player_open_playing_xxx: untranslated!(player_open_playing_xxx),
    previous_track: untranslated!(previous_track),
    purchase_downloads: reviewed!("Kup pobranie"),
    purchase_permalink: reviewed!("zakup"),
    recommended_format: reviewed!("Polecany Format"),
    rss_feed: reviewed!("Źródło RSS"),
    search: untranslated!(search),
    this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
    unlisted: untranslated!(unlisted),
    unlock: reviewed!("Odblokuj"),
    unlock_downloads: reviewed!("Odblokuj pobieranie"),
    unlock_permalink: reviewed!("odblokuj"),
    unlock_code_seems_incorrect: reviewed!("Kod do odblokowania nie jest prawidłowy, sprawdź literówki."),
    unlock_manual_instructions: reviewed!("\
Aby odblokować pobieranie, proszę dokonać poniższych \
zmian w pasku adresu przeglądarki.\
<br><br>\
Zanim to rozpoczniesz bądź świadomy że zły kod albo \
modyfikacja adresu zabierze Cię na stronę 404 page. W takim wypadku \
użyj guzika Wstecz i precyzyjnie podążaj za instrukcjami.\
<br><br>\
Zamień ostatnią część adresu - /{unlock_permalink}/{page_hash}{index_suffix} - \
na /{downloads_permalink}/[twój-kod-odblokowujący]{index_suffix} i naciśnij Enter."),
    unmute: untranslated!(unmute),
    up_to_xxx: reviewed!("Aż do {xxx}"),
    visual_impairment: untranslated!(visual_impairment),
    volume: untranslated!(volume),
    xxx_and_others: untranslated!(xxx_and_others),
    xxx_minutes: untranslated!(xxx_minutes),
    xxx_or_more: reviewed!("{xxx} i więcej")
};
