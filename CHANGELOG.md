<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Changelog

# 0.21.0

- Introduce configurable M3U playlist option for the entire catalog (0cebc42)
- Add hash-based cache invalidation for all linked assets and images (041cf4f, 482f9ba)
- Allow navigating to browse/search items by clicking their thumbnails (29ffa6e)
- Fix browse/search overlay closing too easily when focus is lost (375f8c5)
- Fix linking to non-existent artist pages when releases have varying but non-featured artists (1562d64)
- Left-align list markers inside margin, use disc or square style based on theme (aeffbf5)
- Update dutch translations (n3wjack, 1101b94, f5e3beb)
- Update french translations (Vincent Knobil, 1511536)
- Update italian translations (Tommaso Croce, b510376)
- Update lithuanian translations (Vac, 3cff359)

## 0.20.1

- Exclude featured but unlisted artists from browsing/searching (b298804)

## 0.20.0

- Introduce global browse/search feature (23b7c68, 061391c)
- Introduce external downloads (9f17493)
- Implement catalog/release manifest option for enabling/disabling M3U playlists (c3f76fd)
- Merge custom payment options into payment_text option (dd04650)
- Link and render "more" sections only when extended content is present (0f42bcf)
- Restore missing border-radius declarations for round_corners theme option (ef2a4f3)
- Fix layout regressions and improve readability for single file downloads (ef88fff)
- Add lithuanian translations (Vac, abe8d67)
- Add serbian (cyrillic and latin) translations (DURAD, 022ef22, d2eb3b7)
- Extend/improve french, italian, turkish and spanish translations (sknob, Tommaso Croce, atomkarinca, c67c399, 762bab0, fc573a0)
- Resolve subtle issues around hardcoded left/right spacing in RTL layouts (d354f50)
- Display advanced theming spectrum/swatch widget by default (4f5a56b)
- Prevent seeking during left/right arrow key interaction with the volume slider (4094c1a)
- Differentiate page titles for release download/embed/purchase/unlock pages (befae3f)
- Differentiate track title styles between release and track page (dbe0ee6)
- Escape html in synopsis fields (d5dd67f)
- Use a single, generic iframe title for both release and track embeds (7b4dbfc)
- Semantically tag reprise headers, emphasize artist links on release page (3c64737)
- Accessibly label invisible close button and fix background for cover overlay (8cd9a3b)
- Reimplement cover overlay as modal dialog with href fallback for disabled js (9dfef6b)
- Accessibly announce playback position using localized, written out format (25b88d9)
- Hide images from screenreaders where left undescribed by site operator (d33a7bb)
- Announce open/closed status of docked player to screenreaders (ed70730)
- Visually indicate player seekbar keyboard focus, strengthen hover emphasis (76340ef)
- Explicitly style visible focus on titles in track list (8941f3d)
- Scroll elements into view from below docked player when focused with keyboard (e619b9f)
- Provide textual playback position slider context label for screenreaders (ee2cf6e)
- Treat tiny cover images as decorative elements with limited interactivity (85f7a01)
- Increase internal spacing and tall playback button variant in track list (b337935)
- Correct price input pattern to allow any number of decimal places (fd90d67)
- Explicitly style visible focus on catalog/release title in header (ae52b9d)
- Provide textual volume slider context label for screenreaders (e4ebfde)
- Dynamically toggle textual mute/unmute label for volume button at runtime (817d446)
- Apply blur/darkening to docked player backdrop (529cff2)

## 0.19.0

- Introduce M3U playlists for releases (b5ecf9f)
- Introduce link fields for catalog and releases (dab361e)
- Introduce label overrides for catalog/release level "More" links (2d3cee4, f16763d)
- Redesign track lists, visually reconnect them to release/track headers (406d1ff, f5156b6)
- Move Download/Embed/Copy-Link buttons and links above the fold, further compact pages, drop Releases/Tracks buttons (3246a18)
- Refine alignment and adaptive, responsive behavior for page header elements (c4ab209)
- Split site scripts into clipboard and player scripts and load them on demand (4e8b39f, 27ccc62)
- Underline links in custom payment texts, remove deprecated/undocumented liberapay option (2cfe1b7)
- Add Ukrainian translations (Denys Nykula, 4341dcf)
- Localize volume button label (7409bbe)
- Remove scroll hints (6c419f7)
- Visually widen download icon (5b682f5)

## 0.18.1

- Unbreak theming widget scripting after ESM import changes (52f3e24)
- Fix missing alpha in overlay colors during interactive theming (81c4fda)

## 0.18.0

- Rewrite and complete embed implementation (d856595, 6979d4a, 63c74b3, ac74250)
- Transform external inline/auto links in markdown texts to open in new tabs (96a569d)
- Implement experimental initial track override parameter for release player (7a1e6ff)
- Revert inclusion of scripts as ESM to restore direct viewing from disk (df415b0)
- Dynamically translate listen/pause button label at runtime (3f67db8)
- Fix accidental linking to non-existent download/purchase/unlock track sub-pages (c1d9e18)
- Enforce paragraph width and remove experimental stats rendering on artist pages (b5b103b)
- Fix links in the RSS feed not following --no-clean-urls setting (94b873f)
- Refine italian translations (Tommaso Croce, 548b03c)

## 0.17.0

- Introduce support for writing embedded cover images for flac and mp3 (45f6881)
- Reduce layout spacing, making consecutive sections come out above the fold again (3fbbab8)
- Augment three-dot button with a textual "More" label (4ce85b0)
- Bring back a simplified breadcrumb navigation for release sub-pages (d7f53a8)
- Resolve usability friction between docked player and overlaid iOS OS interface elements (e299fc1)
- Fix stand-alone track page links not following --no-clean-urls setting (598a05a)
- Prevent payment confirmation toggle being filled out by autocomplete (1c1de89)
- Restore occasionally missing button styles after theme redesign (f5865f0)

## 0.16.1

- Add italian translations (Tommaso Croce, 9502586)
- Fix title not being displayed in the docked player on track pages (2919dc4)
- Fix listen button toggling playback only for the first track on release pages (2edf0f2)

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
