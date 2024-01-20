# Say the Magic Word

This example catalog demonstrates further concepts using the fictional
solo artist *Say the Magic Word* which thus far has only released a single EP,
and who on their faircamp site offers downloads behind an unlock code,
which can be obtained by listeners on another site which the artist
just uses to receive and manage financial support from his audience. 

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
The *Say the Magic Word EP* is out. Stream here and for downloads get your
unlock code by [becoming a patron](https://tinyurl.com/say-support)!
-- text

# theme
 
base: white

custom_font: MagicSansV1.3-Book.woff2

link_hue: 33
link_saturation: 90
link_lightness: 65
tint_back: 20
tint_front: 10
text_hue: 49
```

Inside the file `saythemagicword/saythemagicword-ep/ep.eno`:

```eno
# download

> The artist offers two different tiers of patronage. On the standard tier,
> they give access to the first EP released (these patrons received the
> unlock code "magicfanlove"). Supporters that paid extra, are in the special
> tier and got the unlock code "magicsuperfanspectacular" which the artist
> will then also add to the upcoming releases, so these patrons can unlock
> all downloads by the artist.
codes:
- magicfanlove
- magicsuperfanspectacular

formats:
- flac
- mp3
- opus

> This is the text that is shown on the page where visitors need to enter
> a code to unlock the downloads.
-- unlock_text
Get your unlock code by [becoming a patron](https://tinyurl.com/say-support)!
-- unlock_text

# release

permalink: say-the-magic-word-ep
date: 2023-11-15

cover:
description = My dog Winston with a paper party hat (he's tiny) 
file = ep-cover.png
```