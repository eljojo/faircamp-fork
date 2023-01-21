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

pub fn checkout_html(build: &Build, catalog: &Catalog, release: &Release) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let release_prefix = "../../";
    let root_prefix = "../../../";

    let (content, heading) = if let DownloadOption::Paid(currency, range) = &release.download_option {
         let price = if range.end == f32::INFINITY {
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

            let download_page_hash = build.hash_generic(&[&release.permalink.slug, "download"]);

            let content = formatdoc!(r#"
                {price}

                <br><br>

                {payment_options}

                <br><br>

                <a href="{release_prefix}download/{download_page_hash}{explicit_index}">I have made the payment — Continue</a>
            "#);

            (content, "Buy Release")
    } else if let DownloadOption::Codes(_) = &release.download_option {
        let content = formatdoc!(r#"
            <div class="unlock_scripted">
                Downloads for this release are available by entering an unlock
                code. If you don't already have a code you need to obtain one
                from the artists/people who run this site - get in touch with
                them or see if there's any information on the release page
                itself. Download codes may sometimes be offered as perks on
                crowdfunding campaigns or subscriptions, so also check these
                if you know of any!

                <!-- TODO: Alternative text configurable by site owner -->

                <br><br>

                <form id="unlock">
                    <input autofocus class="unlock_code" placeholder="Enter code here" type="text">
                    <button name="unlock">Unlock</button>
                </form>
                <script>
                    document.querySelector('#unlock').addEventListener('submit', event => {{
                        event.preventDefault();
                        const code = document.querySelector('.unlock_code').value;
                        const url = `../../download/${{code}}{explicit_index}`;
                        fetch(url, {{ method: 'HEAD', mode: 'no-cors' }})
                            .then(response => {{
                                window.location = url;
                            }})
                            .catch(error => {{
                                alert('The unlock code seems to be incorrect, please check for typos.');
                            }});
                    }});
                </script>
            </div>
            <div class="unlock_manual">
                To unlock the download, please make the below described
                changes to the address in your browser's adress bar.

                <br><br>

                Before you embark on this please be aware that wrong codes or
                address modifications take you to a 404 page. In this case
                use the Back button and closely follow the instructions again.

                <br><br>

                Replace the final part of the address that
                looks like this - /checkout/[some-random-letters]{explicit_index} -
                with /download/[your-unlock-code]{explicit_index} and then press Enter.
            </div>
        "#);

        (content, "Enter Code")
    } else {
        unreachable!();
    };

    let artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists);
    let cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None);
    let release_title_escaped = html_escape_outside_attribute(&release.title);

    let body = formatdoc!(r#"
        <div class="center">
            <h1>{heading}</h1>

            <br><br>

            <div class="cover_listing" style="max-width: 12rem">
                {cover}
            </div>
            <div>{release_title_escaped}</div>
            <div>{artists}</div>

            <br><br>

            {content}
        </div>
    "#);

    let breadcrumbs = &[
        format!(r#"<a href="../..{explicit_index}">{release_title_escaped}</a>"#),
        format!("<span>{heading}</span>")
    ];

    layout(root_prefix, &body, build, catalog, &release.title, breadcrumbs)
}
