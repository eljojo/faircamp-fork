use indoc::formatdoc;

use crate::{
    build::Build,
    catalog::Catalog,
    payment_option::PaymentOption,
    release::Release,
    render::{image, layout, list_artists}
};

pub fn checkout_html(build: &Build, catalog: &Catalog, release: &Release, download_page_uid: &str) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../";

    let payment_options = &release.payment_options
        .iter()
        .map(|option|
            match &option {
                PaymentOption::Custom(html) => {
                    format!(
                        r#"
                            <div>
                                <div>{message}</div>
                                <a href="../../download/{download_page_uid}{explicit_index}">I have made the payment — Continue</a>
                            </div>
                        "#,
                        download_page_uid=download_page_uid,
                        explicit_index = explicit_index,
                        message=html.to_string()
                    )
                },
                PaymentOption::Liberapay(account_name) => {
                    let liberapay_url = format!("https://liberapay.com/{}", account_name);

                    format!(
                        r#"
                            <div>
                                <div>
                                    Pay on liberapay: <a href="{liberapay_url}">{liberapay_url}</a>
                                </div>
                                <a href="../../download/{download_page_uid}{explicit_index}">I have made the payment — Continue</a>
                            </div>
                        "#,
                        download_page_uid=download_page_uid,
                        explicit_index = explicit_index,
                        liberapay_url=liberapay_url
                    )
                }
            }
        )
        .collect::<Vec<String>>()
        .join("\n");

    let body = formatdoc!(
        r#"
            {cover}

            <h1>Buy {title}</h1>
            <div>{artists}</div>

            {payment_options}
        "#,
        artists = list_artists(explicit_index, root_prefix, &release.artists),
        payment_options = payment_options,
        cover = image(root_prefix, &release.cover),
        title = release.title
    );

    layout(root_prefix, &body, build, catalog, &release.title)
}
