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
git checkout https://codeberg.org/simonrepp/faircamp.git
```

Switch into the repository root:

```bash
cd faircamp
```

If you haven't yet, set up [rust](https://rust-lang.org) on your system following the official [installation instructions](https://www.rust-lang.org/tools/install).

If not yet available, install [libvips](https://www.libvips.org/) on your system. A few pointers for different distros are provided below. If you have serious troubles with this step, please read on, the next step offers a workaround.

```bash
sudo apt install libvips-dev  # Debian, Ubuntu, etc.
sudo pacman -S libvips        # Arch
sudo pamac install libvips    # Manjaro
``` 

If installing libvips is an obstacle on your OS (e.g. because you are on Windows) you can
also use an alternative build step here. Check out the [integrate-image-crate](https://codeberg.org/simonrepp/faircamp/src/branch/integrate-image-crate) branch, which replaces the
libvips based image processing with a pure-rust library:

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