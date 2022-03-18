use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{layout, releases}
};

pub fn releases_html(build: &Build, catalog: &Catalog) -> String {
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let body = formatdoc!(
        r#"
            <header class="center" style="display: flex; align-items: center; margin-top: 0;">
                <a href="" style="color: #fff; font-size: 2em;">
                    {title}
                </a>
            </header>
            <div class="center">
                <div class="releases">
                    {releases}
                </div>
            </div>
        "#,
        releases = releases(root_prefix, catalog.releases.iter().collect()),
        title = catalog_title
    );

    layout(root_prefix, &body, build, catalog, &catalog_title)
}
