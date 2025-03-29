// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-FileCopyrightText: 2024 Damian Szetela
// SPDX-FileCopyrightText: 2023 Harald Eilertsen
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

use translations::{
    CA,
    DE,
    EN,
    ES,
    FR,
    HE,
    IT,
    JA,
    LT,
    NB,
    NL,
    PL,
    PT_PT,
    RU,
    SR_CYRL,
    SR_LATN,
    SV,
    TR,
    UK
};
use translations::Translations;

pub struct Locale {
    /// Language code such as "en", "de" etc.
    /// This is notably used in the lang attribute on the html tag on all
    /// generated pages, and should therefore conform to BCP 78 (for reference
    /// see https://datatracker.ietf.org/doc/html/rfc5646 and/or the more general
    /// https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang).
    pub language: String,
    pub text_direction: TextDirection,
    pub translations: Translations,
}

pub enum TextDirection {
    Ltr,
    Rtl
}

// TODO: Runtime-based mechanism for adding or customizing locales
impl Locale {
    pub fn default() -> Locale {
        Locale::new("en", EN, TextDirection::Ltr)
    }

    pub fn from_code(language: &str) -> Locale {
        match language {
            "ca" => Locale::new("ca", CA, TextDirection::Ltr),
            "de" => Locale::new("de", DE, TextDirection::Ltr),
            "en" => Locale::new("en", EN, TextDirection::Ltr),
            "es" => Locale::new("es", ES, TextDirection::Ltr),
            "fr" => Locale::new("fr", FR, TextDirection::Ltr),
            "he" => Locale::new("he", HE, TextDirection::Rtl),
            "it" => Locale::new("it", IT, TextDirection::Ltr),
            "ja" => Locale::new("ja", JA, TextDirection::Ltr),
            "lt" => Locale::new("lt", LT, TextDirection::Ltr),
            "nb" => Locale::new("nb", NB, TextDirection::Ltr),
            "nl" => Locale::new("nl", NL, TextDirection::Ltr),
            "pl" => Locale::new("pl", PL, TextDirection::Ltr),
            "pt-pt" => Locale::new("pt-pt", PT_PT, TextDirection::Ltr),
            "ru" => Locale::new("ru", RU, TextDirection::Ltr),
            "sr-cyrl" => Locale::new("sr-cyrl", SR_CYRL, TextDirection::Ltr),
            "sr-latn" => Locale::new("sr-latn", SR_LATN, TextDirection::Ltr),
            "sv" => Locale::new("sv", SV, TextDirection::Ltr),
            "tr" => Locale::new("tr", TR, TextDirection::Ltr),
            "uk" => Locale::new("uk", UK, TextDirection::Ltr),
            _ => Locale::new(language, EN, TextDirection::from_code(language))
        }
    }

    pub fn keys() -> Locale {
        Locale::new("en", Translations::KEYS, TextDirection::Ltr)
    }

    fn new(
        language: &str,
        translations: Translations,
        text_direction: TextDirection
    ) -> Locale {
        Locale {
            language: language.to_owned(),
            text_direction,
            translations
        }
    }
}

impl TextDirection {
    /// Language codes compiled based on these (slightly diverging) lists:
    /// - https://meta.wikimedia.org/wiki/Template:List_of_language_names_ordered_by_code
    /// - https://localizejs.com/articles/localizing-for-right-to-left-languages-the-issues-to-consider/
    /// - https://lingohub.com/blog/right-to-left-vs-left-to-right
    /// - https://localizely.com/iso-639-1-list/
    pub fn from_code(code: &str) -> TextDirection {
        match code {
            "ar" |
            "arc" |
            "arz" |
            "ckb" |
            "dv" |
            "fa" |
            "ha" |
            "he" |
            "khw" |
            "ks" |
            "ku" |
            "ps" |
            "sd" |
            "ur" |
            "uz_AF" |
            "yi" => TextDirection::Rtl,
            _ => TextDirection::Ltr
        }
    }

    pub fn is_rtl(&self) -> bool {
        match self {
            TextDirection::Ltr => false,
            TextDirection::Rtl => true
        }
    }
}
