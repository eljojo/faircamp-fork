# Faircamp (codename)

A self-hostable, statically generated bandcamp alternative — *[see screenshots at simonrepp.com/faircamp](https://simonrepp.com/faircamp)*

## Concept Overview

Faircamp takes a directory on your disk - your *Catalog* - and from it produces a
fancy-looking (and technically simple and completely *static*) website, which
presents your music in a way similar to how popular commercial service
*bandcamp* does it.

You can upload the files faircamp generates to any webspace - no database and no programming language support (PHP or such) is required. If your webspace supports SSH access, faircamp can be configured to upload your website for you automatically, otherwise you can use FTP or whichever means you prefer to do that manually.

### The Catalog

Your *catalog* is a set of directories with a structure of your choosing, the
only convention you need to follow is that directories that directly contain
audio files will  be presented as *releases* (think albums, singles and
playlists) with their own page. Faircamp will automatically gather metadata from
your audio files and make good use of it - if your audio files are properly
tagged and there are cover images within each release directory you will likely
get an end result that is pretty much spot-on the first time you run faircamp.

### Configuration

Besides the audio and image files in your catalog faircamp allows you to put
simple text files - so called *manifests* - inside your directories. In these
manifests you can set and override options (e.g. which download formats a
release should have) that are applied to all files within the same directory and
below (\*). So by putting a manifest in the top level directory of your catalog you
can at once set an option for *all* of your releases, and by placing
manifests further down in the directory structure, you can make specific adjustments all the way down to
the *release* (single, album, playlist) level - and within the manifest itself also down to the *track* (single song or
recording within a release) level.

(\*) Note that a few select options do not propagate to other folders as it
would make no sense, e.g. a release's permalink must be unique and therefore
must not be applied twice.

## Current development state

Faircamp already does a lot of things (reading, transcoding, zipping, caching,
rendering, deploying), and for testing purposes faircamp can be stably run on
the main branch. For production usage you might still want to wait though,
faircamp is still more of an advanced prototype and demo.

## Documentation

Mostly complete and accurate but keep in mind that things are still being developed and in motion.

### Commandline arguments

Consult `faircamp --help` for the most authoritative and up-to-date information on available arguments.

That said here's a glimpse at some particularly interesting ones:
- `--build-dir <BUILD_DIR>` Override build directory (default is .faircamp_build/ inside the current working directory)
- `--cache-dir <CACHE_DIR>` Override cache directory (default is .faircamp_cache/ inside the current working directory)
- `--catalog-dir <CATALOG_DIR>` Override catalog directory (default is the current working directory)
- `--exclude <EXCLUDE_PATTERNS>` Excludes all file paths that contain the specified pattern from being processed. Multiple can be supplied. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--include <INCLUDE_PATTERNS>` Pass this so only file paths that contain the specified pattern will get processed. Multiple can be supplied. Matching is done by simple case-sensitive string comparison - no glob/regex
- `--no-clean-urls` Generate full links, e.g. "/my-album/index.html" instead of "/my-album/". Creates a build that is fully browsable from your local disk without a webserver
- `--preview` Opens the built site locally in your browser after building so you can check it out
- `--theming-widget` Injects a small widget into the page which allows you to interactively explore different theme color configurations (see section `Theme` further below)


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

Artists are automatically created by faircamp when they are encountered in
audio file metadata (e.g. the artist "Alice" will be created if any ID3 tag
says a track is by "Alice"). To add further information to an artist, you can
expliclity define it in a manifest. The name you assign will be used to match
the explicitly defined artist (by you in the manifest) to the implicitly
defined one (by the audio file metadata) so pay close attention that both are
written the same (NB: lowercase/uppercase is ignored for matching). If
(as often happens) different audio files use slightly different versions of
an artist name (e.g. "Motörhead" vs. "Motorhead"), or the artist appears in a
collaboration (e.g. "Alice (feat. Bob)"), you can additionally specify
`aliases` that will also be matched against to map the artist to the right
tracks.

```eno
# artist

name: Heston Exchange
permalink: heston-exchange

aliases:
- heston exchange
- Heston Exchange (feat. Bar Foo)

image:
description = All four bandmembers against a bright blue sky, wearing pink velvet top-hats
file = heston_exchange.jpg

-- text
Classic Dada-core formation founded in the 90ies.

Only ever known to publicly perform at birthday parties and the gym at their hometown.
-- text
```

Note that the `text` field supports markdown.

#### Catalog

By default faircamp operates in "single artist mode", i.e. it will lay out and
render the pages in a way that best fits a single artist/band presenting
their works, meaning it will automatically take the artist associated
with the highest number of releases/tracks and name the catalog after them,
make the catalog description the description of that artist, etc..

The `label_mode` flag can be used if one wants to present multiple artists
on a single faircamp site. This adds an additional layer of information to the
page that differentiates the artists, gives them each their own page, etc.

Asides this main mode toggle you can set the global site title (which appears
at the title of browser tabs, inside the RSS feed, etc.), the base url
(required for generation of embeds and the RSS feed), an optional RSS feed
image, as well as a description text for your catalog here.

Lastly, the `rotate_download_urls` flag can be specified to let faircamp
generate new download urls on each deployment (rendering invalid all
previously existing urls), which helps you to fight blatant hotlinking to
your downloads, should it ever occur. Similarly, you can specify
`freeze_download_urls: [put-any-text-here]`, to manually control the
invalidation of download urls: Whatever text you put on the right is used to
generate unique download urls on each deployment (note that the text itself
never shows up in the urls themselves, it is merely used for randomization).
The download urls stay valid as long as the text does not change. Any time
you update the text, all download urls are refreshed, and thereby all old
ones invalidated. Practically speaking, it makes sense to use some kind of
(current) calendar data as the text on the right, that way e.g.
`freeze_download_urls: 1 April 2022` could tell you that your current download
urls have been valid since that day. You could also use "October 2022" or
even just the year, given that one usually will not manually invalidate the
urls on a daily basis.

```eno
# catalog

base_url: https://myawesomemusic.site/
feed_image: exported_logo_v3.jpg
label_mode
title: My awesome music

-- text
My self hosted faircamp site,
which presents some of my awesome music.

Nice of you to stop by!
-- text
```

#### Download

By default your visitors can only stream your releases. There are three
mutually exclusive download modes you can enable for your releases:

1. `free` – Free download

    For example, to enable free downloads in Opus format:

    ```eno
    # download

    free

    format: opus
    ```

2. `code` or `codes` – An unlock code (like a coupon/token) needs to entered to access downloads

    For example, enabling FLAC and Opus downloads for people who received your download code "crowdfunding2023!" for backing you:

    ```eno
    # download

    code: crowdfunding2023!

    formats:
    - flac
    - opus
    ```

    Or for example, if you have subscribers in multiple tiers, you can configure access with multiple codes:

    ```eno
    # download

    codes:
    - GOLDsupporter
    - SILVERsupporter

    formats:
    - mp3
    - opus
    ```

3. `price` – A soft (i.e. not technically enforced) paycurtain needs to be passed before downloading

    For example in order to ask for 4€ for accessing the FLAC downloads on a release:

    ```eno
    # download

    format: flac

    price: EUR 4+
    ```

    The `price` option accepts an [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code and a price range such as:

    - `USD 0+` (Name your price, including zero dollars as a valid option)
    - `3.50 GBP` (Exactly 3.50 Pounds)
    - `KRW 9080` (Exactly 9080 south korean won)
    - `INR 230+` (230 indian rupees or more)
    - `JPY 400-800` (Between 400 and 800 japanese yen)

    In conjunction with this mode you will also need to specify at least one payment option, see the readme section on "Payment" below.

4. `disabled` – Disable downloads for specific releases when they have been enabled in a manifest above in the hierarchy

    ```eno
    # download

    disabled
    ```

Lastly here's a listing of all download formats you can currently enable. In
practice a minimal lossless/lossy combination is recommended, e.g. `flac` and
`opus`. Note that `opus` is an alias for `opus_128`.

```eno
formats:
- aac
- aiff
- flac
- mp3
- ogg_vorbis
- opus
- opus_48
- opus_96
- opus_128
- wav
```

#### Embedding

This allows external sites to embed a widget that presents music from your site.
The embed code can be copied from each release page where embedding is enabled.

Embedding is enabled by default. You can use `disabled` to disable it and
`enabled` to re-enable it for specific albums.

```eno
# embedding

disabled
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

By default faircamp strips all metadata off the audio files that you supply
when it transcodes them for streaming and downloading, only adding back
those tags that it needs and manages itself, i.e. the title, artist(s),
release artist(s) and release title. The `rewrite_tags` option lets you
control this: Set it to 'no' and faircamp will transfer all tags 1:1 from
the source files onto the transcoded files, as you provided them.

```eno
# release

artist: Heston Exchange
permalink: ape-affairs-bonus-track-edition
rewrite_tags: no
title: Ape Affairs (Bonus Track Edition)
track_numbering: disabled

cover:
description = An ink drawing of a barren tree with monkeys in its branches
file = cover.jpg

-- text
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- text
```

If you provide a cover image, use `description` to include an image description
for it. `track_numbering` allows configuration of the numbering style
used - by default it's `arabic` (01 02 03 …) but can be set to `hexadecimal`
(0x01 0x02 0x03 …), `roman` (I II
III …) or `disabled`.

#### Streaming

Always enabled.

You can optionally set the encoding quality for streaming from `standard`
(the default) to `frugal`. This uses 1/3 less bandwidth, reduces emissions
and improves load times for listeners, especially on slow connections.

```eno
# streaming

quality: frugal
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

Note that there is a `--theming-widget` CLI option that lets you interactively
explore different settings for `hue`, `hue_spread`, `tint_back` and
`tint_front`. Just build your catalog with the option enabled and open it in
the browser - the page will then contain the theming widget.

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

Faircamp compiles on recent stable rust, there are two external dependencies
you need to install (if not already present on your system): 

- For compilation to succeed you need `libvips` on your system. On debian
  based systems (Debian, Ubuntu, etc.) you can run `sudo apt install libvips-dev` to install it.
- As a purely *runtime* dependency, *FFmpeg* needs to be installed, such that `ffmpeg -version` called in a terminal
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

If you tried out previous versions of faircamp before and find that running an
updated version crashes when you tried to re-build a previously built site, this
is most likely due to incompatible cache data - simply delete the `.faircamp_cache`
folder and try again. If the problem persists do open an issue, I'm happy to figure
it out together with you and improve stability for all users.

To get faircamp off your system again, simply run:

```bash
cargo uninstall faircamp
```

## License

Faircamp is licensed under the [AGPL-3.0-or-later](https://spdx.org/licenses/AGPL-3.0-or-later.html).

Builds generated with faircamp re-distribute the [Barlow](https://tribby.com/fonts/barlow/) font, licensed under the [OFL 1.1](src/assets/OFL.txt)

## Faircamp Alternatives

If you're looking for a bandcamp alternative, but faircamp does not tick your
boxes, here are some faircamp alternatives for you to explore:

- [Ampled](https://www.ampled.com/) – «A Co-op For Musicians. Collectively owned, community supported.»
- [blamscamp](https://suricrasia.online/blamscamp/) – «create a bandcamp-style audio player for selling albums on itch.io.»
- [CD-R 700mb](https://github.com/thebaer/cdr) – «Static site generator for making web mixtapes in 2022.»
- [Funkwhale](https://funkwhale.audio/) – «Funkwhale is a community-driven project that lets you listen and share music and audio within a decentralized, open network. »
- [Rauversion](https://github.com/rauversion/rauversion-phx) – «Rauversion is an open source music sharing platform.»

