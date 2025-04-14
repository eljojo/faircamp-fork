<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Building from source

This provides simple, copy/pasteable build instructions for mainstream linux
distributions and macOS. For BSD, Windows or other OSes orient yourself along
the general instructions in "Other platforms" at the bottom of the page.

## Arch Linux, Manjaro

Install all required dependencies (if you manually installed rust via [rustup](https://rustup.rs/) remove it from the list):

```bash
sudo pacman -S cmake gcc git ffmpeg libvips opus rust
```

Now check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Debian 12, elementary OS 8, Linux Mint 22, Ubuntu 23.04 - 24.10

Install rust through the official [rustup](https://rustup.rs/) installer,
then install all required dependencies through the package manager:

```bash
sudo apt install cmake ffmpeg gcc git libopus-dev libvips-dev
```

Now check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Debian 11, elementary OS 7, Linux Mint 21, Ubuntu 22.04 LTS - 22.10

Install rust through the official [rustup](https://rustup.rs/) installer,
then install all required dependencies through the package manager:

```bash
sudo apt install cmake ffmpeg gcc git
```

Now check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features image --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Fedora 38

Install all required dependencies (if you manually installed rust via [rustup](https://rustup.rs/) remove it from the list):

```bash
sudo dnf install cmake ffmpeg-free gcc git opus-devel rust vips-devel
```

Now check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Fedora 36 - 37, CentOS, RHEL

> CentOS and RHEL have not been tested, but technically should work
> the same as Fedora - please report if there are any issues.

Install all required dependencies (if you manually installed rust via [rustup](https://rustup.rs/) remove it from the list):

```bash
sudo dnf install cmake ffmpeg-free gcc git opus-devel rust
```

Check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features image --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Alpine Linux

> Alpine Linux has not been tested, but technically should work
> as long as there's no major oddities between musl and glibc -
> please report if there are any issues.

Install all required dependencies (if you manually installed rust
via [rustup](https://rustup.rs/) remove it from the list):

```bash
doas apk add alpine-sdk rust cargo ffmpeg opus-dev vips-dev
```

Check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## FreeBSD

First install the required dependencies via FreeBSD's default package manager (ffmpeg, rust, vips).

Now check out, build and install faircamp:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --locked --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## macOS

First install Homebrew by following the instructions on https://brew.sh.

Now you can build and install faircamp with a single command:

```bash
brew install faircamp
```

## Other platforms

If you attempt the build process on not yet covered platforms, it would be
much appreciated if you submit your findings to this document so it can help others.
Also, likewise, if you run into issues it would be great if you provide a
report about it in the [issue tracker](https://codeberg.org/simonrepp/faircamp/issues)
so we can resolve it for everyone - thank you!

What you need to have installed to build and run faircamp:
- Rust
  - Version 1.76.0 or later when compiling with `--feature libvips`
  - Version 1.79.0 or later when compiling with `--feature image`
- ffmpeg (any somewhat recent version should work)
- libopus (often already provided by ffmpeg)
- libvips 8.13.3 or later (optional)

**1. Install required dependencies**

Install [rust](https://rust-lang.org) on your system following the official [installation instructions](https://www.rust-lang.org/tools/install).

Install [ffmpeg](https://ffmpeg.org) on your system, see [this page](https://ffmpeg.org/download.html) for instructions for various platforms. 

Install [libopus](https://opus-codec.org/), e.g. from [this page](https://opus-codec.org/downloads/).

**2. Optionally install libvips**

Faircamp will run perfectly fine without libvips, but compiling with libvips
adds certain image processing benefits:

- Significantly faster and more robust image processing
- Slightly better image quality
- Support for HEIF images

As installing libvips (and the right version of it - at least
[v8.13.3](https://github.com/libvips/libvips/releases/tag/v8.13.3))
can be quite difficult, you might want to skip this step, in this case
just move to the next section.

Installation instructions can be found [here](https://www.libvips.org/).
Make sure to install both the library and its header files.

**3. Run the compilation/installation command**

Now you're ready to build and install faircamp on your system.

First check out and enter the repository:

```bash
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
```

If you **skipped** the installation of libvips run this command:

```bash
cargo install --features image --locked --path .
```

If you successfully installed libvips [v8.13.3](https://github.com/libvips/libvips/releases/tag/v8.13.3) (or later) run this command:

```bash
cargo install --features libvips --locked --path .
```

**Uninstalling**

If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Additional build options

Both the `faircamp` application and the `manual` can be built with an override
package version, which is primarily used to distribute prerelase builds and
documentation for testing with a temporary updated version such as `2.0.0~pre1`:

```
FAIRCAMP_VERSION=2.0.0~pre1 cargo build --features libvips --locked
```


