<!--
    SPDX-FileCopyrightText: 2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Faircamp Manual

To generate the manual run this binary with a single argument,
specifying the folder to which the manual should be written.

**Take extra care specifying the manual path, this directory
gets wiped and rewritten in the process.**

`cargo run -- /path/to/manual`

## Check internal links/references

For automatically checking integrity of internal cross-links in the manual,
including references to anchors on pages, use a tool like
<https://linkchecker.github.io/linkchecker/index.html> (with the
`AnchorCheck` plugin enabled in
<https://linkchecker.github.io/linkchecker/man/linkcheckerrc.html>) on the
generated manual.

For instance:

```
linkchecker [manual-path]/index.html
```