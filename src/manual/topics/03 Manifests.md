# Manifests

To specify metadata and settings create files with the extension `.eno` and any
filename of your choosing anywhere inside the catalog. Each manifest applies to
the folder  it is contained in, as well as (recursively) to all subfolders
therein. Manifests located deeper down in the folder hierarchy can override
metadata and settings specified in manifests in folders above.

```
catalog/
├─ my_top_level_manifest.eno
├─ release_a/
│  ├─ my_release_manifest_a.eno
│  ├─ track_a1.mp3
│  ├─ track_a2.mp3
│  └─ track_a3.mp3
└─ release_b/
   ├─ my_release_manifest_b.eno
   ├─ track_b1.mp3
   ├─ track_b2.mp3
   └─ track_b3.mp3
```

In the example above, everything defined in `my_top_level_manifest.eno` applies
to everything within `release_a` and `release_b`, but
`my_release_manifest_a.eno` can selectively override certain things for
everything in its containing folder `release_a`, as likewise
`my_release_manifest_b.eno` can selectively override certain things for
everything in its containing folder `release_b`.

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
syntax used in the manifest files consult the [eno language guide](https://eno-lang.org/guide/), simply modifying the examples in the manual
should get you there without any problems as well though, the example here is
pretty much as complex as it gets.