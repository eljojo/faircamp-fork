<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Theme

With this you can adjust the visual appearance of your faircamp site.

Theme customizations can be made in a top-level manifest at the root of the
catalog (setting the theme for the homepage and all release pages), but
they can also be made locally for a group of releases or for each release
on its own (added in 0.15.0).

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

accent_brightening: 85
accent_chroma: 50
accent_hue: 23
base_chroma: 34
base_hue: 180
```

All themes are initially monochromatic (without color).

With `base_chroma` (0-100 (%)) you can control the overall "colorfulness"
of your site, while the `base_hue` (0-360 (degrees)) setting adjusts
what base color the site has.

Some elements on the page are accentuated (prominent buttons and the
"timeline" of the audio player). The colorfulness of the accentuation can be
controlled by the `accent_chroma` (0-100 (%)) setting, while the
`accent_hue` (0-360 (degrees)) setting again adjusts its shade. The
`accent_brightening` (0-100 (%)) setting allows you to brighten or darken
this color accent (it's at 50% by default), which also lets you set stronger
or deeper colors.

> Note that there is a `--theming-widget` CLI option that lets you interactively
> explore these detail settings. Just build your catalog with the option enabled and
> every page will then contain the theming widget (don't forget to turn it off
> before deployment).

## Background image

The above described settings are completely carefree - no matter the settings,
your site will stay readable (at worst it may look funny). Not so with this
option. A background image can lend a nice quality to your site design, but
choose carefully what image you use and how opaque you make it. Sharp details
and strong contrasts within the image and against the text of the site will
render the site hard to read or even unreadable. That said,
`background_image` lets you reference the image to use, and with
`background_alpha` (0-100 (%)) you can optionally control its opaqueness.

```eno
# theme

background_alpha: 23
background_image: squiggly_monsters_texture.jpg
```

## Round corners on release covers

To give a softer feel to your page, enable the `round_corners` option.
This will visually round off the corners of covers on all pages.

```eno
# theme

round_corners
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

## Disabling waveforms altogether

This will not display waveforms on the release page, resulting in a more compact layout.

```eno
# theme

disable_waveforms
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
