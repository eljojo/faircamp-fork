<!--
    SPDX-FileCopyrightText: 2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Linking to timecodes/tracks

From version 1.3 onwards, you can create links that directly seek/jump to a
specific timecode or track on a release page, track page or even in an
embedded track or release player.

## Examples

Let's assume you just released your new album on your faircamp site at
`https://example.com/new-album/` and want to point your followers to a slick
guitar riff that starts 20 seconds into the third track. The following link
will take people to the release page, automatically opening the player with
track number 3 at timecode 0:20:

`https://example.com/release/#track=3&time=20s`

Now let's assume you want to point some friends to an easteregg you placed way
back towards the end (timecode 7:32) of the the outro track (number 11) of
the same album. To link to this in the context of the track itself only, you
take the track page (`https://example.com/release/11/`) and add a time
parameter like this:

`https://example.com/release/11/#time=7m32s`

The two previous examples were about taking people to a specific track and
timecode from an *external* page or platform, but the same also works directly
from within a release or track page. Assuming you are an interview podcast
producer, just released a new episode at `https://example.com/new-episode/`
and want to link to various sections of an interview, this is what you could
put into your `more` field for the episode:

```eno
-- more
In today's episode I talk to Alice about Foo, enjoy!

[Introduction](#time=1m20s)
[What is Foo anyway](#time=14m)
[Practical strategies for using Foo](#time=52m12s)
[Discussing the impacts of Foo](#time=1h26m3s)
[Farewell and preview for the next episode](#time=2h7m)
-- more
```

As a last example, assuming you are embedding your podcast episode from
the previous example in your blog and want it to directly open the interview
section on "Discussing the impacts of Foo", this is how you could modify the
embed code to achieve this:

```html
<iframe
    loading="lazy"
    src="https://example.com/episode/#time=1h26m3s"
    style="border: none; min-width: 480px;"
    title="Audio player widget for 'Talking to Alice about Foo'">
</iframe>
```

## Details on the syntax used

You can supply a track number, a timecode, or both. If you leave out the track number, the first track is assumed by default:

- `#time=30s` (Open the first track at timecode 0:30)
- `#track=2` (Open the second track)
- `#track=2&time=30s` (Open the second track at timecode 0:30)

You can abbreviate parameter names (`t` as in **t**ime, `n` as in track **n**umber):

- `#t=30s` (Open the first track at timecode 0:30)
- `#n=2` (Open the second track)
- `#n=2&t=30s` (Open the second track at timecode 0:30)

Timecodes are flexible, but keep the order **h**ours -> **m**inutes -> **s**econds:

- `#time=30s` (30 seconds)
- `#time=2m` (2 minutes)
- `#time=2m30s` (2 minutes, 30 seconds)
- `#time=1h` (1 hour)
- `#time=1h30s` (1 hour, 30 seconds)
- `#time=1h2m` (1 hour, 2 minutes)
- `#time=1h2m30s` (1 hour, 2 minutes, 30 seconds)
- **Not allowed**: `#time=30s2m1h`

The order of `time` and `track` is not relevant, these are both allowed:

- `#time=30s&track=2`
- `#track=2&time=30s`
