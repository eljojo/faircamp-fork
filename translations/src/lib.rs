// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ops::Deref;

/// The Reviewed("Example") macro basically just serves as a marker for
/// a translation, to say it has been checked by at least one native speaker
/// or expert of a given language. Otherwise no transformation occurs.
macro_rules! reviewed {
    ($_value:literal) => {
        crate::Translation::Reviewed($_value)
    };
}

macro_rules! unreviewed {
    ($_value:literal) => {
        crate::Translation::Unreviewed($_value)
    };
}

/// In debug builds all untranslated strings return "UNTRANSLATED",
#[cfg(all(debug_assertions, not(test)))]
macro_rules! untranslated {
    ($_key:ident) => {
        crate::Translation::Untranslated("UNTRANSLATED")
    };
}

// In release and test builds all untranslated strings return an english fallback translation
#[cfg(any(not(debug_assertions), test))]
macro_rules! untranslated {
    ($key:ident) => {
        crate::EN.$key
    };
}

mod de;
mod en;
mod es;
mod fr;
mod he;
mod it;
mod nb;
mod new;
mod nl;
mod pl;
mod sr_cyrl;
mod sr_latn;
mod sv;
mod tr;
mod uk;

pub use de::DE;
pub use en::EN;
pub use es::ES;
pub use fr::FR;
pub use he::HE;
pub use it::IT;
pub use nb::NB;
pub use new::NEW;
pub use nl::NL;
pub use pl::PL;
pub use sr_cyrl::SR_CYRL;
pub use sr_latn::SR_LATN;
pub use sv::SV;
pub use tr::TR;
pub use uk::UK;

pub fn all_languages() -> Vec<LabelledTranslations> {
    vec![
        LabelledTranslations { code: "de", name: "German", translations: DE },
        LabelledTranslations { code: "en", name: "English", translations: EN },
        LabelledTranslations { code: "es", name: "Spanish", translations: ES },
        LabelledTranslations { code: "fr", name: "French", translations: FR },
        LabelledTranslations { code: "he", name: "Hebrew", translations: HE },
        LabelledTranslations { code: "it", name: "Italian", translations: IT },
        LabelledTranslations { code: "nb", name: "Norwegian Bokmål", translations: NB },
        LabelledTranslations { code: "nl", name: "Dutch", translations: NL },
        LabelledTranslations { code: "pl", name: "Polish", translations: PL },
        LabelledTranslations { code: "sr-cyrl", name: "Serbian (Cyrillic)", translations: SR_CYRL },
        LabelledTranslations { code: "sr-latn", name: "Serbian (Latin)", translations: SR_LATN },
        LabelledTranslations { code: "sv", name: "Swedish", translations: SV },
        LabelledTranslations { code: "tr", name: "Turkish", translations: TR },
        LabelledTranslations { code: "uk", name: "Ukrainian", translations: UK }
    ]
}

pub fn new_language() -> LabelledTranslations {
    LabelledTranslations { code: "..", name: "New Language", translations: NEW }
}

pub struct LabelledTranslations {
    pub code: &'static str,
    pub name: &'static str,
    pub translations: Translations
}

pub enum Translation {
    Reviewed(&'static str),
    Unreviewed(&'static str),
    Untranslated(&'static str)
}

impl Translation {
    pub fn status(&self) -> &'static str {
        match self {
            Translation::Reviewed(_) => "reviewed",
            Translation::Unreviewed(_) => "unreviewed",
            Translation::Untranslated(_) => "untranslated"
        }
    }
}

impl Deref for Translation {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            Translation::Reviewed(value) => value,
            Translation::Unreviewed(value) => value,
            Translation::Untranslated(value) => value
        }
    }
}

impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Translation::Reviewed(value) => value,
            Translation::Unreviewed(value) => value,
            Translation::Untranslated(value) => value
        };

        write!(f, "{}", text)
    }
}

