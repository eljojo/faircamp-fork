<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Example artist site with soft paycurtain – Electric Curtain

A fictional Darksynth producer with the pseudonym *Electric Curtain* that has
many singles, who on their faircamp site offers downloads behind a soft
paycurtain, that is, asking for payment for downloads through third party
platforms, which are however not technically enforced.

This is how their catalog directory looks like:

```
Electric Curtain/             <--- Catalog
├─ catalog.eno                  <--- Catalog manifest
├─ abstractsyntaxthreat.png     <--- Background image
├─ 2023/                        <--- Extra Nesting (just for organizing)
│  └─ Enter the Maze/             <--- Release
│     ├─ release.eno                <--- Release manifest
│     ├─ enterthemaze.png           <--- Release cover
│     └─ enterthemaze.wav           <--- Track
├─ 2022/                        <--- Extra Nesting (just for organizing)
│  ├─ Network Angst/              <--- Release
│  │  ├─ release.eno                <--- Release manifest
│  │  ├─ networkangst.png           <--- Release cover
│  │  └─ networkangst.wav           <--- Track
│  └─ Dark Cybernetic Beings/     <--- Release
│     ├─ release.eno                <--- Release manifest
│     ├─ darkcyberneticbeings.png   <--- Release cover
│     └─ darkcyberneticbeings.wav   <--- Track
├─ 2021/
│  └─ ...
└─ ...
```

Inside the file `Electric Curtain/catalog.eno`:

```eno
title: Electric Curtain
base_url: https://curtain.electric/

> To save bandwidth and storage, the artist here reduces the
> streaming quality a little bit.
streaming_quality: frugal

-- more
Hailing from the small town of Welkenraedt, Electric Curtain sucks you into
a gigantesque dystopian world of hard and harsh bass-driven synth.

Support me on [ko-fi](https://ko-fi.com/electriccurtainisfiction)
-- more

release_download_access: paycurtain

> These settings apply to all releases, here we just set
> the download format for all of them. As each of them
> has a different price, that setting is individually set
> in each of the .eno files alongside the releases.
release_downloads: flac

> For each release these two payment options will be shown,
> as the settings here apply to all releases
-- payment_info
Option 1: Pay via [ko-fi](https://ko-fi.com/electriccurtainisfiction)

Option 2: Pay via [paypal](https://paypal.me/electriccurtainisfiction)
-- payment_info

> The artist uses a short-form artist declaration here, because the only
> needed adjustments here concern the name and aliases, which can be done
> with the short form. (With a page in label mode, when artists are featured
> with their own pages, including images and longer texts, a separate
> artist.eno manifest file per artist is the way to go instead)
external_artist:
> On the page we stylize the name with upside-down pentagrams, because.
name = ⛧ Electric Curtain ⛧
> Any release or track that has artist metadata matching one of
> the three aliases below will be associated with this artist.
alias = Electric Curtain
alias = Electric Curtain feat. Miley Vaniley
alias = Electric Curtain × Die Arbeit der Nacht

> There are two guest artists, for which the artist also uses a short-form
> artist definition in order to correctly associate them with the
> respective tracks (which specify an artist name that actually mixes two
> artists together, and as such couldn't be auto-detected as two artists)

external_artist:
name = Miley Vaniley
> One track features Miley Vaniley, and through an alias
> we correctly associate it with them.
alias = Electric Curtain feat. Miley Vaniley

external_artist:
name = Die Arbeit der Nacht
> One track features Die Arbeit der Nacht, and through an alias
> we correctly associate it with them.
alias = Electric Curtain × Die Arbeit der Nacht

theme:
background_alpha = 36
background_image = abstractsyntaxthreat.png
> The dark theme with high dynamic range (= deep black) nicely fits the darkness of the music
base = dark
dynamic_range = 100
```

Inside the file `Electric Curtain/2023/Enter the Maze/release.eno`:

```eno
permalink: enter-the-maze
date: 2023-05-15

release_price: 4+ USD

cover:
description = Enter the maze
file = enterthemaze.png
```

Inside the file `Electric Curtain/2022/Network Angst/release.eno`:

```eno
permalink: network-angst
date: 2022-12-20

release_price: 0+ USD

cover:
description = A 56k modem in neon colors
file = networkangst.png
```

Inside the file `Electric Curtain/2022/Dark Cybernetic Beings/release.eno`:

```eno
permalink: network-angst
date: 2022-09-02

release_price: 0+ USD

cover:
description = An abstract depiction of a crowd of people in a backalley, like in matrix, but more sinister
file = darkcyberneticbeings.png
```
