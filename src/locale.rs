// SPDX-FileCopyrightText: 2023-2024 Simon Repp
// SPDX-FileCopyrightText: 2024 Damian Szetela
// SPDX-FileCopyrightText: 2023 Harald Eilertsen
// SPDX-FileCopyrightText: 2023 Deborah Pickett
// SPDX-License-Identifier: AGPL-3.0-or-later

/// In debug builds all untranslated strings return "UNTRANSLATED"
#[cfg(debug_assertions)]
macro_rules! untranslated {
    ($_key:ident) => {
        String::from("UNTRANSLATED")
    };
}

// In release builds all untranslated strings return an english fallback translation
#[cfg(not(debug_assertions))]
macro_rules! untranslated {
    ($key:ident) => {
        super::en::translations().$key
    };
}

mod de;
mod en;
mod es;
mod fr;
mod nb;
mod nl;
mod pl;
mod translations;

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
        Locale::new("en", en::translations(), TextDirection::Ltr)
    }

    pub fn from_code(language: &str) -> Locale {
        match language {
            "de" => Locale::new("de", de::translations(), TextDirection::Ltr),
            "en" => Locale::new("en", en::translations(), TextDirection::Ltr),
            "es" => Locale::new("es", es::translations(), TextDirection::Ltr),
            "fr" => Locale::new("fr", fr::translations(), TextDirection::Ltr),
            "nb" => Locale::new("nb", nb::translations(), TextDirection::Ltr),
            "nl" => Locale::new("nl", nl::translations(), TextDirection::Ltr),
            "pl" => Locale::new("pl", pl::translations(), TextDirection::Ltr),
            _ => Locale::new(language, en::translations(), TextDirection::from_code(language))
        }
    }

    pub fn keys() -> Locale {
        Locale::new("en", Translations::keys(), TextDirection::Ltr)
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
