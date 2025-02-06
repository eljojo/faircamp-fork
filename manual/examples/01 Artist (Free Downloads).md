<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Example artist site with free downloads – Free Metal

A fictional Metal band called *Free Metal* with two albums (*Much Doublebass*
and *Very Tapping*), which on its faircamp site offers free downloads only.

This is how their catalog looks like:

```
Free Metal/                   <--- Catalog
├─ catalog.eno                  <--- Catalog manifest (applies to all releases)
├─ so-tough.jpg                 <--- Home image
├─ Much Doublebass/             <--- Release
│  ├─ cover.jpg                   <--- Release cover
│  ├─ release.eno                 <--- Release manifest
│  ├─ 01 Tatatatatata.wav         <--- Track
│  ├─ 02 Badababadaba.wav
│  └─ ...
└─ Very Tapping/                <--- Release
   ├─ album.jpg                   <--- Release cover
   ├─ release.eno                 <--- Release manifest
   ├─ 01 Didididididi.aiff        <--- Track
   ├─ 02 Dabadidabadi.aiff
   └─ ...
```

In the file `Free Metal/catalog.eno` we find the following content:

```eno
title: Free Metal
base_url: https://example.com/

home_image:
description = The band, looking tough
file = so-tough.jpg

release_downloads:
- opus
- mp3

-- synopsis
Hey Metalheads! Check our newest releases "Much Doublebass" and "Very Tapping" - we've
got them out for free download for you cuties! xoxo, Free Metal
-- synopsis
```

This sets the `title` on the frontpage, the `base_url` (which is not mandatory,
but needed for some features to work), specifies the `home_image` that is shown on
the frontpage (plus its description for screen reader users), a short
`synopsis` text that is shown on the frontpage and it specifies that downloads
are served in the formats `opus` and `mp3` (which is all you need to do to enable
free downloads).

As this is the `catalog.eno` manifest, download settings are applied to all
releases further down in the directory hierarchy (in this case two of them),
so they don't need to be repeated for each release anymore!

In the file `Free Metal/Much Doublebass/release.eno` we find the following content:

```eno
title: Much Doublebass (Deluxe Edition)
permalink: much-doublebass-album
date: 2023-10-13

cover:
description = The band, looking tough (with yellow plush hats)
file = cover.jpg

-- synopsis
We're so excited to share our latest release with you, enjoy!
-- synopsis

-- more
Mastered by our good friends at the Doom Dungeon.
-- more
```

Here the `title` of the release is explicitly set because the band forgot to
include "(Deluxe Edition)" when they tagged the audio files - generally
faircamp simply takes the title form the audio files, so there's normally no
need to set it manually. The `permalink` ensures that the release is made
available under the url `https://example.com/much-doublebass-album/`, so as
you can see it simply gets added to the base url of the page (an in-depth
explanation of permalinks can be found on the [Concepts Explained](concepts-explained.html) page).
The cover image would have been automatically picked by faircamp, but to describe
the image for those who cannot see it, they included the description here, as
should always be done. Again a short `synopsis` text is provided (this is shown on the
top of the release page) as well as a production note in the `more` field,
which however could also be used to include content of any length, and supports
[Markdown](https://commonmark.org/help/).

In the file `Free Metal/Very Tapping/release.eno` we find the following content:

```eno
permalink: very-tapping-album
date: 2022-07-01

cover:
description = The band, looking not so tough for a change
file = album.jpg
```