/// A key-value mapping for every translatable string found in the interface.
/// Used at build time to interpolate text in the right language.
/// Translations whose fields are not public are instead accessed through
/// a method of the same name - these are translations that need to be called
/// as a function because they interpolate some parameter into the translation.
pub struct Translations {
    pub audio_format_alac: Translation,
    pub audio_format_average: Translation,
    pub audio_format_flac: Translation,
    pub audio_format_mp3: Translation,
    pub audio_format_opus_48: Translation,
    pub audio_format_opus_96: Translation,
    pub audio_format_opus_128: Translation,
    pub audio_format_uncompressed: Translation,
    pub audio_player_widget_for_xxx: Translation,
    pub auto_generated_cover: Translation,
    pub available_formats: Translation,
    pub buy: Translation,
    pub close: Translation,
    pub copied: Translation,
    pub copy: Translation,
    pub copy_link: Translation,
    pub confirm: Translation,
    pub r#continue: Translation,
    pub cover_image: Translation,
    pub default_unlock_text: Translation,
    pub dimmed: Translation,
    pub download: Translation,
    pub downloads: Translation,
    /// Must be unique and only contain url-safe characters
    pub downloads_permalink: Translation,
    pub embed: Translation,
    pub embed_entire_release: Translation,
    pub enter_code_here: Translation,
    pub external_link: Translation,
    pub extras: Translation,
    pub failed: Translation,
    pub feed: Translation,
    pub fixed_price: Translation,
    pub image_descriptions: Translation,
    pub image_descriptions_guide: Translation,
    /// Must be unique and only contain url-safe characters
    pub image_descriptions_permalink: Translation,
    pub listen: Translation,
    pub loading: Translation,
    pub m3u_playlist: Translation,
    pub made_or_arranged_payment: Translation,
    pub missing_image_description_note: Translation,
    pub more: Translation,
    pub mute: Translation,
    pub muted: Translation,
    pub name_your_price: Translation,
    pub next_track: Translation,
    pub pause: Translation,
    pub play: Translation,
    pub previous_track: Translation,
    pub purchase_downloads: Translation,
    /// Must be unique and only contain url-safe characters
    pub purchase_permalink: Translation,
    pub recommended_format: Translation,
    pub rss_feed: Translation,
    pub search: Translation,
    pub this_site_was_created_with_faircamp: Translation,
    pub unlisted: Translation,
    pub unlock: Translation,
    pub unlock_downloads: Translation,
    /// Must be unique and only contain url-safe characters
    pub unlock_permalink: Translation,
    pub unlock_code_seems_incorrect: Translation,
    pub unlock_manual_instructions: Translation,
    pub unmute: Translation,
    pub up_to_xxx: Translation,
    pub visual_impairment: Translation,
    pub volume: Translation,
    pub xxx_and_others: Translation,
    pub xxx_minutes: Translation,
    pub xxx_or_more: Translation
}

