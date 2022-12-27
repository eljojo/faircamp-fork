# Architectural Notes

This file documents design decisions/thoughts that are not necessarily trivial
to arrive at, so they don't have to be thought through over and over again.
Also if changes becomes necessary the thinking process can start from an
already documented thought process, instead of starting at zero.

## Different artists may share the same alias

A catalog may have the following explicitly defined artists next to each other:

- "Alice" (name) with an alias "Alice (feat. Bob)"
- "Bob" (name) with an alias "Alice (feat. Bob)"

This allows both "Alice" and "Bob" to exist and be assigned to a track with
metadata storing an artist "Alice (feat. Bob)".

## Hotlinking countermeasures

Faircamp does not generally try to obfuscate anything about the site/url
hierarchy it generates - it would be technically pointless, and faircamp
rather aims to provide a site that is highly serviceable and easy to study
and understand on a technical level. However, if an artist or a label faces
blatant cases of hotlinking, e.g. publicly circulating direct download urls
to not-for-free releases, faircamp provides mechanisms for changing/rotating
parts of the asset download urls with a new deployment, thereby rendering
any already circulating hotlinks to downloads dysfunctional.

## Image descriptions are brought to the fore

Barriers that the blind or weak-sighted face on the web are most often
invisible to those that can see. An image without a description is just an
image to those who can see it, and the problem thus stays out of sight and
out of mind to those able to solve it. Faircamp brings those images to
everyone's attention instead, pointing them out not only during building, but
also in the generated site itself, where it's then in plain sight to everyone
that there are barriers to those without sight, until solved.

## Permalink conflicts are never automatically solved

Faircamp does not automatically resolve permalink conflicts because doing so
might inadvertently break links that people out on the web are already
using.
