# Catalog

By default faircamp operates in "single artist mode", i.e. it will lay out and
render the pages in a way that best fits a single artist/band presenting
their works, meaning it will automatically take the artist associated
with the highest number of releases/tracks and name the catalog after them,
make the catalog description the description of that artist, etc..

The `label_mode` flag can be used if one wants to present multiple artists
on a single faircamp site. This adds an additional layer of information to the
page that differentiates the artists, gives them each their own page, etc.

Asides this main mode toggle you can set the global site title (which appears
at the title of browser tabs, inside the RSS feed, etc.), the base url
(required for generation of embeds and the RSS feed), an optional RSS feed
image, as well as a description text for your catalog here.

Lastly, the `rotate_download_urls` flag can be specified to let faircamp
generate new download urls on each deployment (rendering invalid all
previously existing urls), which helps you to fight blatant hotlinking to
your downloads, should it ever occur. Similarly, you can specify
`freeze_download_urls: [put-any-text-here]`, to manually control the
invalidation of download urls: Whatever text you put on the right is used to
generate unique download urls on each deployment (note that the text itself
never shows up in the urls themselves, it is merely used for randomization).
The download urls stay valid as long as the text does not change. Any time
you update the text, all download urls are refreshed, and thereby all old
ones invalidated. Practically speaking, it makes sense to use some kind of
(current) calendar data as the text on the right, that way e.g.
`freeze_download_urls: 1 April 2022` could tell you that your current download
urls have been valid since that day. You could also use "October 2022" or
even just the year, given that one usually will not manually invalidate the
urls on a daily basis.

```eno
# catalog

base_url: https://myawesomemusic.site/
feed_image: exported_logo_v3.jpg
label_mode
title: My awesome music

-- text
My self hosted faircamp site,
which presents some of my awesome music.

Nice of you to stop by!
-- text
```