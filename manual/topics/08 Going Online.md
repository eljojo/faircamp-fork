<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Going Online

Faircamp provides you with a complete website - technically this takes the
form of a directory on your computer, by default the `.faircamp_build`
directory (hidden, as it starts on a dot) that gets created right inside your
catalog directory. In order to actually get your site online, these things
are needed:

- A domainname such as *example.com*

  These can be bought from a number of providers online, and are paid yearly.
  Price varies a lot depending on the TLD (the .com/.net/.something part),
  and a litte depending on the provider. The cheapest domains start out at
  about 10-20$ a year (usually long-existing, popular TLDs like .com). Ask
  online or among friends for provider recommendations, compare prices, also
  pay attention that some providers will offer very cheap rates for the first
  year only, but then let you pay the actual price from the second year on.

- A place to host your files (webhosting)

  This too can be bought from a number of providers online. It's recommended
  to get a webhosting subscription (which might also be called "webspace" or
  differently with some providers) from the same provider that you get your
  domain from, this simplifies the process. Make sure that your webhosting
  package includes enough space (a faircamp site will conventionally have
  between a few hundred MB to a few GB, this depends a lot on your content).
  Here as well it pays to ask around a bit and compare options. Webhosting
  will cost you a few $ per month, starting at around 2$, again depends on
  the specifics and provider. You can also host a faircamp site from a
  computer on your own premises, but this is a bit of a different story and
  for now not covered here.

- A means to upload your files (e.g. FTP or SSH)

  Conventionally, your webhosting provider allows you to set up an FTP
  account, that is, practically, an address, username and password that you
  enter in an *FTP client* in order to get access to the file system of your
  webhost and be able to transfer files from your computer up to their
  server. A very capable, free and open source FTP client that is available
  on all platforms is [FileZilla](https://filezilla-project.org/), but there
  are plenty others you can use. Using such a client you upload the generated
  faircamp site (the *contents* of the `.faircamp_build` directory) into the
  right directory on your server (your provider should provide documentation
  where to place it), and then you might still need to "connect" your domain
  name to this folder using your webprovider's user interface - this too
  should be documented by your provider. Here as well don't shy away from
  asking online, asking friends, or asking a search engine for guides on
  this, they are out there.

  A more advanced way to upload the site, if supported by the provider, which
  brings additional convenience once you get it to work, is to use a tool
  like [rsync](https://www.digitalocean.com/community/tutorials/how-to-use-rsync-to-sync-local-and-remote-directories)
  to automatically synchronize the contents between your locally generated
  faircamp site and the online hosted version. This requires a webprovider
  that offers ssh access - most, but not all, do.

## Known Gotchas

**Encoding of special characters in filenames**

In very rare circumstances it can happen that the way some special characters
are encoded in filenames in your faircamp site is subtly changed during
upload to the server, leading to that file not being accesible from the
faircamp site, although present. Concretely speaking, this has been observed
by someone who used [Cyberduck](https://cyberduck.io/) to upload their site.
One song containing "å" was not accessible because it was encoded as just "å"
on their computer, but converted to "a" + "°" during upload (a perfectly valid
and visually indistinguishable way to encode the character using unicode). If
you're observing something similar, try using another FTP client (or rsync),
and/or it should theoretically also be possible to configure Cyberduck to not
do this conversion on upload.
