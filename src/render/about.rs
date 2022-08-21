use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{SHARE_WIDGET, layout}
};

pub fn about_html(build: &Build, catalog: &Catalog) -> String {
    let root_prefix = "../";
    
    let catalog_title = catalog.title();

    let text = catalog.text
        .as_ref()
        .map(|title| title.as_str())
        .unwrap_or("");

    let body = formatdoc!(
        r#"
            <div class="center">
                <div class="vpad">
                    <h1>{title}</h1>
                </div>

                <div class="vpad">
                    {text}
                </div>

                {share_widget}
            </div>
        "#,
        share_widget = SHARE_WIDGET,
        text = text,
        title = catalog_title
    );

    layout(root_prefix, &body, build, catalog, &catalog_title)
}
