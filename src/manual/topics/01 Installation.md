# Installation

Installation from source is currently the way to go. Official packages will be added here as soon as they are available!

## Installing from source

Faircamp compiles on recent stable rust, there are two external dependencies
you need to install (if not already present on your system): 

- For compilation to succeed you need `libvips` on your system. On debian
  based systems (Debian, Ubuntu, etc.) you can run `sudo apt install libvips-dev` to install it.
- As a purely *runtime* dependency, *FFmpeg* needs to be installed, such that `ffmpeg -version` called in a terminal
  at any location confirms ffmpeg being available. On Linux you can use your distro's
  package manager to install `ffmpeg`, it's readily available on all major distros.

Faircamp has so far only been tested on Linux - architecturally there should be
no blockers for running faircamp on other platforms though (e.g. BSD, maOS, Windows).

Run this to build and install faircamp on your system:

```bash
cargo install --locked --path .
```

If you want to uninstall faircamp again at any point, simply run:

```bash
cargo uninstall faircamp
```