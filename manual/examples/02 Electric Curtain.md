<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Electric Curtain

This example catalog demonstrates various concepts using a fictional Darksynth
producer with the pseudonym *Electric Curtain* that has many singles, who on
their faircamp site offers downloads behind a soft paycurtain, that is,
asking for payment for downloads through third party platforms, which are
however not technically enforced.

This is how their catalog directory looks like:

```
Electric Curtain/             <--- Catalog
├─ catalog.eno                  <--- Manifest (applies to all releases)
├─ abstractsyntaxthreat.png     <--- Background image
├─ 2023/                        <--- Extra Nesting (just for organizing)
│  └─ Enter the Maze/             <--- Release
│     ├─ release.eno                <--- Manifest (only for this release)
│     ├─ enterthemaze.png           <--- Release cover
│     └─ enterthemaze.wav           <--- Track
├─ 2022/                        <--- Extra Nesting (just for organizing)
│  ├─ Network Angst/              <--- Release
│  │  ├─ release.eno                <--- Manifest (only for this release)
│  │  ├─ networkangst.png           <--- Release cover
│  │  └─ networkangst.wav           <--- Track
│  └─ Dark Cybernetic Beings/     <--- Release
│     ├─ release.eno                <--- Manifest (only for this release)
│     ├─ darkcyberneticbeings.png   <--- Release cover
│     └─ darkcyberneticbeings.wav   <--- Track
├─ 2021/
│  └─ ...
└─ ...
```

Inside the file `Electric Curtain/catalog.eno`:

```eno
# artist

> Stylize the name with upside-down pentagrams.
name: ⛧ Electric Curtain ⛧

> Any release or track that has artist metadata matching one of
> the three aliases below will be associated with this artist.
aliases:
- Electric Curtain
- Electric Curtain feat. Miley Vaniley
- Electric Curtain × Die Arbeit der Nacht

# artist

name: Miley Vaniley

> One track features Miley Vaniley, and through an alias
> we correctly associate it with them.
alias: Electric Curtain feat. Miley Vaniley

# artist

name: Die Arbeit der Nacht

> One track features Die Arbeit der Nacht, and through an alias
> we correctly associate it with them.
alias: Electric Curtain × Die Arbeit der Nacht

# catalog

title: Electric Curtain

base_url: https://curtain.electric/

-- text
Hailing from the small town of Welkenraedt, Electric Curtain sucks you into
a gigantesque dystopian world of hard and harsh bass-driven synth.

Support me on [ko-fi](https://ko-fi.com/electriccurtainisfiction)
-- text

# download

> These settings apply to all releases, here we just set
> the download format for all of them. As each of them
> has a different price, that setting is individually set
> in each of the .eno files alongside the releases.
format: flac

# payment

> For each release these two payment options will be shown,
> as the settings here apply to all releases

-- custom
Pay via [ko-fi](https://ko-fi.com/electriccurtainisfiction)
-- custom

-- custom
Pay via [paypal](https://paypal.me/electriccurtainisfiction)
-- custom

# streaming

> To save bandwidth and storage, the artist here reduces the
> streaming quality a little bit.
quality: frugal

# theme

> The black theme nicely fits the darkness of the music
base: black

background_alpha: 36
background_image: abstractsyntaxthreat.png
```

Inside the file `Electric Curtain/2023/Enter the Maze/release.eno`:

```eno
# download

price: 4+ USD

# release

permalink: enter-the-maze
date: 2023-05-15

cover:
description = Enter the maze
file = enterthemaze.png
```

Inside the file `Electric Curtain/2022/Network Angst/release.eno`:

```eno
# download

price: 0+ USD

# release

permalink: network-angst
date: 2022-12-20

cover:
description = A 56k modem in neon colors
file = networkangst.png
```

Inside the file `Electric Curtain/2022/Dark Cybernetic Beings/release.eno`:

```eno
# download

price: 0+ USD

# release

permalink: network-angst
date: 2022-09-02

cover:
description = An abstract depiction of a crowd of people in a backalley, like in matrix, but more sinister
file = darkcyberneticbeings.png
```
