mod de;
mod en;
mod es;
mod fr;
mod nb;
mod nl;
mod pl;

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

/// A key-value mapping for every translatable string found in the interface.
/// Used at build time to interpolate text in the right language.
/// Translations whose fields are not public are instead accessed through
/// a method of the same name - these are translations that need to be called
/// as a function because they interpolate some parameter into the translation.
pub struct Translations {
    pub audio_format_alac: String,
    pub audio_format_average: String,
    pub audio_format_flac: String,
    pub audio_format_mp3: String,
    pub audio_format_opus_48: String,
    pub audio_format_opus_96: String,
    pub audio_format_opus_128: String,
    pub audio_format_uncompressed: String,
    audio_player_widget_for_release: String,
    audio_player_widget_for_track: String,
    pub auto_generated_cover: String,
    pub available_formats: String,
    pub close: String,
    pub copied: String,
    pub copy: String,
    pub confirm: String,
    pub r#continue: String,
    pub cover_image: String,
    pub default_unlock_text: String,
    pub downloads: String,
    /// Must be unique and only contain url-safe characters
    pub downloads_permalink: String,
    pub embed: String,
    pub embed_entire_release: String,
    pub enter_code_here: String,
    pub extras: String,
    pub failed: String,
    pub feed: String,
    pub fixed_price: String,
    pub image_descriptions: String,
    pub image_descriptions_guide: String,
    /// Must be unique and only contain url-safe characters
    pub image_descriptions_permalink: String,
    pub made_or_arranged_payment: String,
    pub missing_image_description_note: String,
    pub name_your_price: String,
    pub option: String,
    pub pay_on_liberapay: String,
    pub payment_options: String,
    pub purchase_downloads: String,
    /// Must be unique and only contain url-safe characters
    pub purchase_permalink: String,
    pub recommended_format: String,
    pub rss_feed: String,
    pub share: String,
    pub share_not_available_navigator_clipboard: String,
    pub share_not_available_requires_javascript: String,
    pub unlock: String,
    pub unlock_downloads: String,
    /// Must be unique and only contain url-safe characters
    pub unlock_permalink: String,
    pub unlock_code_seems_incorrect: String,
    unlock_manual_instructions: String,
    up_to_xxx: String,
    xxx_or_more: String
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

impl Translations {
    pub fn audio_player_widget_for_release(&self, title: &str) -> String {
        self.audio_player_widget_for_release.replace("{title}", title)
    }

    pub fn audio_player_widget_for_track(&self, title: &str) -> String {
        self.audio_player_widget_for_track.replace("{title}", title)
    }

    pub fn keys() -> Translations {
        Translations {
            audio_format_alac: String::from("audio_format_alac"),
            audio_format_average: String::from("audio_format_average"),
            audio_format_flac: String::from("audio_format_flac"),
            audio_format_mp3: String::from("audio_format_mp3"),
            audio_format_opus_48: String::from("audio_format_opus_48"),
            audio_format_opus_96: String::from("audio_format_opus_96"),
            audio_format_opus_128: String::from("audio_format_opus_128"),
            audio_format_uncompressed: String::from("audio_format_uncompressed"),
            audio_player_widget_for_release: String::from("audio_player_widget_for_release"),
            audio_player_widget_for_track: String::from("audio_player_widget_for_track"),
            auto_generated_cover: String::from("audio_player_widget_for_track"),
            available_formats: String::from("available_formats"),
            close: String::from("close"),
            copied: String::from("copied"),
            copy: String::from("copy"),
            confirm: String::from("confirm"),
            r#continue: String::from("continue"),
            cover_image: String::from("cover_image"),
            default_unlock_text: String::from("default_unlock_text"),
            downloads: String::from("downloads"),
            downloads_permalink: String::from("downloads_permalink"),
            embed: String::from("embed"),
            embed_entire_release: String::from("embed_entire_release"),
            enter_code_here: String::from("enter_code_here"),
            extras: String::from("extras"),
            failed: String::from("failed"),
            feed: String::from("feed"),
            fixed_price: String::from("fixed_price"),
            image_descriptions: String::from("image_descriptions"),
            image_descriptions_guide: String::from("image_descriptions_guide"),
            image_descriptions_permalink: String::from("image_descriptions_permalink"),
            made_or_arranged_payment: String::from("made_or_arranged_payment"),
            missing_image_description_note: String::from("missing_image_description_note"),
            name_your_price: String::from("name_your_price"),
            option: String::from("option"),
            pay_on_liberapay: String::from("pay_on_liberapay"),
            payment_options: String::from("payment_options"),
            purchase_downloads: String::from("purchase_downloads"),
            purchase_permalink: String::from("purchase_permalink"),
            recommended_format: String::from("recommended_format"),
            rss_feed: String::from("rss_feed"),
            share: String::from("share"),
            share_not_available_navigator_clipboard: String::from("share_not_available_navigator_clipboard"),
            share_not_available_requires_javascript: String::from("share_not_available_requires_javascript"),
            unlock: String::from("unlock"),
            unlock_downloads: String::from("unlock_downloads"),
            unlock_permalink: String::from("unlock_permalink"),
            unlock_code_seems_incorrect: String::from("unlock_code_seems_incorrect"),
            unlock_manual_instructions: String::from("unlock_manual_instructions"),
            up_to_xxx: String::from("up_to_xxx"),
            xxx_or_more: String::from("xxx_or_more")
        }
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

    pub fn xxx_or_more(&self, xxx: &str) -> String {
        self.xxx_or_more.replace("{xxx}", xxx)
    }
}

#[test]
fn check_translations() {
    let locales = [
        de::translations(),
        en::translations(),
        es::translations(),
        fr::translations(),
        nb::translations(),
        nl::translations(),
        pl::translations()
    ];

    for translations in &locales {
        assert!(&translations.audio_player_widget_for_release.contains("{title}"));
        assert!(&translations.audio_player_widget_for_track.contains("{title}"));
        assert!(&translations.unlock_manual_instructions.contains("{downloads_permalink}"));
        assert!(&translations.unlock_manual_instructions.contains("{index_suffix}"));
        assert!(&translations.unlock_manual_instructions.contains("{page_hash}"));
        assert!(&translations.unlock_manual_instructions.contains("{unlock_permalink}"));
        assert!(&translations.up_to_xxx.contains("{xxx}"));
        assert!(&translations.xxx_or_more.contains("{xxx}"));

        let disallowed_char = |c: char| !c.is_ascii_alphanumeric() && c != '-' ;

        assert!(!&translations.downloads_permalink.contains(disallowed_char));
        assert!(!&translations.image_descriptions_permalink.contains(disallowed_char));
        assert!(!&translations.purchase_permalink.contains(disallowed_char));
        assert!(!&translations.unlock_permalink.contains(disallowed_char));
    }
}