<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Catalog

Site-wide metadata and settings, such as the title and site URL.

```eno
# catalog

base_url: https://example.com/my-music/
cache_optimization: delayed
favicon: my_favicon.png
embedding: disabled
label_mode
language: en
m3u: disabled
more_label: About
show_support_artists
title: My awesome music

home_image:
description = Me in my studio
file = studio_3.png

link:
url = https://example.com/my-music-elsewhere/

link:
label = Blog
url = https://example.com/my-blog/

-- synopsis
My self hosted faircamp site, presenting some of my music.
-- synopsis

-- text
[Here be long form about/description text]
-- text

theme:
accent_brightening = 85
accent_chroma = 50
accent_hue = 23
base = light
base_chroma = 34
base_hue = 180
```

## `cache_optimization`

```eno
cache_optimization: delayed
```

Advanced control over caching strategy.

Allowed options: `delayed`, `immediate`, `wipe`, `manual`

Faircamp maintains an asset cache that holds the results of all
computation-heavy build artifacts (transcoded audio files, images, and
compressed archives). By default this cache uses the `delayed` optimization
strategy: Any asset that is not directly used in a build gets marked as stale
and past a certain period (e.g. 24 hours) gets purged from the cache during a
follow-up build (if it is not meanwhile reactivated because it's needed
again). This strikes a nice balance for achieving instant build speeds during
editing (after assets have been generated initially) without inadvertently
growing a storage resource leak in a directory you don't ever look at
normally.

If you're short on disk space you can switch to `immediate` optimization,
which purges stale assets right after each build (which might result in small
configuration mistakes wiping cached assets that took long to generate as a
drawback).

If you're even shorter on disk space you can use `wipe` optimization, which
just completely wipes the cache right after each build (so everything needs
to be regenerated on each build).

If you want full control you can use `manual` optimization, which does not
automatically purge anything from the cache but instead reports stale
assets after each build and lets you use `faircamp --optimize-cache`
and/or `faircamp --wipe-cache` accordingly whenever you're done with
your changes and e.g. don't expect to generate any new builds for a while.

## `embedding`

This allows external sites to embed a widget that presents music from your site.
The embed code can be copied from each release page where embedding is enabled.

Embedding is disabled by default. If you want to enable it you also need to
set the catalog's `base_url` (embeds work by displaying something from your
site on another site, for this the other site needs to point to your site's
address), and then set `enabled`, for the catalog, or for specific
releases. If you set it `enabled` at the catalog level, you can also use
`disabled` at release level to re-disable it for specific releases.

```eno
embedding: enabled
```

## `label_mode`

```eno
label_mode
```

By default faircamp operates in *artist mode* - it will lay out the site
in a way that best fits a single artist or band presenting
their works, meaning it will automatically take the artist associated
with the highest number of releases/tracks and name the catalog after them,
make the catalog description the description of that artist, etc..

The `label_mode` flag can be used if one wants to present multiple artists
on a single faircamp site. This adds an additional layer of information to the
page that differentiates the artists, gives them each their own page, etc.

## `link`

```eno
link:
url = https://example.com/my/music/elsewhere/

link:
label = Blog
url = https://example.com/my-blog/

link:
url = https://social.example.com/@account-a
verification = rel-me

link:
url = https://social.example.com/@account-b
verification = rel-me-hidden
```

You can supply any number of `link` fields, these are prominently displayed in the
header/landing area of your catalog homepage. A `link` must have at least a `url`
attribute. Optionally you can also supply a `label` which is what is visibly
displayed instead of the `url`, when given.

