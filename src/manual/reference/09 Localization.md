# Localization

## Supported languages

Faircamp currently ships with four languages:

- English `en`
- French `fr`
- German `de`
- Spanish `es`

English is the default, another language can be configured like this:

```eno
# localization

language: fr
```

Translation corrections or improvements are very welcome (french and spanish probably direly need them), just [open an issue](https://codeberg.org/simonrepp/faircamp/issues).

## Unsupported languages

If you are eager to use faircamp with a not yet supported language, you can
easily help to make it happen: Open the [english locale](https://codeberg.org/simonrepp/faircamp/src/branch/main/src/locale/en.rs),
copy its content and just replace the english translations with the ones in
your language. Post the results in an [issue](https://codeberg.org/simonrepp/faircamp/issues)
or send them to simon@fdpl.io, then we'll wrap up the rest together. If you're
code/git savvy you can also directly submit a PR. Either way, don't worry about
making mistakes, I'll help you with the technical details, it's much appreciated.

Note that even with an unsupported language you can still set a custom
language code (which will be used e.g. for the RSS feed metadata) and writing
direction (`ltr` or `rtl`).

```eno
# localization

language: he
writing_direction: rtl
```