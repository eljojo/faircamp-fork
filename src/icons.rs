//! Note that the svg files in `icons/` are not actually included anywhere,
//! their contents are copy-pasted inside the `generate` function inside this
//! module. The reason the files exist is to allow conveniently modifying the
//! icons with an editor (e.g. inkscape) and then copying the code, without
//! having to create the files first.

use indoc::formatdoc;
use std::fs;

use crate::Build;

pub fn generate(build: &Build) {
    let pane_color = format!(
        "hsl(0, 0%, {l}%)",
        l = build.theme.base.pane_l
    );

    let text_color = format!(
        "hsl(0, 0%, {l}%)",
        l = build.theme.base.text_l
    );

    if build.base_url.is_some() {   
        let feed_svg = formatdoc!(
            r#"
                <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                    <g fill="{text_color}">
                        <path d="m41.597 22.351c-8.6734-8.6734-20.213-13.45-32.496-13.45v9.0364c9.8677 0 19.139 3.8367 26.106 10.803 6.9666 6.9667 10.803 16.238 10.803 26.106h9.0364c-1.5e-4 -12.281-4.7769-23.823-13.45-32.496z"/>
                        <path d="m9.0076 24.192v9.0364c11.92 0 21.618 9.6975 21.618 21.618h9.0364c0-16.902-13.751-30.654-30.654-30.654z"/>
                        <circle cx="15.423" cy="48.625" r="6.4721"/>
                    </g>
                </svg>
            "#,
            text_color = text_color
        );

        fs::write(build.build_dir.join("feed.svg"), feed_svg).unwrap();
    }

    if build.missing_image_descriptions {
        let visual_impairment_svg = formatdoc!(
            r#"
                <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                    <path d="m53.433 8.1297-6.9884 6.9604c-3.8566-2.0177-8.5209-3.1971-14.444-3.1971-16.47 1e-6 -23.223 9.2695-29.821 20.107 3.5336 5.9261 7.1335 11.278 12.256 14.971l-6.391 6.3656 2.5237 2.5331 6.9864-6.9591 4.444-4.426 2.3432-2.3332 31.614-31.489zm-21.433 11.103a12.768 12.768 0 0 1 7.6545 2.621l-17.807 17.735a12.768 12.768 0 0 1-2.615-7.5886 12.768 12.768 0 0 1 12.767-12.768zm20.347 0.07327-8.3572 8.3232a12.768 12.768 0 0 1 0.77797 4.3714 12.768 12.768 0 0 1-12.768 12.767 12.768 12.768 0 0 1-4.3921-0.82193l-6.5162 6.4902c3.107 1.0724 6.6877 1.6718 10.908 1.6718 16.47 2e-6 23.223-9.2695 29.821-20.107-2.8599-4.7962-5.7582-9.2224-9.4742-12.695z" fill="{text_color}"/>
                </svg>
            "#,
            text_color = text_color
        );

        fs::write(build.build_dir.join("visual_impairment.svg"), visual_impairment_svg).unwrap();
    }

    let pause_svg = formatdoc!(
        r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <path d="m36.915 56.995v-49.985h15.592v49.985zm-25.421 0v-49.985h15.592v49.985z" fill="{text_color}"/>
            </svg>
        "#,
        text_color = text_color
    );

    fs::write(build.build_dir.join("pause.svg"), pause_svg).unwrap();

    let play_svg = formatdoc!(
        r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <path d="m11.116 59.677 45.476-26.453-45.544-28.093z" fill="{text_color}"/>
            </svg>
        "#,
        text_color = text_color
    );

    fs::write(build.build_dir.join("play.svg"), play_svg).unwrap();    

    let corner_tag_svg = formatdoc!(
        r#"
            <svg width="64" height="64" version="1.1" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
                <path d="m64 64v-64l-64 64z" fill="{pane_color}"/>
            </svg>
        "#,
        pane_color = pane_color
    );

    fs::write(build.build_dir.join("corner_tag.svg"), corner_tag_svg).unwrap();
}