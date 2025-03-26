// SPDX-FileCopyrightText: 2025 naskya
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Reviewed, Translations};

pub const JA: Translations = Translations {
    audio_format_alac: Reviewed("可逆圧縮 – FLAC よりも Apple 製品の利用者に適した形式"),
    audio_format_average: Reviewed("平均的な圧縮効率 – 対応した良い形式が他に無い場合に用いられる"),
    audio_format_flac: Reviewed("可逆圧縮 – アーカイブに最適な形式"),
    audio_format_mp3: Reviewed("非効率な圧縮 – 古い再生機器との互換性が高い形式"),
    audio_format_opus_48: Reviewed("高効率な圧縮・低品質 – 空き容量が限られている場合に適した形式"),
    audio_format_opus_96: Reviewed("高効率な圧縮・標準品質 – オフライン再生に適した形式"),
    audio_format_opus_128: Reviewed("高効率な圧縮・高品質 – オフライン再生に最適な形式"),
    audio_format_uncompressed: Reviewed("非圧縮 – ファイルが大きいため音楽制作にのみ用いられる"),
    audio_player_widget_for_xxx: Reviewed("「{title}」の再生ウィジェット"),
    auto_generated_cover: Reviewed("自動生成されたアートワーク"),
    available_formats: Reviewed("利用可能な形式："),
    browse: Reviewed("閲覧"),
    buy: Reviewed("購入"),
    close: Reviewed("閉じる"),
    copied: Reviewed("コピーしました"),
    copy: Reviewed("コピー"),
    copy_link: Reviewed("リンクをコピー"),
    confirm: Reviewed("確認"),
    r#continue: Reviewed("続ける"),
    cover_image: Reviewed("アートワーク"),
    default_unlock_info: Reviewed("ダウンロードコードが必要です。コードの取得方法はサイト管理者にお尋ねください。"),
    download: Reviewed("ダウンロード"),
    download_code_seems_incorrect: Reviewed("ダウンロードコードが間違っています。入力ミスが無いか確かめてください。"),
    downloads: Reviewed("ダウンロード"),
    downloads_permalink: Reviewed("downloads"),
    embed: Reviewed("埋め込み"),
    embed_entire_release: Reviewed("このリリース全体の埋め込み"),
    enter_code_here: Reviewed("コードをここに入力"),
    external_link: Reviewed("外部リンク"),
    extras: Reviewed("特典"),
    failed: Reviewed("失敗しました"),
    feed: Reviewed("フィード"),
    fixed_price: Reviewed("定価："),
    image_descriptions: Reviewed("画像の説明文"),
    image_descriptions_guide: Reviewed("\
視覚障害などの理由から、数百万人もの人々が\
スクリーンリーダーを使ってウェブを閲覧しています。\
彼らは説明文の無い画像を利用することができません。\
ですから、私達は彼らのために画像に説明文を追加する必要があります。<br><br>\
\
説明文の書き方は faircamp の README を参考にしてください。\
これは簡単で心温まる行動です。<br><br>\
\
良い説明文を書くためのヒント：<br>\
- どんな説明文も、無いよりはあったほうが良いです。ミスを恐れずに書きましょう。<br>\
- 簡潔に書きましょう。必要な事項を記入しつつ、なるべく文を短くしましょう。<br>\
- 画像内にあるものを、そのまま記述しましょう。画像の内容を分析したり感想を述べたりする必要はありません。<br>\
- 色を説明文に含めてもよいです。特に後天的に視覚を失った方々にとって、色を使った説明は役立ちます。"),
    image_descriptions_permalink: Reviewed("image-descriptions"),
    javascript_is_disabled_listen_at_xxx: Reviewed("JavaScript が無効です – {link} で再生する"),
    javascript_is_disabled_text: Reviewed("JavaScript が無効なので、利用できない機能があります。"),
    listen: Reviewed("聴く"),
    loading: Reviewed("読み込み中"),
    m3u_playlist: Reviewed("M3U プレイリスト"),
    made_or_arranged_payment: Reviewed("既に支払いを行いました"),
    missing_image_description_note: Reviewed("画像に説明文がありません<br>クリックして詳細を表示"),
    more: Reviewed("もっと見る"),
    mute: Reviewed("消音"),
    name_your_price: Reviewed("金額を指定"),
    next_track: Reviewed("次のトラック"),
    nothing_found_for_xxx: Reviewed("「{query}」は見つかりませんでした"),
    pause: Reviewed("一時停止"),
    play: Reviewed("再生"),
    playback_position: Reviewed("再生位置"),
    player_closed: Reviewed("プレーヤーを閉じました"),
    player_open_playing_xxx: Reviewed("プレーヤーを起動、「{title}」を再生中"),
    previous_track: Reviewed("前のトラック"),
    purchase_downloads: Reviewed("購入してダウンロードする"),
    purchase_permalink: Reviewed("purchase"),
    recommended_format: Reviewed("推奨の形式"),
    search: Reviewed("検索"),
    showing_featured_items: Reviewed("注目の作品を表示中"),
    showing_xxx_results_for_xxx: Reviewed("「{query}」の {count} 件の検索結果を表示中"),
    skip_to_main_content: Reviewed("メインコンテンツにスキップ"),
    unlisted: Reviewed("非表示"),
    unlock: Reviewed("ロック解除"),
    unlock_downloads: Reviewed("ダウンロードのロックを解除"),
    unlock_manual_instructions: Reviewed("\
ダウンロードのロックを解除するには、ブラウザのアドレスバーに表示されている \
URL に以下の変更を加えてください。\
<br><br>\
URL が正しくない場合、404 エラーのページが表示されます。\
その場合にはブラウザの「戻る」ボタンを利用して再度操作を行ってください。\
<br><br>\
URL 末尾の /{unlock_permalink}/{page_hash}{index_suffix} の部分を \
/{downloads_permalink}/[ダウンロードコード]{index_suffix} に置き換えて \
Enter キーを押してください。"),
    unlock_permalink: Reviewed("unlock"),
    unmute: Reviewed("消音解除"),
    up_to_xxx: Reviewed("{xxx} 以下"),
    visual_impairment: Reviewed("視覚補助"),
    volume: Reviewed("音量"),
    xxx_and_others: Reviewed(r#"{xxx} と<a href="{others_link}">その他</a>"#),
    xxx_hours: Reviewed("{xxx} 時間"),
    xxx_minutes: Reviewed("{xxx} 分"),
    xxx_or_more: Reviewed("{xxx} 以上"),
    xxx_seconds: Reviewed("{xxx} 秒"),
    ..Translations::UNTRANSLATED
};
