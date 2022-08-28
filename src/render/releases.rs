use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{layout, releases}
};

pub fn releases_html(build: &Build, catalog: &Catalog) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let body = formatdoc!(
        r#"
            <header class="center" style="display: flex; align-items: center; margin-top: 0;">
                <a href=".{explicit_index}" style="color: #fff; font-size: 2em;">
                    {title}
                </a>
            </header>
            <div class="center">
                <div class="releases">
                    {releases}
                </div>
            </div>
        "#,
        explicit_index = explicit_index,
        releases = releases(explicit_index, root_prefix, &catalog.releases),
        title = catalog_title
    );

    layout(root_prefix, &body, build, catalog, &catalog_title)
}
