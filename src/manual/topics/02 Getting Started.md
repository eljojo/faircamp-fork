# Getting Started

Before we begin let's quickly look at the input faircamp needs:

```
Superstar Artist/         <--- Top directory ("Catalog")
├─ Greatest Hits/           <--- Nested Directory ("Release")
│  ├─ One Hit Wonder.mp3      <--- Audio File ("Track") 
│  ├─ Summer Hit.mp3
│  └─ Underrated Cult Song.mp3
└─ Bootlegs/                <-- Extra nesting (optional)
   └─ Live in Megacity/       <--- Nested Directory ("Release")
      ├─ Best Ballad Ever.mp3   <--- Audio File ("Track")
      ├─ Another Summer Hit.mp3
      └─ CD Sleeve.jpg          <--- Cover image (optional)
```

We see: Faircamp takes a *directory with arbitrarily nested
directories* as input. The only convention to follow: *Directories
that directly contain audio files* will be presented as *releases*
(think albums, singles and playlists) with their own page.

Now to use faircamp, prepare your catalog folder (similar to `Superstar Artist/` above), `cd` into it  and run:

```
faircamp --preview
```

By default, faircamp will create a `.faircamp_build` and a `.faircamp_cache` folder inside the catalog directory. With `--preview` specified, it will automatically open
`.faircamp_build/index.html` inside your browser after building is complete.

And that's it, your faircamp site is now alive and kicking.