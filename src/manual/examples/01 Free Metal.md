# Free Metal

This first example catalog explains some of faircamp using a fictional Metal
band called *Free Metal* with two albums (*Much Doublebass* and *Very Tapping*),
which on its faircamp site offers free downloads only.

This is how their catalog looks like:

```
Free Metal/                   <--- Catalog
├─ free-metal.eno               <--- Manifest (applies to all releases)
├─ so-tough.jpg                 <--- Home image
├─ Much Doublebass/             <--- Release
│  ├─ much-doublebass.eno         <--- Manifest (only for this release)
│  ├─ cover.jpg                   <--- Release cover
│  ├─ 01 Tatatatatata.wav         <--- Track
│  ├─ 02 Badababadaba.wav
│  └─ ... 
└─ Very Tapping/                <--- Release
   ├─ very-tapping.eno            <--- Manifest (only for this release)
   ├─ album.jpg                   <--- Release cover
   ├─ 01 Didididididi.aiff        <--- Track
   ├─ 02 Dabadidabadi.aiff
   └─ ... 
```

In the file `Free Metal/free-metal.eno` we find the following content:

```eno
# catalog

title: Free Metal

base_url: https://freemetal.metal/

home_image:
description = The band, looking tough
file = so-tough.jpg

-- text
Hey Metalheads!

Check our newest releases - *Much Doublebass* and *Very Tapping* - we've
got them out for free download for you cuties!

xoxo, Free Metal
-- text

# download

free
formats:
- opus
- mp3
```

This first sets the title on the frontpage, the base url (which is not
mandatory, but needed for some features to work), specifies the image that is
shown on the frontpage (plus its description for screen reader users) and
a text that is also shown on the frontpage.

In the download section it specifies that downloads are served free and
in the formats opus and mp3.

Because this file is in the root directory of the catalog (`Free Metal/`), the
download settings are applied to all releases further down in the directory
hierarchy (in this case two of them), so they don't need to be repeated for
each release anymore!

In the file `Free Metal/Much Doublebass/much-doublebass.eno` we find the following content:

```eno
# release

title: Much Doublebass (Deluxe Edition)
permalink: much-doublebass-album
date: 2023-10-13

cover:
description = The band, looking tough (with yellow plush hats)
file = cover.jpg

-- text
We're so excited to share our latest release with you, enjoy!
Mastered by our good friends at the Doom Dungeon.
-- text
```

Here the title of the release is explicitly set because the band forgot
to include "(Deluxe Edition)" when they tagged the audio files, otherwise
faircamp would have automatically picked it from the audio files as well.
The permalink ensures that the release is made available under the url
`https://freemetal.metal/much-doublebass-album/`, so as you can see it
simply gets added to the base url of the page (an in-depth explanation of
permalinks can be found on the "Concepts Explained" page). The cover image would
have been automatically picked by faircamp, but to describe the image
for those who cannot see it, they included the description here, as
we all should always do! Again a text is provided that is shown
on the release page.

In the file `Free Metal/Very Tapping/very-tapping.eno` we find the following content:

```eno
# release

permalink: very-tapping-album
date: 2022-07-01

cover:
description = The band, looking not so tough for a change
file = album.jpg
```
