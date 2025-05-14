<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Manifests

Four different types of so called *manifests* are used in order to specify
metadata and settings:
- A single [catalog.eno](catalog-catalog-eno.html) file which always is
  placed at the root of the catalog directory provides settings that apply
  to the site in general, as well as to all releases, tracks and artists on
  that site.
- The [release.eno](releases-release-eno.html) manifests which are always
  placed alongside audio files (that is, in release directories), allow
  specifying options that apply to that specific release only, and they can
  override options that were set in the `catalog.eno` file.
- The [track.eno](tracks-track-eno.html) manifests, which can be used to set
  and override options on specific tracks if needed. In order to use this
  type of manifest, wrap an audio file in a release into its own track
  directory and place the `track.eno` file next to it, within its track
  directory.
- The [artist.eno](artists-artist-eno.html) manifests each go into a separate
  directory that is dedicated to a single artist (note that this is mostly
  relevant if you have a site that features multiple artists and uses
  [label mode](catalog-catalog-eno.html#label_mode)). As you'd expect,
  this is where you specify options and metadata for that specific artist.

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
   └─ Track 3/
      ├─ track.eno
      └─ track_3.mp3
```

In the example above, everything defined in `catalog.eno` applies to `An Artist`,
`Another Artist`, `First Release` and `Second Release`, but the `artist.eno`
and `release.eno` manifests can selectively override options for the artist/release
directories they are placed in. Inside `Second Release` one of the audio files is
additionally wrapped into its own track directory with its own `track.eno` file,
which augments and/or overrides any settings made to it at a higher level (release
or catalog-wide).

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
