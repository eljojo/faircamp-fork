<!--
    SPDX-FileCopyrightText: 2022-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Architectural Notes

This file documents design decisions/thoughts that are not necessarily trivial
to arrive at, so they don't have to be thought through over and over again.
Also if changes becomes necessary the thinking process can start from an
already documented thought process, instead of starting at zero.

## Client-side scripting does not use ESM

None of the client-side JavaScript in faircamp uses `<script ... type="module">` in its inclusion.
The reason for this is that scripts included with `type="module"` are subject to CORS browser
security, and therefore cannot be included directly from disk through the `file://` protocol.
A build generated with `--no-clean-urls` primarily makes sense for local, disk-only consumption,
but this only works if scripts can still run, and that necessitates working without `type="module"`
(otherwise everyone would need to browse their local faircamp build with a server, always).

## Heuristic to pick a cover image when there are multiple images

The file name without extension is taken and made lowercase. If
it equals "cover" it's our first pick, followed by "front", then
"album". If there are e.g. both "cover.jpg" and "cover.png" in
the directory it's going to be a random pick between the two,
same thing if none of our "special" strings appear, then we just
randomly pick the first image we iterate over. 

## Steps/algorithm to arrive at release.main_artists, release.support_artists

A release has *main artist(s)* and *support artist(s)*. Picture e.g. a
release with ten tracks by Alice, where on one track she collaborated with
Bob. That makes Alice the main artist, and Bob a support artist on the
release.

Faircamp uses the following cascade of conditions to determine main and
support artists, with the first that applies outranking the ones below in
priority:
- Manifest options `release.artist`, `release.artists`
  (`release.support_artist` and `release.support_artists` are *planned*
  to be added soon but don't exist yet)
- `Album Artist` tag (if present on any audio file(s) in the release)
  adds the given artist to the main artists
- If no `Album Artist` tag is present on any track, the artist encountered
  in the `Artist` tag on the highest number of tracks will be the main
  artist (the others will appear as support artists), in case of a tie,
  all artists with the same highest number of track appearances will
  become main artists.

## Steps/algorithm to arrive at catalog.artist

In *artist mode* (the default), one artist is made the *catalog artist*. If
there are multiple artists across all releases/tracks the following is the
logic to automatically arrive at the catalog artist:

- Pick the artist that is a main artist on the highest number of releases
- In case of a tie, pick the artist among the tied ones that has the highest
  number of track associated with them
- If there's yet another tie, abritrarily pick the first one of the tied

## Different artists may share the same alias

A catalog may have the following explicitly defined artists next to each other:

- "Alice" (name) with an alias "Alice (feat. Bob)"
- "Bob" (name) with an alias "Alice (feat. Bob)"

This allows both "Alice" and "Bob" to exist and be assigned to a track with
metadata storing an artist "Alice (feat. Bob)".

## Heuristic audio metadata

If the filenames of the majority of the tracks in a release fulfill certain
criteria, track number and title metadata is derived from the filename. This
metadata is used as fallback if we don't have track number or title metadata
in the audio file tag data itself.

E.g.: A release with the track filenames "1. Alice Song" and "2. Bob Song" will
result in track #1 "Alice Song" and track #2 "Bob Song". A release with
the track filenames "1 Good Example" and "3 Things I wanted to tell you" won't
(track numbers don't monotonically increase). A release with the track
filenames "01 - Foo" and "02  Bar" won't (inconsistent separator) - but if
there were a larger number of tracks in the release and only a certain
amount of the tracks had a "wrong" separator that would be tolerated and
"  " considered a separator exceptionally although the others would for
instance be " - ".

## Hotlinking countermeasures

Faircamp does not generally try to obfuscate anything about the site/url
hierarchy it generates - it would be technically pointless, and faircamp
rather aims to provide a site that is highly serviceable and easy to study
and understand on a technical level. However, if an artist or a label faces
blatant cases of hotlinking, e.g. publicly circulating direct download urls
to not-for-free releases, faircamp provides mechanisms for changing/rotating
parts of the asset download urls with a new deployment, thereby rendering
any already circulating hotlinks to downloads dysfunctional.

## Image descriptions are brought to the fore

Barriers that the blind or weak-sighted face on the web are most often
invisible to those that can see. An image without a description is just an
image to those who can see it, and the problem thus stays out of sight and
out of mind to those able to solve it. Faircamp brings those images to
everyone's attention instead, pointing them out not only during building, but
also in the generated site itself, where it's then in plain sight to everyone
that there are barriers to those without sight, until solved.

## Permalink conflicts are never automatically solved

Faircamp does not automatically resolve permalink conflicts because doing so
might inadvertently break links that people out on the web are already
using.

## Visibility considerations around unlisted releases

- Unlisted releases are not listed anywhere, that means neither on the
  home/index/releases page, nor on any artist's page
- Artists that have only unlisted releases themselves become implicitly
  unlisted, that is, their name still appears below their releases, but
  is never linked to their page (it has to be noted though that guessing
  the permalink of an artist is usually not too complex, so the page might
  still be discovered if a visitor has the intent and energy to look for it)
- A visitor that views an unlisted thing (release or artist) is made aware
  of it through an "Unlisted" badge next to the title or name, hinting at
  the fact that it is unwanted for the thing to be publicly shared

## JavaScript-free functionality

On all pages, the first thing appearing within the `body` element is an inline
script appending the `.js_enabled` class to the `body` element:

```js
<script>document.body.classList.add('js_enabled');</script>
```

Browser JS engines immediately execute this *before* further parsing the
document, hence we can make alterations based on whether JS is available or
not without introducing any flicker or jumping on the page during the initial
load phase. At the time of writing this mechanism is also likely to be more
robust than the `@media (scripting)` feature (which is not yet sufficiently
available in all browsers and also misses JavaScript being blocked by way
of plugins sometimes).

## Behavior around release and track extras

Release extras are by default bundled inside release downloads (zip archives).
If bundled release extras and track extras are enabled, track extras will
be included inside release downloads.

The presence of release and track extras can cause downloads (archive or
seperate) to be generated even if there are no download formats
(release_downloads/track_downloads) configured (!).

## Reference reading on single vs. multiple enclosures

As faircamp renders RSS feeds for data where one *item* (in the RSS sense) is
associated with multiple tracks (which would logically map to multiple
*enclosures* in the RSS sense) the question of the permissibility of multiple
enclosures per item became relevant. These are three posts that provided good
insight on the question, for future reference:

Dave Winer – One enclosure per item or multiple?: http://scripting.com/2017/05/21/oneEnclosurePerItemOrMultiple.html
The RSS Blog – Multi-Enclosures, Part I: https://rssweblog.com/?guid=20070520140855
The RSS Blog – Multi-Enclosures, Part II: https://rssweblog.com/?guid=20070522234541
