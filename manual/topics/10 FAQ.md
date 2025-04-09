<!--
    SPDX-FileCopyrightText: 2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Frequently Asked Questions

## Why does faircamp transcode audio files into more formats than I requested?

You might have observed that faircamp generally transcodes audio files into
multiple formats, and in fact, more than one requests. For instance if
you have a release for which you requested release downloads in MP3 format
only, this is what you might be seeing in faircamp's output:

```
[TRANSCODING] "Release/Track.mp3" to Opus 96
[TRANSCODING] "Release/Track.mp3" to MP3 V5
[TRANSCODING] "Release/Track.mp3" to MP3 V0
```

The reason for this is *streaming formats*. Faircamp provides streaming assets
(the audio files that visitors listen to when they play your audio directly
from the website player) primarily in Opus format (because of its high efficiency,
see [en.wikipedia.org/wiki/Opus_(audio_format)](https://en.wikipedia.org/wiki/Opus_(audio_format)#Quality_comparison_and_low-latency_performance)
and/or [opus-codec.org/comparison](https://opus-codec.org/comparison/)) and as a fallback also in MP3 (this is
because Apple/Safari does not support Opus in the way that every other browser
and system does). If you have requested MP3 downloads, there will still be two
transcodes to MP3 formats happening because for streaming, a significantly
lower bitrate is used, to reduce both load time for visitors and bandwith usage
on your server.
