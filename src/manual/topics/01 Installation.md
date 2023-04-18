# Installation

Installation from source is currently the way to go. Official packages will be added here as soon as they are available.

## Installing from source

A quick overview of what we will do here:

1. Check out the repository
2. Install (up to) three common dependencies: [ffmpeg](https://ffmpeg.org), [libvips](https://www.libvips.org/), [rust](https://rust-lang.org)
3. Run the compilation/installation command

On a standard linux distro these steps should take a few minutes at most. If
you're on macOS or Windows you're entering uncharted territory, because:

> Heads up: Faircamp has so far only been tested on Linux - generally there should be
> no blockers for running faircamp on other platforms though (e.g. BSD, macOS, Windows).

If you attempt the build process on macOS or Windows and run into issues it would be fantastic if you provided a report about it in the [issue tracker](https://codeberg.org/simonrepp/faircamp/issues), so we can resolve it for you and others.

And now without further ado, installing from source:

Open your terminal and check out the repository:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
```

Switch into the repository root:

```bash
cd faircamp
```

If you haven't yet, set up [rust](https://rust-lang.org) on your system following the official [installation instructions](https://www.rust-lang.org/tools/install).

The upcoming step that now follows can be tricky (or even impossible) on some
systems, for this reason an alternative install path is provided as well. If
you run into troubles with this next step, or find that your system is not
compatible with the requirements, simply skip it - a workaround is
described right after.

So next, you need to install [libvips](https://www.libvips.org/) (including its
header files) on your system. Many linux distributions offer separate
packages for the library and for the headers (e.g. on debian there is a
`libvips` and `libvips-dev` package) - it is vitally important to install
both packages in this case (a few pointers for different
distros are provided below).

Lastly note that there is also a minimum version requirement for libvips ([v8.12.0](https://github.com/libvips/libvips/releases/tag/v8.12.0) at least).
On some slower moving linux distributions this might pose a problem
(e.g. debian-11 does not provide a sufficiently up-to-date libvips version
unfortunately). If fulfilling this version requirement is not possible on
your platform, just like if installing libvips at all poses a problem on your
platform, skip ahead to the workaround described below.

```bash
sudo apt install libvips-dev  # Debian, Ubuntu, etc.
sudo pacman -S libvips        # Arch
sudo pamac install libvips    # Manjaro
``` 

If installing (a recent enough version) libvips is an obstacle on your OS
(e.g. because you are on Windows or on a slow moving linux distro) you can
also use an alternative build step here. Check out the [integrate-image-crate](https://codeberg.org/simonrepp/faircamp/src/branch/integrate-image-crate)
branch, which replaces the libvips based image processing with a pure-rust
library:

```bash
git checkout integrate-image-crate  # only run this if you couldn't install libvips above (!)
```

If not yet installed, install [ffmpeg](https://ffmpeg.org) on your system. A few pointers for different distros:

```bash
sudo apt install ffmpeg    # Debian, Ubuntu, etc.
sudo dnf install ffmpeg    # Fedora
sudo pacman -S ffmpeg      # Arch
sudo pamac install ffmpeg  # Manjaro
```

Check out [this page](https://ffmpeg.org/download.html) if you don't know how to install ffmpeg on your OS.

Finally, run this to build and install faircamp on your system:

```bash
cargo install --locked --path .
```

If you want to uninstall faircamp again at any point, simply run:

```bash
cargo uninstall faircamp
```