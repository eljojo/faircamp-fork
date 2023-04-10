# Faircamp

A static site generator for audio producers

<img src="https://simonrepp.com/faircamp/readme.png?1" alt="Three faircamp site screenshots"/>

For more screenshots and a feature overview see the [website](https://simonrepp.com/faircamp).

Already set on using faircamp? Then jump right into the [manual](https://simonrepp.com/faircamp/manual).

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

- Embeds (available but incomplete implementation)
- Buy page (functionally there but rather bare in usability still)
- No-javascript mode (faircamp sites work without javascript too, some things still need to be wrapped up though)

## Documentation

See faircamp's comprehensive [Manual](https://simonrepp.com/faircamp/manual).

## Build/Install

See *Installing from source* in the [Installation](https://simonrepp.com/faircamp/manual/installation.html) guide in the [Manual](https://simonrepp.com/faircamp/manual). If the online version of the manual should be inaccessible, the same information is also available in the repository itself, in [01 Installation.md](https://codeberg.org/simonrepp/faircamp/src/branch/main/src/manual/topics/01%20Installation.md).

## License

Faircamp is licensed under the [AGPL-3.0-or-later](https://spdx.org/licenses/AGPL-3.0-or-later.html).

Builds generated with faircamp re-distribute the [Barlow](https://tribby.com/fonts/barlow/) font, licensed under the [OFL 1.1](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL).

The faircamp manual re-distributes the [Fira Mono](https://github.com/mozilla/Fira) and [Titillium Web](http://nta.accademiadiurbino.it/titillium/) fonts, licensed under the [OFL 1.1](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL).
