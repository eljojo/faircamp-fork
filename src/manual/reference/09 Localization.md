# Localization

This allows you to configure a language code (used e.g. for the RSS feed
metadata) and to switch from left-to-right to right-to-left presentation for
e.g. arabic and hebrew scripts. Note that the interface itself is already
fully translatable internally, this only needs some wrapping up before it
can be exposed through configuration.

```eno
# localization

language = he
writing_direction = rtl
```