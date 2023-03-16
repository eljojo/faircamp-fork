# Artists

Artists are automatically created by faircamp when they are encountered in
audio file metadata (e.g. the artist "Alice" will be created if any ID3 tag
says a track is by "Alice"). To add further information to an artist, you can
expliclity define it in a manifest. The name you assign will be used to match
the explicitly defined artist (by you in the manifest) to the implicitly
defined one (by the audio file metadata) so pay close attention that both are
written the same (NB: lowercase/uppercase is ignored for matching). If
(as often happens) different audio files use slightly different versions of
an artist name (e.g. "Motörhead" vs. "Motorhead"), or the artist appears in a
collaboration (e.g. "Alice (feat. Bob)"), you can additionally specify
`aliases` that will also be matched against to map the artist to the right
tracks.

```eno
# artist

name: Heston Exchange
permalink: heston-exchange

aliases:
- heston exchange
- Heston Exchange (feat. Bar Foo)

image:
description = All four bandmembers against a bright blue sky, wearing pink velvet top-hats
file = heston_exchange.jpg

-- text
Classic Dada-core formation founded in the 90ies.

Only ever known to publicly perform at birthday parties and the gym at their hometown.
-- text
```

Note that the `text` field supports markdown.