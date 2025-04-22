<!--
    SPDX-FileCopyrightText: 2023-2025 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Changelog

## 1.4

## Custom assets and metadata

- Introduce site_assets and site_metadata options for site-wide custom assets/metadata (023996f)

## Misc

- Extend roman numeral range from 40 to 3999 using modern subtractive notation (a2adfd3)
- Rename optional FAIRCAMP_PKG_VERSION build parameter to FAIRCAMP_VERSION (e3f1e1c)
- Add css minification in minify feature (023996f)

## Fixes

- Work around browsers with missing Media Session API support (Sunny, 06c5291)
- Fix incorrect url construction in browse/search when clean urls are disabled (f0dcf39)
- Double-escape html-like plain text in generic rss, escape names in podcast rss (2cd6544)
- Escape names and replace accidental summary tag with subtitle tag in atom feed (eb052ad)
- Prevent scrolling during playback speed button mouse wheel interaction (4e660e2)

## 1.3

### Podcast RSS, Atom, extended feeds

- Introduce Podcast RSS and Atom feeds (e2d67f1)
- Expand feed(s) with generator info, channel image description, height and width (a182fe9)

### New and improved player controls

- Introduce playback speed controls (7c7001c)
- Introduce custom volume slider, extend keyboard interaction, improve usability (e9578b0, 57e3880, b87137a, 6be02f2)
- Fall back to mute/unmute functionality in browsers with read-only volume (limitation on apple devices) (f4c81a4)

### Linking to timecodes/tracks

- Support linking to timecodes/tracks from external pages (jump/seek on page load) (6b0d7e2)
- Support linking to timecodes/tracks on release/track pages themselves (jump/seek at runtime) (c422f73)

### Improved procedural covers

- Switch to optimized, raster-based procedural cover images (37df168)
- Extend cache invalidation to procedural covers (9ca7a17)
- Implement experimental block pattern cover generator (cc51758)

### Misc

- Implement artist images in search, extend cache invalidation (e5b3ec7)
- Add --preview-ip flag (Sandro Santilli, dca0365)
- Add generator meta to pages, include git revision with detailed version output (5e91eb9)
- Introduce compile-time minification of javascript assets as opt-in build feature (6400a67, b77d492)
- Remove embed iframe border (7265814)
- Remove release date year display while still officially sort-only (5c3c00f)
- Remove redundant textual descriptions for copy icons (179cb7e)

### Fixes

- Differentiate release and track price when assembling download access data (ed562d6)
- Prevent playback of multiple tracks after foregrounding the browser (bug on apple devices) (425390a, 7265814)
- Fix linebreaks between marker and paragraph inside bullet points (4036d20)

### Translations

- Add portuguese (european) translations (N4ta, 3f94b4f, 6d7a6cb)
- Update catalan translations (Elx Cat, 7fbdd42)
- Update lithuanian translations (Vac, 5970352)
- Update spanish translations (Patricio Maripani, b7d50fd)
- Update swedish translations (Patrik Wallström, 4f8e7e8)

## 1.2

- Introduce track directories, manifests and new discrete release/track options (c3662b5, 85fddde)
- Hide volume controls when volume is read-only (primarily for browsers on iOS) (5aedf10)
- Escape all relevant content in opengraph metadata (ceae35f)
- Add catalan translations (Elx Cat, 5d361c2)
- Update polish translations (janinainfa, 3f463fc)
- Update italian translations (Tommaso Croce, ad1ad9f)

## 1.1.1

- Update french translations (Élie Khalil, 1173b6b)
- Open release embed track links in parent context (Sunny, 65128dc)
- Fix critical artist M3U playlist generation issue (Sunny, 3336f57)
- Fix release embed track links (Sunny, 5aa03e3)

## 1.1

### New functionality

- Introduce opt-in opengraph metadata provision (159e717)
- Implement M3U playlists for artists (2ed079c)
- Implement tags option in catalog and artist manifests (4782110)
- Return exit code for failure/success on process termination (f018070)

### Bugfixes

- Fix more_label assignment in the catalog manifest (92f2005)
- Add missing ffmpeg qscale arguments for MP3 V5/V7 audio format encoding (a4a8409)
- Fix decoding of opus files originally encoded from non-native sample rates (8fae371)
- Fix oklch computation glitch at 100% background alpha in theming widget (a2572c2)

### Translations

- Add Japanese translations (naskya, e989fe1)
- Update swedish translations (Filip, cc0dc7e)
- Update norwegian translations (Mathias, d80e5a6)

## 1.0

### Manifests

