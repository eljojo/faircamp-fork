# Releases

Release artists and titles are automatically derived from audio file metadata,
however as you will possibly want to provide a textual description or tweak
the displayed title and artists for display in the browser, such data can
be provided through the manifests.

By default faircamp strips all metadata off the audio files that you supply
when it transcodes them for streaming and downloading, only adding back those
tags that it needs and manages itself, i.e. the title, track number, artist
(s), release artist(s) and release title. The `rewrite_tags` option lets you
control this: Set it to 'no' and faircamp will transfer all tags 1:1 from the
source files onto the transcoded files, as you provided them.

```eno
# release

artist: Heston Exchange
date: 2019-11-03
permalink: ape-affairs-bonus-track-edition
rewrite_tags: no
title: Ape Affairs (Bonus Track Edition)
track_numbering: disabled

cover:
description = An ink drawing of a barren tree with monkeys in its branches
file = cover.jpg

-- text
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- text
```

If you provide a cover image, use `description` to include an image description
for it.

The `text` field allows the use of [Markdown](https://commonmark.org/help/).

If your release has multiple principal artists (e.g. a split EP), instead of
`artist: Alice` you can also use the following to make faircamp present two
discrete artists as main artists of the release:

```eno
artists:
- Alice
- Bob
```

`track_numbering` allows configuration of the numbering style
used - by default it's `arabic` (01 02 03 …) but can be set to `hexadecimal`
(0x01 0x02 0x03 …), `roman` (I II
III …) or `disabled`.

Tracks are sorted by the track numbers found in the audio file metadata,
otherwise they are alphabetically sorted. Tracks with track numbers in
metadata are sorted before those without them, if you happen to have such
mixed material.

Note that the `date` is used for sorting only: Both on the homepage, as well
as on artist pages (label mode) releases that have the most recent date are
displayed on top, followed by older ones and lastly followed by those that
have no date specified at all (those will follow no intentional order).
Dates must be supplied strictly in the format `YYYY-MM-DD`.
