// SPDX-FileCopyrightText: 2024 Damian Szetela
// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Bezstratne i skompresowane, jesli używasz tylko produtów Apple, to wybierz ten format zamiast FLAC"),
        audio_format_average: String::from("Średnia kompresja, stosowne jeśli twój odtwarzacz nie wspiera lepszych formatów"),
        audio_format_flac: String::from("Bezstratne i skompresowane, najlepszy wybór w celach archiwalnych"),
        audio_format_mp3: String::from("Niedoskonała kompresja, stosowne jeśli kompatybilność ze starszymi odtwarzaczami jest potrzebna"),
        audio_format_opus_48: String::from("Świetna kompresja, średnia jakość, dobry wybór jeśli miejsce jest limitowane"),
        audio_format_opus_96: String::from("Świetna kompresja, standardowa jakość, dobry wybór do odsłuchu offline"),
        audio_format_opus_128: String::from("Świetna kompresja, najwyższa jakość, najlepszy wybór do odsłuchu offline"),
        audio_format_uncompressed: String::from("Nieskompresowane duże pliki, stosowne tylko do produkcji audio"),
        audio_player_widget_for_release: String::from(r#"Odtwarzacz audio dla wydania "{title}""#),
        audio_player_widget_for_track: String::from(r#"Odtwarzacz audio dla utworu "{title}""#),
        auto_generated_cover: String::from("Automatycznie wygenerowana okładka"),
        available_formats: String::from("Dostępne czczionki:"),
        buy: untranslated!(buy),
        copied: String::from("Skopiowane"),
        copy: String::from("Kopiuj"),
        copy_link: untranslated!(copy_link),
        confirm: String::from("Potwierdź"),
        r#continue: String::from("Kontynuuj"),
        cover_image: String::from("Okładka"),
        default_unlock_text: String::from("Musisz podać kod aby odblokować pobieranie. Spytaj administratora strony jak dostać jeden."),
        dimmed: untranslated!(dimmed),
        download: untranslated!(download),
        downloads: String::from("Pobieranie"),
        downloads_permalink: String::from("pobieranie"),
        embed: String::from("Osadź"),
        embed_entire_release: String::from("Osadź całe wydanie"),
        enter_code_here: String::from("Wpisz kod tutaj"),
        external_link: untranslated!(external_link),
        extras: String::from("Ekstra"),
        failed: String::from("Niepowodzenie"),
        feed: String::from("Żródło RSS"),
        fixed_price: String::from("Stała cena:"),
        image_descriptions: String::from("Opisy obrazka"),
        image_descriptions_guide: String::from("\
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
        image_descriptions_permalink: String::from("opisy-obrazkow"),
        listen: untranslated!(listen),
        loading: untranslated!(loading),
        m3u_playlist: untranslated!(m3u_playlist),
        made_or_arranged_payment: String::from("Zrobiłem albo ustawiłem zapłatę"),
        missing_image_description_note: String::from("Brakujący opis obrazka<br>Kliknij by dowiedzieć się więcej"),
        more: untranslated!(more),
        muted: untranslated!(muted),
        name_your_price: String::from("Ustal swoją cenę"),
        next_track: untranslated!(next_track),
        option: String::from("Opcja"),
        pause: untranslated!(pause),
        payment_options: String::from("Opcje płatności:"),
        play: untranslated!(play),
        previous_track: untranslated!(previous_track),
        purchase_downloads: String::from("Kup pobranie"),
        purchase_permalink: String::from("zakup"),
        recommended_format: String::from("Polecany Format"),
        rss_feed: String::from("Źródło RSS"),
        this_site_was_created_with_faircamp: untranslated!(this_site_was_created_with_faircamp),
        unlisted: untranslated!(unlisted),
        unlock: String::from("Odblokuj"),
        unlock_downloads: String::from("Odblokuj pobieranie"),
        unlock_permalink: String::from("odblokuj"),
        unlock_code_seems_incorrect: String::from("Kod do odblokowania nie jest prawidłowy, sprawdź literówki."),
        unlock_manual_instructions: String::from("\
Aby odblokować pobieranie, proszę dokonać poniższych \
zmian w pasku adresu przeglądarki.\
<br><br>\
Zanim to rozpoczniesz bądź świadomy że zły kod albo \
modyfikacja adresu zabierze Cię na stronę 404 page. W takim wypadku \
użyj guzika Wstecz i precyzyjnie podążaj za instrukcjami.\
<br><br>\
Zamień ostatnią część adresu - /{unlock_permalink}/{page_hash}{index_suffix} - \
na /{downloads_permalink}/[twój-kod-odblokowujący]{index_suffix} i naciśnij Enter."),
        up_to_xxx: String::from("Aż do {xxx}"),
        visual_impairment: untranslated!(visual_impairment),
        volume: untranslated!(volume),
        xxx_and_others: untranslated!(xxx_and_others),
        xxx_or_more: String::from("{xxx} i więcej")
    }
}
