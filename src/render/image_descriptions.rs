// SPDX-FileCopyrightText: 2022-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use indoc::formatdoc;

use crate::{Build, Catalog, CrawlerMeta, Scripts};
use crate::render::layout;

pub fn image_descriptions_html(build: &Build, catalog: &Catalog) -> String {
    let root_prefix = "../";

    let t_image_descriptions = &build.locale.translations.image_descriptions;
    let t_image_descriptions_guide = &build.locale.translations.image_descriptions_guide;
    
    let body = formatdoc!(r#"
        <div class="page">
            <div class="page_center page_90vh">
                <div style="max-width: 28rem;">
                    <h1 style="margin-bottom: 2rem;">{t_image_descriptions}</h1>
                    {t_image_descriptions_guide}
                </div>
            </div>
        </div>
    "#);

    layout(
        root_prefix,
        &body,
        build,
        catalog,
        Scripts::None,
        &catalog.theme,
        t_image_descriptions,
        CrawlerMeta::NoIndexNoFollow,
        None
    )
}
