use indoc::formatdoc;

use crate::{Build, Catalog, render::layout};

pub fn image_descriptions_html(build: &Build, catalog: &Catalog) -> String {
    let root_prefix = "../";

    let t_image_descriptions = &build.locale.translations.image_descriptions;
    let t_image_descriptions_guide = &build.locale.translations.image_descriptions_guide;
    
    let body = formatdoc!(r#"
        <div class="hcenter_wide mobile_hpadding vcenter_generic vpad_adaptive">
            <div class="vpad">
                <h1>{t_image_descriptions}</h1>
            </div>
            <div class="vpad">
                {t_image_descriptions_guide}
            </div>
        </div>
    "#);

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        t_image_descriptions,
        &[]
    )
}
