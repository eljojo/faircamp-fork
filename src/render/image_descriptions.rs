use indoc::formatdoc;

use crate::{
    build::Build,
    catalog::Catalog,
    render::layout
};

const TITLE: &str = "Image Descriptions";

pub fn image_descriptions_html(build: &Build, catalog: &Catalog) -> String {
    let root_prefix = "../";
    
    let body = formatdoc!(
        r#"
            <div class="center">
                <div class="vpad">
                    <h1>{title}</h1>
                </div>

                <div class="vpad">
                
                    Millions of people browse the web using screen-readers
                    because they can not see (or not well enough). Images
                    without textual descriptions are inaccessible to them,
                    and this is why we should make the effort to provide
                    image descriptions for them.<br><br>

                    Consult the faircamp README for how to add image
                    descriptions, it's simple and an act of
                    kindness.<br><br>

                    Here are some tips for writing good image descriptions:<br>
                    - Any description is better than having no description, don't worry about doing it wrong.<br>
                    - Make it concise. Write as much as needed, but at the same time keep it as short as possible.<br>
                    - Don't interpret. Describe what is there and relevant for its understanding, don't analyze beyond that.<br>
                    - You can use colors where it makes sense - many people only lost their sight later on and understand and appreciate colors.<br>
                </div>
            </div>
        "#,
        title = TITLE
    );

    layout(root_prefix, &body, build, catalog, TITLE)
}
