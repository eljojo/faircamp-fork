use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    artist::Artist,
    build::Build,
    catalog::Catalog,
    image_format::ImageFormat,
    localization::WritingDirection,
    release::Release,
    track::Track,
    util
};

pub mod about;
pub mod artist;
pub mod release;
pub mod releases;

const SHARE_WIDGET: &str = include_str!("templates/share_widget.html");

fn cover(root_prefix: &str, release: &Release) -> String {
    match &release.cover {
        Some(image) => format!(
            r#"<a href="{root_prefix}{filename}"><img alt="{alt}" src="{root_prefix}{filename}"></a>"#,
            alt = release.image_description.as_ref().unwrap_or(&String::from("Cover image of this release")),
            filename = image.borrow().get_as(&ImageFormat::Jpeg).as_ref().unwrap().filename,
            root_prefix = root_prefix
        ),
        None => String::from(r#"<div></div>"#)
    }
}

fn layout(root_prefix: &str, body: &str, build: &Build, catalog: &Catalog, title: &str) -> String {
    let (feed_meta_link, feed_user_link) = match &build.base_url.is_some() {
        true => (
            format!(
                r#"<link rel="alternate" type="application/rss+xml" title="RSS Feed" href="{root_prefix}feed.rss">"#,
                root_prefix = root_prefix
            ),
            format!(
                r#"<a href="{root_prefix}feed.rss">RSS</a>"#,
                root_prefix = root_prefix
            ),
        ),
        false => (String::new(), String::new())
    };

    let dir_attribute = match build.localization.writing_direction {
        WritingDirection::Ltr => "",
        WritingDirection::Rtl => "dir=\"rtl\""
    };

    let theming_widget = if build.theming_widget {
        include_str!("templates/theming_widget.html")
    } else {
        ""
    };

    format!(
        include_str!("templates/layout.html"),
        body = body,
        catalog_title = catalog.title(),
        dir_attribute = dir_attribute,
        feed_meta_link = feed_meta_link,
        feed_user_link = feed_user_link,
        root_prefix = root_prefix,
        theming_widget = theming_widget,
        title = title
    )
}

fn list_artists(root_prefix: &str, artists: &Vec<Rc<RefCell<Artist>>>) -> String {
    artists
        .iter()
        .map(|artist| {
            let artist_ref = artist.borrow();
            format!(
                r#"<a href="{root_prefix}{permalink}/">{name}</a>"#,
                name = artist_ref.name,
                permalink = artist_ref.permalink.get(),
                root_prefix = root_prefix
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn releases(root_prefix: &str, releases: Vec<&Release>) -> String {
    releases
        .iter()
        .map(|release| {
            let track_snippets = release.tracks
                .iter()
                .enumerate()
                .map(|(index, track)| waveform_snippet(track, index, 2.0))
                .collect::<Vec<String>>();

            formatdoc!(
                r#"
                    <div class="vpad" style="display: flex;">
                        <a class="cover_listing" href="{root_prefix}{permalink}/">
                            {cover}
                        </a>
                        <div>
                            <a class="large" href="{root_prefix}{permalink}/" style="color: #fff;">{title} <span class="runtime">{runtime}</span></a>
                            <div>{artists}</div>
                            <span class="">{track_snippets}</span>
                        </div>
                    </div>
                "#,
                artists = list_artists(root_prefix, &release.artists),
                cover = cover(root_prefix, release),
                permalink = release.permalink.get(),
                root_prefix = root_prefix,
                runtime = util::format_time(release.runtime),
                title = release.title,
                track_snippets = track_snippets.join("&nbsp;&nbsp;&nbsp;&nbsp;")
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn waveform_snippet(track: &Track, snippet_index: usize, track_duration_width_em: f32) -> String {
    let step = 1;

    if let Some(peaks) = &track.cached_assets.source_meta.peaks {
        let height = 10;
        let width = 50;
        let mut enumerate_peaks = peaks.iter().skip(width * 2).step_by(step).enumerate();

        let mut d = format!("M 0,{}", (1.0 - enumerate_peaks.next().unwrap().1) * height as f32);

        while let Some((index, peak)) = enumerate_peaks.next() {
            // if index > width { break; }

            if index % width == 0 {
                let command = format!(
                    r#"" /> <path class="levels_{snippet_index}" d="M 0,{y}"#,
                    snippet_index = snippet_index,
                    y = (1.0 - peak) * height as f32
                );

                d.push_str(&command);
            }

            let command = format!(
                " L {x},{y}",
                x = (index % width) * step,
                y = (1.0 - peak) * height as f32
            );

            d.push_str(&command);
        }

        formatdoc!(
            r##"
                <svg class="waveform"
                     preserveAspectRatio="none"
                     style="width: {track_duration_width_em}em;"
                     viewBox="0 0 {width} {height}"
                     xmlns="http://www.w3.org/2000/svg">
                    <style>
                        .levels_{snippet_index} {{
                            mix-blend-mode: screen;
                            stroke: hsl(var(--text-h), var(--text-s), var(--text-l), .1);
                            stroke-width: 2px;
                        }}
                    </style>
                    <path class="levels_{snippet_index}" d="{d}" />
                </svg>
            "##,
            d = d,
            height = height,
            snippet_index = snippet_index,
            track_duration_width_em = track_duration_width_em,
            width = width
        )
    } else {
        String::new()
    }
}
