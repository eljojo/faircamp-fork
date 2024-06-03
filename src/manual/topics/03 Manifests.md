<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Manifests

To specify metadata and settings create files with the extension `.eno` and any
filename of your choosing anywhere inside the catalog. Each manifest applies to
the folder it is contained in, as well as (recursively) to all subfolders.
Manifests located deeper down in the folder hierarchy can override
metadata and settings specified in manifests in folders above.

```
catalog/
├─ a.eno
├─ first_release/
│  ├─ b.eno
│  ├─ track_1.mp3
│  ├─ track_2.mp3
│  └─ track_3.mp3
└─ second_release/
   ├─ c.eno
   ├─ track_1.mp3
   ├─ track_2.mp3
   └─ track_3.mp3
```

In the example above, everything defined in `a.eno` applies to `first_release`
and `second_release`, but `b.eno` can selectively override options for
`first_release`, as likewise `c.eno` can selectively override options for
`second_release`.

Here is an example manifest to give you an idea of how they work:

```eno
# release

cover:
description = An ink drawing of a barren tree with monkeys in its branches
file = cover.jpg

-- text
Recorded in the summer of '94 at West Callaghan Ranch, XE.

Featuring Ted Tukowsky on Trombone and Lisa Merringfield on Theremin.
-- text
```

Note that manifest lines such as `# release` are not comments but denote
sections (and instead `> these are comments`). For a detailed guide on the
syntax used in the manifest files see the language guide on the [eno website](https://simonrepp.com/eno/),
simply modifying the examples in the manual should get you there without any
problems as well though, the example here is pretty much as complex as it gets.