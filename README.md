# Faircamp (codename)

A self-hostable, statically generated bandcamp alternative — *[see screenshots/posts on mastodon](https://post.lurk.org/@freebliss/105685776449364587)*

## Concept Overview

Faircamp takes a directory on your disk - your *Catalog* - and from it produces a
fancy-looking (and technically simple and completely *static*) website, which
presents your music in a way similar to how popular commercial service
*bandcamp* does it.

You can upload the files faircamp generates to any webspace - no database and no programming language support (PHP or such) is required. If your webspace supports SSH access, faircamp can be configured to upload your website for you automatically, otherwise you can use FTP or whichever means you prefer to do that manually.

### The Catalog

Your *catalog* is a set of directories with a structure of your choosing, the
only convention you need to follow is that directories that directly contain
audio files will  be presented as *releases* (thinks albums, singles and
playlists) with their own page. Faircamp will automatically gather metadata from
your audio files and make good use of it - if your audio files are properly
tagged and there are cover images within each release directory you will likely
get an end result that is pretty much perfect the first time you run faircamp.

### Configuration

Besides the audio and image files in your catalog faircamp allows you to put
simple text files - so called *manifests* - inside your directories. In these
manifests you can set and override options (e.g. which download formats a
release should have) that are applied to all files within the same directory and
below. So by putting a manifest in the top level directory of your catalog you
can at once set an option for *all* of your releases, and by placing
manifests further down in the directory structure, you can make specific adjustments all the way down to
the *release* (single, album, playlist) level - and within the manifest itself also down to the *track* (single song or
recording within a release) level.

## Current development state

Faircamp already does a lot of things (reading, transcoding, zipping, caching,
rendering, deploying), and for testing purposes faircamp can be stably run on
the main branch. For production usage you might still want to wait though,
faircamp is still more of an advanced prototype and demo.

## Documentation

Mostly complete and accurate but some parts are subject to change.

### Commandline arguments

Consult `faircamp --help` for up-to-date information on that.

### Manifests

To specify metadata and settings create files with the extension `.eno` and any
filename of your choosing anywhere inside the catalog. Each manifest applies to
the folder  it is contained in, as well as (recursively) to all subfolders
therein. Manifests located deeper down in the folder hierarchy can override
metadata and settings specified in manifests in folders above.

```
catalog/
├─ my_top_level_manifest.eno
├─ release_a/
│  ├─ my_release_manifest_a.eno
│  ├─ track_a1.mp3
│  ├─ track_a2.mp3
│  └─ track_a3.mp3
└─ release_b/
   ├─ my_release_manifest_b.eno
   ├─ track_b1.mp3
   ├─ track_b2.mp3
   └─ track_b3.mp3
```

In the example above, everything defined in `my_top_level_manifest.eno` applies
to everything within `release_a` and `release_b`, but
`my_release_manifest_a.eno` can selectively override certain things for
everything in its containing folder `release_a`, as likewise
`my_release_manifest_b.eno` can selectively override certain things for
everything in its containing folder `release_b`.

### Manifest options by example

This demonstrates a subset of available options. None of them are required to get started and depending on your usecase you might only need very few of them in the end as well.

Note that manifest lines such as `# catalog` are not comments but denote
sections (and instead `> these are comments`). For a detailed guide on the
syntax used in the manifest files consult the [eno language
guide](https://eno-lang.org/guide/), simply modifying the examples given below
should likely get you there as well though.

#### Artist

Artists are already automatically created by faircamp from the metadata on tracks,
but if you explicitly define an artist you can specify additional fields like a
permalink or text for the artist. Make sure that the artist name in the track metadata
is the same as the one you specify here in the manifest - this is used to associate
the track's artist with its artist metadata!

```eno
# artist

name: Heston Exchange
permalink: heston-exchange

-- text
Classic Dada-core formation founded in the 90ies.

Only ever known to publicly perform at birthday parties and the gym at their hometown.
-- text
```

#### Catalog

Here you can set the title of your site (which appears at the title of browser
tabs, inside the RSS feed, etc.), as well as the global about page text for your
site.

```eno
# catalog

-- text
My self hosted faircamp site,
which presents some of my awesome music.

Nice of you to stop by!
-- text

title: My awesome music
```

#### Download

By default only streaming is enabled, so you need to specify the `free` or `price` option to enable downloading.

```eno
# download

> Use this to disable downloading for specific releases when it has been enabled in a manifest above in the hierarchy
disabled

> This enables downloading unconditionally without asking for recompensation
free

> All enabled for demonstration purposes (in practice less is recommended, e.g. flac and mp3)
formats:
- aac
- aiff
- flac
- mp3_320
- mp3_v0
- ogg_vorbis
- wav

> This enables downloads behind a soft (i.e. not technically enforced) paycurtain
price: RUB 700+
```

The `price` option accepts an [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code and a price range such as:
- `USD 0+` (Name your price, including zero dollars as a valid option)
- `3.50 EUR` (Exactly 3.50 euros)
- `KRW 9080` (Exactly 9080 south korean won)
- `INR 230+` (230 indian rupees or more)
- `JPY 400-800` (Between 400 and 800 japanese yen)

Note that in practice you won't use `disabled`, `free` and `price` in the same manifest because these options mutually exclude each other, they are just shown here together for demonstration purposes.

#### Embedding

This allows external sites to embed a widget that presents music from your site.
The embed code can be copied from each release page where embedding is enabled.

Embedding is enabled by default. You can use `disabled` to disable it and
`enabled` to re-enable it for specific albums.

```eno
# embedding

disabled
```

#### Feed

You need to specify the base url under which you're hosting your faircamp site in order for the RSS feed to be generated. The image for the feed is optional.

```eno
# feed

base_url: https://myawesomemusic.site/
image: exported_logo_v3.jpg
```

#### Localization

This allows you to configure a language code (used e.g. for the RSS feed
metadata) and more importantly to switch from left-to-right to right-to-left
presentation for e.g. arabic and hebrew scripts.

```eno
# localization

language = he
writing_direction = rtl
```

#### Payment

This sets payment options that are shown when someone wants to buy one of your
releases. For liberapay just provide your account name.

```eno
# payment

liberapay: ThatAwesomeArtist42

-- custom
I'm playing a show at *Substage Indenhoven* on Dec 19th - you can get the
digital album now and meet me at the merch stand in december in person to give
me the money yourself!
-- custom

-- custom
If you're in europe you can send the money via SEPA, contact me at
[lila@thatawesomeartist42.com](mailto:lila@thatawesomeartist42.com) and I'll
send you the account details.
-- custom
```

#### Release

Release artists and titles are automatically derived from audio file metadata,
however as you will possibly want to provide a textual description or tweak
the displayed title and artists for display in the browser, such data can
be provided through the manifests.

```eno
# release

artist: Heston Exchange
image_description: An ink drawing of a barren tree with monkeys in its branches
permalink: ape-affairs-bonus-track-edition
title: Ape Affairs (Bonus Track Edition)
track_numbering: disabled

-- text
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- text
```

If you provide a cover image, `image_description` should be used to provide an
alt text for it. `track_numbering` allows configuration of the numbering style
used - by default it's `arabic` (01 02 03 …) but can be set to `hexadecimal`
(0x01 0x02 0x03 …), `roman` (I II
III …) or `disabled`.

#### Streaming

Always enabled.

You can optionally set the encoding quality for streaming in the browser from `standard` (the default) to `transparent`. The `transparent` option uses significantly more bandwidth (and consequently produces more CO2) and people on slow connections might not wait for your files to load (i.e. you might lose potential future fans that way), therefore it is not recommended to change this without an actual, good reason.

```eno
# streaming

quality: standard
```

#### Theme

You can adjust the visual appearance of your faircamp site to your liking. A
*background_image* can be specified, the *base* theme can be chosen from *dark*
and *light*, the accent color used in the theme can be set through *hue* (0-360)
and a *hue_spread* (e.g. -12, 3, 320) can be defined, which makes the site more
colorful (where 0 = mono-colored). In order for *hue_spread* to have an effect,
make sure to turn up tint_back (0-100) and/or tint_front (0-100) to add a
varyingly strong color tint to either the background, or the elements in the
foreground (most prominently: text).

```eno
# theme

background_image: squiggly_monsters_texture.jpg
base: dark
hue: 13
hue_spread: 0
tint_back: 0
tint_front: 0
```

#### Advanced control over caching strategy

```eno
# cache

optimization: [delayed|immediate|manual|wipe]
```

Faircamp maintains an asset cache that holds the results of all computation-heavy build
artifacts (transcoded audio files, images, and compressed archives). By default this cache uses a delayed optimization strategy: Any asset that is not directly used in a build gets marked as stale and past a certain period (e.g. 24 hours) gets purged from the cache during a follow-up build (if it is not meanwhile reactivated because it's needed again). This strikes a nice balance for achieving instant build speeds during editing (after assets have been generated initially) without inadvertently growing a storage resource leak in a directory you don't ever look at normally.

If you're short on disk space you can switch to `immediate` optimization, which purges stale assets right after each build (which might result in small configuration mistakes  wiping assets that took long to generate as a drawback).

If you're even shorter on disk space you can use `wipe` optimization, which just completely wipes the cache right after each build (so everything needs to be regenerated on each build).

If you're more the structured type you can use  `manual` optimization, which does not
automatically purge anything from the cache but instead prints back reports on stale
assets after each build and lets you use `faircamp --optimize-cache` and `faircamp --wipe-cache` appropriately whenever you're done with your changes and don't expect to generate
any new builds for a while.

## Build

Faircamp compiles on recent stable rust, its only *runtime* requirement is that
you have *FFmpeg* installed, such that `ffmpeg -version` called in a terminal
at any location confirms ffmpeg being available. On Linux you can use your distro's
package manager to install `ffmpeg`, it's readily available on all major distros.

Faircamp has so far only been tested on Linux - architecturally there should be
no blockers for running faircamp on other platforms though (e.g. BSD, maOS, Windows).

**Note that faircamp is still in alpha development and you're running it at your own risk.**

Run this to build and install faircamp on your system:

```bash
cargo install --locked --path .
```

Then run it *inside a directory that contains directories that contain audio files*:

```bash
faircamp
```

With its default settings, faircamp will create a `.faircamp_build` and a `.faircamp_cache` folder inside the directory you called it from. As you might have guessed you will want to open `.faircamp_build/index.html` inside your browser after building is complete.

Run `faircamp -h` to get some help on command line options (there are a few already).

To get faircamp off your system again, simply run:

```bash
cargo uninstall faircamp
```

## License

Faircamp is licensed under the [GPLv3+](https://www.gnu.org/licenses/gpl-3.0.html).

Builds generated with faircamp re-distribute the [Barlow](https://tribby.com/fonts/barlow/) font, licensed under the [OFL 1.1](src/assets/OFL.txt)
