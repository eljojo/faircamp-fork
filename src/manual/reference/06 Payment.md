<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Payment

Important: The options described here are only needed and only processed in
conjunction with the soft paycurtain option (see the reference
on "Downloads"). If you want to place general links for financial support
that are not directly tied to downloads/purchases, you can put them in the
catalog text (frontpage) and/or release page texts.

This sets payment options that are shown when someone wants to purchase one of
your releases. The idea here is simply that you provide external links to one
or more payment, donation or patronage platforms that you use, be it
liberapay, ko-fi, paypal, stripe, you name it. You could even link to
bandcamp if you want to use it in parallel with your faircamp site, why not!

Each `custom` field specifies one payment option, and you can use
[Markdown](https://commonmark.org/help/) in them.

Below is an example configuration with 4 different payment options specified,
to show that pretty much anything goes.

```eno
# payment

-- custom
Most easily you can transfer the money for your purchase
via my [liberapay account](https://liberapay.com/somewhatsynthwave)
-- custom

-- custom
Another option is supporting me through my [ko-fi page](https://ko-fi.com/satanclaus92)
-- custom

-- custom
On Dec 19th I'm playing a show at *Substage Indenhoven* - you can get the
digital album now and meet me at the merch stand in december in person to give
me the money yourself as well, make sure to make a note of it though! :)
-- custom

-- custom
If you're in europe you can send the money via SEPA, contact me at
[lila@thatawesomeartist42.com](mailto:lila@thatawesomeartist42.com) and I'll
send you the account details.
-- custom
```
