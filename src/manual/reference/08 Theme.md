# Theme

With this you can adjust the visual appearance of your faircamp site.

## Base

```eno
# theme

base: light
```

This sets the overall tone, think of it as a base preset. Choose from:

- `black`
- `black_alternate`
- `dark`
- `light`
- `white`
- `white_alternate`

If you don't set a base theme, the current default is `dark`.

## Detail adjustments

```eno
# theme

link_hue: 13
link_saturation: 100
link_lightness: 75
tint_back: 50
tint_front: 10
text_hue: 89
```

The link color can be set through `link_hue` (0-360), `link_lightness`
(0-100) and `link_saturation` (0-100) - make sure to pick a color that
clearly differs from regular text and is easily readable against the site's
background.

If you increase `tint_back` (0-100), the link color will give a color tone to
the entire page (except for the black/white base themes, one does not simply
tone black/white). Similarly, `tint_front` applies color tinting to regular
text, but the hue for this comes from `text_hue` (not from `link_hue`). 

> Note that there is a `--theming-widget` CLI option that lets you interactively
> explore these detail settings. Just build your catalog with the option enabled and
> every page will then contain the theming widget (don't forget to turn it off
> before deployment).

## Background image

The previously described settings are (link color aside) mostly harmless: No
matter the settings, your site will stay readable. Not so with this option. A
background image can lend a nice quality to your site design, but choose
carefully what image you use and how opaque you make it. Sharp details and
strong contrasts within the image and against the text of the site will
render the site hard to read or even unreadable. That said,
`background_image` lets you reference the image to use, and with
`background_alpha` (0-100) you can optionally control its opaqueness.

```eno
# theme

background_alpha: 23
background_image: squiggly_monsters_texture.jpg
```