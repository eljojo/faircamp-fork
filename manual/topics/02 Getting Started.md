<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Getting Started

First let's quickly look at the input faircamp needs:

```
The Artist/               <--- Top directory ("Catalog")
├─ Greatest Hits/           <--- Nested Directory ("Release")
│  ├─ One Hit Wonder.mp3      <--- Audio File ("Track")
│  ├─ Summer Hit.mp3
│  └─ Underrated Cult Song.mp3
└─ Bootlegs/                <-- Extra nesting (optional)
   └─ Live in Megacity/       <--- Nested Directory ("Release")
      ├─ Best Ballad Ever.mp3   <--- Audio File ("Track")
      ├─ Another Summer Hit.mp3
      ├─ CD Sleeve.jpg          <--- Cover image (optional)
      └─ Booklet.pdf            <--- Extra files to include (optional)
```

We see that faircamp takes a directory with arbitrarily nested directories as
input. The only convention to follow: Directories that directly contain audio
files will be presented as *releases* (think albums, singles and playlists)
with their own page.

To use faircamp, prepare your catalog folder (similar to `The Artist/` above), `cd` into it and run:

```
faircamp --preview
```

By default, faircamp will write the site to a `.faircamp_build` folder inside
the catalog directory. With `--preview` specified, it will automatically open
the site inside your browser after building is complete. (You can press
`Ctrl+C` in the terminal to kill the preview server again)

And that's it, your faircamp site is now alive and kicking.