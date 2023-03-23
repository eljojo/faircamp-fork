# Localization

## Supported languages

Faircamp currently only ships with a prototypical set of three languages:

- English (`en`)
- German (`de`)
- Spanish (`es`)

English is the default, german or spanish can be configured like this:

```eno
# localization

language: de
```

Translation corrections or improvements are very welcome, just [open an issue](https://codeberg.org/simonrepp/faircamp/issues).

## Unsupported languages

With a (for now) unsupported language you can still set a custom language code (which will be used e.g. for the RSS feed metadata) and writing direction (`ltr` or `rtl`).

```eno
# localization

language: he
writing_direction: rtl
```

If you are very eager about using faircamp in a not yet supported language
(and ready to help with the translations), please do open an issue to flag it
for prioritization. If you are familiar with coding you can already look at
[locale.rs](https://codeberg.org/simonrepp/faircamp/src/branch/main/src/locale.rs) to
get a feeling what will need to be done (it's not really complex).