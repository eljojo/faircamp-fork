<!--
    SPDX-FileCopyrightText: 2023-2024 Simon Repp
    SPDX-License-Identifier: CC0-1.0
-->

# Localization

## Available languages

Faircamp currently ships with these languages:

- Dutch `nl`
- English `en`
- French `fr`
- German `de`
- Norwegian Bokmål `nb`
- Polish `pl`
- Spanish `es`

English is the default, another language can be configured like this:

```eno
# localization

language: fr
```

Translation corrections or improvements are very welcome (dutch and spanish probably direly need them), just [open an issue](https://codeberg.org/simonrepp/faircamp/issues).

## Not yet available languages

Note that even if there are no translations for your language yet, you can still set the
language code, which is then used to auto-determine the text direction (LTR/RTL),
and declare the language for your content on the site and in RSS feed metadata -
only the interface texts will still be in english.

```eno
# localization

language: ar
```

## Contributing translations

If you are eager to use faircamp with a not yet supported language, you can
easily help to make it happen: Open the [english locale](https://codeberg.org/simonrepp/faircamp/src/branch/main/src/locale/en.rs),
copy its content and just replace the english translations with the ones in
your language. Post the results in an [issue](https://codeberg.org/simonrepp/faircamp/issues)
or send them to simon@fdpl.io, then we'll wrap up the rest together. If you're
code/git savvy you can also directly submit a PR. Either way, don't worry about
making mistakes, I'll help you with the technical details, it's much appreciated.
