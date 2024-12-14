<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Say the Magic Word

This example catalog demonstrates further concepts using the fictional solo
artist *Say the Magic Word*, which has released a single EP, and who on their
faircamp site provides access to downloads through download codes, which can
be obtained on another site which the artist uses to receive and manage
financial support from his audience.

This is how their catalog directory looks like:

```
saythemagicword/             <--- Catalog
├─ general.eno                 <--- Manifest (applies to all releases)
├─ MagicSansV1.3-Book.woff2    <--- Custom font
└─ saythemagicword-ep/           <--- Release
   ├─ ep.eno                     <--- Manifest (only for this release)
   ├─ ep-cover.png               <--- Release cover
   ├─ booklet.pdf                <--- Extra
   ├─ 01.flac                    <--- Track
   ├─ 02.flac                    <--- Track
   ├─ 03.flac                    <--- Track
   ├─ 04.flac                    <--- Track
   └─ 05.flac                    <--- Track
```

Inside the file `saythemagicword/general.eno`:

```eno
# catalog

title: Say the Magic Word

base_url: https://magicmagicmagic.se/

-- text
The *Say the Magic Word EP* is out. Stream here, and to access downloads get a
download code by [becoming a patron](https://tinyurl.com/say-support)!
-- text

theme:
accent_brightening = 13
accent_chroma = 20
accent_hue = 163
base = light
base_chroma = 14
base_hue = 116
custom_font = MagicSansV1.3-Book.woff2
```

Inside the file `saythemagicword/saythemagicword-ep/ep.eno`:

```eno
# release

permalink: say-the-magic-word-ep
date: 2023-11-15

cover:
description = My dog Winston with a paper party hat (he's tiny)
file = ep-cover.png

archive_downloads:
- flac
- mp3
- opus

> The artist offers two different tiers of patronage. On the standard tier,
> they give access to the first EP released (these patrons received the
> download code "magicfanlove"). Supporters that paid extra, are in the
> special tier and got the download code "magicsuperfanspectacular" which
> the artist will then also add to the upcoming releases, so these patrons
> can access all downloads by the artist.
download_codes:
- magicfanlove
- magicsuperfanspectacular

> This is the text that is shown on the page where visitors need to enter
> a code to access the downloads.
-- unlock_info
You can obtain a download code by [becoming a patron](https://tinyurl.com/say-support)!
-- unlock_info
```