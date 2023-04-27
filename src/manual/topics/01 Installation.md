# Installation

For debian-based Linux (Debian, Ubuntu, etc.) faircamp can already be installed via packages.
For Arch Linux, Manjaro and RPM-based distros simple copy and paste build/install instructions are available.
On BSD, Mac and Windows you are entering uncharted territory, but in general there should be no hard blockers
for running faircamp on these platforms either.

## Arch Linux, Manjaro

Install rust through the official [rustup](https://rustup.rs/) installer,
then install all required dependencies through the package manager:

```
sudo pacman -S base-devel ffmpeg
```

Now check out, build and install faircamp:
```
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Debian 12, Ubuntu 23.04

Download and install the
[Debian 12 package](https://simonrepp.com/faircamp/packages/faircamp_0.1.0-1+deb12_amd64.deb)

## Debian 11, Ubuntu 22.04 LTS - Ubuntu 22.10

Download and install
[Debian 11 package](https://simonrepp.com/faircamp/packages/faircamp_0.1.0-1+deb11_amd64.deb)

## Fedora 38

Install rust through the official [rustup](https://rustup.rs/) installer,
then install all required dependencies through the package manager:

```
sudo dnf install cmake ffmpeg-free gcc git opus-devel vips-devel
```

Now check out, build and install faircamp:
```
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Fedora 36 - 37, CentOS, RHEL

> CentOS and RHEL have not been tested, but technically should be the same as
> older Fedora releases - please report if there are any issues.

Install rust through the official [rustup](https://rustup.rs/) installer.

Install the dependencies: 

```
sudo dnf install cmake ffmpeg-free gcc git opus-devel
```

Check out, build and install faircamp:
```
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
cargo install --features libvips --path .
```

That's it! If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```

## Installing from source on other platforms

If you attempt the build process on not yet covered platforms and run into
issues it would be fantastic if you provide a report about it in the
[issue tracker](https://codeberg.org/simonrepp/faircamp/issues) so we can
resolve it for you and others - thank you!

What you need to have installed to build and run faircamp:
- Rust 1.64.0 or later
- libopus
- ffmpeg
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

```
git clone https://codeberg.org/simonrepp/faircamp.git
cd faircamp
```

If you **skipped** the installation of libvips run this command:

```bash
cargo install --features image --path .
```

If you successfully installed libvips [v8.13.3](https://github.com/libvips/libvips/releases/tag/v8.13.3) (or later) run this command:

```bash
cargo install --features libvips --path .
```

**Uninstalling**

If you want to uninstall faircamp at any point, run:

```bash
cargo uninstall faircamp
```