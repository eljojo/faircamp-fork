<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Supported Formats

This documents all audio and image formats that faircamp supports as *input* files.
In other words, these are the file formats you can put into your *catalog*. This page
does *not* document the formats that faircamp can output - this is a different set of
file formats, and documented in the reference (Downloads page).

Are there any formats missing that are important for you? Bring it up in the [issue tracker](https://codeberg.org/simonrepp/faircamp/issues) so we can look into it.

## Supported audio formats

- aiff
- alac
- flac
- mp3
- ogg
- opus
- wav

## Supported image formats

- gif
- heif (*)
- jpg/jpeg
- png
- webp

(*) only when compiled with libvips - you can run `faircamp --version` to see
if your installed faircamp build was compiled with libvips.