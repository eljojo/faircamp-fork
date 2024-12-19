<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Artists

Artists are automatically created by faircamp when they are encountered in
audio file metadata (e.g. the artist "Alice" will be created if any ID3 tag
says a track is by "Alice"). To add further information to an artist, you need
to explicitly define the artist, which can be done in two ways:

For one, you can use the `artist` field inside a `catalog.eno` or `release.eno`
manifest, which is primarily intended as a shortcut with limited options,
especially to link external artists to their own pages. See the manual pages for
catalog and releases for more info on that.

On the other hand, to specify a full-featured artist with its own page on the
faircamp site, create a directory for it anywhere in your catalog, create a
(plain text) file with the name `artist.eno` inside it and specify at least
the `name` field, so your artist can be associated with its tracks in your
catalog.

Here is an example `artist.eno` file, below it the fields are explained one-by-one.

```eno
name: Alice
permalink: alice-artist

aliases:
- Älice
- Alice (feat. Bob)

image:
description = Alice in a field
file = example.jpg

-- text
Alice is a classic recording artist from the 70ies.

Over the years she has appeared in various collaborations, most notably with Bob.
-- text
```

## `aliases`

```eno
aliases:
- Älice
- Alice (feat. Bob)
```

If, as often happens, different audio files use slightly different versions of
an artist name (e.g. "Motörhead" vs. "Motorhead"), or the artist appears in a
collaboration (e.g. "Alice (feat. Bob)"), you can specify `aliases` that will
be matched against in addition to the `name` to map the artist to the right
tracks.

## `copy_link`

To disable the "Copy link" button (by default it's enabled) you can use the `copy_link` option, with either `enabled` or `disabled` as value. This is also inherited by all releases, but can be changed on a granular basis for single releases or groups of releases in their manifests.

```eno
copy_link: disabled
```

## `image`

```eno
image:
description = Alice in a field
file = example.jpg
```

With `file` specify the path of the image, relative to the directory the
manifest is in (so if the image "example.jpg" is in the same folder, just
write "example.jpg", if it's in the parent folder write "../example.jpg", and
so on). Make sure to include a `description` for non-sighted people too.

## `link`

```eno
link: https://example.com/this/artist/elsewhere/

link:
url = https://example.com/this/artist/elsewhere/

link:
label = A review of the artist
url = https://example.com/some-blog/some-review/
```

You can supply any number of `link` fields, these are prominently displayed in
the header/landing area of your artist page. A `link` must at least
provide a url, either as a simple value or as an `url` attribute. Optionally
you can also supply a `label` which is what is visibly displayed instead of
the `url`, when given.

## `more_label`

```eno
more_label: Biography
```

If you provide long-form text content for the artist (which can be anything
you want, content-wise) through the `text` field, by default there will be a
link with the label "More" on the artist page, leading to the section
containing the long-form text. If you want to customize that label so it
specifically refers to the type of content you are providing there, the
`more_label` field allows you to do that. Some typical examples of custom
`more_label`s one might use for the artist text: "Details", "Shows",
"Discography", "Bio", "About" etc.

## `name`

```eno
name: Alice
```

The `name` you assign is how the artist is represented **everywhere**,
including in tags on your downloads (unless you enable `tags: copy`, or a
similar setting).

Very importantly, the name is also used to match your explicit definition of
the artist (by you in the manifest) to any implicit definition (through audio
file metadata), so pay close attention that they are spelled exactly the same
in all places - including casing (lower-/uppercase). If the artist is
frequently written in different ways (e.g. in one audio file the artist is
tagged as "Punkband", in another "PunkBand", and in yet another
"Punkbänd"), a simple way to still correctly associate it with your single
 explicit definition is to use the `aliases` option, e.g.:

 ```eno
 name: Punkband

 aliases:
 - PunkBand
 - Punkbänd
 ```

## `permalink`

```eno
permalink: alice-artist
```

For an explanation what a `permalink` is please see the "Concepts Explained" page,
unter "Topics".

## `text`

```eno
-- text
Alice is a classic recording artist from the 70ies.

Over the years she has appeared in various collaborations, most notably with Bob.
-- text
```

The `text` field supports [Markdown](https://commonmark.org/help/).
