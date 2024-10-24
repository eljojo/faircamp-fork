// SPDX-FileCopyrightText: 2024 atomkarinca
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: String::from("Kayıpsız ve sıkıştırılmış, eğer yalnızca Apple ürünleri kullanıyorsanız FLAC yerine bunu seçin"),
        audio_format_average: String::from("Ortalama sıkıştırma, eğer oynatıcınız daha iyi dosya türlerini desteklemiyorsa uygundur"),
        audio_format_flac: String::from("Kayıpsız ve sıkıştırılmış, arşivleme için en iyi seçenek"),
        audio_format_mp3: String::from("Verimsiz sıkıştırma, eski oynatıcılarla uyum gerekiyorsa uygundur"),
        audio_format_opus_48: String::from("Üst düzey sıkıştırma, yeterli kalite, eğer depolama alanı kısıtlıysa iyi bir seçenek"),
        audio_format_opus_96: String::from("Üst düzey sıkıştırma, ortalama kalite, çevrimdışı dinleme için iyi bir seçenek"),
        audio_format_opus_128: String::from("Üst düzey sıkıştırma, yüksek kalite, çevrimdışı dinleme için en iyi seçenek"),
        audio_format_uncompressed: String::from("Sıkıştırılmamış büyük dosyalar, yalnızca ses prodüksiyonu için uygundur"),
        audio_player_widget_for_release: String::from(r#""{title}" albümü için oynatma aygıtı"#),
        audio_player_widget_for_track: String::from(r#""{title}" şarkısı için oynatma aygıtı"#),
        auto_generated_cover: String::from("Otomatik oluşturulmuş albüm kapağı"),
        available_formats: String::from("Mevcut dosya türleri:"),
        buy: String::from("Satın al"),
        copied: String::from("Kopyalandı"),
        copy: String::from("Kopyala"),
        copy_link: String::from("Bağlantıyı kopyala"),
        copy_link_to_track: String::from("Şarkı bağlantısını kopyala"),
        confirm: String::from("Onayla"),
        r#continue: String::from("Devam et"),
        cover_image: String::from("Albüm Kapağı"),
        default_unlock_text: String::from("Bu indirmelere ulaşmak için kod girişi yapmanız gerekiyor. Site yöneticilerine danışarak buna ulaşabilirsiniz."),
        dimmed: String::from("Soluk"),
        download: String::from("İndir"),
        downloads: String::from("İndirelenler"),
        downloads_permalink: String::from("indirilenler"),
        embed: String::from("Sayfaya göm"),
        embed_entire_release: String::from("Tüm albümü sayfaya göm"),
        enter_code_here: String::from("Kodu buraya girin"),
        external_link: String::from("Harici bağlantı"),
        extras: String::from("Ekstralar"),
        failed: String::from("Başarısız oldu"),
        feed: String::from("Kaynak"),
        fixed_price: String::from("Sabit fiyat:"),
        image_descriptions: String::from("Görüntü Açıklamaları"),
        image_descriptions_guide: String::from("\
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
        image_descriptions_permalink: String::from("goruntu-aciklamalari"),
        listen: String::from("Dinle"),
        loading: String::from("Yükleniyor"),
        m3u_playlist: untranslated!(m3u_playlist),
        made_or_arranged_payment: String::from("Ödemeyi yaptım ya da ayarladım"),
        missing_image_description_note: String::from("Görüntü açıklaması eksik<br>Daha fazla öğrenmek için tıklayın"),
        more: String::from("Daha fazla"),
        muted: String::from("Ses kısıldı"),
        name_your_price: String::from("Tutar girin"),
        next_track: String::from("Sonraki parça"),
        option: String::from("Seçenek"),
        pause: String::from("Duraklat"),
        pay_on_liberapay: String::from("Liberapay üzerinden öde:"),
        payment_options: String::from("Ödeme seçenekleri:"),
        play: String::from("Oynat"),
        previous_track: untranslated!(previous_track),
        purchase_downloads: String::from("İndirmeleri satın al"),
        purchase_permalink: String::from("satin-al"),
        recommended_format: String::from("Tavsiye edilen dosya türü"),
        rss_feed: String::from("RSS Kaynağı"),
        this_site_was_created_with_faircamp: String::from("Bu sayfa {faircamp_link} ile oluşturulmuştur"),
        unlisted: String::from("Yayınlanmamış"),
        unlock: String::from("Kilidi aç"),
        unlock_downloads: String::from("İndirmelerin kilidini aç"),
        unlock_permalink: String::from("kilidi-ac"),
        unlock_code_seems_incorrect: String::from("Kilit açma kodu yanlış görünüyor, lütfen yazım hatası olup olmadığını kontrol edin."),
        unlock_manual_instructions: String::from("\
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
        up_to_xxx: String::from("{xxx} öğesine kadar"),
        visual_impairment: String::from("Görme Bozukluğu"),
        volume: untranslated!(volume),
        xxx_and_others: String::from(r#"{xxx} ve <a href="{others_link}">diğerleri</a>"#),
        xxx_or_more: String::from("{xxx} ya da daha fazlası")
    }
}
