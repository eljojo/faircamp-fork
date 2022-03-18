use indoc::formatdoc;
use std::rc::Rc;

use crate::{
    artist::Artist,
    build::Build,
    catalog::Catalog,
    release::Release,
    render::{SHARE_WIDGET, layout, releases}
};

pub fn artist_html(build: &Build, artist: &Rc<Artist>, catalog: &Catalog) -> String {
    let root_prefix = &"../".repeat(1);

    let artist_releases = catalog.releases
        .iter()
        .filter(|release| {
            release.artists
                .iter()
                .find(|release_artist| Rc::ptr_eq(release_artist, artist))
                .is_some()
        })
        .collect::<Vec<&Release>>();

    let text = if let Some(text) = &artist.text {
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
                <!-- TODO: Artist image -->

                <div class="vpad">
                    <h1><a href="{root_prefix}">All Releases</a> &gt; {artist_name}</h1>
                </div>

                {text}

                {share_widget}

                {releases}
            </div>
        "#,
        artist_name = artist.name,
        releases = releases(root_prefix, artist_releases),
        root_prefix = root_prefix,
        share_widget = SHARE_WIDGET,
        text = text
    );

    layout(root_prefix, &body, build, catalog, &artist.name)
}