impl Translations {
    /// (key, value, is_multiline)
    pub fn all_strings(&self) -> Vec<(&'static str, &Translation, bool)> {
        vec![
            ("audio_format_alac", &self.audio_format_alac, false),
            ("audio_format_average", &self.audio_format_average, false),
            ("audio_format_flac", &self.audio_format_flac, false),
            ("audio_format_mp3", &self.audio_format_mp3, false),
            ("audio_format_opus_48", &self.audio_format_opus_48, false),
            ("audio_format_opus_96", &self.audio_format_opus_96, false),
            ("audio_format_opus_128", &self.audio_format_opus_128, false),
            ("audio_format_uncompressed", &self.audio_format_uncompressed, false),
            ("audio_player_widget_for_xxx", &self.audio_player_widget_for_xxx, false),
            ("auto_generated_cover", &self.auto_generated_cover, false),
            ("available_formats", &self.available_formats, false),
            ("buy", &self.buy, false),
            ("close", &self.close, false),
            ("copied", &self.copied, false),
            ("copy", &self.copy, false),
            ("copy_link", &self.copy_link, false),
            ("confirm", &self.confirm, false),
            ("continue", &self.r#continue, false),
            ("cover_image", &self.cover_image, false),
            ("default_unlock_text", &self.default_unlock_text, false),
            ("dimmed", &self.dimmed, false),
            ("download", &self.download, false),
            ("downloads", &self.downloads, false),
            ("downloads_permalink", &self.downloads_permalink, false),
            ("embed", &self.embed, false),
            ("embed_entire_release", &self.embed_entire_release, false),
            ("enter_code_here", &self.enter_code_here, false),
            ("external_link", &self.external_link, false),
            ("extras", &self.extras, false),
            ("failed", &self.failed, false),
            ("feed", &self.feed, false),
            ("fixed_price", &self.fixed_price, false),
            ("image_descriptions", &self.image_descriptions, false),
            ("image_descriptions_guide", &self.image_descriptions_guide, true),
            ("image_descriptions_permalink", &self.image_descriptions_permalink, false),
            ("listen", &self.listen, false),
            ("loading", &self.loading, false),
            ("m3u_playlist", &self.m3u_playlist, false),
            ("made_or_arranged_payment", &self.made_or_arranged_payment, false),
            ("missing_image_description_note", &self.missing_image_description_note, false),
            ("more", &self.more, false),
            ("mute", &self.mute, false),
            ("muted", &self.muted, false),
            ("name_your_price", &self.name_your_price, false),
            ("next_track", &self.next_track, false),
            ("pause", &self.pause, false),
            ("play", &self.play, false),
            ("previous_track", &self.previous_track, false),
            ("purchase_downloads", &self.purchase_downloads, false),
            ("purchase_permalink", &self.purchase_permalink, false),
            ("recommended_format", &self.recommended_format, false),
            ("rss_feed", &self.rss_feed, false),
            ("search", &self.search, false),
            ("this_site_was_created_with_faircamp", &self.this_site_was_created_with_faircamp, false),
            ("unlisted", &self.unlisted, false),
            ("unlock", &self.unlock, false),
            ("unlock_downloads", &self.unlock_downloads, false),
            ("unlock_permalink", &self.unlock_permalink, false),
            ("unlock_code_seems_incorrect", &self.unlock_code_seems_incorrect, false),
            ("unlock_manual_instructions", &self.unlock_manual_instructions, true),
            ("unmute", &self.unmute, false),
            ("up_to_xxx", &self.up_to_xxx, false),
            ("visual_impairment", &self.visual_impairment, false),
            ("volume", &self.volume, false),
            ("xxx_and_others", &self.xxx_and_others, false),
            ("xxx_minutes", &self.xxx_minutes, false),
            ("xxx_or_more", &self.xxx_or_more, false)
        ]
    }

    pub fn audio_player_widget_for_xxx(&self, title: &str) -> String {
        self.audio_player_widget_for_xxx.replace("{title}", title)
    }

    pub fn count_untranslated(&self) -> usize {
        self.all_strings()
            .iter()
            .filter(|string|
                if let Translation::Untranslated(_) = string.1 { true } else { false }
            )
            .count()
    }

    pub fn count_unreviewed(&self) -> usize {
        self.all_strings()
            .iter()
            .filter(|string|
                if let Translation::Unreviewed(_) = string.1 { true } else { false }
            )
            .count()
    }

    pub fn keys() -> Translations {
        Translations {
            audio_format_alac: reviewed!("audio_format_alac"),
            audio_format_average: reviewed!("audio_format_average"),
            audio_format_flac: reviewed!("audio_format_flac"),
            audio_format_mp3: reviewed!("audio_format_mp3"),
            audio_format_opus_48: reviewed!("audio_format_opus_48"),
            audio_format_opus_96: reviewed!("audio_format_opus_96"),
            audio_format_opus_128: reviewed!("audio_format_opus_128"),
            audio_format_uncompressed: reviewed!("audio_format_uncompressed"),
            audio_player_widget_for_xxx: reviewed!("audio_player_widget_for_xxx"),
            auto_generated_cover: reviewed!("auto_generated_cover"),
            available_formats: reviewed!("available_formats"),
            buy: reviewed!("buy"),
            close: reviewed!("close"),
            copied: reviewed!("copied"),
            copy: reviewed!("copy"),
            copy_link: reviewed!("copy_link"),
            confirm: reviewed!("confirm"),
            r#continue: reviewed!("continue"),
            cover_image: reviewed!("cover_image"),
            default_unlock_text: reviewed!("default_unlock_text"),
            dimmed: reviewed!("dimmed"),
            download: reviewed!("download"),
            downloads: reviewed!("downloads"),
            downloads_permalink: reviewed!("downloads_permalink"),
            embed: reviewed!("embed"),
            embed_entire_release: reviewed!("embed_entire_release"),
            enter_code_here: reviewed!("enter_code_here"),
            external_link: reviewed!("external_link"),
            extras: reviewed!("extras"),
            failed: reviewed!("failed"),
            feed: reviewed!("feed"),
            fixed_price: reviewed!("fixed_price"),
            image_descriptions: reviewed!("image_descriptions"),
            image_descriptions_guide: reviewed!("image_descriptions_guide"),
            image_descriptions_permalink: reviewed!("image_descriptions_permalink"),
            listen: reviewed!("listen"),
            loading: reviewed!("loading"),
            m3u_playlist: reviewed!("m3u_playlist"),
            made_or_arranged_payment: reviewed!("made_or_arranged_payment"),
            missing_image_description_note: reviewed!("missing_image_description_note"),
            more: reviewed!("more"),
            mute: reviewed!("mute"),
            muted: reviewed!("muted"),
            name_your_price: reviewed!("name_your_price"),
            next_track: reviewed!("next_track"),
            pause: reviewed!("pause"),
            play: reviewed!("play"),
            previous_track: reviewed!("previous_track"),
            purchase_downloads: reviewed!("purchase_downloads"),
            purchase_permalink: reviewed!("purchase_permalink"),
            recommended_format: reviewed!("recommended_format"),
            rss_feed: reviewed!("rss_feed"),
            search: reviewed!("search"),
            this_site_was_created_with_faircamp: reviewed!("this_site_was_created_with_faircamp"),
            unlisted: reviewed!("unlisted"),
            unlock: reviewed!("unlock"),
            unlock_downloads: reviewed!("unlock_downloads"),
            unlock_permalink: reviewed!("unlock_permalink"),
            unlock_code_seems_incorrect: reviewed!("unlock_code_seems_incorrect"),
            unlock_manual_instructions: reviewed!("unlock_manual_instructions"),
            unmute: reviewed!("unmute"),
            up_to_xxx: reviewed!("up_to_xxx"),
            visual_impairment: reviewed!("visual_impairment"),
            volume: reviewed!("volume"),
            xxx_and_others: reviewed!("xxx_and_others"),
            xxx_minutes: reviewed!("xxx_minutes"),
            xxx_or_more: reviewed!("xxx_or_more")
        }
    }

    pub fn percent_reviewed(&self) -> f32 {
        let mut total = 0;
        let mut reviewed = 0;

        for string in self.all_strings() {
            total += 1;

            match string.1 {
                Translation::Reviewed(_) => {
                    reviewed += 1;
                }
                Translation::Unreviewed(_) |
                Translation::Untranslated(_) => ()
            }
        }

        (reviewed as f32 / total as f32) * 100.0
    }

    pub fn percent_translated(&self) -> f32 {
        let mut total = 0;
        let mut translated = 0;

        for string in self.all_strings() {
            total += 1;

            match string.1 {
                Translation::Reviewed(_) |
                Translation::Unreviewed(_) => {
                    translated += 1;
                }
                Translation::Untranslated(_) => ()
            }
        }

        (translated as f32 / total as f32) * 100.0
    }

    pub fn this_site_was_created_with_faircamp(&self, faircamp_link: &str) -> String {
        self.this_site_was_created_with_faircamp.replace("{faircamp_link}", faircamp_link)
    }

    pub fn unlock_manual_instructions(&self, page_hash: &str, index_suffix: &str) -> String {
        self.unlock_manual_instructions
            .replace("{downloads_permalink}", &self.downloads_permalink)
            .replace("{index_suffix}", index_suffix)
            .replace("{page_hash}", page_hash)
            .replace("{unlock_permalink}", &self.unlock_permalink)
    }

    pub fn up_to_xxx(&self, xxx: &str) -> String {
        self.up_to_xxx.replace("{xxx}", xxx)
    }

    pub fn xxx_and_others(&self, xxx: &str, others_link: &str) -> String {
        self.xxx_and_others
            .replace("{xxx}", xxx)
            .replace("{others_link}", others_link)
    }

    pub fn xxx_minutes(&self, xxx: &str) -> String {
        self.xxx_minutes.replace("{xxx}", xxx)
    }

    pub fn xxx_or_more(&self, xxx: &str) -> String {
        self.xxx_or_more.replace("{xxx}", xxx)
    }
}

#[test]
fn check_translations() {
    let locales = [DE, EN, ES, FR, HE, IT, NB, NL, PL, SR_CYRL, SV, TR];

    for translations in &locales {
        assert!(&translations.audio_player_widget_for_xxx.contains("{title}"));
        assert!(&translations.this_site_was_created_with_faircamp.contains("{faircamp_link}"));
        assert!(&translations.unlock_manual_instructions.contains("{downloads_permalink}"));
        assert!(&translations.unlock_manual_instructions.contains("{index_suffix}"));
        assert!(&translations.unlock_manual_instructions.contains("{page_hash}"));
        assert!(&translations.unlock_manual_instructions.contains("{unlock_permalink}"));
        assert!(&translations.up_to_xxx.contains("{xxx}"));
        assert!(&translations.xxx_and_others.contains("{xxx}"));
        assert!(&translations.xxx_and_others.contains("{others_link}"));
        assert!(&translations.xxx_minutes.contains("{xxx}"));
        assert!(&translations.xxx_or_more.contains("{xxx}"));

        let disallowed_char = |c: char| !c.is_ascii_alphanumeric() && c != '-' ;

        assert!(!&translations.downloads_permalink.contains(disallowed_char));
        assert!(!&translations.image_descriptions_permalink.contains(disallowed_char));
        assert!(!&translations.purchase_permalink.contains(disallowed_char));
        assert!(!&translations.unlock_permalink.contains(disallowed_char));
    }
}
