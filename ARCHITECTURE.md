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

## Permalink conflicts are never automatically solved

Faircamp does not automatically resolve permalink conflicts because doing so
might inadvertently break links that people out on the web are already
using.
