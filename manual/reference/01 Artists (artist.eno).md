<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Artist manifests – artist.eno

> All options at a glance: [alias(es)](#aliases), [copy_link](#copy_link), [download_code(s)](#download_codes), [embedding](#embedding), [external_page](#external_page), [image](#image), [link](#link), [m3u](#m3u), [more](#more), [more_label](#more_label), [name](#name), [payment_info](#payment_info), [permalink](#permalink), [release_download_access](#release_download_access), [release_downloads](#release_downloads), [release_extras](#release_extras), [release_price](#release_price), [speed_controls](#speed_controls), [streaming_quality](#streaming_quality), [synopsis](#synopsis), [tags](#tags), [theme](#theme), [track_download_access](#track_download_access), [track_downloads](#track_downloads), [track_extras](#track_extras), [track_numbering](#track_numbering), [track_price](#track_price), [unlock_info](#unlock_info)

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

## <a name="external_page"></a> `external_page`

Artists that appear only on some tracks/releases but have their own website
away from the faircamp page they are featured on, can be linked to that page
through this option. In that case the artist's name, wherever it appears, is
always linked to that external page (and no distinct page is rendered for the
artist on the faircamp site). To re-emphasize: Activating this will discard
the `permalink`, `synopsis`, `more`, etc. options for this artist - the artist
will just be linked to an external page everywhere, but not get their own on
the faircamp site itself.

```eno
external_page: https://example.com
```

When using this option, it often makes more sense to use a shortcut artist
definition using the [artist](catalog-catalog-eno.html#artist) field in a
[catalog.eno](catalog-catalog-eno.html) or [release.eno](releases-release-eno.html)
manifest.

## <a name="image"></a> `image`

```eno
image:
description = Alice in a field
file = example.jpg
```

With `file` specify the path of the image, relative to the directory the
manifest is in (so if the image "example.jpg" is in the same folder, just
write "example.jpg", if it's in the parent folder write "../example.jpg", and
so on). Make sure to include a `description` for non-sighted people too,
this is used as alt text on the image.

### How to ensure certain content in an artist image always is visible

The artist's `image` is shown in different ways depending on the screen
size and browser viewport of a visitor's device. If you include e.g. a logo
in the `image`, parts of it might be invisible due to the automated
cropping done by faircamp. This section describes how to include content
within the `image` in such a way that it never gets cropped away:

The least wide the `image` is shown is at an aspect ratio of 2.25:1
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

Note that all of this also applies 1:1 to the catalog's `home_image`.

## <a name="link"></a> `link`

```eno
link: https://example.com/this/artist/elsewhere/

link:
url = https://example.com/this/artist/elsewhere/

link:
label = A review of the artist
url = https://example.com/some-blog/some-review/

link:
url = https://social.example.com/@account-a
verification = rel-me

link:
url = https://social.example.com/@account-b
verification = rel-me-hidden
```

You can supply any number of `link` fields, these are prominently displayed in
the header/landing area of your artist page. A `link` must at least
provide a url, either as a simple value or as an `url` attribute. Optionally
you can also supply a `label` which is what is visibly displayed instead of
the `url`, when given.

Additionally, you can configure [rel="me"](https://microformats.org/wiki/rel-me)
linking, by supplying the attribute `verification = rel-me`.
This allows you to verify the artist/yourself as the author/owner
when you place a link to the artist page from (e.g.) a fediverse
profile. With `verification = rel-me-hidden` you can have the link be
included on your faircamp site without it showing up on the page, thus
serving only for verification purposes.

## <a name="m3u"></a> `m3u`

This controls the generation of an [M3U](https://en.wikipedia.org/wiki/M3U) playlist
for the artist (provided on the artist page) - it is disabled by default.

To enable the M3U playlist for an artist:

```eno
m3u: enabled
```

To disable the M3U playlist for an artist:

```eno
m3u: disabled
```

This behavior can also be globally configured (for all artists/releases) in the
catalog manifest.

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
via my [liberapay account](https://example.com)

Another option is supporting me through my [ko-fi page](https://example.com)

If you're in europe you can send the money via SEPA, contact me at
[lila@thatawesomeartist42.com](mailto:lila@thatawesomeartist42.com) and I'll
send you the account details.

On Dec 19th I'm playing a show at *Substage Indenhoven* - you can get the
digital album now and meet me at the merch stand in december in person to give
me the money yourself as well, make sure to make a note of it though! :)
-- payment_info
```

## <a name="permalink"></a> `permalink`

```eno
permalink: alice-artist
```

For an explanation what a `permalink` is please see the [Concepts Explained](concepts-explained.html) page.

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
A mysterious artist of many talents.
-- synopsis
```

A short (256 characters max), plain-text introduction text for the artist.

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
