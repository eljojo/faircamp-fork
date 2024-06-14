// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    pub audio_player_widget_for_release: String,
    pub audio_player_widget_for_track: String,
    pub auto_generated_cover: String,
    pub available_formats: String,
    pub buy: String,
    /// We keep this one around for now as we could possibly use it for a feed selection overlay
    pub close: String,
    pub copied: String,
    pub copy: String,
    pub copy_link: String,
    pub confirm: String,
    pub r#continue: String,
    pub cover_image: String,
    pub default_unlock_text: String,
    pub download: String,
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
    pub loading: String,
    pub made_or_arranged_payment: String,
    pub missing_image_description_note: String,
    pub name_your_price: String,
    pub option: String,
    pub pause: String,
    pub pay_on_liberapay: String,
    pub payment_options: String,
    pub play: String,
    pub purchase_downloads: String,
    /// Must be unique and only contain url-safe characters
    pub purchase_permalink: String,
    pub recommended_format: String,
    pub rss_feed: String,
    pub share: String,
    pub unlisted: String,
    pub unlock: String,
    pub unlock_downloads: String,
    /// Must be unique and only contain url-safe characters
    pub unlock_permalink: String,
    pub unlock_code_seems_incorrect: String,
    pub unlock_manual_instructions: String,
    pub up_to_xxx: String,
    pub visual_impairment: String,
    pub xxx_or_more: String
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
            buy: String::from("buy"),
            close: String::from("close"),
            copied: String::from("copied"),
            copy: String::from("copy"),
            copy_link: String::from("copy_link"),
            confirm: String::from("confirm"),
            r#continue: String::from("continue"),
            cover_image: String::from("cover_image"),
            default_unlock_text: String::from("default_unlock_text"),
            download: String::from("download"),
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
            loading: String::from("loading"),
            made_or_arranged_payment: String::from("made_or_arranged_payment"),
            missing_image_description_note: String::from("missing_image_description_note"),
            name_your_price: String::from("name_your_price"),
            option: String::from("option"),
            pause: String::from("pause"),
            pay_on_liberapay: String::from("pay_on_liberapay"),
            payment_options: String::from("payment_options"),
            play: String::from("play"),
            purchase_downloads: String::from("purchase_downloads"),
            purchase_permalink: String::from("purchase_permalink"),
            recommended_format: String::from("recommended_format"),
            rss_feed: String::from("rss_feed"),
            share: String::from("share"),
            unlisted: String::from("unlisted"),
            unlock: String::from("unlock"),
            unlock_downloads: String::from("unlock_downloads"),
            unlock_permalink: String::from("unlock_permalink"),
            unlock_code_seems_incorrect: String::from("unlock_code_seems_incorrect"),
            unlock_manual_instructions: String::from("unlock_manual_instructions"),
            up_to_xxx: String::from("up_to_xxx"),
            visual_impairment: String::from("visual_impairment"),
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
