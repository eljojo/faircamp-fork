<!--
    SPDX-FileCopyrightText: 2025 Simon Repp
    SPDX-FileCopyrightText: 2025 marqh
    SPDX-License-Identifier: CC0-1.0
-->

# Track manifests – track.eno

> All options at a glance: [copy_link](#copy_link), [cover](#cover), [download_code(s)](#download_codes), [embedding](#embedding), [link](#link), [more](#more), [more_label](#more_label), [payment_info](#payment_info), [speed_controls](#speed_controls), [streaming_quality](#streaming_quality), [synopsis](#synopsis), [tags](#tags), [theme](#theme), [title](#title), [track_artist(s)](#track_artists), [track_download_access](#track_download_access), [track_downloads](#track_downloads), [track_extras](#track_extras), [track_price](#track_price), [unlock_info](#unlock_info)

Track manifests are an optional way to specify metadata and settings at the
track level. A `track.eno` manifest **must** be placed inside a track
directory. To set up a track directory simply take the audio file of the
respective track and put it into its own directory (the naming of that
directory is entirely up to you, it does not affect anything). That is then
your track directory, where you also put the `track.eno` manifest in. For
example:

```
Example EP/               <--- Release directory
├─ cover.jpg                 <--- Release cover image
├─ release.eno               <--- Release manifest
├─ 01 First Track/           <--- Track directory
│  ├─ cover.jpg                 <--- Track cover image
│  ├─ track.eno                 <--- Track manifest
│  └─ 01 First Track.mp3        <--- Track audio file
├─ 02 Second Track.mp3       <--- Track audio file
└─ 03 Third Track.mp3        <--- Track audio file
```

As you can see, in a track directory you can also put an image file to be
used as cover for that track - this is picked up automatically! Additionally,
extra files in a track directory will be treated as *track extras*, by default
separately downloadable from the track's download page.

If you provide a cover image, use `description` within the [cover](#cover)
option to include an image description for it. This image
description functions as alt text, improving the accessibility of your site
for people with limited vision, screen readers, etc.

For all the things that can be customized in a `track.eno` manifest, see the
options documented below (and the "at a glance" overview at the top of the
page).

## <a name="copy_link"></a> `copy_link`

To disable the "Copy link" button (by default it's enabled) you can use the
`copy_link` option, with either `enabled` or `disabled` as value.

```eno
copy_link: disabled
```

## <a name="cover"></a> `cover`

```eno
cover:
description = An ink drawing of a barren tree with monkeys in its branches
file = cover.jpg
```

`file` is the path (or just filename) of the image, relative from the
manifest's location.

The `description` is used as image alt text, which improves accessibility
for those visiting your site with screen readers.

Note that track cover images are always displayed in square aspect ratio. If
you supply a non-square image it will be square-cropped for display.

## <a name="download_codes"></a> `download_code(s)`

To set a single download code that can be entered to access downloads:

```eno
download_code: crowdfunding2023
```

To set multiple download codes that can be entered to access downloads:

```eno
download_codes:
- GOLDsupporter
- SILVERsupporter
```

Note that you also need to use the [track_download_access](#track_download_access) option
(`track_download_access: code`) to activate download codes. In addition it is highly
recommended to use the [unlock_info](#unlock_info) option to provide a text that is
displayed alongside the code input prompt.

## <a name="embedding"></a> `embedding`

This allows external sites to embed a widget that lets people play back the
track from your site. The embed code can be copied from a page that is linked
from each release or track page where embedding is enabled.

Embedding is disabled by default. If you want to enable it you also need to
set the catalog's [base_url](catalog-catalog-eno.html#base_url) (embeds work
by displaying something from your site on another site, for this the other site
needs to point to your site's address), and then set `embedding: enabled`,
either in a catalog, artist, release or track manifest. If you set it `enabled`
at the catalog level, you can also use `disabled` at lower level to
re-disable it for specific releases.

```eno
embedding: enabled
```

## <a name="link"></a> `link`

```eno
link: https://example.com/this/track/elsewhere/

link:
url = https://example.com/this/track/elsewhere/

link:
label = A review for this track
url = https://example.com/some-blog/some-review/
```

You can supply any number of `link` fields, these are prominently displayed in
the header/landing area of your track page. A `link` must at least
provide a url, either as a simple value or as an `url` attribute. Optionally
you can also supply a `label` which is what is visibly displayed instead of
the `url`, when given.

## <a name="more"></a> `more`

```eno
-- more
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- more
```

This field lets you provide long-form content of any kind to augment the
track page with: Liner notes, production staff credits, lyrics,
making of stories, etc. When provided, this content appears right
after the track player/waveform on the track page.

The `more` field supports [Markdown](https://commonmark.org/help/).

## <a name="more_label"></a> `more_label`

```eno
more_label: Lyrics
```

If you provide long-form content for your track (which can be anything
you want, content-wise) through the [more](#more) field, by default there will be a
link with the label "More" on your track page, leading to the section
containing that content. This link and label will also appear right by the track
where it appears on the release page. If you want to customize that label so it
specifically refers to the type of content you are providing there, the
`more_label` field allows you to do that. Some typical examples of custom
labels one might use in the context of a track: "Lyrics", "Details", "Liner Notes",
"About" etc.

## <a name="payment_info"></a> `payment_info`

This is used together with the `paycurtain` setting of the [track_download_access](#track_download_access)
option (`track_download_access: paycurtain`) to set the text that is displayed before downloads are accessed.

The general idea here is to provide external links to one or more payment,
donation or patronage platforms that you use, be it liberapay, ko-fi, paypal,
stripe, etc. You can use [Markdown](https://commonmark.org/help/) to place
links, bullet points, etc. in the text.

```eno
-- payment_info
Most easily you can transfer the money for your purchase
via my [liberapay account](https://liberapay.com/somewhatsynthwave)

Another option is supporting me through my [ko-fi page](https://ko-fi.com/satanclaus92)

If you're in europe you can send the money via SEPA, contact me at
[lila@thatawesomeartist42.com](mailto:lila@thatawesomeartist42.com) and I'll
send you the account details.

On Dec 19th I'm playing a show at *Substage Indenhoven* - you can get the
digital album now and meet me at the merch stand in december in person to give
me the money yourself as well, make sure to make a note of it though! :)
-- payment_info
```

## <a name="speed_controls"></a> `speed_controls`

By default, faircamp's audio player(s) provide no playback speed controls,
assuming that it's intended for your audience to listen to the material at
its original speed only (e.g. for music, where this is usually the norm).

If you publish (e.g.) a narrative podcast or audio book and want people to
listen to the narration at their desired speed, you can enable playback speed
controls with this option:

```eno
speed_controls: enabled
```

By using `disabled` as value this setting can also be reverted.

## <a name="streaming_quality"></a> `streaming_quality`

```eno
streaming_quality: frugal
```

You can set the encoding quality for streaming from `standard` (the
default) to `frugal`. This uses considerably less bandwidth, reduces
emissions and improves load times for listeners, especially on slow
connections.

## <a name="synopsis"></a> `synopsis`

```eno
-- synopsis
Nobody thought it possible, until somebody did it. The track that started it all!
-- synopsis
```

A short (256 characters max), plain-text introduction text for your track,
this is prominently featured atop your track page.

## <a name="tags"></a> `tags`

By default faircamp strips all metadata off the audio files that you supply
when it transcodes them for streaming and downloading, only adding back those
tags that it needs and manages itself, i.e. the title, track number, artist
(s), release artist(s) and release title. The `tags` option lets you control
this behavior:

Set it to `copy` and faircamp will transfer all tags 1:1 from the
source file onto the transcoded files, as you provided them.

```eno
tags: copy
```

Set it to `remove` and faircamp will produce entirely untagged files for
streaming and download.

```eno
tags: remove
```

In order to assert fine-grained control over tags, you can also specify
precise behavior per tag. The available tags at this point are `album`,
`album_artist`, `artist`, `image`, `title` and `track` (= track number). The
available actions for each tag are `copy` (copy 1:1 from the source audio
files) and `rewrite` (set it from whichever information you implicitly or
explicitly gave faircamp that would override the original tag, or fall back
to the original tag value if there is no override). There is also `remove`,
but as any tag you don't explicitly provide in this form is implicitly set
to be removed, this is redundant. Note that support for writing embedded
cover images differs wildly between target formats, at this point pretty much
only the `flac` and `mp3` formats can be expected to reliably contain them,
no matter what you specify for `image`.

A random example of this:

```eno
tags:
album = rewrite
album_artist = remove
artist = rewrite
image = copy
title = copy
track = copy
```

The default behavior can be explicitly (re-)applied with the `normalize` option.

```eno
tags: normalize
```

When written out explicitly using the fine-grained notation, the default behavior
(that is, `tags: normalize`) corresponds to the following settings:

```eno
tags:
album = rewrite
album_artist = rewrite
artist = rewrite
image = remove
title = rewrite
track = rewrite
```

## <a name="theme"></a> `theme`

With this you can adjust the visual appearance of your faircamp site.

Theme customizations can be made in a top-level manifest at the root of the
catalog (setting the theme for the homepage and all release pages), but
they can also be made locally for a group of releases or for each release
on its own.

> Tip: There is a `--theming-widget` CLI option that lets you interactively
> explore color-related theme settings. Just build your catalog with the option enabled and
> every page will then contain the theming widget (don't forget to turn it off
> before deployment).

### Base

```eno
theme:
base = light
```

This sets the overall appearance, choose between `dark` (the default) and `light`.

### Dynamic range

```eno
theme:
dynamic_range = 24
```

At the highest dynamic range (100%) the theme appears the most "black" or "white"
(depending on your theme `base`) and the least colorful (depending on your chroma
settings, see below). The lower the dynamic range (0% being the default) the more it
will have a differentiated gray feeling (again interacting with your theme `base`),
and become over-all more colorfully tinted with rising base chroma levels. Tip: By
trying different values with the --theming-widget option you can interactively get
a good feeling of what this does and how you want to set it.

### Detail color adjustments

```eno
theme:
accent_brightening = 85
accent_chroma = 50
accent_hue = 23
base_chroma = 34
base_hue = 180
```

A site's theme is initially monochromatic (without color).

With `base_chroma` (0-100 (%)) you can control the overall "colorfulness"
of your site, while the `base_hue` (0-360 (degrees)) setting adjusts
what base color the theme is built on.

Some elements on the page are accentuated (prominent buttons and the
"timeline" of the audio player). The colorfulness of the accentuation can be
customized with the `accent_chroma` (0-100 (%)) setting, while the
`accent_hue` (0-360 (degrees)) setting adjusts its shade. The
`accent_brightening` (0-100 (%)) setting allows you to brighten or darken
this color accent (it's at 50% by default), which allows for stronger
and deeper colors still.

### Background image

```eno
theme:
background_alpha = 23
background_image = squiggly_monsters_texture.jpg
```

The previously described settings can be handled carefree - no matter the settings,
your site will stay readable (at worst it may look funny). When you set a
background image however, choose carefully what image you use and how opaque
you make it. Sharp details and strong contrasts within the image and against
the text of the site will render the site hard to read or even unreadable.
That said, `background_image` lets you reference the image to use, and with
`background_alpha` (0-100 (%)) you can optionally control its opaqueness.

### Round corners on release covers

To give a softer feel to your page, set the `round_corners` option to `enabled`.
This will visually round off the corners of covers on all pages. By setting it
back to `disabled` (the default) you can disable it for specific releases again.

```eno
theme:
round_corners = enabled
```

### Disabling relative waveform lengths

By default, the width of each track's waveform on a release page will render
at a different length, reflecting the duration of the track in relation to
the longest track on the release - for instance if the longest track on a
release is about two minutes long, that one will span the full width, but
another track that is only about one minute long will span only half of that
width. If you publish releases whose tracks have wildly varying lengths,
shorter tracks might get very narrow in the interface. If this is a concern
to you, or you just generally want all tracks to be full-width as an
aesthetic choice, you can enable this alternative behavior with this setting:

```eno
theme:
waveforms = absolute
```

### Disabling waveforms altogether

This will not display waveforms on the release page, resulting in a more compact layout.

```eno
theme:
waveforms = disabled
```

With `waveforms = enabled` you can turn this back on for specific releases if you want.

### Font

By default, faircamp bundles and uses the [Barlow](https://tribby.com/fonts/barlow/)
font on a generated site, but alternatively you can also configure your site to only
use your visitors' system font(s):

Using the standard sans serif font from the system of the visitor:

```eno
theme:
system_font = sans
```

Using the standard monospace font from the system of the visitor:

```eno
theme:
system_font = mono
```

Usage of custom fonts entails complexities and responsibilities that faircamp
can not generically automate away, therefore this requires manual integration
through the [site_assets](catalog-catalog-eno.html#site_assets) and
[site_metadata](catalog-catalog-eno.html#site_metadata) options - this also
gives a great amount of flexibility, including the possibility to use multiple
fonts and tweak their integration down to the last detail where needed.

## <a name="title"></a> `title`

The track title is automatically derived from the audio file metadata
(title tag) or filename of the track, however you can also override it with
this option.

```eno
title: Is it Friday yet?
```

## <a name="track_artists"></a> `track_artist(s)`

If your audio file is not tagged, or the tags contain inaccurate values, or
for any other reason, you can use the `track_artist` and `track_artists`
option to explicitly set one or multiple main artists for the track. Note
that this can implicitly affect the release artist (which can be explicitly
set with the [release_artist(s)](releases-release-eno.html#release_artists)
option in the release.eno manifest too).

To set a single artist for tracks on a release:

```eno
track_artist: Alice
```

To set a multiple artist for tracks on a release:

```eno
track_artists:
- Alice
- Bob
```

## <a name="track_download_access"></a> `track_download_access`

By default your visitors can only *stream* your tracks.

To enable free downloads of a track all you need to do is set one or more
download formats with the [track_downloads](#track_downloads) option.

Beyond this, the `track_download_access` option controls how visitors can
access downloads - by default as free downloads - but this can be changed to
external downloads, downloads accessible through download codes, or downloads
placed behind a soft paycurtain, and you can also disable access to downloads
here.

### Free downloads

This is the default (you don't need to set it yourself), but in case you want
to re-enable it in a manifest:

```eno
track_download_access: free
```

### External downloads

If you want to use your faircamp site purely to let people stream your audio,
but there is another place on the web where your track(s) can be
downloaded, external downloads allow you to display a download button that
merely takes people to the external download page.

For example, to display a download button that takes people to `https://example.com/artist/purchase/`,
simply use that url as the value for this setting:

```eno
track_download_access: https://example.com/artist/purchase/
```

### Download code(s)

A download code (like a coupon/token) needs to be entered to access downloads.

To protect downloads with a code:

```eno
track_download_access: code
```

In combination with this use the [download_code(s)](#download_codes) option to
set the codes for accessing downloads and the [payment_info](#payment_info)
option to provide a text that is displayed with the code input field (to give
your audience directions on how to obtain a download code).

### Soft Paycurtain

A soft (i.e. not technically enforced) paycurtain needs to be passed before downloading.

To provide downloads behind a soft paycurtain:

```eno
track_download_access: paycurtain
```

In combination with this option, use the [track_price](#track_price) and
[payment_info](#payment_info) options to set a price and give instructions
for where the payment can be made.

### Disable downloads

Downloads can also be disabled explicitly (e.g. if you quickly want to take them offline at some point):

```eno
track_download_access: disabled
```

## <a name="track_downloads"></a> `track_downloads`

Sets the formats in which single tracks can be separately downloaded.
By default none are specified, so this needs to be set in order to
enable separate downloads for single tracks.

To set a single download format:

```eno
track_downloads: flac
```

To set multiple download formats:

```eno
track_downloads:
- flac
- mp3
- opus
```

All currently available formats:
- `aac`
- `aiff`
- `alac`
- `flac`
- `mp3`
- `ogg_vorbis`
- `opus` (this is an alias for `opus_128`)
- `opus_48`
- `opus_96`
- `opus_128`
- `wav`

In practice a minimal combination of a lossy state of the art format
(e.g. `opus`), a lossy format with high compatibility (e.g. `mp3`) and a
lossless format (e.g. `flac`) is recommended.

## <a name="track_extras"></a> `track_extras`

Any additional files in a track directory besides the audio file, cover
image and track.eno manifest are considered "extras" and by default
offered as separate downloads on the track download page (think artwork, liner notes, lyrics,
etc.).

To turn this off and entirely omit track extras:

```eno
track_extras: disabled
```

To provide track extras as separate downloads (default behavior):

```eno
track_extras: enabled
```

Release downloads (zip archives) that include a track will also include the
extras of that track if `track_extras` is `enabled` for the track and the
`release_extras` option for the release is (or includes) `bundled`.

## <a name="track_price"></a> `track_price`

This is used together with the `paycurtain` setting of the [track_download_access](#track_download_access)
option (`track_download_access: paycurtain`) to set the price for track downloads that is
displayed before the downloads are accessed.

For example in order to ask for 4€ for accessing the downloads of a track:

```eno
price: EUR 4+
```

The `price` option accepts an [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code and a price range such as:

- `USD 0+` (Name your price, including zero dollars as a valid option)
- `3.50 GBP` (Exactly 3.50 Pounds)
- `KRW 9080` (Exactly 9080 south korean won)
- `INR 230+` (230 indian rupees or more)
- `JPY 400-800` (Between 400 and 800 japanese yen)

## <a name="unlock_info"></a> `unlock_info`

In combination with the `code` setting of the [track_download_access](#track_download_access)
option (`track_download_access: code`) and [download_code(s)](#download_codes) option, this
option lets you set the text that is displayed to your visitors when they are prompted for
a download code. Usually you will want to put instructions in the text that tell your
visitors how they can obtain a download code.

```eno
-- unlock_info
You should have received a download code in your confirmation mail
for this year's crowdfunding. Stay tuned in case you missed it,
we're currently planning the next run!
-- unlock_info
```
