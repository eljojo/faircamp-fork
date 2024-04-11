# Changelog

## 0.14.0

- Disallow crawling/indexing on unlisted and auxiliary pages (91a64e6)
- Introduce unlisted releases (683b11d)
- Avoid layout shifts through image and font changes during loading (James Fenn, 29c521a, e91609d)
- Constrain fullscreen cover image display size to intrinsic image dimensions (a2fc546)
- Resolve panic in image crate 24.9+ when attempting to save rgba8 to JPEG (d532111)
- Optimize image resizing (re-use decoded images, determine resize by overshoot) (c347d6c)
- Introduce disable_waveforms option (4743423, 9f07fd1)
- Fix out of bounds sampling error in client-side waveform computation (6bc77ad)
- Determine text direction automatically, disable writing_direction option (3ba13e0)
- Update all depdencies to latest (4b861a9, 0b85f94)
- Interpolate translated permalinks, fix unsafe permalinks (1e5710c)
- Add translations for polish (Damian Szetela, 4a7a928)
- Alphabetically sort featured artists on homepage in label mode (d9821df)

## 0.13.0

- Introduce support for alac as input format (234b345)
- Released round_corners theme option (c91d048)
- Prevent edge case panic when all release tracks are in an unsupported format (5c51a6b)
- Include artists in feed item title, release text as optional item description (b464a73)
- Automate feed image generation, deprecate/skip manual feed_image option (26023be)
- Let release text fully show or peek out from under the fold if there is space (fb715ea)
- Avoid track transcoding when only archive is needed and already available (c6a14b1)
- Ensure waveform rendering is only conditionally run on release pages (ee173a6)
- Fix track waveform width determination at transitional viewport widths (1f72a82)
- Pull in enolib patch fixing missing line breaks in manual (8fa4905)

## 0.12.0

- Make the disable_relative_waveforms theme option public (5e00ddf)
- Update eno parser, removing field/attribute/item continuations (b2a4201)
- Fix iOS background image scaling (James Fenn, 344a87c)
- Fix critical edge case in which certain browser extensions can break client-side js (0c5f54a)
- Enforce configured price range in "name your price" flow, skip payment step for 0 amount (2dba5e4)
- Add locale lang attribute to html tags (James Fenn, 0094959)

## 0.11.0

- Support disabling the favicon altogether (e2983bd)
- Encode filenames of archives, tracks and extras in href/src attributes (a333b57)
- Add translations for norwegian bokmaal (Harald Eilertsen, c84262d)
- Introduce markdown to plaintext rendering and html escapes for feed content (cb9f540)

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