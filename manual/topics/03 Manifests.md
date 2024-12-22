<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Manifests

Three different types of so called *manifests* are used in order to specify
metadata and settings:
- A single `catalog.eno` file, which always is placed at the root of the
  catalog directory provide settings that apply to the site in general, as
  well as to all releases and artists on that site.
- The `release.eno` manifests, which are always placed alongside audio files
  (that is, in release directories), allow specifying options that apply to
  that specific release only, and they can override options that were set
  in the `catalog.eno` file.
- The `artist.eno` manifests each go into a separate directory that is
  dedicated to a single artist (note that this is only relevant if you
  have a site that features multiple artists and uses *label mode*. As
  you'd expect, this is where you specify options and metadata for that
  specific artist.

```
Catalog/
├─ catalog.eno
├─ An Artist/
│  └─ artist.eno
├─ Another Artist/
│  └─ artist.eno
├─ First Release/
│  ├─ release.eno
│  ├─ track_1.mp3
│  ├─ track_2.mp3
│  └─ track_3.mp3
└─ Second Release/
   ├─ release.eno
   ├─ track_1.mp3
   ├─ track_2.mp3
   └─ track_3.mp3
```

In the example above, everything defined in `catalog.eno` applies to `An Artist`,
`Another Artist`, `First Release` and `Second Release`, but the `artist.eno`
and `release.eno` manifests can selectively override options for the artist/release
directories they are placed in.

Here is an example `release.eno` manifest to give you an idea of how they work:

```eno
title: Second Release

cover:
description = An ink drawing of a barren tree with monkeys in its branches
file = cover.jpg

release_downloads:
- mp3
- opus

-- more
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- more
```

For details on the syntax used in the manifest files see the eno language
guide on the [eno website](https://simonrepp.com/eno/), simply modifying the
examples in the manual should get you there without any problems though, the
example here is pretty much as complex as it gets.
