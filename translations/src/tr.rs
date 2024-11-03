// SPDX-FileCopyrightText: 2024 atomkarinca
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Translations;

pub fn translations() -> Translations {
    Translations {
        audio_format_alac: reviewed!("Kayıpsız ve sıkıştırılmış, eğer yalnızca Apple ürünleri kullanıyorsanız FLAC yerine bunu seçin"),
        audio_format_average: reviewed!("Ortalama sıkıştırma, eğer oynatıcınız daha iyi dosya türlerini desteklemiyorsa uygundur"),
        audio_format_flac: reviewed!("Kayıpsız ve sıkıştırılmış, arşivleme için en iyi seçenek"),
        audio_format_mp3: reviewed!("Verimsiz sıkıştırma, eski oynatıcılarla uyum gerekiyorsa uygundur"),
        audio_format_opus_48: reviewed!("Üst düzey sıkıştırma, yeterli kalite, eğer depolama alanı kısıtlıysa iyi bir seçenek"),
        audio_format_opus_96: reviewed!("Üst düzey sıkıştırma, ortalama kalite, çevrimdışı dinleme için iyi bir seçenek"),
        audio_format_opus_128: reviewed!("Üst düzey sıkıştırma, yüksek kalite, çevrimdışı dinleme için en iyi seçenek"),
        audio_format_uncompressed: reviewed!("Sıkıştırılmamış büyük dosyalar, yalnızca ses prodüksiyonu için uygundur"),
        audio_player_widget_for_release: reviewed!(r#""{title}" albümü için oynatma aygıtı"#),
        audio_player_widget_for_track: reviewed!(r#""{title}" şarkısı için oynatma aygıtı"#),
        auto_generated_cover: reviewed!("Otomatik oluşturulmuş albüm kapağı"),
        available_formats: reviewed!("Mevcut dosya türleri:"),
        buy: reviewed!("Satın al"),
        copied: reviewed!("Kopyalandı"),
        copy: reviewed!("Kopyala"),
        copy_link: reviewed!("Bağlantıyı kopyala"),
        confirm: reviewed!("Onayla"),
        r#continue: reviewed!("Devam et"),
        cover_image: reviewed!("Albüm Kapağı"),
        default_unlock_text: reviewed!("Bu indirmelere ulaşmak için kod girişi yapmanız gerekiyor. Site yöneticilerine danışarak buna ulaşabilirsiniz."),
        dimmed: reviewed!("Soluk"),
        download: reviewed!("İndir"),
        downloads: reviewed!("İndirelenler"),
        downloads_permalink: reviewed!("indirilenler"),
        embed: reviewed!("Sayfaya göm"),
        embed_entire_release: reviewed!("Tüm albümü sayfaya göm"),
        enter_code_here: reviewed!("Kodu buraya girin"),
        external_link: reviewed!("Harici bağlantı"),
        extras: reviewed!("Ekstralar"),
        failed: reviewed!("Başarısız oldu"),
        feed: reviewed!("Kaynak"),
        fixed_price: reviewed!("Sabit fiyat:"),
        image_descriptions: reviewed!("Görüntü Açıklamaları"),
        image_descriptions_guide: reviewed!("\
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
        image_descriptions_permalink: reviewed!("goruntu-aciklamalari"),
        listen: reviewed!("Dinle"),
        loading: reviewed!("Yükleniyor"),
        m3u_playlist: untranslated!(m3u_playlist),
        made_or_arranged_payment: reviewed!("Ödemeyi yaptım ya da ayarladım"),
        missing_image_description_note: reviewed!("Görüntü açıklaması eksik<br>Daha fazla öğrenmek için tıklayın"),
        more: reviewed!("Daha fazla"),
        muted: reviewed!("Ses kısıldı"),
        name_your_price: reviewed!("Tutar girin"),
        next_track: reviewed!("Sonraki parça"),
        pause: reviewed!("Duraklat"),
        play: reviewed!("Oynat"),
        previous_track: untranslated!(previous_track),
        purchase_downloads: reviewed!("İndirmeleri satın al"),
        purchase_permalink: reviewed!("satin-al"),
        recommended_format: reviewed!("Tavsiye edilen dosya türü"),
        rss_feed: reviewed!("RSS Kaynağı"),
        this_site_was_created_with_faircamp: reviewed!("Bu sayfa {faircamp_link} ile oluşturulmuştur"),
        unlisted: reviewed!("Yayınlanmamış"),
        unlock: reviewed!("Kilidi aç"),
        unlock_downloads: reviewed!("İndirmelerin kilidini aç"),
        unlock_permalink: reviewed!("kilidi-ac"),
        unlock_code_seems_incorrect: reviewed!("Kilit açma kodu yanlış görünüyor, lütfen yazım hatası olup olmadığını kontrol edin."),
        unlock_manual_instructions: reviewed!("\
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
        up_to_xxx: reviewed!("{xxx} öğesine kadar"),
        visual_impairment: reviewed!("Görme Bozukluğu"),
        volume: untranslated!(volume),
        xxx_and_others: reviewed!(r#"{xxx} ve <a href="{others_link}">diğerleri</a>"#),
        xxx_or_more: reviewed!("{xxx} ya da daha fazlası")
    }
}
