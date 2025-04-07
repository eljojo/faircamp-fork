<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Example artist site with download codes – Say the Magic Word

A fictional solo artist *Say the Magic Word*, which has released a single EP,
and who on their faircamp site provides access to downloads through download
codes, which can be obtained on another site which the artist uses to receive
and manage financial support from his audience.

This is how their catalog directory looks like:

```
saythemagicword/             <--- Catalog
├─ catalog.eno                 <--- Catalog manifest
├─ MagicSansV1.3-Book.woff2    <--- Custom font
└─ saythemagicword-ep/           <--- Release
   ├─ release.eno                <--- Release manifest
   ├─ ep-cover.png               <--- Release cover
   ├─ booklet.pdf                <--- Extra
   ├─ 01.flac                    <--- Track
   ├─ 02.flac                    <--- Track
   ├─ 03.flac                    <--- Track
   ├─ 04.flac                    <--- Track
   └─ 05.flac                    <--- Track
```

## `saythemagicword/catalog.eno`:

```eno
title: Say the Magic Word
base_url: https://example.com

link:
label = Become a Patron
url = https://example.com/become-patron/

-- synopsis
The Say the Magic Word EP is out. Stream here, and to access downloads get a
download code by becoming a patron!
-- synopsis

theme:
accent_brightening = 13
accent_chroma = 20
accent_hue = 163
base = light
base_chroma = 14
base_hue = 116
```

## `saythemagicword/saythemagicword-ep/release.eno`:

```eno
permalink: say-the-magic-word-ep
date: 2023-11-15

cover:
description = My dog Winston with a paper party hat (he's tiny)
file = ep-cover.png

release_downloads:
- flac
- mp3
- opus

> The artist offers two different tiers of patronage. On the standard tier,
> they give access to the first EP released (these patrons received the
> download code "magicfanlove"). Supporters that paid extra, are in the
> special tier and got the download code "magicsuperfanspectacular" which
> the artist will then also add to the upcoming releases, so these patrons
> can access all downloads by the artist.
release_download_access: code
download_codes:
- magicfanlove
- magicsuperfanspectacular

> This is the text that is shown on the page where visitors need to enter
> a code to access the downloads.
-- unlock_info
You can obtain a download code by [becoming a patron](https://tinyurl.com/say-support)!
-- unlock_info
```

## How it works

When you enable download codes, the "Download" button sends the user to an
unlock page instead of directly to the download page. The unlock page prompts
visitors to enter a download code, and if a correct download code is entered
they are taken to the download page.
