# Theme

You can adjust the visual appearance of your faircamp site to your liking. A
`background_image` can be specified, the `base` theme can be chosen from `dark`
and `light`, the accent color used in the theme can be set through `hue` (0-360)
and a `hue_spread` (e.g. -12, 3, 320) can be defined, which makes the site more
colorful (where 0 = mono-colored). In order for `hue_spread` to have an effect,
make sure to turn up tint_back (0-100) and/or tint_front (0-100) to add a
varyingly strong color tint to either the background, or the elements in the
foreground (most prominently: text).

```eno
# theme

background_image: squiggly_monsters_texture.jpg
base: dark
hue: 13
hue_spread: 0
tint_back: 0
tint_front: 0
```

Note that there is a `--theming-widget` CLI option that lets you interactively
explore different settings for `hue`, `hue_spread`, `tint_back` and
`tint_front`. Just build your catalog with the option enabled and open it in
the browser - every page will then contain the theming widget.