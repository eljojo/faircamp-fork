<!--
    SPDX-FileCopyrightText: 2024-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Example label site – Gemeindebau Records

A fictional label called *Gemeindebau Records* with multiple artists, all with
different info, options and download methods configured.

This is how their catalog directory looks like:

```
Gemeindebau Records/        <--- Catalog
├─ catalog.eno                <--- Catalog manifest
├─ gemeindebau.png            <--- Background image
├─ Mitzi MC/                    <--- Artist
│  ├─ artist.eno                  <--- Artist manifest
│  └─ GAFAM WTF OIDA/             <--- Release
│     ├─ release.eno                <--- Release manifest
│     ├─ oida.png                   <--- Release cover
│     └─ oida.wav                   <--- Track
├─ DJ Ferdl Sedlaček/           <--- Artist
│  ├─ artist.eno                  <--- Artist manifest
│  ├─ Hauptsache Italien Sampler/ <--- Release
│  │  ├─ release.eno                <--- Release manifest
│  │  ├─ Hauptsache Italien.png     <--- Release cover
│  │  └─ Hauptsache Italien.wav     <--- Track
│  └─ Caorle Split EP/            <--- Release
│     ├─ release.eno                <--- Release manifest
│     ├─ Caorle.png                 <--- Release cover
│     └─ Caorle.wav                 <--- Track
├─ J.J. (Jacqueline & Jessica)/
│  └─ ...
└─ ...
```

## `Gemeindebau Records/catalog.eno`

```eno
title: Gemeindebau Records
base_url: https://example.com
language: de

label_mode

home_image:
description = Die Crew beim Sandkasten, mit Jogginghosen und Baseballcaps
file = gemeindebau.png

-- synopsis
Hereinspaziert, Hereinspaziert! Gemeindebau Records ist ein leiwandes Label
mit freshen Artists aus dem Herzen des sozialen Wiener Wohnbaus.
-- synopsis

theme:
base = light
```

## `Gemeindebau Records/Mitzi MC/artist.eno`

```eno
name: Mitzi MC
permalink: mitzi-mc

link:
label = Mitzi Merchandise
url = https://example.com/

link:
label = Website
url = https://example.com/
```

## `Gemeindebau Records/Mitzi MC/GAFAM WTF OIDA/release.eno`

```eno
permalink: gafam-wtf-oida
release_downloads: flac

cover:
description = Das Wort "Oida", mit bunter Kreide auf den Gehsteig gekritzelt
file = oida.png
```

## `Gemeindebau Records/DJ Ferdl Sedlaček/artist.eno`

```eno
name: DJ Ferdl Sedlaček
permalink: dj-ferdl-sedlacek

-- synopsis
Beste Samples, Feinste Cuts, einfach der Ferdl!
-- synopsis
```

## `Gemeindebau Records/DJ Ferdl Sedlaček/Hauptsache Italien Sampler/release.eno`

```eno
permalink: hauptsache-italien-sampler
tags: copy

track_downloads:
- opus
- mp3

track_download_access: paycurtain
track_price: EUR 0+

-- payment_info
Der Ferdl ist ein bescheidener Typ, braucht nicht viel im Leben,
aber wenns mehr Ferdl-Releases vom Feinsten geben soll, schickts
ihm euren finanziellen Support über [PayPal](https://example.com)!
-- payment_info

cover:
description = "Hauptsache Italien" in riesiger Schrift und Retrofarbpalette
file = Hauptsache Italien.png
```

## `Gemeindebau Records/DJ Ferdl Sedlaček/Caorle Split EP/release.eno`

```eno
permalink: caorle-split-ep

release_download_access: https://example.com

cover:
description = Schwarz-Weiß-Foto vom Strand einer italienischen Küstenstadt
file = Caorle.png
```
