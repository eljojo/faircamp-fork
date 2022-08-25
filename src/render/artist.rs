use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    Artist,
    Build,
    Catalog,
    ImageFormat,
    Release,
    render::{SHARE_WIDGET, image, layout, releases}
};

pub fn artist_html(build: &Build, artist: &Rc<RefCell<Artist>>, catalog: &Catalog) -> String {
    let explicit_index = if build.clean_urls { "/" } else { "/index.html" };
    let root_prefix = "../";

    // TODO: Possibly prepare these associations earlier, when mapping artists to releases based on artists_to_map
    let artist_releases = catalog.releases
        .iter()
        .filter(|release| {
            release.artists
                .iter()
                .find(|release_artist| Rc::ptr_eq(release_artist, artist))
                .is_some()
        })
        .collect::<Vec<&Release>>();

    let artist_ref = artist.borrow();

    let text = if let Some(text) = &artist_ref.text {
        formatdoc!(
            r#"
                <div class="vpad">
                    {text}
                </div>
            "#,
            text = text
        )
    } else {
        String::new()
    };

    let body = formatdoc!(
        r#"
            <div class="center">
                <div class="cover">
                    {artist_image}
                </div>

                <div class="vpad">
                    <h1><a href="{root_prefix}.{explicit_index}">All Releases</a> &gt; {artist_name}</h1>
                </div>

                {text}

                {share_widget}

                {releases}
            </div>
        "#,
        artist_image = image(explicit_index, root_prefix, &artist_ref.image, ImageFormat::Artist),
        artist_name = artist_ref.name,
        explicit_index = explicit_index,
        releases = releases(explicit_index, root_prefix, artist_releases),
        root_prefix = root_prefix,
        share_widget = SHARE_WIDGET,
        text = text
    );

    layout(root_prefix, &body, build, catalog, &artist_ref.name)
}
