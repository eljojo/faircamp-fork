use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    DownloadOption,
    PaymentOption,
    Release,
    render::{cover_image, layout, list_artists},
    util::html_escape_outside_attribute
};

pub fn checkout_html(build: &Build, catalog: &Catalog, release: &Release, download_page_uid: &str) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../../";

    let price = if let DownloadOption::Paid { currency, range, .. } = &release.download_option {
         if range.end == f32::INFINITY {
            if range.start > 0.0 {
                format!(
                    "{currency_symbol}{min_price} {currency_code} or more",
                    currency_code=currency.code(),
                    currency_symbol=currency.symbol(),
                    min_price=range.start
                )
            } else {
                format!("Name Your Price ({})", currency.code())
            }
        } else if range.start == range.end {
            format!(
                "{currency_symbol}{price} {currency_code}",
                currency_code=currency.code(),
                currency_symbol=currency.symbol(),
                price=range.start
            )
        } else if range.start > 0.0 {
            format!(
                "{currency_symbol}{min_price}-{currency_symbol}{max_price} {currency_code}",
                currency_code=currency.code(),
                currency_symbol=currency.symbol(),
                max_price=range.end,
                min_price=range.start
            )
        } else {
            format!(
                "Up to {currency_symbol}{max_price} {currency_code}",
                currency_code=currency.code(),
                currency_symbol=currency.symbol(),
                max_price=range.end
            )
        }
    } else {
        unreachable!();
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

                    formatdoc!(r#"
                        <div>
                            Pay on liberapay: <a href="{liberapay_url}">{liberapay_url}</a>
                        </div>
                    "#)
                }
            };

            formatdoc!(r#"
                <div>
                    <h2>Option {number}:</h2>
                    {option_rendered}
                </div>
            "#)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = format!(
        "{root_prefix}{permalink}/",
        permalink = release.permalink.slug
    );

    let body = formatdoc!(
        r#"
            <div class="center">
                <h1>Buy Release</h1>

                <br><br>

                <div class="cover_listing" style="max-width: 12rem">
                    {cover}
                </div>
                <div>{title}</div>
                <div>{artists}</div>

                <br><br>

                {price}

                <br><br>

                {payment_options}

                <br><br>

                <a href="{root_prefix}download/{permalink}/{download_page_uid}{explicit_index}">I have made the payment — Continue</a>
            </div>
        "#,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        permalink = release.permalink.slug,
        title = html_escape_outside_attribute(&release.title)
    );

    layout(root_prefix, &body, build, catalog, &release.title, None)
}
