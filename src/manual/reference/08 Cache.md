<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->


# Cache

Advanced control over caching strategy.

```eno
# cache

optimization: [delayed|immediate|manual|wipe]
```

Faircamp maintains an asset cache that holds the results of all
computation-heavy build artifacts (transcoded audio files, images, and
compressed archives). By default this cache uses a delayed optimization
strategy: Any asset that is not directly used in a build gets marked as stale
and past a certain period (e.g. 24 hours) gets purged from the cache during a
follow-up build (if it is not meanwhile reactivated because it's needed
again). This strikes a nice balance for achieving instant build speeds during
editing (after assets have been generated initially) without inadvertently
growing a storage resource leak in a directory you don't ever look at
normally.

If you're short on disk space you can switch to `immediate` optimization,
which purges stale assets right after each build (which might result in small
configuration mistakes wiping cached assets that took long to generate as a
drawback).

If you're even shorter on disk space you can use `wipe` optimization, which
just completely wipes the cache right after each build (so everything needs
to be regenerated on each build).

If you're more the structured type you can use  `manual` optimization, which
does not automatically purge anything from the cache but instead prints back
reports on stale assets after each build and lets you use
`faircamp --optimize-cache` and `faircamp --wipe-cache` appropriately
whenever you're done with your changes and don't expect to generate any new
builds for a while.