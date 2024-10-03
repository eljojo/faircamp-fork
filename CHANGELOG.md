<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Changelog

## 0.16.0

- Read dynamic_range option from manifests (64253fc)
- Add Turkish translation (atomkarinca, ee4e130)
- Add Swedish translation (Miró Allard, 208db36)
- Announce aria-valuetext on docked player timeline, improve keyboard control (3cbc01b)
- Add debug option with basic debug info printing implementation (2447627)
- Automatically display varying track artists in track list and docked player (718d0cf)
- Enable smooth scrolling only when there is no preference for reduced motion (2666b31)
- Overhaul embed choices page layout, fix cover size in compact release widget (03249c4)
- Automatically rename extras whose name collides with cover or track filenames (5eec1be)
- Introduce dynamic range based fluid theming, extend/rewrite theming widget (7e3d469)
- Switch entirely to base/accent theming system (b6a5b91)
- Refine docked player featureset, design and layout, link titles to track pages (8f763dc)
- Adaptively truncate long artist listings, linking to full list (dab66ff)
- Introduce page-based layout, iterate on theme rewrite, extend docked player (db00518, 462cb4b)
- Add dotted and non-padded track numbering options (bccf6b8)
- Render an entirely waveform-less, compact layout when waveforms are disabled (0951d6a)
- Skip empty and undecodable input audio files, printing relevant errors (95192c1)
- Improve docked player timeline readability/visibility (c1af5dc)
- Add accessible value description and fine-grained key control to volume slider (ba953a8)
- Indicate disabled state for previous/next track buttons in docked player (a2ea77e)
- Port docked player and layout changes from release to track pages (3b22f06)
- Remove experimental active waveform rendering, clean up after layout changes (5d86ab9)
- Generate theme styles for artists with own pages (8b3c9a7)
- Iterate on volume control design and interaction, simplify volume abstraction (d9d471f)
- Introduce docked player, iterate on release page layout and usability (efad8df)
- Introduce custom styling for dividers and lists in markdown texts (b044f02)
- Flesh out volume control design and interaction (3ab89ea)
- Remove breadcrumbs in header (1871fb5)
- Ensure computed source metadata is persisted to cache right after computation (c25c6cd)
- Recognize and reject unsupported aac/m4a audio file extensions in the catalog (40b2ff7)
- Fix opus sample count computation (e9aa120)
- Generalize tag extraction patterns for alac, flac and opus (01f273d)
- Generalize id3 patch and tag extraction, support multiple artist tags in id3 (ed82dcb)
- Scaffold backend implementation for volume control (c2affc5)
- Communicate intermittent buffering and improve state robustness in player (bcf0b76)
- Fix playback key handler overriding keyboard access to copy track link button (f6bcef0)
- Simplify breadcrumb hrefs for current page, fix track parent breadcrumb href (b41b4db)
- Conditionally hide big play button on small viewports (99e0a4b)
- Work around delayed execution of pause event when switching tracks (9d3b942)
- Implement copying links to tracks, hide redundant copy icons for screenreaders (f6910d5)
- Replace obsolete/misassigned "share" label with "external link" (92d48a6)
- Fix touch interaction with additional track options in chromium based browsers (da338ba)
- Support media key shortcuts, allow seek updates while loading, refactor player (51907a9)
- Bypass direct waveform input interaction to fix clicking and seeking glitches (d7545ac)
- Account for float inaccuracies in chromium in audio/seek time comparison (256c06b)
- Introduce seamless morphing from pause icon to new loading animation (94159d6)
- Fix cache optimization messages looking confusing/broken (366120a)
- Move cache optimization option into catalog manifest section (53d27f6)
- Move streaming quality option into catalog/release manifest sections (20af519)
- Ensure sufficient preloading when directly playing a track from a seek offset (0ba880d)
- Iterate on player accessability/usability, link to track pages (4dec4ed)
- Move language option to catalog manifest section (f760d43)
- Move embedding option to catalog/release, require catalog options at root (5d10bc9)
- Enable keyboard control and accessible readout of the player waveform element (4b0c05f)
- Implement proof of concept for dedicated artist directories/manifests (1f19b90)

## 0.15.1

- Ensure that tags are correctly copied/written by ffmpeg for any possible source/target format combination (7127c61)
- Update outdated tag configuration hints for manifest field deprecations/errors (724eebb)

## 0.15.0

- Apply round corners to home/artist images when shown detached (b87802d)
- Underline links in catalog and release texts (f4b542f)
- Switch from two-click "Share" button and overlay to direct "Copy link" button (7e9277f, c44a8ed)
- Introduce granular tag rewriting control (per tag), support explicit copy/remove for embedded images (90ca77a)
- Support disabling the share button at the catalog and release level (3e3aaab)
- Introduce new cache architecture (01c9ad8)
- Introduce type-based cache data versioning, improve cache corruption handling (383e203)
- Support disabling the RSS feed (10c1ebb)
- Fix redundant optimization/reporting of cached assets (71b670a)
- Derive cache manifest filenames by hashing (97c2d08)
- Switch to inline svg icons with translated descriptions (0c84d7f, f4e4fb8)
- Introduce theme customizations at release level (64d9f39, 16461b0, d2e09ee, 88478a8, fc13569)
- Automatically derive track number and title from filename, based on heuristics (9ca7604)
- Visually indicate unlisted release pages, do not display unlisted releases on unlisted artist pages (94d1c68)
- Ensure trailing slashes in urls when serving directories in preview (04bf953)
- Introduce compliance with REUSE (832d26a)
- Fix client-side time formatting for tracks longer than an hour (0e0dc9e)

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
- Update all dependencies to latest (4b861a9, 0b85f94)
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
