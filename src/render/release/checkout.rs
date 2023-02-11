use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadOption,
    PaymentOption,
    Release,
    render::{compact_release_identifier, layout},
    util::html_escape_outside_attribute
};

pub fn checkout_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let index_suffix = build.index_suffix();
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let (content, heading) = if let DownloadOption::Paid(currency, range) = &release.download_option {
        let currency_code = currency.code();
        let currency_symbol = currency.symbol();
        let mut max = String::new();
        let mut min = String::new();

        let placeholder = if range.end == f32::INFINITY {
            if range.start > 0.0 {
                min = format!(r#"min="{}" "#, range.start);
                let min_price_formatted = format!(
                    "{currency_symbol}{min_price} {currency_code}",
                    min_price = range.start
                );

                build.locale.strings.xxx_or_more(&min_price_formatted)
            } else {
                let t_name_your_price = &build.locale.strings.name_your_price;
                format!("{t_name_your_price} ({currency_code})")
            }
        } else if range.start == range.end {
            min = format!(r#"min="{}" "#, range.start);
            max = format!(r#"max="{}" "#, range.end);
            format!(
                "{currency_symbol}{price} {currency_code}",
                price = range.start
            )
        } else if range.start > 0.0 {
            min = format!(r#"min="{}" "#, range.start);
            max = format!(r#"max="{}" "#, range.end);
            format!(
                "{currency_symbol}{min_price}-{max_price} {currency_code}",
                max_price = range.end,
                min_price = range.start
            )
        } else {
            max = format!(r#"max="{}" "#, range.end);
            let max_price_formatted = format!(
                "{currency_symbol}{max_price} {currency_code}",
                max_price = range.end
            );

            build.locale.strings.up_to_xxx(&max_price_formatted)
        };

        let price_input = format!(
            r#"<input {max}{min}placeholder="{placeholder}" step="any" type="number"> {currency_symbol}"#
        );

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

                        let t_pay_on_liberapay = &build.locale.strings.pay_on_liberapay;
                        formatdoc!(r#"
                            <div>
                                {t_pay_on_liberapay}<br>
                                <a href="{liberapay_url}">{liberapay_url}</a>
                            </div>
                        "#)
                    }
                };

                let t_option = &build.locale.strings.option;
                formatdoc!(r#"
                    <div style="align-items: center; display: flex; margin-bottom: 1rem;">
                        <div style="font-size: .8rem; margin-right: 1rem; white-space: nowrap;">{t_option} {number}</div>
                        {option_rendered}
                    </div>
                "#)
            })
            .collect::<Vec<String>>()
            .join("\n");

            let download_page_hash = build.hash_generic(&[&release.permalink.slug, "download"]);

            let t_confirm = &build.locale.strings.confirm;
            let t_continue = &build.locale.strings.r#continue;
            let t_made_or_arranged_payment = &build.locale.strings.made_or_arranged_payment;
            let content = formatdoc!(r#"
                <form id="confirm">
                    {price_input} <button name="confirm">{t_confirm}</button>
                </form>

                <script>
                    document.querySelector('#confirm').addEventListener('submit', event => {{
                        event.preventDefault();
                        document.querySelector('#confirm').style.display = 'none';
                        document.querySelector('.payment').classList.add('active');
                    }});
                </script>

                <div class="payment">
                    {payment_options}

                    <br><br>

                    <input id="confirm_payment" onchange="document.querySelector('#continue').classList.toggle('disabled', !this.checked)" type="checkbox"> <label for="confirm_payment">{t_made_or_arranged_payment}</label>

                    <br><br>

                    <a class="button disabled"
                       href="{release_prefix}download/{download_page_hash}{index_suffix}"
                       id="continue"
                       onclick="if (!document.querySelector('#confirm_payment').checked) {{ event.preventDefault() }}">
                        {t_continue}
                    </a>
                </div>
            "#);

            (content, build.locale.strings.buy_release.as_str())
    } else if let DownloadOption::Codes { unlock_text, .. } = &release.download_option {
        let custom_or_default_unlock_text = unlock_text
            .as_ref()
            .map(|text| text.to_string())
            .unwrap_or(build.locale.strings.default_unlock_text.clone());

        let t_enter_code_here = &build.locale.strings.enter_code_here;
        let t_unlock = &build.locale.strings.unlock;
        let t_unlock_code_seems_incorrect = &build.locale.strings.unlock_code_seems_incorrect;
        let t_unlock_manual_instructions = &build.locale.strings.unlock_manual_instructions(index_suffix);
        let content = formatdoc!(r#"
            <div class="unlock_scripted">
                {custom_or_default_unlock_text}

                <br><br>

                <form id="unlock">
                    <input autofocus class="unlock_code" placeholder="{t_enter_code_here}" type="text">
                    <button name="unlock">{t_unlock}</button>
                </form>
                <script>
                    document.querySelector('#unlock').addEventListener('submit', event => {{
                        event.preventDefault();
                        const code = document.querySelector('.unlock_code').value;
                        const url = `../../download/${{code}}{index_suffix}`;
                        fetch(url, {{ method: 'HEAD', mode: 'no-cors' }})
                            .then(response => {{
                                window.location = url;
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

        (content, build.locale.strings.enter_code.as_str())
    } else {
        unreachable!();
    };

    let release_link = format!("../..{index_suffix}");

    let compact_release_identifier_rendered = compact_release_identifier(
        &catalog,
        index_suffix,
        &release,
        &release_link,
        release_prefix,
        root_prefix,
    );

    let body = formatdoc!(r#"
        <div class="center_release">
            <h1>{heading}</h1>
            {compact_release_identifier_rendered}
            {content}
        </div>
    "#);

    let release_title_escaped = html_escape_outside_attribute(&release.title);
    let breadcrumbs = &[
        format!(r#"<a href="{release_link}">{release_title_escaped}</a>"#),
        format!("<span>{heading}</span>")
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}
