<!--
    SPDX-FileCopyrightText: 2023 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Concepts Explained

Faircamp (like any static site generator) has a divided nature when it comes
to simplification: On the one hand it dramatically lowers the bar for what it
takes to host and maintain ones own site, indeed for many people managing to
bring this into the realm of possibility at all. However, entering this world
after previously being accustomed to platforms like bandcamp or soundcloud,
it at the same time also heightens the bar for creating a site. This is to
some degree owed to the nature of self-empowerment itself - if you grow your
own tomatoes instead of buying them from the supermarket you have to
familiarize yourself at least to some degree with new knowledge around that -
but it also implicates a responsibility for the tool (faircamp) to make this
path to self-empowerment as accessible as it can be, so as not to lose those
who are willing to put in the effort but find unsurmountable odds, and
frustration coming from missing explanations.

This page then is a first step to address frustrating barriers for anyone
who does not come to faircamp with a computer science degree. We won't get
it perfect from the get-go but we want to get there eventually - if there
are any blockers, unclear concepts or wording, anything like that really,
please do report it either on the [issue tracker](https://codeberg.org/simonrepp/faircamp/issues),
by mentioning [@freebliss@post.lurk.org](https://post.lurk.org/@freebliss) on the
fediverse, or as a fallback also via email to simon@fdpl.io.

## What is a permalink?

Within faircamp, each of your releases (or artists in the case of labels)
is represented on its own page, and each page can be reached through its
own address (URL), i.e. the link that you can share with someone that you
want to point to a release or artist of yours.

All of these links will necessary share its basis - the domain under which
you present your faircamp site, let's say "https://examplerecordlabel.com" -
but also each of your pages needs some unique identifier (a "name" if you
will) by which is can be clearly, unambiguously and permanently (now and
in the future) identified. This part is the *permalink*.

So let's say our example record label publishes a release called
"Small Things EP" - what should the permalink for that be? Naturally we
would go for exactly the same ("Small Things EP"), but there is yet one
more catch: For technical reasons a permalink should contain only a
certain set of characters, only lowercase alphabetical letters (a-z),
digits (0-9) or dashes (-). Hence, one valid permalink we can set for
the release, probably the most logical one, would be "small-things-ep".
(Note though that it can literally be anything you want, as long as you
stick to the allowed characters)

In faircamp we would put this somewhere in a manifest (it depends on the
context where exactly, this is explained on other pages) like this:

```eno
permalink: small-things-ep
```

Ultimately by this we would achieve that any visitor to our site can
now and in the future always reach that release page under the URL
"https://examplerecordlabel.com/small-things-ep/", so as you see the
permalink simply gets appended to the base url of a site to make up
the full link that you can then share with your listeners.

## What is a static site generator?

Most websites we interact with on a daily basis are *dynamic*. If we liken a
website to a book for the sake of metaphor, we could say that every time we
look at a page, that page is written from scratch and then sent to us.
Sending an already written page is an easy task that almost any server that
you can rent for little money accomplishes. *Writing a page* however is
complex, and it requires a great deal of technical expertise to find the
right server for the task and upfront and continuous time investment to keep
the server in a state where it writes pages the way it should. A *static*
website then, is a site that is written once, ahead of time - a book whose
writing you finish on your own computer already, that is then transfered to a
server, whose sole responsibility then is to send it to everyone who wants to
read it, an easy task that it will accomplish for you for a long, long time,
practically without maintenance, as long as you pay your monthly server bill.
A static site generator, lastly, is the tool, the program, you use to write
the book. Faircamp is a static site generator, and the books it writes happen
to be music websites. This has been a lot of metaphors, hopefully some light
has been shed on the topic.
