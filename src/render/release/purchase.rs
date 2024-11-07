// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ops::Range;

use indoc::formatdoc;
use iso_currency::Currency;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    Release,
    Scripts
};
use crate::render::{compact_release_identifier, layout};
use crate::util::html_escape_outside_attribute;

pub fn purchase_html(
    build: &Build,
    catalog: &Catalog,
    currency: &Currency,
    payment_text: &str,
    range: &Range<f32>,
    release: &Release
) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let currency_code = currency.code();
    let currency_symbol = currency.symbol();

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
                       pattern="[0-9]+([.,][0-9])?"
                       placeholder="{placeholder}"
                       style="padding-left: 1.5rem; width: 8rem;"
                       type="text">
                {currency_code}
            </div>
            <br>
        "#)
    };

    let price_input_rendered = if range.end == f32::INFINITY {
        price_input(
            range,
            &build.locale.translations.xxx_or_more(&range.start.to_string())
        )
    } else if range.start == range.end {
        let t_fixed_price = &build.locale.translations.fixed_price;
        format!(
            "{t_fixed_price} {currency_symbol}{price} {currency_code}",
            price = &range.start
        )
    } else if range.start > 0.0 {
        price_input(
            range,
            &format!("{}-{}", range.start, range.end)
        )
    } else {
        price_input(
            range,
            &build.locale.translations.up_to_xxx(&range.end.to_string())
        )
    };

    let t_downloads_permalink = &build.locale.translations.downloads_permalink;
    let download_page_hash = build.hash_with_salt(&[&release.permalink.slug, t_downloads_permalink]);

    let formats = release.download_formats
        .iter()
        .map(|audio_format| audio_format.user_label())
        .collect::<Vec<&str>>()
        .join(", ");

    let t_available_formats = &build.locale.translations.available_formats;
    let t_confirm = &build.locale.translations.confirm;
    let t_continue = &build.locale.translations.r#continue;
    let t_made_or_arranged_payment = &build.locale.translations.made_or_arranged_payment;
    let content = formatdoc!(r#"
        <form action="{release_prefix}{t_downloads_permalink}/{download_page_hash}{index_suffix}"
              id="confirm">
            {price_input_rendered}
            <button>{t_confirm}</button>
            <div style="font-size: .9rem; margin: 1rem 0;">
                {t_available_formats} {formats}
            </div>
        </form>

        <script>
            document.querySelector('#confirm').addEventListener('submit', event => {{
                event.preventDefault();

                const priceField = event.target.price;
                if (priceField) {{
                    const max = priceField.dataset.max ? parseFloat(priceField.dataset.max) : null;
                    const min = priceField.dataset.min ? parseFloat(priceField.dataset.min) : null;
                    const price = parseFloat(priceField.value.replace(',', '.'));

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

                document.querySelector('#confirm').style.display = 'none';
                document.querySelector('.payment').classList.add('active');
            }});
        </script>

        <div class="payment">
            <div class="text">
                {payment_text}
            </div>

            <input autocomplete="off" id="confirm_payment" onchange="document.querySelector('#continue').classList.toggle('disabled', !this.checked)" type="checkbox"> <label for="confirm_payment">{t_made_or_arranged_payment}</label>

            <br><br>

            <a class="button disabled"
               href="{release_prefix}{t_downloads_permalink}/{download_page_hash}{index_suffix}"
               id="continue"
               onclick="if (!document.querySelector('#confirm_payment').checked) {{ event.preventDefault() }}">
                {t_continue}
            </a>
        </div>
    "#);

    let release_link = format!("../..{index_suffix}");

    let compact_release_identifier_rendered = compact_release_identifier(
        build,
        catalog,
        index_suffix,
        release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let t_purchase_downloads = &build.locale.translations.purchase_downloads;
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center page_100vh">
                <div style="max-width: 28rem;">
                    <h1>{t_purchase_downloads}</h1>
                    {compact_release_identifier_rendered}
                    {content}
                </div>
            </div>
        </div>
    "#);

    let release_title = &release.title;
    let release_title_escaped = html_escape_outside_attribute(release_title);
    let breadcrumb = Some(format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#));

    let page_title = format!("{t_purchase_downloads} – {release_title}");

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::None,
        &release.theme,
        &page_title,
        CrawlerMeta::NoIndexNoFollow,
        breadcrumb
    )
}
