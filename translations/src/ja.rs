// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Translations, Unreviewed};

pub const JA: Translations = Translations {
    audio_format_alac: Unreviewed("可逆圧縮 – FLAC よりも Apple 製品の利用者に適した形式"),
    audio_format_average: Unreviewed("平均的な圧縮効率 – 対応した良い形式が他に無い場合に用いられる"),
    audio_format_flac: Unreviewed("可逆圧縮 – アーカイブに最適な形式"),
    audio_format_mp3: Unreviewed("非効率な圧縮 – 古い再生機器との互換性が高い形式"),
    audio_format_opus_48: Unreviewed("高効率な圧縮・低品質 – 空き容量が限られている場合に適した形式"),
    audio_format_opus_96: Unreviewed("高効率な圧縮・標準品質 – オフライン再生に適した形式"),
    audio_format_opus_128: Unreviewed("高効率な圧縮・高品質 – オフライン再生に最適な形式"),
    audio_format_uncompressed: Unreviewed("非圧縮 – ファイルが大きいため音楽制作にのみ用いられる"),
    audio_player_widget_for_xxx: Unreviewed("「{title}」の再生ウィジェット"),
    auto_generated_cover: Unreviewed("自動生成されたアートワーク"),
    available_formats: Unreviewed("利用可能な形式："),
    browse: Unreviewed("閲覧"),
    buy: Unreviewed("購入"),
    close: Unreviewed("閉じる"),
    copied: Unreviewed("コピーしました"),
    copy: Unreviewed("コピー"),
    copy_link: Unreviewed("リンクをコピー"),
    confirm: Unreviewed("確認"),
    r#continue: Unreviewed("続ける"),
    cover_image: Unreviewed("アートワーク"),
    default_unlock_info: Unreviewed("ダウンロードコードが必要です。コードの取得方法はサイト管理者にお尋ねください。"),
    download: Unreviewed("ダウンロード"),
    download_code_seems_incorrect: Unreviewed("ダウンロードコードが間違っています。入力ミスが無いか確かめてください。"),
    downloads: Unreviewed("ダウンロード"),
    downloads_permalink: Unreviewed("downloads"),
    embed: Unreviewed("埋め込み"),
    embed_entire_release: Unreviewed("このリリース全体の埋め込み"),
    enter_code_here: Unreviewed("コードをここに入力"),
    external_link: Unreviewed("外部リンク"),
    extras: Unreviewed("特典"),
    failed: Unreviewed("失敗しました"),
    feed: Unreviewed("フィード"),
    fixed_price: Unreviewed("定価："),
    image_descriptions: Unreviewed("画像の説明文"),
    image_descriptions_guide: Unreviewed("\
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
    image_descriptions_permalink: Unreviewed("image-descriptions"),
    javascript_is_disabled_listen_at_xxx: Unreviewed("JavaScript が無効です – {link} で再生する"),
    javascript_is_disabled_text: Unreviewed("JavaScript が無効なので、利用できない機能があります。"),
    listen: Unreviewed("聴く"),
    loading: Unreviewed("読み込み中"),
    m3u_playlist: Unreviewed("M3U プレイリスト"),
    made_or_arranged_payment: Unreviewed("既に支払いを行いました"),
    missing_image_description_note: Unreviewed("画像に説明文がありません<br>クリックして詳細を表示"),
    more: Unreviewed("もっと見る"),
    mute: Unreviewed("消音"),
    name_your_price: Unreviewed("金額を指定"),
    next_track: Unreviewed("次のトラック"),
    nothing_found_for_xxx: Unreviewed("「{query}」は見つかりませんでした"),
    pause: Unreviewed("一時停止"),
    play: Unreviewed("再生"),
    playback_position: Unreviewed("再生位置"),
    player_closed: Unreviewed("プレーヤーを閉じました"),
    player_open_playing_xxx: Unreviewed("プレーヤーを起動、「{title}」を再生中"),
    previous_track: Unreviewed("前のトラック"),
    purchase_downloads: Unreviewed("購入してダウンロードする"),
    purchase_permalink: Unreviewed("purchase"),
    recommended_format: Unreviewed("推奨の形式"),
    rss_feed: Unreviewed("RSS フィード"),
    search: Unreviewed("検索"),
    showing_featured_items: Unreviewed("注目の作品を表示中"),
    showing_xxx_results_for_xxx: Unreviewed("「{query}」の {count} 件の検索結果を表示中"),
    skip_to_main_content: Unreviewed("メインコンテンツにスキップ"),
    unlisted: Unreviewed("非表示"),
    unlock: Unreviewed("ロック解除"),
    unlock_downloads: Unreviewed("ダウンロードのロックを解除"),
    unlock_manual_instructions: Unreviewed("\
ダウンロードのロックを解除するには、ブラウザのアドレスバーに表示されている \
URL に以下の変更を加えてください。\
<br><br>\
URL が正しくない場合、404 エラーのページが表示されます。\
その場合にはブラウザの「戻る」ボタンを利用して再度操作を行ってください。\
<br><br>\
URL 末尾の /{unlock_permalink}/{page_hash}{index_suffix} の部分を \
/{downloads_permalink}/[ダウンロードコード]{index_suffix} に置き換えて \
Enter キーを押してください。"),
    unlock_permalink: Unreviewed("unlock"),
    unmute: Unreviewed("消音解除"),
    up_to_xxx: Unreviewed("{xxx} 以下"),
    visual_impairment: Unreviewed("視覚補助"),
    volume: Unreviewed("音量"),
    xxx_and_others: Unreviewed(r#"{xxx} と<a href="{others_link}">その他</a>"#),
    xxx_hours: Unreviewed("{xxx} 時間"),
    xxx_minutes: Unreviewed("{xxx} 分"),
    xxx_or_more: Unreviewed("{xxx} 以上"),
    xxx_seconds: Unreviewed("{xxx} 秒")
};