- Switch to dedicated artist.eno, catalog.eno and release.eno manifests, overhaul error handling (df6454a)
- Switch to case-sensitive matching when comparing/associating artists by name (34651bc)
- Switch to theme options as a field inside the artist/catalog/release manifests (638dc1a)
- Extend downloads customizability, move options into artist/catalog/release manifests (5529c60, eb30ac5, e5f38e7, e348c4d)
- Extend artist customizability, generalize and reorganize manifest code/options (71b040a, 77d2cc3)
- Introduce artist shortcut and external pages, rename original artists field (4fbcbbe, 67aa339)
- Implement synopsis on artist, document missing options for artist.eno (166895e)
- Rename 'text' option to 'more', improve/extend its documentation and examples (a71471b)
- Provide a short-hand link option, improve/extend link documentation to artists (c228e88)
- Support track_numbering option at the catalog level as well (17439a2)

### Terminal usability

- Always abort builds on error, allow forced override with --ignore-errors flag (0a85676)
- List all options supported by the manifest when an unsupported option is found (fcb9668)
- Provide interactive error guidance for migrating from obsolete options (271a2e5, c1dc284, d608c60, f112865)
- Provide explicit help message for handling an artist with varying spelling (140200b)
- Provide a hint for shutting down the preview server (cf5b8a8)
- Refine faircamp termination/progress hint (5e034ac)
- Print relative path (in catalog) when informing about decoding operations (feddff8)
- Adaptively format build time metric in milliseconds, seconds or minutes (d1ef0b7)
- Minor readability tweaks in misc manifest error messages (97c18ee)

### Layout

- Introduce global footer and configurable faircamp signature (9435de4)
- Compact the layout further, implement responsive/sticky footer behavior (c7eb3a5, 52e33be)

### Misc

- Change M3U playlists to be opt-in (f7db4d7)
- Change the default track numbering style to arabic-dotted (e112267)
- Build catalog/release descriptions for RSS feed from synopsis fields first (cede9fe)
- Update lithuanian translations (Vac, 43f91f1, db143c4)
- Improve debug feature output (2647f9a)

### Manual

- Add label example in manual, improve artist example titles and intros (96c9eaf)
- Introduce cross-links, options overview and extend content on reference pages (161ab10)
- Document --debug option (b5c84ce)
- Revisit manual background shades (6a42c01)
- Reorder and structure shuffled docs around copy_link, date and track sorting (9a4e4eb)
- Remove redundant manual instructions for rel="me" link placement (39f96ca)

### Bugfixes

- Ensure mutually exclusive artist assignment to catalog's main/support artists (6a860ba)
- Ensure mutually exclusive artist assignment to release's main/support artists (9aa9bd5)
- Fix printing of unsupported color codes in windows command prompt (514c30b)

### Build integrity

- Pin pacmog crate dependency to an exact version (4fba9ee)
- Pin libvips crate dependency to an exact version (281c447)
- Mirror version in translations subcrate to work around build/tooling issues (f9904d2)

## 0.23.0

- Implement hybrid ltr/rtl layouting and better space usage for docked and embedded players (8f8b3c4, 0198081)
- Communicate muted state through the volume icon, remove dimmed volume hint (242e289)
- Provide a way to skip to main content in keyboard-based navigation (fd72feb)
- Add accessible labels to all focusable playback position and volume sliders (0a99254)
- Improve responsive behavior, extend direction-agnostic spacing, drop stale css (c93e14c)
- Focus browse/search modal when it is open and we regain focus on the page (b777dbd)
- Fix dynamic restoration of keyboard interaction on track playback buttons (6578a2e)
- Improve input placeholder text readability (f927e8c)
- Fix focus/hover style conflicts on track titles (b7728ec)
- Fix track playback button icons being obscured by cover placeholder (cdab466)
- Use more broadly supported ID3v2.3 tags when transcoding to MP3 (Andy Berdan, c3a3752)
- Allow arbitrary font-weights for custom fonts (thurti, d7efd8d)
- Update bundled Barlow font, use tabular numbers for dynamic track time display (thurti, James Fenn, 501083d)
- Add russian translations (wileyfoxyx, ef7f45e, a852932)
- Update italian translations (Tommaso Croce, efe02fa)
- Update lithuanian translations (Vac, b2e4547)

## 0.22.1

- Fix critical build issue related to accidental commit of a subcrate versioning change (31a4b0f)

## 0.22.0

- Reimplement handling of disabled js for player, browser and copy link buttons (099272c)
- Reimplement handling of disabled js for purchase/unlock flows (f04687f)
- Implement handling of disabled js for embeds (7639162)
- Expand "more" section to full width on narrow viewports (bfef577)
- Map support artists also when main artists are explicitly set (91c5f26)

## 0.21.0

- Introduce configurable M3U playlist option for the entire catalog (0cebc42)
- Add hash-based cache invalidation for all linked assets and images (041cf4f, 482f9ba)
- Allow navigating to browse/search items by clicking their thumbnails (29ffa6e)
- Fix browse/search overlay closing too easily when focus is lost (375f8c5)
- Fix linking to non-existent artist pages when releases have varying but non-featured artists (1562d64)
- Left-align list markers inside margin, use disc or square style based on theme (aeffbf5)
- Update dutch translations (n3wjack, 1101b94, f5e3beb)
- Update french translations (sknob, 1511536)
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
