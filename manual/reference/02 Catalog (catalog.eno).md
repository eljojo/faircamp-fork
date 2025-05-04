<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# The catalog manifest – catalog.eno

> All options at a glance: [artist](#artist), [base_url](#base_url), [cache_optimization](#cache_optimization), [copy_link](#copy_link), [download_code(s)](#download_codes), [embedding](#embedding), [faircamp_signature](#faircamp_signature), [favicon](#favicon), [feature_support_artists](#feature_support_artists), [feeds](#feeds), [freeze_download_urls](#freeze_download_urls), [home_image](#home_image), [label_mode](#label_mode), [language](#language), [link](#link), [m3u](#m3u), [more](#more), [more_label](#more_label), [opengraph](#opengraph), [payment_info](#payment_info), [release_download_access](#release_download_access), [release_downloads](#release_downloads), [release_extras](#release_extras), [release_price](#release_price), [rotate_download_urls](#rotate_download_urls), [show_support_artists](#show_support_artists), [site_assets](#site_assets), [site_metadata](#site_metadata), [speed_controls](#speed_controls), [streaming_quality](#streaming_quality), [synopsis](#synopsis), [tags](#tags), [theme](#theme), [title](#title), [track_download_access](#track_download_access), [track_downloads](#track_downloads), [track_extras](#track_extras), [track_numbering](#track_numbering), [track_price](#track_price), [unlock_info](#unlock_info)

The most central place in which changes to your site can be made
is the catalog manifest. Simply create a (plain text) file called
`catalog.eno` at the root directory of your catalog, and put any
of the options documented on this page in it.

As a short overview, this is where you set global options for the site itself
(such as the title, URL, language, etc.), options that globally affect all
pages (such as the design/theme), as well as options that are only passed on
to the releases (such as whether they can be downloaded and in which formats).
In general, any option that is set in the catalog manifest can be overwritten in
a release manifest by specifying override settings there.

An example `catalog.eno` file to give an overview (not all options are shown):

```eno
title: My music
base_url: https://example.com/my-music/
language: en

label_mode
show_support_artists

embedding: disabled
m3u: enabled
more_label: About

release_downloads:
- flac
- mp3
- opus

home_image:
description = Me in my studio
file = studio_3.png

link:
url = https://example.com/my-music-elsewhere/

link:
label = Blog
url = https://example.com/my-blog/

-- synopsis
Just some of my music
-- synopsis

-- more
Some of my music released between 1999-2005.

For further information check out my [website](https://example.com)
-- more

theme:
accent_brightening = 85
accent_chroma = 50
accent_hue = 23
base = light
base_chroma = 34
base_hue = 180
```

## <a name="artist"></a> `artist`

The artist field is a shortcut (with limited options) to define artists
without creating an explicit artist directory and `artist.eno` manifest. It
is especially useful for creating external artists - those that appear only
on some tracks/releases but have their own website away from the faircamp
page they are featured on. You can (but don't have to) use the
`external_page` attribute to set an external page, in that case the artist's
name, wherever it appears, is always linked to that external page (and no
distinct page is rendered for the artist on the faircamp site).

```eno
artist:
name = Alice
external_page = https://example.com
alias = Älice
alias = Älicë
```

When creating a non-external artist, the permalink option can be used
for explicitly defining the internal permalink (for external artists
this option has no use and is ignored):

```eno
artist:
name = Alice
permalink = alice-artist
```

For defining an artist with all options see the documentation for
[artist.eno](artists-artist-eno.html) manifests.

## <a name="base_url"></a> `base_url`

To allow embeds, M3U playlists and feeds to be generated (whether they are
enabled is configured on its own) you have to set `base_url`:

```eno
base_url: https://example.com
```

This url should be the website url under which you want your faircamp site to
go online.

## <a name="cache_optimization"></a> `cache_optimization`

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

## <a name="copy_link"></a> `copy_link`

To disable the "Copy link" button (by default it's enabled) you can use the `copy_link` option, with either `enabled` or `disabled` as value. This is also inherited by all releases, but can be changed on a granular basis for single releases or groups of releases in their manifests.

```eno
copy_link: disabled
```

## <a name="download_codes"></a> `download_code(s)`

To set a single download code that can be entered to access downloads:

```eno
download_code: crowdfunding2023
```

To set multiple download codes that can be entered to access downloads:

```eno
download_codes:
- GOLDsupporter
- SILVERsupporter
```

Note that you also need to use the [release_download_access](#release_download_access)
and/or [track_download_access](#track_download_access) options (e.g. `release_download_access: code`)
to activate download codes. In addition it is highly recommended to use the [unlock_info](#unlock_info)
option to provide a text that is displayed alongside the code input prompt.

## <a name="embedding"></a> `embedding`

This allows external sites to embed a widget that presents music from your site.
The embed code can be copied from each release page where embedding is enabled.

Embedding is disabled by default. If you want to enable it you also need to
set the catalog's [base_url](catalog-catalog-eno.html#base_url) (embeds work
by displaying something from your site on another site, for this the other site
needs to point to your site's address), and then set `embedding: enabled`,
either in a catalog, artist, release or track manifest. If you set it `enabled`
at the catalog level, you can also use `disabled` at lower level to
re-disable it for specific releases.

```eno
embedding: enabled
```

## <a name="faircamp_signature"></a> `faircamp_signature`

```eno
faircamp_signature: disabled
```

By default faircamp adds a subtle faircamp signature (faircamp logo, title and
version) to the footer. If you want to disable this (no judgement! :)) set it
to `disabled`.

## <a name="favicon"></a> `favicon`

A custom [favicon](#favicon) can be set, this currently only supports `.png`
and `.ico` files. `favicon: none` can be used to build the site without any
favicon at all.

```eno
favicon: my_favicon.png
```

## <a name="feature_support_artists"></a> `feature_support_artists`

By default, support artists (think features, guest artists, collaborators on
tracks/releases) are never linked to, and also don't have
their own artist page. The `feature_support_artists`  flag can be used to
link them to, and give them their own, artist pages (this implicitly enables
`show_support_artists`). Note that this flag only affects [label mode](#label_mode). In
artist mode no artist pages exist, instead the homepage *is* the one and only
artist page (the catalog artist's page).

```eno
feature_support_artists
```

## <a name="feeds"></a> `feeds`

**Heads up**: You need to set [base url](#base_url) so that faircamp can generate feeds.

By default a faircamp site provides two feeds that your visitors can subscribe
to: An atom feed and a generic RSS feed (RSS 2.0 without media or podcast
extensions), both of which are simple "blog-like" (purely text and image
based) feeds that provide updates linking to your new releases so that people
can listen to new material on your site only.

If you are publishing a podcast on your faircamp site, set the `feeds`
option to either `podcast_rss` (to provide only a single, Podcast RSS based feed)
or to `all` (to provide a podcast RSS feed as well as the default atom and
generic RSS feed options), for instance:

```eno
feeds: podcast_rss
```

Note that (Podcast) RSS only allows for a single track to be associated with
each *item* (this is the RSS term for a release/episode/show), therefore you
need to make sure that for podcast usage on your faircamp site, each release
only has a single track - all further tracks would simply not show up in the
(Podcast RSS) feed otherwise. Note however that in the near future Media RSS
will be added to faircamp, which will allow visitors to subscribe to an RSS
feed that provides multiple audio tracks with each item.

If you want to disable all feeds you can use this option:

```eno
feeds: disabled
```

If you want to be very specific in your feed configuration, you can
also use the discrete options `atom` and `generic_rss`. For instance to
provide only an atom feed:

```eno
feeds: atom
```

Or, to provide only generic RSS and Podcast RSS feeds:

```eno
feeds:
- generic_rss
- podcast_rss
```

## <a name="freeze_download_urls"></a> `freeze_download_urls`

When third parties hotlink to your site's resources, or when you discover that
people are blatantly sharing direct download links to your releases,
faircamp offers two related configuration options to combat this,
one of them being:

```eno
freeze_download_urls: [put-any-text-here]
```

Whatever text you put on the right is used to generate unique download urls
during site generation - but note that the text itself never shows up in the urls
themselves, it is merely used for randomization. The download urls stay valid
as long as the text does not change. Any time you update the text, all
download urls are regenerated, and thereby all old ones invalidated.
Practically speaking it makes sense to use some kind of a date as the text on
the right, for instance `freeze_download_urls: 1 April 2022` could tell you
that your current download urls have been valid since that day. You could
also use "2022-04", "Spring 2022" or such, given that one usually will not
manually invalidate the urls on a daily basis.

Note that this is a general countermeasure against hotlinking to your assets
and against people linking directly to your download pages (rather than your
release pages for instance), but that it (obviously) cannot stop people from
passing around your download codes (if you use download codes to protect your
downloads). If you witness malicious actions of that kind, you can instead
configure and distribute new download codes to your legitimate
buyers/subscribers, which will prevent anyone who obtained or still obtains
the old codes from using them, effectively blocking their access to your
downloads.

If you need an even stronger mechanism, you can also use the
[rotate_download_urls](#rotate_download_urls) option, which will automatically
renew all download urls each time you generate the site. Note however that
without additional manual tweaks to your deployment routine, this will have
very adverse effects on your deployment time, prompting a re-upload of all
your audio files each time you deploy, so use this with caution and only
when it's really needed.

## <a name="home_image"></a> `home_image`

The `home_image` is an image that will be displayed on the homepage, e.g. a logo
for your label or a band photo or such.

```eno
home_image:
description = Me in my studio
file = studio_3.png
```

`file` is the path (or just filename) of the image, relative from the
manifest's location.

The `description` is used as image alt text, which improves accessibility
for those visiting your site with screen readers.

### How to ensure certain content in a home_image always is visible

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

## <a name="label_mode"></a> `label_mode`

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

## <a name="language"></a> `language`

```eno
language: fr
```

### Available languages

Faircamp currently ships with these languages:

- Catalan `ca`
- Dutch `nl`
- English `en` (default)
- French `fr`
- German `de`
- Italian `it`
- Japanese `ja`
- Lithuanian `lt`
- Norwegian Bokmål `nb`
- Polish `pl`
- Portuguese (European) `pt-pt`
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
and declare the language for your content on the site and in feed metadata -
the interface texts will still be in english then of course.

```eno
language: ar
```

## <a name="link"></a> `link`

```eno
link: https://example.com/my/music/elsewhere/

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

You can supply any number of `link` fields, these are prominently displayed in
the header/landing area of your catalog homepage. A `link` must at least
provide a url, either as a simple value or as an `url` attribute. Optionally
you can also supply a `label` which is what is visibly displayed instead of
the `url`, when given.

Additionally, you can configure [rel="me"](https://microformats.org/wiki/rel-me)
linking, by supplying the attribute `verification = rel-me`. This allows you
to verify yourself as the site owner when you place a link to your faircamp
site from (e.g.) a fediverse profile. With `verification = rel-me-hidden` you
can have the link be included on your faircamp site without it showing up
on the page, thus serving only for verification purposes.

## <a name="m3u"></a> `m3u`

This controls the generation of [M3U](https://en.wikipedia.org/wiki/M3U) playlists
both for the entire catalog (provided on the landing page), as well as for each
release (provided on each release page) - both are disabled by default.

To enable M3U playlists both for the entire catalog and for all releases:

```eno
m3u: enabled
```

To enable only the M3U playlist for the entire catalog (provided on the homepage):

```eno
m3u: catalog
```

To enable only the M3U playlists for the releases:

```eno
m3u: releases
```

You can granularly enable/disable M3U playlists for single releases as well
(in the release manifests).

## <a name="more"></a> `more`

```eno
-- more
Our label explores a niche between 90ies Italo Disco and scottish
folk music from the early 50ies.

Among the represented artists are: ...
-- more
```

This field lets you provide long-form content of any kind to augment the
catalog homepage with: A biography/discography, mission statement,
about text, links to related pages, etc. When provided, this content appears right
after the releases on the catalog homepage.

The `more` field supports [Markdown](https://commonmark.org/help/).

## <a name="more_label"></a> `more_label`

```eno
more_label: About
```

If you provide long-form content for your catalog (which can be anything you
want, content-wise) through the [more](#more) field, by default there will be
a link with the label "More" on your homepage, leading to the section
containing that content. If you want to customize that label so it
specifically refers to the type of content you are providing there, the
`more_label` field allows you to do that. Some typical examples of custom
labels one might use in the context of the catalog homepage: "About",
 "Biography", "Artist Statement", "Read on", "Artist roster" etc.

## <a name="opengraph"></a> `opengraph`

Facebook's [Open Graph](https://ogp.me) protocol is used by many platforms to
crawl and harvest content from (linked) websites in order to present them
using a uniform "card" design pattern inside social feeds and timelines.

By default, faircamp does not render Open Graph tags, but this can be enabled with:

```eno
opengraph: enabled
```

Open Graph properties always rendered are the mandatory `og:title`, `og:image`
(including `alt`, `height` and `width`), `og:type` (always as `website`) and
`og:url` properties, as well as `og:locale` and `og:site_name`.
Where present, the `synopsis` field is rendered as the
`og:description` property additionally.

## <a name="payment_info"></a> `payment_info`

This is used together with the `paycurtain` setting of the [release_download_access](#release_download_access)
and/or [track_download_access](#track_download_access) options (e.g. `release_download_access: paycurtain`)
to set the text that is displayed before downloads are accessed.

The general idea here is to provide external links to one or more payment,
donation or patronage platforms that you use, be it liberapay, ko-fi, paypal,
stripe, etc. You can use [Markdown](https://commonmark.org/help/) to place
links, bullet points, etc. in the text.

```eno
-- payment_info
Most easily you can transfer the money for your purchase
via my [liberapay account](https://liberapay.com/somewhatsynthwave)

Another option is supporting me through my [ko-fi page](https://ko-fi.com/satanclaus92)

If you're in europe you can send the money via SEPA, contact me at
[lila@thatawesomeartist42.com](mailto:lila@thatawesomeartist42.com) and I'll
send you the account details.

On Dec 19th I'm playing a show at *Substage Indenhoven* - you can get the
digital album now and meet me at the merch stand in december in person to give
me the money yourself as well, make sure to make a note of it though! :)
-- payment_info
```

# <a name="release_download_access"></a> `release_download_access`

By default your visitors can only *stream* your releases.

To enable free downloads all you need to do is set one or more download
formats with the [release_downloads](#release_downloads) option.

Beyond this, the `release_download_access` option controls how visitors can
access downloads - by default as free downloads - but this can be changed to
external downloads, downloads accessible through download codes, or downloads
placed behind a soft paycurtain, and you can also disable access to downloads
here.

### Free downloads

This is the default (you don't need to set it yourself), but in case you want
to re-enable it in a manifest:

```eno
release_download_access: free
```

### External downloads

If you want to use your faircamp site purely to let people stream your audio,
but there is another place on the web where your release(s) can be
downloaded, external downloads allow you to display a download button that
merely takes people to the external download page.

For example, to display a download button that takes people to `https://example.com/artist/purchase/`, simply use that url as the value for this setting:

```eno
release_download_access: https://example.com/artist/purchase/
```

### Download code(s)

A download code (like a coupon/token) needs to be entered to access downloads.

To protect downloads with a code:

```eno
release_download_access: code
```

In combination with this use the [download_code(s)](#download_codes) option to
set the codes for accessing downloads and the [payment_info](#payment_info)
option to provide a text that is displayed with the code input field (to give
your audience directions on how to obtain a download code).

### Soft Paycurtain

A soft (i.e. not technically enforced) paycurtain needs to be passed before downloading.

To provide downloads behind a soft paycurtain:

```eno
release_download_access: paycurtain
```

In combination with this option, use the [release_price](#release_price) and
[payment_info](#payment_info) options to set a price and give instructions
for where the payment can be made.

### Disable downloads

Downloads can also be disabled explicitly (e.g. if you quickly want to take them offline at some point):

```eno
release_download_access: disabled
```

## <a name="release_downloads"></a> `release_downloads`


Sets the formats in which entire releases can be downloaded
as a (zip) archive. By default none are specified, so this needs
to be set in order to enable downloads for the entire release.

To set a single download format:

```eno
release_downloads: flac
```

To set multiple download formats:

```eno
release_downloads:
- flac
- mp3
- opus
```

All currently available formats:
- `aac`
- `aiff`
- `alac`
- `flac`
- `mp3`
- `ogg_vorbis`
- `opus` (this is an alias for `opus_128`)
- `opus_48`
- `opus_96`
- `opus_128`
- `wav`

In practice a minimal combination of a lossy state of the art format
(e.g. `opus`), a lossy format with high compatibility (e.g. `mp3`) and a
lossless format (e.g. `flac`) is recommended.

## <a name="release_extras"></a> `release_extras`

Any additional files in a release directory besides the audio files, cover
image and release.eno manifest are considered "extras" and by default
`bundled` with archive downloads (think artwork, liner notes, lyrics,
etc.).

To turn this off and entirely omit release extras:

```eno
release_extras: disabled
```

To provide release extras as separate downloads only:

```eno
release_extras: separate
```

To provide release extras both as separately downloadable and bundled with archive downloads:

```eno
release_extras:
- bundled
- separate
```

## <a name="release_price"></a> `release_price`

This is used together with the `paycurtain` setting of the [release_download_access](#release_download_access)
option (`release_download_access: paycurtain`) to set the price for release downloads that is
displayed before the downloads are accessed.

For example in order to ask for 4€ for accessing the downloads of a release:

```eno
price: EUR 4+
```

The `price` option accepts an [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code and a price range such as:

- `USD 0+` (Name your price, including zero dollars as a valid option)
- `3.50 GBP` (Exactly 3.50 Pounds)
- `KRW 9080` (Exactly 9080 south korean won)
- `INR 230+` (230 indian rupees or more)
- `JPY 400-800` (Between 400 and 800 japanese yen)

## <a name="rotate_download_urls"></a> `rotate_download_urls`

When third parties hotlink to your site's resources, or when you discover that
people are blatantly sharing direct download links to your releases,
faircamp offers two related configuration options to combat this,
one of them being:

```eno
rotate_download_urls
```

With `rotate_download_urls` enabled, faircamp will automatically generate new
download urls on each deployment (rendering invalid all previously existing
urls). This is a very strong measure. Usually it's enough to work with
less frequent, manual download url renewals using the
[freeze_download_urls](#freeze_download_urls) option.

Note that this is a general countermeasure against hotlinking to your assets
and against people linking directly to your download pages (rather than your
release pages for instance), but that it (obviously) cannot stop people from
passing around your download codes (if you use download codes to protect your
downloads). If you witness malicious actions of that kind, you can instead
configure and distribute new download codes to your legitimate
buyers/subscribers, which will prevent anyone who obtained or still obtains
the old codes from using them, effectively blocking their access to your
downloads.

## <a name="show_support_artists"></a> `show_support_artists`

By default, support artists (think features, guest artists, collaborators on
tracks/releases) are not listed in the interface. You can use the
`show_support_artists` flag to make them show up in listings:

```eno
show_support_artists
```

## <a name="site_assets"></a> `site_assets`

This allows you to specify arbitrary files for inclusion in the build (more precisely,
in the site's build root directory). Some of the most common usecases for this are for
instance:

- Providing script files for analytics
- Providing stylesheet files that customize the style beyond faircamp's theming system
- Providing complementary assets like font files, images, etc.

For instance, To include multiple files:

```eno
site_assets:
- analytics.js
- custom.css
- my_font.woff2
```

Or, to include a single file, you can also use a shorthand form:

```eno
site_assets: custom.css
```

Any files you provide will be included in the build with exactly the filename
you specify. If your filename(s) collide with any directory or file names
that faircamp generates, faircamp will point this out to you and abort the
build - in this case you need to assign new, non-conflicting name(s).

Note that specifying a `.css` file (for instance) does **not** mean the styles
will be automatically applied on the page, neither does specifying a `.js`
file lead to the script being executed on the page automatically. If you want
to directly use the assets in this way on your faircamp page (which might not
be your usecase, hence this is not done automatically), you additionally need
to use the [site_metadata](#site_metadata) option to include the respective
directives for it in the `<head>…</head>` section of your site.

## <a name="site_metadata"></a> `site_metadata`

This allows you to specify arbitrary tags for inclusion in the
`<head>…</head>` section of every page on your faircamp site. Some of the
most common usecases for this are for instance:

- Including an external analytics script file
- Specifying style customizations or linking to a stylesheet containing them
- Adding your own site-wide `<meta …>` tags

Now **before copying/pasting from the snippets** below, take note that assets
included through [site_assets](#site_assets) must **not** be included simply
by referring to their filename alone - instead, take their filename
(e.g. `custom.css`) and wrap it in double curly braces, like so: `{{custom.css}}`.

This way, faircamp will identify them as explicit references to your
[site_assets](#site_assets) and replace them with the correct, relative path on every page
(`custom.css` on the homepage, `../custom.css` on artist and release pages,
`../../custom.css` on track pages, and so on). A side-benefit from this is
that faircamp will ensure the reference's integrity for you - if the filename
in your reference does not match the file on disk, you will know before the
site is even built.

For instance, to link to an analytics script file included through [site_assets](#site_assets):

```eno
site_assets: analytics.js

-- site_metadata
<script src="{{analytics.js}}"></script>
-- site_metadata
```

To link to a custom css file that internally references a png
file, with both of them included through [site_assets](#site_assets):

```eno
site_assets:
- custom.css
- marker.png

-- site_metadata
<link href="{{custom.css}}" rel="stylesheet">
-- site_metadata
```

Note that when (for example) `custom.css` **internally** references
`marker.png`, such internal references must **not** use the double curly
braces otherwise used in `site_metadata`, as the `.css` and `.png` file
always keep the same relative location to each other - and faircamp does not
touch what's inside your site assets anyhow.

To directly specify style tweaks that additionally use an svg included through [site_assets](#site_assets):

```eno
site_assets: marker.svg

-- site_metadata
<style>
    a { text-decoration-thickness: .2em; }
    ul {
        list-style: georgian inside url("{{marker.svg}}");
        padding-left: 0;
    }
</style>
-- site_metadata
```

To include `fediverse:creator` metadata:

```eno
-- site_metadata
<meta name="fediverse:creator" content="@JohnMastodon@example.com"/>
-- site_metadata
```

Lastly, please keep in mind that modifications you make through this feature
have the capability to break readability, accessibility, functionality, etc.
for the visitors on your page - you are without guardrails here -
so be considerate what changes you make. Additionally, if you make any such
modifications on your page and at some point observe bugs on your faircamp
site, it's strongly recommended to confirm that your modifications are not
somehow responsible for them before reporting them, and to mention them
alongside your bugreports as well, just to be sure.

## <a name="speed_controls"></a> `speed_controls`

By default, faircamp's audio player(s) provide no playback speed controls,
assuming that it's intended for your audience to listen to the material at
its original speed only (e.g. for music, where this is usually the norm).

If you publish (e.g.) a narrative podcast or audio book and want people to
listen to the narration at their desired speed, you can enable playback speed
controls with this option:

```eno
speed_controls: enabled
```

By using `disabled` as value this setting can also be reverted.

## <a name="streaming_quality"></a> `streaming_quality`

```eno
streaming_quality: frugal
```

You can set the encoding quality for streaming from `standard` (the
default) to `frugal`. This uses considerably less bandwidth, reduces
emissions and improves load times for listeners, especially on slow
connections.

## <a name="synopsis"></a> `synopsis`

```eno
-- synopsis
My self hosted faircamp site, presenting some of my music.
Thanks for stopping by!
-- synopsis
```

A short (256 characters max), plain-text introduction text for your catalog,
this is the first thing visitors will see - make it count!

## <a name="tags"></a> `tags`

By default faircamp strips all metadata off the audio files that you supply
when it transcodes them for streaming and downloading, only adding back those
tags that it needs and manages itself, i.e. the title, track number, artist
(s), release artist(s) and release title. The `tags` option lets you control
this behavior:

Set it to `copy` and faircamp will transfer all tags 1:1 from the
source files onto the transcoded files, as you provided them.

```eno
tags: copy
```

Set it to `remove` and faircamp will produce entirely untagged files for
streaming and download.

```eno
tags: remove
```

In order to assert fine-grained control over tags, you can also specify
precise behavior per tag. The available tags at this point are `album`,
`album_artist`, `artist`, `image`, `title` and `track` (= track number). The
available actions for each tag are `copy` (copy 1:1 from the source audio
files) and `rewrite` (set it from whichever information you implicitly or
explicitly gave faircamp that would override the original tag, or fall back
to the original tag value if there is no override). There is also `remove`,
but as any tag you don't explicitly provide in this form is implicitly set
to be removed, this is redundant. Note that support for writing embedded
cover images differs wildly between target formats, at this point pretty much
only the `flac` and `mp3` formats can be expected to reliably contain them,
no matter what you specify for `image`.

A random example of this:

```eno
tags:
album = rewrite
album_artist = remove
artist = rewrite
image = copy
title = copy
track = copy
```

The default behavior can be explicitly (re-)applied with the `normalize` option.

```eno
tags: normalize
```

When written out explicitly using the fine-grained notation, the default behavior
(that is, `tags: normalize`) corresponds to the following settings:

```eno
tags:
album = rewrite
album_artist = rewrite
artist = rewrite
image = remove
title = rewrite
track = rewrite
```

## <a name="theme"></a> `theme`

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
font on a generated site, but alternatively you can also configure your site to only
use your visitors' system font(s):

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

Usage of custom fonts entails complexities and responsibilities that faircamp
can not generically automate away, therefore this requires manual integration
through the [site_assets](catalog-catalog-eno.html#site_assets) and
[site_metadata](catalog-catalog-eno.html#site_metadata) options - this also
gives a great amount of flexibility, including the possibility to use multiple
fonts and tweak their integration down to the last detail where needed.

## <a name="title"></a> `title`

The over-all title of your site, which appears in the header, footer, inside
the RSS Feed, etc.

```eno
title: My music
```

## <a name="track_download_access"></a> `track_download_access`

By default your visitors can only *stream* your tracks.

To enable free downloads of a track all you need to do is set one or more
download formats with the [track_downloads](#track_downloads) option.

Beyond this, the `track_download_access` option controls how visitors can
access downloads - by default as free downloads - but this can be changed to
external downloads, downloads accessible through download codes, or downloads
placed behind a soft paycurtain, and you can also disable access to downloads
here.

### Free downloads

This is the default (you don't need to set it yourself), but in case you want
to re-enable it in a manifest:

```eno
track_download_access: free
```

### External downloads

If you want to use your faircamp site purely to let people stream your audio,
but there is another place on the web where your track(s) can be
downloaded, external downloads allow you to display a download button that
merely takes people to the external download page.

For example, to display a download button that takes people to `https://example.com/artist/purchase/`,
simply use that url as the value for this setting:

```eno
track_download_access: https://example.com/artist/purchase/
```

### Download code(s)

A download code (like a coupon/token) needs to be entered to access downloads.

To protect downloads with a code:

```eno
track_download_access: code
```

In combination with this use the [download_code(s)](#download_codes) option to
set the codes for accessing downloads and the [payment_info](#payment_info)
option to provide a text that is displayed with the code input field (to give
your audience directions on how to obtain a download code).

### Soft Paycurtain

A soft (i.e. not technically enforced) paycurtain needs to be passed before downloading.

To provide downloads behind a soft paycurtain:

```eno
track_download_access: paycurtain
```

In combination with this option, use the [track_price](#track_price) and
[payment_info](#payment_info) options to set a price and give instructions
for where the payment can be made.

### Disable downloads

Downloads can also be disabled explicitly (e.g. if you quickly want to take them offline at some point):

```eno
track_download_access: disabled
```

## <a name="track_downloads"></a> `track_downloads`

Sets the formats in which single tracks can be separately downloaded.
By default none are specified, so this needs to be set in order to
enable separate downloads for single tracks.

To set a single download format:

```eno
track_downloads: flac
```

To set multiple download formats:

```eno
track_downloads:
- flac
- mp3
- opus
```

All currently available formats:
- `aac`
- `aiff`
- `alac`
- `flac`
- `mp3`
- `ogg_vorbis`
- `opus` (this is an alias for `opus_128`)
- `opus_48`
- `opus_96`
- `opus_128`
- `wav`

In practice a minimal combination of a lossy state of the art format
(e.g. `opus`), a lossy format with high compatibility (e.g. `mp3`) and a
lossless format (e.g. `flac`) is recommended.


## <a name="track_extras"></a> `track_extras`

Any additional files in a track directory besides the audio file, cover
image and track.eno manifest are considered "extras" and by default
offered as separate downloads on the track download page (think artwork, liner notes, lyrics,
etc.).

To turn this off and entirely omit track extras:

```eno
track_extras: disabled
```

To provide track extras as separate downloads (default behavior):

```eno
track_extras: enabled
```

Release downloads (zip archives) that include a track will also include the
extras of that track if `track_extras` is `enabled` for the track and the
`release_extras` option for the release is (or includes) `bundled`.

## <a name="track_numbering"></a> `track_numbering`

```eno
track_numbering: arabic-dotted
```

`track_numbering` allows configuration of the numbering style
used for the track numbers of releases, offering the following choices:

- `arabic` (1 2 3 …)
- `arabic-dotted` (1. 2. 3. …) (default)
- `arabic-padded` (01 02 03 …)
- `disabled` (Don't display track numbers)
- `hexadecimal` (0x1 0x2 0x3 …)
- `hexadecimal-padded` (0x01 0x02 0x03 …)
- `roman` (I II III …)
- `roman-dotted` (I. II. III. …)

## <a name="track_price"></a> `track_price`

This is used together with the `paycurtain` setting of the [track_download_access](#track_download_access)
option (`track_download_access: paycurtain`) to set the price for track downloads that is
displayed before the downloads are accessed.

For example in order to ask for 4€ for accessing the downloads of a track:

```eno
price: EUR 4+
```

The `price` option accepts an [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code and a price range such as:

- `USD 0+` (Name your price, including zero dollars as a valid option)
- `3.50 GBP` (Exactly 3.50 Pounds)
- `KRW 9080` (Exactly 9080 south korean won)
- `INR 230+` (230 indian rupees or more)
- `JPY 400-800` (Between 400 and 800 japanese yen)

## <a name="unlock_info"></a> `unlock_info`

In combination with the `code` setting of the [release_download_access](#release_download_access)
and/or [track_download_access](#track_download_access) options (e.g. `release_download_access: code`) and
[download_code(s)](#download_codes) option, this option lets you set the text that is displayed
to your visitors when they are prompted for a download code. Usually you will want to put
instructions in the text that tell your visitors how they can obtain a download code.

```eno
-- unlock_info
You should have received a download code in your confirmation mail
for this year's crowdfunding. Stay tuned in case you missed it,
we're currently planning the next run!
-- unlock_info
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
