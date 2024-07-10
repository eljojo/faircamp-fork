<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Catalog

Site-wide metadata and settings, such as the title and site URL.

```eno
# catalog

base_url: https://myawesomemusic.site/
favicon: my_favicon.png
embedding: enabled
label_mode
language: en
show_support_artists
title: My awesome music

home_image:
description = Me in my studio
file = studio_3.png

-- text
My self hosted faircamp site,
which presents some of my awesome music.

Nice of you to stop by!
-- text
```

## `embedding`

> Heads up: Embeds are still work in progress - for the moment they will look weird or not work at all.

This allows external sites to embed a widget that presents music from your site.
The embed code can be copied from each release page where embedding is enabled.

Embedding is currently disabled by default, given it's not a finished feature.
If you want to enable it for testing purposes you first need to set the
catalog's `base_url` (this is a technical precondition for generating the
embeds at all), and then set `enabled`, for the catalog, or for
specific releases. If you set it `enabled` at the catalog level, you can also
use `disabled` at release level to re-disable it for specific releases.

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

## General settings

To enable embeds and RSS feed generation you have to set `base_url`. The value
of `title` appears in multiple places on the site, inside the RSS Feed, etc..
The catalog `text` shows up prominently below the title on the homepage and
it supports[Markdown](https://commonmark.org/help/). The `home_image` is an
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

-- text
Lorem ipsum dolor sit amet ...
-- text
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
- Norwegian Bokmål `nb`
- Polish `pl`
- Spanish `es`

Translation corrections or improvements are very welcome, just [open an issue](https://codeberg.org/simonrepp/faircamp/issues).

### Not yet available languages

Note that even if there are no translations for your language yet, you can still set the
language code, which is then used to auto-determine the text direction (LTR/RTL),
and declare the language for your content on the site and in RSS feed metadata -
only the interface texts will still be in english.

```eno
language: ar
```

### Contributing translations

If you are eager to use faircamp with a not yet supported language, you can
easily help to make it happen: Open the [english locale](https://codeberg.org/simonrepp/faircamp/src/branch/main/src/locale/en.rs),
copy its content and just replace the english translations with the ones in
your language. Post the results in an [issue](https://codeberg.org/simonrepp/faircamp/issues)
or send them to simon@fdpl.io, then we'll wrap up the rest together. If you're
code/git savvy you can also directly submit a PR. Either way, don't worry about
making mistakes, I'll help you with the technical details, it's much appreciated.

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
