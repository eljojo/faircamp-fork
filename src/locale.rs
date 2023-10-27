mod de;
mod en;
mod es;
mod fr;

pub struct Locale {
    /// Language code such as "en", "de" etc.
    pub language: String,
    pub translations: Translations,
    pub writing_direction: WritingDirection
}

/// A key-value mapping for every translatable string found in the interface.
/// Used at build time to interpolate text in the right language.
/// Translations whose fields are not public are instead accessed through
/// a method of the same name - these are translations that need to be called
/// as a function because they interpolate some parameter into the translation.
pub struct Translations {
    pub audio_format_description_aac: String,
    pub audio_format_description_aiff: String,
    pub audio_format_description_flac: String,
    pub audio_format_description_mp3_vbr: String,
    pub audio_format_description_ogg_vorbis: String,
    pub audio_format_description_opus_48: String,
    pub audio_format_description_opus_96: String,
    pub audio_format_description_opus_128: String,
    pub audio_format_description_wav: String,
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
    pub download_choice_hints: String,
    pub embed: String,
    pub embed_entire_release: String,
    pub enter_code_here: String,
    pub entire_release: String,
    pub failed: String,
    pub feed: String,
    pub fixed_price: String,
    pub format_guide: String,
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

pub enum WritingDirection {
    Ltr,
    Rtl
}

// TODO: Runtime-based mechanism for adding or customizing locales
impl Locale {
    pub fn default() -> Locale {
        Locale::new("en", en::translations(), WritingDirection::Ltr)
    }

    pub fn from_code(language: &str) -> Option<Locale> {
        match language {
            "de" => Some(Locale::new("de", de::translations(), WritingDirection::Ltr)),
            "en" => Some(Locale::new("en", en::translations(), WritingDirection::Ltr)),
            "es" => Some(Locale::new("es", es::translations(), WritingDirection::Ltr)),
            "fr" => Some(Locale::new("fr", fr::translations(), WritingDirection::Ltr)),
            _ => None
        }
    }

    pub fn keys() -> Locale {
        Locale::new("en", Translations::keys(), WritingDirection::Ltr)
    }

    pub fn new(
        language: &str,
        translations: Translations,
        writing_direction: WritingDirection
    ) -> Locale {
        Locale {
            language: language.to_owned(),
            translations,
            writing_direction
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
            audio_format_description_aac: String::from("audio_format_description_aac"),
            audio_format_description_aiff: String::from("audio_format_description_aiff"),
            audio_format_description_flac: String::from("audio_format_description_flac"),
            audio_format_description_mp3_vbr: String::from("audio_format_description_mp3_vbr"),
            audio_format_description_ogg_vorbis: String::from("audio_format_description_ogg_vorbis"),
            audio_format_description_opus_48: String::from("audio_format_description_opus_48"),
            audio_format_description_opus_96: String::from("audio_format_description_opus_96"),
            audio_format_description_opus_128: String::from("audio_format_description_opus_128"),
            audio_format_description_wav: String::from("audio_format_description_wav"),
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
            download_choice_hints: String::from("download_choice_hints"),
            embed: String::from("embed"),
            embed_entire_release: String::from("embed_entire_release"),
            enter_code_here: String::from("enter_code_here"),
            entire_release: String::from("entire_release"),
            failed: String::from("failed"),
            feed: String::from("feed"),
            fixed_price: String::from("fixed_price"),
            format_guide: String::from("format_guide"),
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
            .replace("{page_hash}", page_hash)
            .replace("{index_suffix}", index_suffix)
    }

    pub fn up_to_xxx(&self, xxx: &str) -> String {
        self.up_to_xxx.replace("{xxx}", xxx)
    }

    pub fn xxx_or_more(&self, xxx: &str) -> String {
        self.xxx_or_more.replace("{xxx}", xxx)
    }
}