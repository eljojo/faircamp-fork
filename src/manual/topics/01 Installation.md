# Installation

Installation from source is currently the way to go. Official packages will be added here as soon as they are available.

## Installing from source

A quick overview of what we will do here:

1. Check out the repository
2. Install [rust](https://rust-lang.org) and [ffmpeg](https://ffmpeg.org)
3. Optionally install [libvips](https://www.libvips.org/)
4. Run the compilation/installation command

On a standard linux distro these steps should take a few minutes at most. If
you're on macOS or Windows you're entering uncharted territory, because:

> Heads up: Faircamp has so far only been tested on Linux - generally there should be
> no blockers for running faircamp on other platforms though (e.g. BSD, macOS, Windows).

If you attempt the build process on macOS or Windows and run into issues it would be fantastic if you provided a report about it in the [issue tracker](https://codeberg.org/simonrepp/faircamp/issues), so we can resolve it for you and others.

And now without further ado, installing from source:

**1. Check out the repository**

Open your terminal and check out the repository:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
```

Switch into the repository root:

```bash
cd faircamp
```

**2. Install rust and ffmpeg**

If you haven't yet, set up [rust](https://rust-lang.org) on your system following the official [installation instructions](https://www.rust-lang.org/tools/install).

If not yet installed, install [ffmpeg](https://ffmpeg.org) on your system. A few pointers for different distros:

```bash
sudo apt install ffmpeg    # Debian, Ubuntu, etc.
sudo dnf install ffmpeg    # Fedora
sudo pacman -S ffmpeg      # Arch
sudo pamac install ffmpeg  # Manjaro
```

Check out [this page](https://ffmpeg.org/download.html) for further options how to install ffmpeg on your OS.

**3. Optionally install libvips**

Faircamp will run perfectly fine without libvips, but compiling with libvips
adds certain image processing benefits:

- Significantly faster and more robust image processing
- Slightly better image quality
- Support for HEIF images

As installing libvips (and the right version of it - at least
[v8.13.3](https://github.com/libvips/libvips/releases/tag/v8.13.3))
can be quite difficult, you might want to skip this step.

On **Arch**, **Manjaro** and **Debian 12** it is currently recommended to attempt
a build with libvips (feel free to report successes on other OSes so it can
be documented here for everyone).

On **Debian 11** it is **not** recommended to attempt a build with libvips, as
its packaged version of libvips is too old. (feel free to report barriers on
further OSes so they can be documented here for everyone).

If you want to skip this, go straight to **4.**, otherwise now install
[libvips](https://www.libvips.org/) (including its header files) on your
system. Many linux distributions offer separate packages for the library and
for the headers (e.g. on debian there is a `libvips` and `libvips-dev`
package) - it is vitally important to install both packages in this case
(a few pointers for different distros are provided below).

```bash
sudo apt install libvips-dev  # Debian, Ubuntu, etc.
sudo pacman -S libvips        # Arch
sudo pamac install libvips    # Manjaro
``` 

**4. Run the compilation/installation command**

Now you're ready to build and install faircamp on your system!

If you **skipped** the installation of libvips run this command:

```bash
cargo install --features image --locked --path .
```

If you successfully installed libvips [v8.13.3](https://github.com/libvips/libvips/releases/tag/v8.13.3) (or later) run this command:

```bash
cargo install --features libvips --locked --path .
```

**Uninstalling**

If you want to uninstall faircamp again at any point, simply run:

```bash
cargo uninstall faircamp
```