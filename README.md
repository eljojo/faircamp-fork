# Faircamp (codename)

In early prototype stage - see the casual [announcement on mastodon](https://post.lurk.org/@freebliss/105685776449364587).

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
