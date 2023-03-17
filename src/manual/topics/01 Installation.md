# Installation

Installation from source is currently the way to go. Official packages will be added here as soon as they are available!

## Installing from source

Faircamp compiles on stable rust, available e.g. via [rustup](https://rustup.rs).

Two (\*) external dependencies need to be installed (if not already installed):

1. Development files for `libvips`
 - On Debian, Ubuntu, etc. install with `sudo apt install libvips-dev`
 - On Arch install with `sudo pacman -S libvips`
 - On Manjaro install with `sudo pamac install libvips`
 - (\*) If installing libvips is an obstacle on your OS, check out and compile the [integrate-image-crate](https://codeberg.org/simonrepp/faircamp/src/branch/integrate-image-crate) branch instead, it replaces libvips with a pure-rust alternative.

2. `ffmpeg` command-line application
  - On Linux you can use your distro's package manager to install `ffmpeg` on all major distros.
  - Otherwise consult <https://ffmpeg.org/download.html>

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