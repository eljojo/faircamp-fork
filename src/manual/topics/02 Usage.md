# Usage

Faircamp expects a hierarchy of directories, wherein the only convention to
follow is that directories that directly contain audio files will  be
presented as *releases* (think albums, singles and playlists) with their own
page.

This for instance would be a catalog with two abums:

```
Superstar Artist Catalog/
├─ The Best Of/
│  ├─ One Hit Wonder.mp3
│  ├─ Supertrack.mp3
│  └─ Summer Hit.mp3
└─ Greatest Hits/
   ├─ Best Ballad Ever.mp3
   ├─ Another Summer Hit.mp3
   └─ Unappreciated Cult Song.mp3
```

To run faircamp you would cd into the root directory (`Superstar Artist Catalog`) and execute:

```
faircamp
```

With its default settings, faircamp will create a `.faircamp_build` and a `.faircamp_cache` folder inside the directory you called it from. Open `.faircamp_build/index.html` inside your browser after building is complete (the `--preview` flag can also be used).

Run `faircamp -h` to get some help on command line options (there are quite a few).