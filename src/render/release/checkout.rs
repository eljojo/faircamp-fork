// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ops::Range;

use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    CrawlerMeta,
    DownloadOption,
    PaymentOption,
    Release
};
use crate::icons;
use crate::render::{compact_release_identifier, layout};
use crate::util::html_escape_outside_attribute;

pub fn checkout_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let (
        content,
        breadcrumb_heading,
        heading,
        icon
    ) = if let DownloadOption::Paid(currency, range) = &release.download_option {
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

        let payment_options = &release.payment_options
            .iter()
            .enumerate()
            .map(|(index, option)| {
                let number = index + 1;

                let option_rendered = match &option {
                    PaymentOption::Custom(message) => {
                        formatdoc!(r#"
                            <div>{message}</div>
                        "#)
                    },
                    PaymentOption::Liberapay(account_name) => {
                        let liberapay_url = format!("https://liberapay.com/{}", account_name);

                        let t_pay_on_liberapay = &build.locale.translations.pay_on_liberapay;
                        formatdoc!(r#"
                            <div>
                                {t_pay_on_liberapay}<br>
                                <a href="{liberapay_url}">{liberapay_url}</a>
                            </div>
                        "#)
                    }
                };

                let t_option = &build.locale.translations.option;
                formatdoc!(r#"
                    <div style="align-items: center; display: flex; margin-bottom: 1rem;">
                        <div style="font-size: .8rem; margin-right: 1rem; white-space: nowrap;">{t_option} {number}</div>
                        {option_rendered}
                    </div>
                "#)
            })
            .collect::<Vec<String>>()
            .join("\n");

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
        let t_payment_options = &build.locale.translations.payment_options;
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
                {t_payment_options}

                {payment_options}

                <br><br>

                <input id="confirm_payment" onchange="document.querySelector('#continue').classList.toggle('disabled', !this.checked)" type="checkbox"> <label for="confirm_payment">{t_made_or_arranged_payment}</label>

                <br><br>

                <a class="button disabled"
                   href="{release_prefix}{t_downloads_permalink}/{download_page_hash}{index_suffix}"
                   id="continue"
                   onclick="if (!document.querySelector('#confirm_payment').checked) {{ event.preventDefault() }}">
                    {t_continue}
                </a>
            </div>
        "#);

        let buy_icon = icons::buy(&build.locale.translations.buy); // TODO: "Purchase" ? (also everywhere else this occurs and in code then of course too)

        (
            content,
            build.locale.translations.downloads.as_str(),
            build.locale.translations.purchase_downloads.as_str(),
            buy_icon
        )
    } else if let DownloadOption::Codes { unlock_text, .. } = &release.download_option {
        let custom_or_default_unlock_text = unlock_text
            .as_ref()
            .map(|text| text.to_string())
            .unwrap_or(build.locale.translations.default_unlock_text.clone());

        let t_unlock_permalink = &build.locale.translations.unlock_permalink;
        let page_hash = build.hash_with_salt(&[&release.permalink.slug, t_unlock_permalink]);

        let t_downloads_permalink = &build.locale.translations.downloads_permalink;
        let t_enter_code_here = &build.locale.translations.enter_code_here;
        let t_unlock = &build.locale.translations.unlock;
        let t_unlock_code_seems_incorrect = &build.locale.translations.unlock_code_seems_incorrect;
        let t_unlock_manual_instructions = &build.locale.translations.unlock_manual_instructions(&page_hash, index_suffix);
        let content = formatdoc!(r#"
            <div class="unlock_scripted">
                {custom_or_default_unlock_text}

                <br><br>

                <form id="unlock">
                    <input class="unlock_code" placeholder="{t_enter_code_here}" type="text">
                    <button name="unlock">{t_unlock}</button>
                </form>
                <script>
                    document.querySelector('#unlock').addEventListener('submit', event => {{
                        event.preventDefault();
                        const code = document.querySelector('.unlock_code').value;
                        const url = `../../{t_downloads_permalink}/${{code}}{index_suffix}`;
                        fetch(url, {{ method: 'HEAD', mode: 'no-cors' }})
                            .then(response => {{
                                if (response.ok) {{
                                    window.location = url;
                                }} else {{
                                    alert('{t_unlock_code_seems_incorrect}');
                                }}
                            }})
                            .catch(error => {{
                                alert('{t_unlock_code_seems_incorrect}');
                            }});
                    }});
                </script>
            </div>
            <div class="unlock_manual">
                {t_unlock_manual_instructions}
            </div>
        "#);

        let unlock_icon = icons::unlock(&build.locale.translations.unlock);

        (
            content,
            build.locale.translations.downloads.as_str(),
            build.locale.translations.unlock_downloads.as_str(),
            unlock_icon
        )
    } else {
        unreachable!();
    };

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

    let body = formatdoc!(r#"
        <div class="vcenter_page_outer">
            <div class="hcenter_narrow mobile_hpadding vcenter_page vpad_adaptive">
                <h1>{heading}</h1>
                {compact_release_identifier_rendered}
                {content}
            </div>
        </div>
    "#);

    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let breadcrumbs = &[
        format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#),
        format!(r#"<a href=".{index_suffix}">{icon} {breadcrumb_heading}</a>"#)
    ];

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        &release.theme,
        &release.title,
        breadcrumbs,
        CrawlerMeta::NoIndexNoFollow
    )
}
