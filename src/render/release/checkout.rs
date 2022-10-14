use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    PaymentOption,
    Release,
    render::{cover_image, layout, list_artists},
    util::html_escape_outside_attribute
};

pub fn checkout_html(build: &Build, catalog: &Catalog, release: &Release, download_page_uid: &str) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../../../";

    let payment_options = &release.payment_options
        .iter()
        .map(|option|
            match &option {
                PaymentOption::Custom(message) => {
                    formatdoc!(
                        r#"
                            <div>
                                <div>{message}</div>
                                <a href="{root_prefix}download/{permalink}/{download_page_uid}{explicit_index}">I have made the payment — Continue</a>
                            </div>
                        "#,
                        permalink = release.permalink.slug
                    )
                },
                PaymentOption::Liberapay(account_name) => {
                    let liberapay_url = format!("https://liberapay.com/{}", account_name);

                    formatdoc!(
                        r#"
                            <div>
                                <div>
                                    Pay on liberapay: <a href="{liberapay_url}">{liberapay_url}</a>
                                </div>
                                <a href="{root_prefix}download/{permalink}/{download_page_uid}{explicit_index}">I have made the payment — Continue</a>
                            </div>
                        "#,
                        permalink = release.permalink.slug
                    )
                }
            }
        )
        .collect::<Vec<String>>()
        .join("\n");

    let release_prefix = format!(
        "{root_prefix}{permalink}/",
        permalink = release.permalink.slug
    );

    let body = formatdoc!(
        r#"
            {cover}

            <h1>Buy {title}</h1>
            <div>{artists}</div>

            {payment_options}
        "#,
        artists = list_artists(explicit_index, root_prefix, &catalog, &release.artists),
        payment_options = payment_options,
        cover = cover_image(explicit_index, &release_prefix, root_prefix, &release.cover, None),
        title = html_escape_outside_attribute(&release.title)
    );

    layout(root_prefix, &body, build, catalog, &release.title, None)
}
