# Changelog

## 0.10.1

- Revert release date as rss item pubDate for further consideration (d006bd9)
- Include error message on ffmpeg process failing to execute (c6c8e83)
- Drop unused color-cover styles and variables (8785c9a)

## 0.10.0

- Augment permalink conflict errors with release directory paths (9d376b1)
- Expose release date as item pubDate in rss feed (Default Media Transmitter, a2d8c5f)
- Support transcoding to ALAC format (Deborah Pickett, ee8b435)
- Introduce custom ico/png favicon support (87590a2)
- Disable embedding by default (until fully implemented) (6d8b12d)
- Handle total track count component when parsing track number tags (bb1dde1)
- Disregard case in cover selection heuristic (121551d)
- Patch upstream slash parsing issue for ID3v2.2/2.3 tagged files (c25acad)

## 0.9.2

- Update enolib, pulling in a critical unicode char width parsing fix (57c3f81)

## 0.9.1

- Prevent cover image from being included in release archive twice (5c9b109)

## 0.9.0

- Improved french translation (Florian Antoine, 4ad9d87)
- Support aif/aifc extensions for input audio files (f0293d7)
- Sort release tracks alphabetically if there is no track number metadata (2be5e50)
- Use clearer "0 or more" placeholder for the "Name your price" form (fb92afc)
- Set track number metadata during transcoding when rewrite_tags is active (c0bb3c2)
- Introduce extra material (artwork, liner notes, etc.) for releases (09410d7)
- Introduce optional single file downloads, redesign downloads page (778c2d2)
- Add dutch locale (9f60b20)

## 0.8.0

First versioned release