use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::layout,
    util::html_escape_outside_attribute
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
            </div>
        "#,
        text = html_escape_outside_attribute(text),
        title = html_escape_outside_attribute(&catalog_title)
    );

    layout(root_prefix, &body, build, catalog, &catalog_title)
}
