<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Releases

In your release manifests you can specify options that apply to
a specific release only. Simple create a (plain text) file called
`release.eno` inside a release directory (a directory that contains
audio files) and put any of the options documented on this page in it.

Release artists and titles, track numbers, etc. are automatically derived from
audio file metadata, but the release manifest allows a plethora of other options,
such as customizing the design/theme per release, displaying a short synopsis
and a long-form about text, making a release unlisted, etc.

```eno
title: Ape Affairs (Bonus Track Edition)
permalink: ape-affairs-bonus-track-edition
date: 2019-11-03

artist: Heston Exchange

archive_downloads:
- flac
- mp3
- opus

m3u: disabled
more_label: Liner Notes
track_numbering: arabic-dotted

cover:
description = An ink drawing of a barren tree with monkeys in its branches
file = cover.jpg

-- synopsis
Nobody thought it possible, until somebody did it. The release that started it all!
-- synopsis

-- text
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- text

theme:
accent_brightening = 85
accent_chroma = 50
accent_hue = 23
base = light
base_chroma = 34
base_hue = 180
```

If you provide a cover image, use `description` to include an image description
for it.

The `text` field allows the use of [Markdown](https://commonmark.org/help/).

For an explanation what a `permalink` is please see the "Concepts Explained" page,
unter "Topics".

Tracks are sorted by the track numbers found in the audio file metadata,
otherwise they are alphabetically sorted. Tracks with track numbers in
metadata are sorted before those without them, if you happen to have such
mixed material.

Note that if there are multiple images in the release directory and you
don't explicitly choose which of them is the cover in your manifest, faircamp
will use a simple heuristic to choose which of them it picks as the
cover: "cover.jpg" before "front.jpg" before "album.jpg", and after that it
will pick randomly. Note that it can also be ".png" or another format, only
the filename before the extension is considered, and case is disregarded as
well, so it can also be "Cover.jpg", for instance.

If your release has multiple principal artists (e.g. a split EP), instead of
`artist: Alice` you can also use the following to make faircamp present two
discrete artists as main artists of the release:

```eno
artists:
- Alice
- Bob
```

## `archive_downloads`

Sets the formats in which entire releases can be downloaded
as a (zip) archive. By default none are specified, so this needs
to be set in order to enable downloads for the entire release.

To set a single download format:

```eno
archive_downloads: flac
```

To set multiple download formats:

```eno
archive_downloads:
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

## `copy_link`

To disable the "Copy link" button (by default it's enabled) you can use the
`copy_link` option, with either `enabled` or `disabled` as value.

```eno
copy_link: disabled
```

## `date`

The `date` field is used for sorting only. Both on the homepage, as well as on
artist pages (in label mode), releases that have the most recent date are
displayed on top, followed by older ones and lastly followed by those that
have no date specified at all (those will follow no intentional order).

Dates must be supplied strictly in the format `YYYY-MM-DD`, for instance:

```eno
date: 1999-12-31
```

## `download_code(s)`

To set a single download code that can be entered to access downloads:

```eno
download_code: crowdfunding2023!
```

To set multiple download codes that can be entered to access downloads:

```eno
download_codes:
- GOLDsupporter
- SILVERsupporter
```

Note that you also need to use the `downloads: code` option to activate
download codes. In addition it is highly recommended to use `unlock_info` to
provide a text that is displayed alongside the code input prompt.

## `downloads`

By default your visitors can only stream your releases.

To enable simple free downloads all you need to do is set one or more download
formats with the `archive_downloads` and/or `track_downloads` option.

The `downloads` option gives you further control over the general download
mode, which by default is free downloads, but can be changed to external
downloads, downloads accessible through download codes, or downloads placed
behind a soft paycurtain, and you can also disable downloads here.

### Free downloads

This is the default (you don't need to set it yourself), but in case you want
to re-enable it in a manifest:

```eno
downloads: free
```

### External downloads

If you want to use your faircamp site purely to let people stream your audio,
but there is another place on the web where your release(s) can be
downloaded, external downloads allow you to display a download button that
merely takes people to the external download page.

For example, to display a download button that takes people to `https://example.com/artist/purchase/`, simply use that url as the setting value:

```eno
downloads: https://example.com/artist/purchase/
```

### Download code(s)

A download code (like a coupon/token) needs to be entered to access downloads.

To protect downloads with a code:

```eno
downloads: code
```

In combination with this use the `download_code` or `download_codes` option to
set the codes for accessing downloads and the `download_info` option to
provide a text that is displayed with the code input field (to give your
audience directions on how to obtain an download code).

### Soft Paycurtain

A soft (i.e. not technically enforced) paycurtain needs to be passed before downloading.

To provide downloads behind a soft paycurtain:

```eno
downloads: paycurtain
```

In combination with this option, use the `price` and `payment_info` options
to set a price and give instructions for where the payment can be made.

### Disable downloads

Downloads can also be disabled explicitly (e.g. if you quickly want to take them offline at some point):

```eno
downloads: disabled
```

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

## `extra_downloads`

Any additional files in a release directory besides the audio files, cover
image and manifests (.eno files) are considered "extras" and by default
`bundled` with archive downloads (think artwork, liner notes, lyrics,
etc.).

To turn this off and entirely omit extra files:

```eno
extra_downloads: disabled
```

To provide extra files as separate downloads only:

```eno
extra_downloads: separate
```

To provide extra files both as separately downloadable and bundled with archive downloads:

```eno
extra_downloads:
- bundled
- separate
```

## `link`

```eno
link: https://example.com/this/release/elsewhere/

link:
url = https://example.com/this/release/elsewhere/

link:
label = An Album review
url = https://example.com/some-blog/some-review/
```

You can supply any number of `link` fields, these are prominently displayed in
the header/landing area of your release page. A `link` must at least
provide a url, either as a simple value or as an `url` attribute. Optionally
you can also supply a `label` which is what is visibly displayed instead of
the `url`, when given.

## `m3u`

This controls the generation of an [M3U](https://en.wikipedia.org/wiki/M3U) playlist
for the release (provided on the release page) - it is disabled by default.

To enable the M3U playlist for a release:

```eno
m3u: enabled
```

To disable the M3U playlist for a release:

```eno
m3u: disabled
```

This behavior can also be globally configured (for all releases) in the
catalog manifest.

## `more_label`

```eno
more_label: Liner Notes
```

If you provide long-form text content for your release (which can be anything
you want, content-wise) through the `text` field, by default there will be a
link with the label "More" on your release page, leading to the section
containing your long-form text. If you want to customize that label so it
specifically refers to the type of content you are providing there, the
`more_label` field allows you to do that. Some typical examples of custom
`more_label`s one might use for the release text: "Details", "Liner Notes",
"Staff", "Lyrics", "About" etc.

## `payment_info`

This is used together with `downloads: paycurtain` to set the text that is
displayed before downloads are accessed.

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

## `price`

This is used together with `downloads: paycurtain` to set the price that is
displayed before downloads are accessed.

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

## `streaming_quality`

```eno
streaming_quality: frugal
```

You can set the encoding quality for streaming from `standard` (the
default) to `frugal`. This uses considerably less bandwidth, reduces
emissions and improves load times for listeners, especially on slow
connections.

## `synopsis`

```eno
-- synopsis
Nobody thought it possible, until somebody did it. The release that started it all!
-- synopsis
```

A short (256 characters max), plain-text introduction text for your release,
this is prominently featured atop your release page - make it count!

## `text`

```eno
-- text
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- text
```

A [markdown](https://commonmark.org/help/)-enabled long-form text (think "About" text), in which you can write
about your release in any length and detail you like.

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

## `track_downloads`

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

## `track_numbering`

```eno
track_numbering: arabic-dotted
```

`track_numbering` allows configuration of the numbering style
used, offering the following choices:

- `arabic` (1 2 3 …)
- `arabic-dotted` (1. 2. 3. …)
- `arabic-padded` (01 02 03 …) (default)
- `disabled` (Don't display track numbers)
- `hexadecimal` (0x1 0x2 0x3 …)
- `hexadecimal-padded` (0x01 0x02 0x03 …)
- `roman` (I II III …)
- `roman-dotted` (I. II. III. …)

## `unlock_info`

In combination with `downloads: code` and `download_code(s)`, this option
lets you set the text that is displayed to your visitors when they are prompted
for a download code. Usually you will want to put instructions in the text that
tell your visitors how they can obtain a download code.

```eno
-- unlock_info
You should have received a download code in your confirmation mail
for this year's crowdfunding. Stay tuned in case you missed it,
we're currently planning the next run!
-- unlock_info
```

## Tag/Metadata settings

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

## Unlisted releases

By including an `unlisted` flag in the release manifest you can configure a
release to be generally present in the built site, but not publicly
referenced anywhere. In other words, visitors will only be able to open an
unlisted release page if they know the permalink. This is potentially
interesting to do a pre-release or such for friends or collaborators.

```eno
unlisted
```

Note that in label mode, artists that have *only* unlisted releases will also
be unlisted, that is, they will not appear on the home/index page. Their
artist page however will still be generated, it too can be visited as an
unlisted page then by those who know the permalink.
