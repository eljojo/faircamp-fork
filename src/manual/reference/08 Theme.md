# Theme

With this you can adjust the visual appearance of your faircamp site.

Note that the theme applies across all pages - customizations on individual
pages are not available.

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

## Detail color adjustments

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

## Disabling relative waveform lengths

By default, the width of each track's waveform on a release page will render
at a different length, reflecting the duration of the track in relation to
the longest track on the release - for instance if the longest track on a
release is about two minutes long, that one will span the full width, but
another track that is only about one minute long will span only half of that
width. If you publish releases whose tracks have wildly varying lengths,
shorter tracks might get very narrow in the interface. If this is a concern
to you, or you just generally want all tracks to be full-width as an
aesthetic choice, you can enable this alternative behavior with this flag:

```eno
# theme

disable_relative_waveforms
```

## Font

By default, faircamp bundles and uses the [Barlow](https://tribby.com/fonts/barlow/)
font on a generated site, but this can be configured.

Using the standard sans serif font from the system of the visitor:

```eno
# theme

system_font: sans
```

Using the standard monospace font from the system of the visitor:

```eno
# theme

system_font: mono
```

Using a specific font (by font name) from the system of the visitor (this should have a rather specific reason, normally you probably don't want to do that):

```eno
# theme

system_font: Arial
```

Bundling and using a custom font (put a `.woff` or `.woff2` file in the same directory as the manifest - other font file types are not supported!):

```eno
# theme

custom_font: MyCustomSans.woff2
```
