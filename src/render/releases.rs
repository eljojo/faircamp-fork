use indoc::formatdoc;

use crate::{
    Build,
    Catalog,
    render::{layout, releases},
    util::html_escape_outside_attribute
};

pub fn releases_html(build: &Build, catalog: &Catalog) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "";
    
    let catalog_title = catalog.title();

    let body = formatdoc!(
        r#"
            <div class="releases">
                {releases}
            </div>
            <div style="position: absolute; bottom: 0; padding; 3em; width: 100%; background-color: black;">
                <a href=".{explicit_index}" style="color: #fff;">
                    {title}
                </a>
            </div>
        "#,
        explicit_index = explicit_index,
        releases = releases(explicit_index, root_prefix, &catalog.releases),
        title = html_escape_outside_attribute(&catalog_title)
    );

    layout(root_prefix, &body, build, catalog, &catalog_title)
}
