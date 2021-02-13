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

Faircamp already does a lot of things (reading, transcoding, zipping, caching, rendering, deploying), and for testing purposes it's already fully usable. For anything productive you will want to wait a few more weeks though, as as of yet faircamp also does not yet do anything really well - for now it's a prototype and a demo!

## Build

Faircamp compiles on recent stable rust, its only *runtime* requirement is that
you have *FFmpeg* installed, such that `ffmpeg -version` called in a terminal
at any location confirms ffmpeg being available. On Linux you can use your distro's
package manager to install `ffmpeg`, it's readily available on all major distros.

Faircamp has so far only been tested on Linux - macOS probably works too,
Windows likely not (yet), although it would likely only require minor
modifications at this point.

**Note that faircamp is still in early and fast development and *might* do bad things - delete/overwrite your existing files, create tons of new files - if you run into unlucky circumstances. For the time being you're running faircamp completely at your own risk.**

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