// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::hash::Hash;
use std::ops::Range;

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    Price,
    Release,
    Track
};
use crate::util::html_escape_outside_attribute;

use super::Layout;
use super::compact_track_identifier;

/// Renders content for pages found under /[release_permalink]/[track_number]/[purchase_permalink]/[hash]/index.html
pub fn track_purchase_html(
    build: &Build,
    catalog: &Catalog,
    payment_info: &str,
    price: &Price,
    release: &Release,
    track: &Track,
    track_number: usize
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../../";
    let root_prefix = "../../../../";
    let track_prefix = "../../";

    let mut layout = Layout::new();

    layout.no_indexing();

    let currency_code = price.currency.code();
    let currency_symbol = price.currency.symbol();

    let price_display = |text: &str| {
        formatdoc!(r#"
            <div style="align-items: center; column-gap: .5rem; display: flex; position: relative;">
                {currency_symbol} {text} {currency_code}
            </div>
            <br>
        "#)
    };

    let price_input = |range: &Range<f32>, placeholder: &str| {
        let data_max = if range.end == f32::INFINITY {
            String::new()
        } else {
            format!(r#"data-max="{}""#, range.end)
        };
        let min = range.start;

        let t_name_your_price = &build.locale.translations.name_your_price;
        formatdoc!(r#"
            <label for="price">{t_name_your_price}</label><br><br>
            <div style="align-items: center; column-gap: .5rem; display: flex; position: relative;">
                <span style="position: absolute; left: .5rem;">{currency_symbol}</span>
                <input autocomplete="off"
                       {data_max}
                       data-min="{min}"
                       id="price"
                       pattern="[0-9]+([.,][0-9]+)?"
                       placeholder="{placeholder}"
                       style="padding-left: 1.5rem; width: 8rem;"
                       type="text">
                {currency_code}
            </div>
            <br>
        "#)
    };

    let r_price_display;
    let r_price_input;
    if price.range.end == f32::INFINITY {
        let placeholder = build.locale.translations.xxx_or_more(&price.range.start.to_string());
        r_price_display = price_display(&placeholder);
        r_price_input = price_input(&price.range, &placeholder);
    } else if price.range.start == price.range.end {
        let t_fixed_price = &build.locale.translations.fixed_price;
        let price = &price.range.start;
        let text = format!("{t_fixed_price} {currency_symbol}{price} {currency_code}");
        r_price_display = price_display(&text);
        r_price_input = text;
    } else if price.range.start > 0.0 {
        let placeholder = format!("{}-{}", price.range.start, price.range.end);
        r_price_display = price_display(&placeholder);
        r_price_input = price_input(&price.range, &placeholder);
    } else {
        let placeholder = build.locale.translations.up_to_xxx(&price.range.end.to_string());
        r_price_display = price_display(&placeholder);
        r_price_input = price_input(&price.range, &placeholder);
    }

    let t_downloads_permalink = &build.locale.translations.downloads_permalink;
    let download_page_hash = build.hash_with_salt(|hasher| {
        release.permalink.slug.hash(hasher);
        track_number.hash(hasher);
        t_downloads_permalink.hash(hasher);
    });

    let mut track_formats_sorted = track.download_formats.clone();
    track_formats_sorted.sort_by_key(|format| format.download_rank());

    let r_formats = track_formats_sorted
        .iter()
        .map(|audio_format| audio_format.user_label())
        .collect::<Vec<&str>>()
        .join(", ");

    let t_available_formats = &build.locale.translations.available_formats;
    let t_confirm = &build.locale.translations.confirm;
    let t_continue = &build.locale.translations.r#continue;
    let t_made_or_arranged_payment = &build.locale.translations.made_or_arranged_payment;
    let content = formatdoc!(r#"
        <div id="confirm_price">
            <div class="interactive">
                <form action="{track_prefix}{t_downloads_permalink}/{download_page_hash}{index_suffix}">
                    {r_price_input}
                    <button>{t_confirm}</button>
                </form>
            </div>
            <div class="non_interactive">
                {r_price_display}
            </div>
            <div style="font-size: .9rem; margin: 1rem 0;">
                {t_available_formats} {r_formats}
            </div>
        </div>
        <div class="payment">
            <div class="text">
                {payment_info}
            </div>

            <form action="{track_prefix}{t_downloads_permalink}/{download_page_hash}{index_suffix}">
                <input autocomplete="off" id="confirm_payment" required type="checkbox">
                <label for="confirm_payment">{t_made_or_arranged_payment}</label>
                <br><br>
                <button id="continue">
                    {t_continue}
                </button>
            </form>
        </div>
        <script>
            document.querySelector('#continue').classList.add('disabled');
            document.querySelector('#continue').addEventListener('click', () => {{
                if (!document.querySelector('#confirm_payment').checked) {{ event.preventDefault() }}
            }});

            document.querySelector('#confirm_price form').addEventListener('submit', event => {{
                event.preventDefault();

                const priceField = event.target.price;
                if (priceField) {{
                    const max = priceField.dataset.max ? parseFloat(priceField.dataset.max) : null;
                    const min = priceField.dataset.min ? parseFloat(priceField.dataset.min) : null;
                    const price = parseFloat(priceField.value.replace(',', '.'));

                    if (isNaN(price)) {{
                        // TODO: Localize (or preferably find way to avoid text)
                        // TODO: Render in interface itself (no alert)
                        alert('Please enter a price');
                        return;
                    }}

                    if (min !== null && price < min) {{
                        // TODO: Localize (or preferably find way to avoid text)
                        // TODO: Render in interface itself (no alert)
                        alert(`Minimum price is ${{min}}`);
                        return;
                    }}

                    if (max !== null && price > max) {{
                        // TODO: Localize (or preferably find way to avoid text)
                        // TODO: Render in interface itself (no alert)
                        alert(`Maximum price is ${{max}}`);
                        return;
                    }}

                    if (price === 0) {{
                        location.href = event.target.action;
                        return;
                    }}
                }}

                document.querySelector('#confirm_price').style.display = 'none';
                document.querySelector('.payment').classList.add('active');
            }});

            document.querySelector('#confirm_payment').addEventListener('change', () => {{
                document.querySelector('#continue').classList.toggle('disabled', !document.querySelector('#confirm_payment').checked)
            }});
        </script>
    "#);

    let track_link = format!("../..{index_suffix}");

    let r_compact_track_identifier = compact_track_identifier(
        build,
        catalog,
        index_suffix,
        release,
        release_prefix,
        root_prefix,
        track,
        &track_link,
        track_prefix
    );

    let t_purchase_downloads = &build.locale.translations.purchase_downloads;
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center">
                <div style="max-width: 28rem;">
                    <h1>{t_purchase_downloads}</h1>
                    {r_compact_track_identifier}
                    {content}
                </div>
            </div>
        </div>
    "#);

    let release_link = format!("../../..{index_suffix}");
    let release_title_escaped = html_escape_outside_attribute(&release.title);

    layout.add_breadcrumb(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let track_title = track.title();
    let page_title = format!("{t_purchase_downloads} – {track_title}");

    layout.render(
        &body,
        build,
        catalog,
        root_prefix,
        &track.theme,
        &page_title
    )
}
