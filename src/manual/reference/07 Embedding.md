# Embedding

> Heads up: Embeds are still work in progress - for the moment they will look weird or not work at all.

This allows external sites to embed a widget that presents music from your site.
The embed code can be copied from each release page where embedding is enabled.

Embedding is currently disabled by default, given it's not a finished feature.
If you want to enable it for testing purposes you first need to set the
catalog's `base_url` (this is a technical precondition for generating the
embeds at all), and then set `enabled`, for your entire catalog, or for
specific releases. If you set it `enabled` at the catalog level, you can also
use `disabled` at release level to re-disable it for specific releases.

```eno
# embedding

enabled
```