<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Artist manifests – artist.eno

> All options at a glance: [alias(es)](#aliases), [copy_link](#copy_link), [image](#image), [link](#link), [more](#more), [more_label](#more_label), [name](#name), [permalink](#permalink), [theme](#theme)

Artists are automatically created by faircamp when they are encountered in
audio file metadata (e.g. the artist "Alice" will be created if any ID3 tag
says a track is by "Alice"). To add further information to an artist, you need
to explicitly define the artist, which can be done in two ways:

For one, you can use the `artist` field inside a `catalog.eno` or `release.eno`
manifest, which is primarily intended as a shortcut with limited options,
especially to link external artists to their own pages. See the manual pages for
catalog and releases for more info on that.

On the other hand, to specify a full-featured artist with its own page on the
faircamp site, create a directory for it anywhere in your catalog, create a
(plain text) file with the name `artist.eno` inside it and specify at least
the [name](#name) field, so your artist can be associated with its tracks in your
catalog.

Here is an example `artist.eno` file, below it the fields are explained one-by-one.

```eno
name: Alice
permalink: alice-artist

aliases:
- Älice
- Alice (feat. Bob)

image:
description = Alice in a field
file = example.jpg

-- more
Alice is a classic recording artist from the 70ies.

Over the years she has appeared in various collaborations, most notably with Bob.
-- more
```

## <a name="aliases"></a> `alias(es)`

To define a single alias for the artist:

```eno
alias: Älice
```

To define multiple aliases for the artist:

```eno
aliases:
- Älice
- Alice (feat. Bob)
```

If, as often happens, different audio files use slightly different versions of
an artist name (e.g. "Motörhead" vs. "Motorhead"), or the artist appears in a
collaboration (e.g. "Alice (feat. Bob)"), you can specify `aliases` that will
be matched against in addition to the [name](#name) to map the artist to the right
tracks.

## <a name="copy_link"></a> `copy_link`

To disable the "Copy link" button (by default it's enabled) you can use the
`copy_link` option, with either `enabled` or `disabled` as value. This is
also inherited by all releases, but can be changed on a granular basis for
single releases or groups of releases in their manifests.

```eno
copy_link: disabled
```

## <a name="image"></a> `image`

```eno
image:
description = Alice in a field
file = example.jpg
```

With `file` specify the path of the image, relative to the directory the
manifest is in (so if the image "example.jpg" is in the same folder, just
write "example.jpg", if it's in the parent folder write "../example.jpg", and
so on). Make sure to include a `description` for non-sighted people too.

## <a name="link"></a> `link`

```eno
link: https://example.com/this/artist/elsewhere/

link:
url = https://example.com/this/artist/elsewhere/

link:
label = A review of the artist
url = https://example.com/some-blog/some-review/
```

You can supply any number of `link` fields, these are prominently displayed in
the header/landing area of your artist page. A `link` must at least
provide a url, either as a simple value or as an `url` attribute. Optionally
you can also supply a `label` which is what is visibly displayed instead of
the `url`, when given.

## <a name="more"></a> `more`

```eno
-- more
Alice is a classic recording artist from the 70ies.

Over the years she has appeared in various collaborations, most notably with Bob.
-- more
```

This field lets you provide long-form content of any kind to augment the artist's
page with: A biography/discography, list of upcoming shows, personal message,
further links to the artist, etc. When provided, this content appears right
after the releases on an artist's page.

The `more` field supports [Markdown](https://commonmark.org/help/).

## <a name="more_label"></a> `more_label`

```eno
more_label: Biography
```

If you provide long-form additional content for the artist (which can be
anything you want, content-wise) through the [more](#more) field, by default
there will be a link with the label "More" on the artist page, leading to the
section containing that content. If you want to customize that label so it
specifically refers to the type of content you are providing there, the
`more_label` field allows you to do that. Some typical examples of custom
labels one might use in the context of an artist: "Details", "Shows",
"Discography", "Bio", "About" etc.

## <a name="name"></a> `name`

```eno
name: Alice
```

The `name` you assign is how the artist is represented **everywhere**,
including in tags on your downloads (unless you enable `tags: copy`, or a
similar setting).

Very importantly, the name is also used to match your explicit definition of
the artist (by you in the manifest) to any implicit definition (through audio
file metadata), so pay close attention that they are spelled exactly the same
in all places - including casing (lower-/uppercase). If the artist is
frequently spelled in different ways (e.g. in one audio file the artist is
tagged as "Alice", in another "alice", and in yet another
"Älicë"), a simple way to still correctly associate it with your single
 explicit definition is to use the `aliases` option, e.g.:

```eno
name: Alice

aliases:
- alice
- Älicë
```

## <a name="permalink"></a> `permalink`

```eno
permalink: alice-artist
```

For an explanation what a `permalink` is please see the "Concepts Explained" page,
unter "Topics".

## <a name="theme"></a> `theme`

With this you can adjust the visual appearance of your artist's page.

> Tip: There is a `--theming-widget` CLI option that lets you interactively
> explore color-related theme settings. Just build your catalog with the option enabled and
> every page will then contain the theming widget (don't forget to turn it off
> before deployment).

### Base

```eno
theme:
base = light
```

This sets the overall appearance, choose between `dark` (the default) and `light`.

### Dynamic range

```eno
theme:
dynamic_range = 24
```

At the highest dynamic range (100%) the theme appears the most "black" or "white"
(depending on your theme `base`) and the least colorful (depending on your chroma
settings, see below). The lower the dynamic range (0% being the default) the more it
will have a differentiated gray feeling (again interacting with your theme `base`),
and become over-all more colorfully tinted with rising base chroma levels. Tip: By
trying different values with the --theming-widget option you can interactively get
a good feeling of what this does and how you want to set it.

### Detail color adjustments

```eno
theme:
accent_brightening = 85
accent_chroma = 50
accent_hue = 23
base_chroma = 34
base_hue = 180
```

A site's theme is initially monochromatic (without color).

With `base_chroma` (0-100 (%)) you can control the overall "colorfulness"
of your site, while the `base_hue` (0-360 (degrees)) setting adjusts
what base color the theme is built on.

Some elements on the page are accentuated (prominent buttons and the
"timeline" of the audio player). The colorfulness of the accentuation can be
customized with the `accent_chroma` (0-100 (%)) setting, while the
`accent_hue` (0-360 (degrees)) setting adjusts its shade. The
`accent_brightening` (0-100 (%)) setting allows you to brighten or darken
this color accent (it's at 50% by default), which allows for stronger
and deeper colors still.

### Background image

```eno
theme:
background_alpha = 23
background_image = squiggly_monsters_texture.jpg
```

The previously described settings can be handled carefree - no matter the settings,
your site will stay readable (at worst it may look funny). When you set a
background image however, choose carefully what image you use and how opaque
you make it. Sharp details and strong contrasts within the image and against
the text of the site will render the site hard to read or even unreadable.
That said, `background_image` lets you reference the image to use, and with
`background_alpha` (0-100 (%)) you can optionally control its opaqueness.

### Round corners on release covers

To give a softer feel to your page, set the `round_corners` option to `enabled`.
This will visually round off the corners of covers on all pages. By setting it
back to `disabled` (the default) you can disable it for specific releases again.

```eno
theme:
round_corners = enabled
```

### Disabling relative waveform lengths

By default, the width of each track's waveform on a release page will render
at a different length, reflecting the duration of the track in relation to
the longest track on the release - for instance if the longest track on a
release is about two minutes long, that one will span the full width, but
another track that is only about one minute long will span only half of that
width. If you publish releases whose tracks have wildly varying lengths,
shorter tracks might get very narrow in the interface. If this is a concern
to you, or you just generally want all tracks to be full-width as an
aesthetic choice, you can enable this alternative behavior with this setting:

```eno
theme:
waveforms = absolute
```

### Disabling waveforms altogether

This will not display waveforms on the release page, resulting in a more compact layout.

```eno
theme:
waveforms = disabled
```

With `waveforms = enabled` you can turn this back on for specific releases if you want.

### Font

By default, faircamp bundles and uses the [Barlow](https://tribby.com/fonts/barlow/)
font on a generated site, but this can be configured.

Using the standard sans serif font from the system of the visitor:

```eno
theme:
system_font = sans
```

Using the standard monospace font from the system of the visitor:

```eno
theme:
system_font = mono
```

Using a specific font (by font name) from the system of the visitor (this should have a rather specific reason, normally you probably don't want to do that):

```eno
theme:
system_font = Arial
```

Bundling and using a custom font (put a `.woff` or `.woff2` file in the same directory as the manifest - other font file types are not supported!):

```eno
theme:
custom_font = MyCustomSans.woff2
```
