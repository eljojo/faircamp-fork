use indoc::formatdoc;

pub struct Theme {
    pub background_lightness_percent: u32,
    pub cover_lightness_percent: u32,
    pub link_hover_lightness_percent: u32,
    pub link_hue_degrees: u32,
    pub link_lightness_percent: u32,
    pub link_saturation_percent: u32,
    pub muted_lightness_percent: u32,
    pub overlay_alpha_factor: f32,
    pub text_lightness_percent: u32
}

pub const DARK_THEME: Theme = Theme {
    background_lightness_percent: 10,
    cover_lightness_percent: 13,
    link_hover_lightness_percent: 82,
    link_hue_degrees: 198,
    link_lightness_percent: 68,
    link_saturation_percent: 62,
    muted_lightness_percent: 23,
    overlay_alpha_factor: 0.5,
    text_lightness_percent: 86
};

pub const LIGHT_THEME: Theme = Theme {
    background_lightness_percent: 90,
    cover_lightness_percent: 87,
    link_hover_lightness_percent: 42,
    link_hue_degrees: 198,
    link_lightness_percent: 42,
    link_saturation_percent: 100,
    muted_lightness_percent: 68,
    overlay_alpha_factor: 0.5,
    text_lightness_percent: 14
};

pub fn generate(theme: &Theme, background: bool) -> String {
    let background_override = if background {
        formatdoc!(r#"
            body {{
                background:
                    linear-gradient(
                        hsla(0deg, 0%, var(--background-lightness), var(--overlay-alpha)),
                        hsla(0deg, 0%, var(--background-lightness), var(--overlay-alpha))
                    ),
                    url(background.jpg) center / cover;
            }}
        "#)
    } else {
        String::new()
    };
    
    formatdoc!(
        r#"
            :root {{
                --background-lightness: {background_lightness_percent}%;
                --cover-lightness: {cover_lightness_percent}%;
                --link-hover-lightness: {link_hover_lightness_percent}%;
                --link-hue: {link_hue_degrees}deg;
                --link-lightness: {link_lightness_percent}%;
                --link-saturation: {link_saturation_percent}%;
                --muted-lightness: {muted_lightness_percent}%;
                --overlay-alpha: {overlay_alpha_factor};
                --text-lightness: {text_lightness_percent}%;
            }}
            {included_static_css}
            {background_override}
        "#,
        background_lightness_percent=theme.background_lightness_percent,
        cover_lightness_percent=theme.cover_lightness_percent,
        link_hover_lightness_percent=theme.link_hover_lightness_percent,
        link_hue_degrees=theme.link_hue_degrees,
        link_lightness_percent=theme.link_lightness_percent,
        link_saturation_percent=theme.link_saturation_percent,
        muted_lightness_percent=theme.muted_lightness_percent,
        overlay_alpha_factor=theme.overlay_alpha_factor,
        text_lightness_percent=theme.text_lightness_percent,
        included_static_css=include_str!("assets/styles.css"),
        background_override=background_override
    )
}