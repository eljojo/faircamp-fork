// SPDX-FileCopyrightText: 2024 atomkarinca
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations, Unreviewed};

pub const TR: Translations = Translations {
    audio_format_alac: Reviewed("Kayıpsız ve sıkıştırılmış, eğer yalnızca Apple ürünleri kullanıyorsanız FLAC yerine bunu seçin"),
    audio_format_average: Reviewed("Ortalama sıkıştırma, eğer oynatıcınız daha iyi dosya türlerini desteklemiyorsa uygundur"),
    audio_format_flac: Reviewed("Kayıpsız ve sıkıştırılmış, arşivleme için en iyi seçenek"),
    audio_format_mp3: Reviewed("Verimsiz sıkıştırma, eski oynatıcılarla uyum gerekiyorsa uygundur"),
    audio_format_opus_48: Reviewed("Üst düzey sıkıştırma, yeterli kalite, eğer depolama alanı kısıtlıysa iyi bir seçenek"),
    audio_format_opus_96: Reviewed("Üst düzey sıkıştırma, ortalama kalite, çevrimdışı dinleme için iyi bir seçenek"),
    audio_format_opus_128: Reviewed("Üst düzey sıkıştırma, yüksek kalite, çevrimdışı dinleme için en iyi seçenek"),
    audio_format_uncompressed: Reviewed("Sıkıştırılmamış büyük dosyalar, yalnızca ses prodüksiyonu için uygundur"),
    audio_player_widget_for_xxx: Reviewed(r#""{title}" için oynatma aygıtı"#),
    auto_generated_cover: Reviewed("Otomatik oluşturulmuş albüm kapağı"),
    available_formats: Reviewed("Mevcut dosya türleri:"),
    buy: Reviewed("Satın al"),
    close: Reviewed("Kapat"),
    copied: Reviewed("Kopyalandı"),
    copy: Reviewed("Kopyala"),
    copy_link: Reviewed("Bağlantıyı kopyala"),
    confirm: Reviewed("Onayla"),
    r#continue: Reviewed("Devam et"),
    cover_image: Reviewed("Albüm Kapağı"),
    default_unlock_info: Reviewed("Bu indirmelere ulaşmak için kod girişi yapmanız gerekiyor. Site yöneticilerine danışarak buna ulaşabilirsiniz."),
    download: Reviewed("İndir"),
    download_code_seems_incorrect: Unreviewed("Kilit açma kodu yanlış görünüyor, lütfen yazım hatası olup olmadığını kontrol edin."),
    downloads: Reviewed("İndirelenler"),
    downloads_permalink: Reviewed("indirilenler"),
    embed: Reviewed("Sayfaya göm"),
    embed_entire_release: Reviewed("Tüm albümü sayfaya göm"),
    enter_code_here: Reviewed("Kodu buraya girin"),
    external_link: Reviewed("Harici bağlantı"),
    extras: Reviewed("Ekstralar"),
    failed: Reviewed("Başarısız oldu"),
    feed: Reviewed("Kaynak"),
    fixed_price: Reviewed("Sabit fiyat:"),
    image_descriptions: Reviewed("Görüntü Açıklamaları"),
    image_descriptions_guide: Reviewed("\
Milyonlarca insan, göremedikleri için (ya da yeterince \
göremedikleri için) internette ekran okuyucularla \
gezinmektedir. Metin açıklamaları olmayan görüntüler \
onlar için erişilmez olmaktadır, bu yüzden onlar için \
görüntü açıklamaları sağlayacak gayreti göstermemiz \
gerekir.<br><br>\
\
Görüntü açıklamaları eklemek için faircamp README \
dosyasına başvurun, kolaydır ve ayrıca nezaket \
gösterisidir.<br><br>\
\
Kaliteli görüntü açıklamaları yazmak için birkaç öneri:<br>\
- Herhangi bir açıklama olması, hiç açıklama olmamasından iyidir; yanlış yapmaktan korkmayın.<br>\
- Özet olmasına dikkat edin. Gerektiği kadar yazın, aynı zamanda olabildiğince kısa tutmaya çalışın.<br>\
- Yorum yapmayın. Olanı ve olanın anlaşılması için gerekenleri tarif edin, bunun ötesinde analiz yapmayın.<br>\
- Mantıklı olan yerlerde renk kullanabilirsiniz - çoğu insan görme yeteneğini sonradan kaybetmiştir ve renkleri anlayıp değerlendirebilir."),
    image_descriptions_permalink: Reviewed("goruntu-aciklamalari"),
    listen: Reviewed("Dinle"),
    loading: Reviewed("Yükleniyor"),
    m3u_playlist: Reviewed("M3U Oynatma Listesi"),
    made_or_arranged_payment: Reviewed("Ödemeyi yaptım ya da ayarladım"),
    missing_image_description_note: Reviewed("Görüntü açıklaması eksik<br>Daha fazla öğrenmek için tıklayın"),
    more: Reviewed("Daha fazla"),
    mute: Reviewed("Sesi kapat"),
    name_your_price: Reviewed("Tutar girin"),
    next_track: Reviewed("Sonraki Parça"),
    pause: Reviewed("Duraklat"),
    play: Reviewed("Oynat"),
    playback_position: Reviewed("Oynatma pozisyonu"),
    player_closed: Reviewed("Oynatıcı kapalı"),
    player_open_playing_xxx: Reviewed("Oynatıcı açık, {title} oynatılıyor"),
    previous_track: Reviewed("Önceki Parça"),
    purchase_downloads: Reviewed("İndirmeleri satın al"),
    purchase_permalink: Reviewed("satin-al"),
    recommended_format: Reviewed("Tavsiye edilen dosya türü"),
    search: Reviewed("Ara"),
    unlisted: Reviewed("Yayınlanmamış"),
    unlock: Reviewed("Kilidi aç"),
    unlock_downloads: Reviewed("İndirmelerin kilidini aç"),
    unlock_manual_instructions: Reviewed("\
İndirmenin kilidini açmak için lütfen tarayıcınızın adres \
satırında aşağıda tarif edilen değişiklikleri yapın.\
<br><br>\
Başlamadan önce, lütfen yanlış kod ya da adres değişikliklerinin \
sizi 404 sayfasına yönlendireceğini göz önünde bulundurun. \
Böyle bir durumda Geri tuşuna basın ve tarif edilenleri daha \
dikkatli bir şekilde tekrar takip edin.\
<br><br>\
Adresin son kısmındaki /{unlock_permalink}/{page_hash}{index_suffix} - \
ibaresini  /{downloads_permalink}/[your-unlock-code]{index_suffix} ile değiştirerek \
Enter tuşuna basın."),
    unlock_permalink: Reviewed("kilidi-ac"),
    unmute: Reviewed("Sesi aç"),
    up_to_xxx: Reviewed("{xxx} öğesine kadar"),
    visual_impairment: Reviewed("Görme Bozukluğu"),
    volume: Reviewed("Ses Düzeyi"),
    xxx_and_others: Reviewed(r#"{xxx} ve <a href="{others_link}">diğerleri</a>"#),
    xxx_minutes: Reviewed("{xxx} dakika"),
    xxx_or_more: Reviewed("{xxx} ya da daha fazlası"),
    ..Translations::UNTRANSLATED
};