Even more optionally, you can configure [rel="me"](https://microformats.org/wiki/rel-me)
linking, by supplying the attribute `verification = rel-me`. This allows you
to verify yourself as the site owner when you place a link to your faircamp
site from (e.g.) a fediverse profile. With `verification = rel-me-hidden` you
can have the link be included on your faircamp site without it showing up
on the page, thus serving only for verification purposes.

## `m3u`

```eno
m3u: disabled
```

By default, an [M3U](https://en.wikipedia.org/wiki/M3U) playlist is generated
for the entire catalog (provided on the landing page), as well as for each
release (provided on each release page). By setting `m3u` to `disabled` in the
catalog manifest, M3U playlists are disabled both for the catalog and for all
releases. By setting `m3u` to `catalog`, solely the playlist for the entire
catalog is generated. By setting `m3u` to `releases`, only the playlists at
the release level are generated. You can granularly enable/disable M3U playlist
generation for single releases as well (in the release manifests).

## `more_label`

```eno
more_label: About
```

If you provide long-form text content for your catalog (which can be anything
you want, content-wise) through the `text` field, by default there will be a
link with the label "More" on your homepage, leading to the section
containing your long-form text. If you want to customize that label so it
specifically refers to the type of content you are providing there, the
`more_label` field allows you to do that. Some typical examples of custom
`more_label`s one might use for the catalog text: "About", "Biography",
"Artist Statement", "Read on", "Artist roster" etc.

## `synopsis`

```eno
-- synopsis
My self hosted faircamp site, presenting some of my music.
Thanks for stopping by!
-- synopsis
```

A short (256 characters max), plain-text introduction text for your catalog,
this is the first thing visitors will see - make it count!

## `text`

```eno
-- text
[Here be long form about/description text]
-- text
```

A [markdown](https://commonmark.org/help/)-enabled long-form text (think "About" text), in which you can write
about your catalog in any length and detail you like.

## `theme`

With this you can adjust the visual appearance of your faircamp site.

Theme customizations can be made in a top-level manifest at the root of the
catalog (setting the theme for the homepage and all release pages), but
they can also be made locally for a group of releases or for each release
on its own.

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

## General settings

To enable embeds, M3U playlists and RSS feed generation you have to set `base_url`. The value
of `title` appears in multiple places on the site, inside the RSS Feed, etc.. The `home_image` is an
image that will be displayed on the homepage, e.g. a logo for your label or a
band photo or such. A custom `favicon` can be set, this currently only
supports `.png` and `.ico` files. `favicon: none` can be used to build the
site without any favicon at all.

```eno
base_url: https://myawesomemusic.site/
favicon: my_favicon.png
title: My awesome music

home_image:
description = Me in my studio
file = studio_3.png
```

To disable RSS feed generation you can use the `disable_feed` option:

```eno
disable_feed
```

To disable "Copy link" button (by default it's enabled) you can use the `copy_link` option,
with either `enabled` or `disabled` as value. This is also inherited by all releases, but can
be changed on a granular basis for single releases or groups of releases in their manifests.

```eno
copy_link: disabled
```

## Main & Support artists

A release can have one or more *main artists*, i.e. principal authors. Artists
that appear as collaborators are called *support artists* in faircamp. The
main artists are auto-detected (e.g. when they are the only artist for a release,
when they appear in the "Album Artist" tag in files, or when they appear as
artist on most tracks of a release).

By default, support artists are not listed in the interface. You can use the
`show_support_artists` flag to make them show up in listings.

```eno
show_support_artists
```

Also by default, support artists are never linked to, and also don't have
their own artist page. The `feature_support_artists`  flag can be used to
link them to, and give them their own, artist pages (this implicitly enables
`show_support_artists`). Note that this flag only affects label mode. In
artist mode no artist pages exist, instead the homepage *is* the one and only
artist page (the catalog artist's page).

```eno
feature_support_artists
```

## `language`

```eno
language: fr
```

### Available languages

Faircamp currently ships with these languages:

- Dutch `nl`
- English `en` (default)
- French `fr`
- German `de`
- Italian `it`
- Lithuanian `lt`
- Norwegian Bokmål `nb`
- Polish `pl`
- Russian `ru`
- Spanish `es`
- Serbian (Cyrillic) `sr-cyrl`
- Serbian (Latin) `sr-latn`
- Swedish `sv`
- Turkish `tr`
- Ukrainian `uk`

You can easily contribute additional or improved language translations by
going to the [translation website](https://simonrepp.com/faircamp/translate/)
and following the instructions. No account and no special knowledge is needed,
all that is required is a little bit of your time and your will to help out.

If there are no translations for your language yet, you can still set the
language code, this is used to auto-determine the text direction (LTR/RTL)
and declare the language for your content on the site and in RSS feed metadata -
the interface texts will still be in english then of course.

```eno
language: ar
```

## Dealing with malicious behavior

When third parties hotlink to your site's resources, or when you discover that
people are blatantly sharing direct download links to your releases,
faircamp offers two related configuration options to combat this:

```eno
rotate_download_urls
```

With `rotate_download_urls` enabled, faircamp will automatically generate new
download urls on each deployment (rendering invalid all previously existing
urls).

Similarly, you can also manually control this mechanism:

```eno
freeze_download_urls: [put-any-text-here]
```

Whatever text you put on the right is used to generate unique download urls
during deployment - but note that the text itself never shows up in the urls
themselves, it is merely used for randomization. The download urls stay valid
as long as the text does not change. Any time you update the text, all
download urls are regenerated, and thereby all old ones invalidated.
Practically speaking it makes sense to use some kind of a date as the text on
the right, for instance `freeze_download_urls: 1 April 2022` could tell you
that your current download urls have been valid since that day. You could
also use "2022-04", "Spring 2022" or such, given that one usually will not
manually invalidate the urls on a daily basis.

## `streaming_quality`

```eno
streaming_quality: frugal
```

You can set the encoding quality for streaming from `standard` (the
default) to `frugal`. This uses considerably less bandwidth, reduces
emissions and improves load times for listeners, especially on slow
connections.

## Verifying yourself as the owner (e.g. Mastodon)

Some social media platforms - Mastodon, in particular - support website
verification using a link with a `rel="me"` attribute whose `href` attribute
value points to the social media profile that should be verified as the
owner of the site.

On a faircamp site you can use raw html inside the catalog text to place
an (invisible) link that verifies you as the owner, like this:

```eno
# catalog

-- text
<a rel="me" href="https://instance.example/@username" style="display: none;">Mastodon</a>
-- text
````

## How to ensure certain content in a home_image always is visible

The catalog's `home_image` is shown in different ways depending on the screen
size and browser viewport of a visitor's device. If you include e.g. a logo
in your `home_image`, parts of it might be invisible due to the automated
cropping done by faircamp. This section describes how to include content
within the `home_image` in such a way that it never gets cropped away:

The least wide the `home_image` is shown is at an aspect ratio of 2.25:1
(corresponding e.g. to a resolution of 225x100, 675x300, etc.), that's on
very wide and very narrow screens. The widest it is shown (when the browser
is just below 960px wide) is at an aspect ratio of 5:1 (corresponding to a
resolution of 500x100, 1500x300, etc.). If you create your image with an
aspect ratio of 5:1, so e.g. at 1500x300, and place the text that should be
not cropped within a rectangle of 2.25:1, so within a 675px wide rectangle at
the center of the example 1500x300 image, the text should always be fully
visible, uncropped, and only the parts to the left and right will get cropped
off.

```
|<-------  5:1 (e.g. 1500×300)  ------->|
┌─────────────┬───────────┬─────────────┐
│             │           │             │
│   CROPPED   │ LOGO SAFE │   CROPPED   │
│             │           │             │
└─────────────┴───────────┴─────────────┘
              |<--------->|
           2.25:1 (e.g. 675×300)
```

Note that all of this also applies 1:1 to artist images in `label_mode`.
