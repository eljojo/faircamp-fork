# Faircamp (codename)

A self-hostable, statically generated bandcamp alternative — *[as recently announced on mastodon](https://post.lurk.org/@freebliss/105685776449364587)*

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

Faircamp already does a lot of things (reading, transcoding, zipping, caching, rendering, deploying), and for testing purposes faircamp can be stably run on the main branch. For anything productive you will want to wait a few more weeks though, as as of yet faircamp also does not yet do anything really well - for now it's a prototype and a demo!

## Early Documentation

This documentation is still incomplete, in parts potentially inaccurate and subject to change.

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

### Example manifest

Note that this demonstrates only a subset of available options, and because it is a demonstration, more options than you will usually see in a manifest:

```eno
> Sets the URL under which you intend to host faircamp, only used for RSS feed generation
base_url: https://myawesomemusic.site/

> Sets the global about page text for your site
catalog_text: My self hosted faircamp site, which presents some of my awesome music. Nice of you to stop by!

> Sets the title of your site, appears at the title of browser tabs, inside the RSS feed, etc.
catalog_title: My awesome music

> Enable all download formats for demonstration purposes (in practice less is recommended, e.g. flac + mp3_v0 + ogg_vorbis)
download_formats:
- aac
- aiff
- flac
- mp3_320
- mp3_v0
- ogg_vorbis
- wav

> This enables downloading of (a) release(s) behind a soft - i.e. not technically enforced - paycurtain (by default only streaming is enabled)
> (This overrides disable_download & free_download and in practice would not be used along side these in the same manifest)
>
> This setting accepts (in any order) a currency code (ISO 4217 [1]) and a price range as in these examples:
> USD 0+ (Name your price, including zero dollars as a valid option)
> 3.50 EUR (Exactly 3.50 euros)
> KRW 9080 (Exactly 9080 south korean won)
> INR 230+ (230 indian rupees or more)
> JPY 400-800 (Between 400 and 800 japanese yen)
>
> [1] https://en.wikipedia.org/wiki/ISO_4217
download_price: RUB 700+

> This can be used to disable downloading (both free & paid) for specific releases (when it has been enabled in a manifest above in the hierarchy)
> (This overrides download_price & free_download and in practice would not be used along side these in the same manifest)
disable_download

> This enables downloading of (a) release(s) unconditionally without mention of financial recompensation (by default only streaming is enabled)
> (This overrides download_price & disable_download and in practice would not be used along side these in the same manifest)
free_download

> This sets payment options that are shown when someone wants to buy one of your releases (for liberapay just provide your account name)
payment_options:
custom = I'm playing a show at *Substage Indenhoven* on Dec 19th - you can get the digital album now and meet me at the merch stand in december in person to give me the money yourself!
custom = If you're in europe you can send the money via SEPA, contact me at [lila@thatawesomeartist42.com](mailto:lila@thatawesomeartist42.com) and I'll send you the account details
liberapay = ThatAwesomeArtist42

> Sets the encoding quality of the files people hear when listening in the browser (standard or transparent).
streaming_quality: standard

> You can choose between a dark and light visual theme
theme: light
```


## Build

Faircamp compiles on recent stable rust, its only *runtime* requirement is that
you have *FFmpeg* installed, such that `ffmpeg -version` called in a terminal
at any location confirms ffmpeg being available. On Linux you can use your distro's
package manager to install `ffmpeg`, it's readily available on all major distros.

Faircamp has so far only been tested on Linux - architecturally there should be
no blockers for running faircamp on other platforms though (e.g. BSD, maOS, Windows).

**Note that faircamp is still in early development and *might* do bad things - delete/overwrite existing files, create tons of files - if you run into unlucky circumstances. For the time being you're running faircamp completely at your own risk.**

Run this to build and install faircamp on your system:

```bash
cargo install --path .
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