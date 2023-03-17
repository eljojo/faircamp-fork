# Faircamp

A static site generator for audio producers

<img src="https://simonrepp.com/faircamp/release_mobile.png?0001"
     alt="Screenshot of Faircamp (Mobile Layout)"
     style="max-height: 30rem;" />

For more screenshots see the [website](https://simonrepp.com/faircamp).

Faircamp adheres to these principles: Personal, plain, small, simple, light, fast, reduced, elegant, stable, low/no-maintenance, free, independent, privacy-respecting, standards-conforming, no-nonsense

Curious? Read on!

## Overview

Point Faircamp to a folder hierarchy on your disk which contains your audio
files. Within minutes, Faircamp builds a complete, static website that
presents your music to your audience. You can view the site on your computer
or upload it to any webhost - no database, no php or such required.

By default, visitors can browse and stream your music. You can enable more
features: Downloads, Embeds, Soft Paycurtain, Unlock codes for Downloads, RSS
Feed, etc.. You can also provide text descriptions for your releases, adjust
the theme of your site and so on, this is all done in *manifests*, simple
text files you place next to your audio files.

If your webspace supports SSH access, faircamp can be configured to upload
your website for you automatically, otherwise you can use FTP or whichever
means you prefer to do that manually.

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
release should have) that are applied to all files within the same directory
and below (\*). So by putting a manifest in the top level directory of your
catalog you can at once set an option for *all* of your releases, and by
placing manifests further down in the directory structure, you can make
specific adjustments all the way down to the *release* (single, album,
playlist) level - and within the manifest itself also down to the *track*
(single song or recording within a release) level.

(\*) Note that a few select options do not propagate to other folders as it
would make no sense, e.g. a release's permalink must be unique and therefore
makes no sense to propagate.

## Faircamp is in Beta

Faircamp can be stably built and run from the main branch. Feature and
design improvements are usually rolled out piece by piece, or in larger waves every few
weeks. Occcasional glitches in the interface might occur, sometimes only temporarily between updates.
Some more advanced features are only half-way implemented (see below).
Technically nothing about the catalog format is set in stone, but practically
speaking actual changes have been few and far between. Faircamp is not
production-grade software, but in a very usable state, in steady development.

These features are knowingly incomplete right now:

- Embeds (they are available but display as a garbled mess for now)
- Buy page (functionally there but rather bare in usability still)
- No-javascript mode (faircamp sites work without javascript too, some things still need to be wrapped up though)

## Documentation

You can access the full documentation at https://simonrepp.com/faircamp/manual

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

**Note that you are running faircamp at your own risk** - Pay close attention when setting custom paths for the build and cache directories as these directories get wiped as part of faircamp's standard operation.

Run this to build and install faircamp on your system:

```bash
cargo install --locked --path .
```

Then run it *inside a directory that contains directories that contain audio files*:

```bash
faircamp
```

With its default settings, faircamp will create a `.faircamp_build` and a `.faircamp_cache` folder inside the directory you called it from. Open `.faircamp_build/index.html` inside your browser after building is complete (the `--preview` flag can also be used).

Run `faircamp -h` to get some help on command line options (there are quite a few).

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

