<!--
    SPDX-FileCopyrightText: 2022-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Architectural Notes

This file documents design decisions/thoughts that are not necessarily trivial
to arrive at, so they don't have to be thought through over and over again.
Also if changes becomes necessary the thinking process can start from an
already documented thought process, instead of starting at zero.

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
- Manifest options `release.artist`, `release.artists` (`release.support_artist` and `release.support_artists` are *planned* to be added soon but don't exist yet)
- `Album Artist` tag (if present on any audio file(s) in the release) adds the given artist to the main artists
- If no `Album Artist` tag is present on any track, the artist encountered in the `Artist` tag on the highest number of tracks will be the main artist (the others will appear as support artists), in case of a tie, all artists with the same highest number of track appearances will become main artists.

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


