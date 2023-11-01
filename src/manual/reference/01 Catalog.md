# Catalog

Site-wide metadata and settings, such as the title and site URL.

```eno
# catalog

base_url: https://myawesomemusic.site/
feed_image: exported_logo_v3.jpg

home_image:
description = Me in my studio
file = studio_3.png

label_mode
show_support_artists
title: My awesome music

-- text
My self hosted faircamp site,
which presents some of my awesome music.

Nice of you to stop by!
-- text
```

## Label vs. Artist mode

By default faircamp operates in *artist mode* - it will lay out the site
in a way that best fits a single artist or band presenting
their works, meaning it will automatically take the artist associated
with the highest number of releases/tracks and name the catalog after them,
make the catalog description the description of that artist, etc..

The `label_mode` flag can be used if one wants to present multiple artists
on a single faircamp site. This adds an additional layer of information to the
page that differentiates the artists, gives them each their own page, etc.

```eno
label_mode
```

## General settings

To enable embeds and RSS feed generation you have to set `base_url`. The value
of `title` appears in multiple places on the site, inside the RSS Feed, etc..
For the RSS feed an optional `feed_image` can be specified. The catalog
`text` shows up prominently below the title on the homepage and it supports
[Markdown](https://commonmark.org/help/). The `home_image` is an image that will be displayed on the homepage,
e.g. a logo for your label or a band photo or such.

```eno
base_url: https://myawesomemusic.site/
feed_image: exported_logo_v3.jpg
title: My awesome music

home_image:
description = Me in my studio
file = studio_3.png

-- text
Lorem ipsum dolor sit amet ...
-- text
```

Note that the `feed_image`, unlike the `home_image`, needs (and supports) no image description, this is because RSS does not support any (hence the different syntax
to specify the image).

## Main & Support artists

A release can have one or more *main artists*, i.e. principal authors. Artists
that appear as collaborators are called *support artists* in faircamp. The
main artists are auto-detected (e.g. when they are the only artist for a release,
when they appear in the "Album Artist" tag in files, or when they appear as
artist on most tracks of a release).

By default, support artists are not listed in the interface. You can use the
`show_support_artists` flag to make them show up in listings.

```eno
show_support_artists
```

Also by default, support artists are never linked to, and also don't have
their own artist page. The `feature_support_artists`  flag can be used to
link them to, and give them their own, artist pages (this implicitly enables
`show_support_artists`). Note that this flag only affects label mode. In
artist mode no artist pages exist, instead the homepage *is* the one and only
artist page (the catalog artist's page).

```eno
feature_support_artists
```

## Dealing with malicious behavior

When third parties hotlink to your site's resources, or when you discover that
people are blatantly sharing direct download links to your releases,
faircamp offers two related configuration options to combat this:

```eno
rotate_download_urls
```

With `rotate_download_urls` enabled, faircamp will automatically generate new
download urls on each deployment (rendering invalid all previously existing
urls).

Similarly, you can also manually control this mechanism:

```eno
freeze_download_urls: [put-any-text-here]
```

Whatever text you put on the right is used to generate unique download urls
during deployment - but note that the text itself never shows up in the urls
themselves, it is merely used for randomization. The download urls stay valid
as long as the text does not change. Any time you update the text, all
download urls are regenerated, and thereby all old ones invalidated.
Practically speaking it makes sense to use some kind of a date as the text on
the right, for instance `freeze_download_urls: 1 April 2022` could tell you
that your current download urls have been valid since that day. You could
also use "2022-04", "Spring 2022" or such, given that one usually will not
manually invalidate the urls on a daily basis.
