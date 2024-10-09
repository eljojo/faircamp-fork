<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Releases

Release artists and titles are automatically derived from audio file metadata,
however as you will possibly want to provide a textual description or tweak
the displayed title and artists for display in the browser, such data can
be provided through the manifests.

```eno
# release

artist: Heston Exchange
date: 2019-11-03
include_extras: no
permalink: ape-affairs-bonus-track-edition
title: Ape Affairs (Bonus Track Edition)
track_numbering: disabled


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
```

If you provide a cover image, use `description` to include an image description
for it.

The `text` field allows the use of [Markdown](https://commonmark.org/help/).

For an explanation what a `permalink` is please see the "Concepts Explained" page,
unter "Topics".

Any additional files in a release directory besides the audio files, cover image
and manifests (.eno files) are considered "extras" and by default included with
the release downloads (think artwork, liner notes, lyrics, etc.). To turn this
off, specify: `include_extras: no` (or respectively `yes` to turn it back on
for single releases). Note that if there are multiple images in the release
directory and you don't explicitly choose which of them is the cover in your
manifest, faircamp will use a simple heuristic to choose which of them it picks
as the cover: "cover.jpg" before "front.jpg" before "album.jpg", and after
that it will pick randomly. Note that it can also be ".png" or another format,
only the filename before the extension is considered, and case is disregarded
as well, so it can also be "Cover.jpg", for instance.

If your release has multiple principal artists (e.g. a split EP), instead of
`artist: Alice` you can also use the following to make faircamp present two
discrete artists as main artists of the release:

```eno
artists:
- Alice
- Bob
```

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

## `track_numbering`

```eno
track_numbering: arabic-dotted
```

`track_numbering` allows configuration of the numbering style
used, offering the following choices:

- `arabic` (1 2 3 …)
- `arabic-padded` (01 02 03 …) (default)
- `arabic-dotted` (1. 2. 3. …)
- `disabled` (Don't display track numbers)
- `hexadecimal` (0x1 0x2 0x3 …)
- `hexadecimal-padded` (0x01 0x02 0x03 …)
- `roman` (I II III …)
- `roman-dotted` (I. II. III. …)

Tracks are sorted by the track numbers found in the audio file metadata,
otherwise they are alphabetically sorted. Tracks with track numbers in
metadata are sorted before those without them, if you happen to have such
mixed material.

Note that the `date` is used for sorting only: Both on the homepage, as well
as on artist pages (label mode) releases that have the most recent date are
displayed on top, followed by older ones and lastly followed by those that
have no date specified at all (those will follow no intentional order).
Dates must be supplied strictly in the format `YYYY-MM-DD`.

To disable the "Copy link" button (by default it's enabled) you can use the `copy_link` option,
with either `enabled` or `disabled` as value.

```eno
copy_link: disabled
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
# release

tags: copy
```

Set it to `remove` and faircamp will produce entirely untagged files for
streaming and download.

```eno
# release

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

```eno
# release

tags:
album = rewrite
album_artist = remove
artist = copy
image = rewrite
title = rewrite
track = rewrite
```

Lastly, the default behavior can be (re-)set with the `normalize` option.

```eno
# release

tags: normalize
```

## Unlisted releases

By including an `unlisted` flag in the release manifest/section you can
configure a release to be generally present in the built site, but not publicly
referenced anywhere. In other words, visitors will only be able to open an
unlisted release page if they know the permalink. This is potentially
interesting to do a pre-release or such for friends or collaborators.

```eno
# release

unlisted
```

Note that in label mode, artists that have *only* unlisted releases will also
be unlisted, that is, they will not appear on the home/index page. Their
artist page however will still be generated, it too can be visited as an
unlisted page then by those who know the permalink.

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

## `streaming_quality`

```eno
streaming_quality: frugal
```

You can set the encoding quality for streaming from `standard` (the
default) to `frugal`. This uses considerably less bandwidth, reduces
emissions and improves load times for listeners, especially on slow
connections.
