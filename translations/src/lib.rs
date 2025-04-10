// SPDX-FileCopyrightText: 2024-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ops::Deref;

mod ca;
mod de;
mod en;
mod es;
mod fr;
mod he;
mod it;
mod ja;
mod lt;
mod nb;
mod nl;
mod pl;
mod pt_pt;
mod ru;
mod sr_cyrl;
mod sr_latn;
mod sv;
mod tr;
mod uk;

pub use ca::CA;
pub use de::DE;
pub use en::EN;
pub use es::ES;
pub use fr::FR;
pub use he::HE;
pub use it::IT;
pub use ja::JA;
pub use lt::LT;
pub use nb::NB;
pub use nl::NL;
pub use pl::PL;
pub use pt_pt::PT_PT;
pub use ru::RU;
pub use sr_cyrl::SR_CYRL;
pub use sr_latn::SR_LATN;
pub use sv::SV;
pub use tr::TR;
pub use uk::UK;

pub use Translation::{Reviewed, Unreviewed, Untranslated};

pub fn all_languages() -> Vec<LabelledTranslations> {
    vec![
        LabelledTranslations { code: "ca", name: "Catalan", translations: CA },
        LabelledTranslations { code: "de", name: "German", translations: DE },
        LabelledTranslations { code: "en", name: "English", translations: EN },
        LabelledTranslations { code: "es", name: "Spanish", translations: ES },
        LabelledTranslations { code: "fr", name: "French", translations: FR },
        LabelledTranslations { code: "he", name: "Hebrew", translations: HE },
        LabelledTranslations { code: "it", name: "Italian", translations: IT },
        LabelledTranslations { code: "ja", name: "Japanese", translations: JA },
        LabelledTranslations { code: "lt", name: "Lithuanian", translations: LT },
        LabelledTranslations { code: "nb", name: "Norwegian Bokmål", translations: NB },
        LabelledTranslations { code: "nl", name: "Dutch", translations: NL },
        LabelledTranslations { code: "pl", name: "Polish", translations: PL },
        LabelledTranslations { code: "pt-pt", name: "Portuguese (European)", translations: PT_PT },
        LabelledTranslations { code: "ru", name: "Russian", translations: RU },
        LabelledTranslations { code: "sr-cyrl", name: "Serbian (Cyrillic)", translations: SR_CYRL },
        LabelledTranslations { code: "sr-latn", name: "Serbian (Latin)", translations: SR_LATN },
        LabelledTranslations { code: "sv", name: "Swedish", translations: SV },
        LabelledTranslations { code: "tr", name: "Turkish", translations: TR },
        LabelledTranslations { code: "uk", name: "Ukrainian", translations: UK }
    ]
}

pub fn new_language() -> LabelledTranslations {
    LabelledTranslations { code: "..", name: "New Language", translations: Translations::UNTRANSLATED }
}

pub struct LabelledTranslations {
    pub code: &'static str,
    pub name: &'static str,
    pub translations: Translations
}

/// These variants serve as markers for whether the translation has been
/// checked by at least one native speaker or expert of a given language.
pub enum Translation {
    Reviewed(&'static str),
    Unreviewed(&'static str),
    Untranslated(&'static str)
}

impl Translation {
    pub const fn as_untranslated(&self) -> Translation {
        match self {
            Reviewed(string) => Untranslated(string),
            Unreviewed(string) => Untranslated(string),
            Untranslated(string) => Untranslated(string)
        }
    }

    pub fn is_unreviewed(&self) -> bool {
        match self {
            Reviewed(_) => false,
            Unreviewed(_) => true,
            Untranslated(_) => false
        }
    }

    pub fn is_untranslated(&self) -> bool {
        match self {
            Reviewed(_) => false,
            Unreviewed(_) => false,
            Untranslated(_) => true
        }
    }

    pub fn status(&self) -> &'static str {
        match self {
            Reviewed(_) => "reviewed",
            Unreviewed(_) => "unreviewed",
            Untranslated(_) => "untranslated"
        }
    }
}

impl Deref for Translation {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            Reviewed(value) => value,
            Unreviewed(value) => value,
            Untranslated(value) => value
        }
    }
}

impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Reviewed(value) => value,
            Unreviewed(value) => value,
            Untranslated(value) => value
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
    pub browse: Translation,
    pub buy: Translation,
    pub close: Translation,
    pub copied: Translation,
    pub copy: Translation,
    pub copy_link: Translation,
    pub confirm: Translation,
    pub r#continue: Translation,
    pub cover_image: Translation,
    pub default_unlock_info: Translation,
    pub download: Translation,
    pub download_code_seems_incorrect: Translation,
    pub downloads: Translation,
    /// Must be unique and only contain url-safe characters
    pub downloads_permalink: Translation,
    pub embed: Translation,
    pub embed_entire_release: Translation,
    pub enter_code_here: Translation,
    pub external_link: Translation,
    // Must only contain filesystem-safe characters (because it is interpolated
    // inside the name of track extras directories inside release archives).
    pub extras: Translation,
    pub failed: Translation,
    pub feed: Translation,
    /// Must be unique and only contain url-safe characters
    pub fixed_price: Translation,
    pub generic_rss: Translation,
    pub image_descriptions: Translation,
    pub image_descriptions_guide: Translation,
    /// Must be unique and only contain url-safe characters
    pub image_descriptions_permalink: Translation,
    pub javascript_is_disabled_listen_at_xxx: Translation,
    pub javascript_is_disabled_text: Translation,
    pub listen: Translation,
    pub loading: Translation,
    pub m3u_playlist: Translation,
    pub made_or_arranged_payment: Translation,
    pub missing_image_description_note: Translation,
    pub more: Translation,
    pub mute: Translation,
    pub name_your_price: Translation,
    pub next_track: Translation,
    pub nothing_found_for_xxx: Translation,
    pub pause: Translation,
    pub play: Translation,
    pub playback_position: Translation,
    pub player_closed: Translation,
    pub player_open_playing_xxx: Translation,
    pub player_open_with_xxx: Translation,
    pub previous_track: Translation,
    pub purchase_downloads: Translation,
    /// Must be unique and only contain url-safe characters
    pub purchase_permalink: Translation,
    pub recommended_format: Translation,
    pub search: Translation,
    pub showing_featured_items: Translation,
    pub showing_xxx_results_for_xxx: Translation,
    pub skip_to_main_content: Translation,
    pub subscribe: Translation,
    pub subscribe_permalink: Translation,
    pub unlisted: Translation,
    pub unlock: Translation,
    pub unlock_downloads: Translation,
    pub unlock_manual_instructions: Translation,
    /// Must be unique and only contain url-safe characters
    pub unlock_permalink: Translation,
    pub unmute: Translation,
    pub up_to_xxx: Translation,
    pub visual_impairment: Translation,
    pub volume: Translation,
    pub xxx_and_others: Translation,
    pub xxx_hours: Translation,
    pub xxx_minutes: Translation,
    pub xxx_or_more: Translation,
    pub xxx_seconds: Translation
}

impl Translations {
    pub const KEYS: Translations = Translations {
        audio_format_alac: Reviewed("audio_format_alac"),
        audio_format_average: Reviewed("audio_format_average"),
        audio_format_flac: Reviewed("audio_format_flac"),
        audio_format_mp3: Reviewed("audio_format_mp3"),
        audio_format_opus_48: Reviewed("audio_format_opus_48"),
        audio_format_opus_96: Reviewed("audio_format_opus_96"),
        audio_format_opus_128: Reviewed("audio_format_opus_128"),
        audio_format_uncompressed: Reviewed("audio_format_uncompressed"),
        audio_player_widget_for_xxx: Reviewed("audio_player_widget_for_xxx"),
        auto_generated_cover: Reviewed("auto_generated_cover"),
        available_formats: Reviewed("available_formats"),
        browse: Reviewed("browse"),
        buy: Reviewed("buy"),
        close: Reviewed("close"),
        copied: Reviewed("copied"),
        copy: Reviewed("copy"),
        copy_link: Reviewed("copy_link"),
        confirm: Reviewed("confirm"),
        r#continue: Reviewed("continue"),
        cover_image: Reviewed("cover_image"),
        default_unlock_info: Reviewed("default_unlock_info"),
        download: Reviewed("download"),
        download_code_seems_incorrect: Reviewed("download_code_seems_incorrect"),
        downloads: Reviewed("downloads"),
        downloads_permalink: Reviewed("downloads_permalink"),
        embed: Reviewed("embed"),
        embed_entire_release: Reviewed("embed_entire_release"),
        enter_code_here: Reviewed("enter_code_here"),
        external_link: Reviewed("external_link"),
        extras: Reviewed("extras"),
        failed: Reviewed("failed"),
        feed: Reviewed("feed"),
        fixed_price: Reviewed("fixed_price"),
        generic_rss: Reviewed("generic_rss"),
        image_descriptions: Reviewed("image_descriptions"),
        image_descriptions_guide: Reviewed("image_descriptions_guide"),
        image_descriptions_permalink: Reviewed("image_descriptions_permalink"),
        javascript_is_disabled_listen_at_xxx: Reviewed("javascript_is_disabled_listen_at_xxx"),
        javascript_is_disabled_text: Reviewed("javascript_is_disabled_text"),
        listen: Reviewed("listen"),
        loading: Reviewed("loading"),
        m3u_playlist: Reviewed("m3u_playlist"),
        made_or_arranged_payment: Reviewed("made_or_arranged_payment"),
        missing_image_description_note: Reviewed("missing_image_description_note"),
        more: Reviewed("more"),
        mute: Reviewed("mute"),
        name_your_price: Reviewed("name_your_price"),
        next_track: Reviewed("next_track"),
        nothing_found_for_xxx: Reviewed("next_track"),
        pause: Reviewed("pause"),
        play: Reviewed("play"),
        player_closed: Reviewed("player_closed"),
        playback_position: Reviewed("playback_position"),
        player_open_playing_xxx: Reviewed("player_open_playing_xxx"),
        player_open_with_xxx: Reviewed("player_open_with_xxx"),
        previous_track: Reviewed("previous_track"),
        purchase_downloads: Reviewed("purchase_downloads"),
        purchase_permalink: Reviewed("purchase_permalink"),
        recommended_format: Reviewed("recommended_format"),
        search: Reviewed("search"),
        showing_featured_items: Reviewed("showing_featured_items"),
        showing_xxx_results_for_xxx: Reviewed("showing_xxx_results_for_xxx"),
        skip_to_main_content: Reviewed("skip_to_main_content"),
        subscribe: Reviewed("subscribe"),
        subscribe_permalink: Reviewed("subscribe_permalink"),
        unlisted: Reviewed("unlisted"),
        unlock: Reviewed("unlock"),
        unlock_downloads: Reviewed("unlock_downloads"),
        unlock_manual_instructions: Reviewed("unlock_manual_instructions"),
        unlock_permalink: Reviewed("unlock_permalink"),
        unmute: Reviewed("unmute"),
        up_to_xxx: Reviewed("up_to_xxx"),
        visual_impairment: Reviewed("visual_impairment"),
        volume: Reviewed("volume"),
        xxx_and_others: Reviewed("xxx_and_others"),
        xxx_hours: Reviewed("xxx_hours"),
        xxx_minutes: Reviewed("xxx_minutes"),
        xxx_or_more: Reviewed("xxx_or_more"),
        xxx_seconds: Reviewed("xxx_seconds")
    };

    pub const UNTRANSLATED: Translations = Translations {
        audio_format_alac: EN.audio_format_alac.as_untranslated(),
        audio_format_average: EN.audio_format_average.as_untranslated(),
        audio_format_flac: EN.audio_format_flac.as_untranslated(),
        audio_format_mp3: EN.audio_format_mp3.as_untranslated(),
        audio_format_opus_128: EN.audio_format_opus_128.as_untranslated(),
        audio_format_opus_48: EN.audio_format_opus_48.as_untranslated(),
        audio_format_opus_96: EN.audio_format_opus_96.as_untranslated(),
        audio_format_uncompressed: EN.audio_format_uncompressed.as_untranslated(),
        audio_player_widget_for_xxx: EN.audio_player_widget_for_xxx.as_untranslated(),
        auto_generated_cover: EN.auto_generated_cover.as_untranslated(),
        available_formats: EN.available_formats.as_untranslated(),
        browse: EN.browse.as_untranslated(),
        buy: EN.buy.as_untranslated(),
        close: EN.close.as_untranslated(),
        confirm: EN.confirm.as_untranslated(),
        r#continue: EN.r#continue.as_untranslated(),
        copied: EN.copied.as_untranslated(),
        copy: EN.copy.as_untranslated(),
        copy_link: EN.copy_link.as_untranslated(),
        cover_image: EN.cover_image.as_untranslated(),
        default_unlock_info: EN.default_unlock_info.as_untranslated(),
        download: EN.download.as_untranslated(),
        download_code_seems_incorrect: EN.download_code_seems_incorrect.as_untranslated(),
        downloads: EN.downloads.as_untranslated(),
        downloads_permalink: EN.downloads_permalink.as_untranslated(),
        embed: EN.embed.as_untranslated(),
        embed_entire_release: EN.embed_entire_release.as_untranslated(),
        enter_code_here: EN.enter_code_here.as_untranslated(),
        external_link: EN.external_link.as_untranslated(),
        extras: EN.extras.as_untranslated(),
        failed: EN.failed.as_untranslated(),
        feed: EN.feed.as_untranslated(),
        fixed_price: EN.fixed_price.as_untranslated(),
        generic_rss: EN.generic_rss.as_untranslated(),
        image_descriptions: EN.image_descriptions.as_untranslated(),
        image_descriptions_guide: EN.image_descriptions_guide.as_untranslated(),
        image_descriptions_permalink: EN.image_descriptions_permalink.as_untranslated(),
        javascript_is_disabled_listen_at_xxx: EN.javascript_is_disabled_listen_at_xxx.as_untranslated(),
        javascript_is_disabled_text: EN.javascript_is_disabled_text.as_untranslated(),
        listen: EN.listen.as_untranslated(),
        loading: EN.loading.as_untranslated(),
        m3u_playlist: EN.m3u_playlist.as_untranslated(),
        made_or_arranged_payment: EN.made_or_arranged_payment.as_untranslated(),
        missing_image_description_note: EN.missing_image_description_note.as_untranslated(),
        more: EN.more.as_untranslated(),
        mute: EN.mute.as_untranslated(),
        name_your_price: EN.name_your_price.as_untranslated(),
        next_track: EN.next_track.as_untranslated(),
        nothing_found_for_xxx: EN.nothing_found_for_xxx.as_untranslated(),
        pause: EN.pause.as_untranslated(),
        play: EN.play.as_untranslated(),
        playback_position: EN.playback_position.as_untranslated(),
        player_closed: EN.player_closed.as_untranslated(),
        player_open_playing_xxx: EN.player_open_playing_xxx.as_untranslated(),
        player_open_with_xxx: EN.player_open_with_xxx.as_untranslated(),
        previous_track: EN.previous_track.as_untranslated(),
        purchase_downloads: EN.purchase_downloads.as_untranslated(),
        purchase_permalink: EN.purchase_permalink.as_untranslated(),
        recommended_format: EN.recommended_format.as_untranslated(),
        search: EN.search.as_untranslated(),
        showing_featured_items: EN.showing_featured_items.as_untranslated(),
        showing_xxx_results_for_xxx: EN.showing_xxx_results_for_xxx.as_untranslated(),
        skip_to_main_content: EN.skip_to_main_content.as_untranslated(),
        subscribe: EN.subscribe.as_untranslated(),
        subscribe_permalink: EN.subscribe_permalink.as_untranslated(),
        unlisted: EN.unlisted.as_untranslated(),
        unlock: EN.unlock.as_untranslated(),
        unlock_downloads: EN.unlock_downloads.as_untranslated(),
        unlock_manual_instructions: EN.unlock_manual_instructions.as_untranslated(),
        unlock_permalink: EN.unlock_permalink.as_untranslated(),
        unmute: EN.unmute.as_untranslated(),
        up_to_xxx: EN.up_to_xxx.as_untranslated(),
        visual_impairment: EN.visual_impairment.as_untranslated(),
        volume: EN.volume.as_untranslated(),
        xxx_and_others: EN.xxx_and_others.as_untranslated(),
        xxx_hours: EN.xxx_hours.as_untranslated(),
        xxx_minutes: EN.xxx_minutes.as_untranslated(),
        xxx_or_more: EN.xxx_or_more.as_untranslated(),
        xxx_seconds: EN.xxx_seconds.as_untranslated()
    };

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
            ("browse", &self.browse, false),
            ("buy", &self.buy, false),
            ("close", &self.close, false),
            ("copied", &self.copied, false),
            ("copy", &self.copy, false),
            ("copy_link", &self.copy_link, false),
            ("confirm", &self.confirm, false),
            ("continue", &self.r#continue, false),
            ("cover_image", &self.cover_image, false),
            ("default_unlock_info", &self.default_unlock_info, false),
            ("download", &self.download, false),
            ("download_code_seems_incorrect", &self.download_code_seems_incorrect, false),
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
            ("generic_rss", &self.generic_rss, false),
            ("image_descriptions", &self.image_descriptions, false),
            ("image_descriptions_guide", &self.image_descriptions_guide, true),
            ("image_descriptions_permalink", &self.image_descriptions_permalink, false),
            ("javascript_is_disabled_listen_at_xxx", &self.javascript_is_disabled_listen_at_xxx, false),
            ("javascript_is_disabled_text", &self.javascript_is_disabled_text, false),
            ("listen", &self.listen, false),
            ("loading", &self.loading, false),
            ("m3u_playlist", &self.m3u_playlist, false),
            ("made_or_arranged_payment", &self.made_or_arranged_payment, false),
            ("missing_image_description_note", &self.missing_image_description_note, false),
            ("more", &self.more, false),
            ("mute", &self.mute, false),
            ("name_your_price", &self.name_your_price, false),
            ("next_track", &self.next_track, false),
            ("nothing_found_for_xxx", &self.nothing_found_for_xxx, false),
            ("pause", &self.pause, false),
            ("play", &self.play, false),
            ("playback_position", &self.playback_position, false),
            ("player_closed", &self.player_closed, false),
            ("player_open_playing_xxx", &self.player_open_playing_xxx, false),
            ("player_open_with_xxx", &self.player_open_with_xxx, false),
            ("previous_track", &self.previous_track, false),
            ("purchase_downloads", &self.purchase_downloads, false),
            ("purchase_permalink", &self.purchase_permalink, false),
            ("recommended_format", &self.recommended_format, false),
            ("search", &self.search, false),
            ("showing_featured_items", &self.showing_featured_items, false),
            ("showing_xxx_results_for_xxx", &self.showing_xxx_results_for_xxx, false),
            ("skip_to_main_content", &self.skip_to_main_content, false),
            ("subscribe", &self.subscribe, false),
            ("subscribe_permalink", &self.subscribe_permalink, false),
            ("unlisted", &self.unlisted, false),
            ("unlock", &self.unlock, false),
            ("unlock_downloads", &self.unlock_downloads, false),
            ("unlock_manual_instructions", &self.unlock_manual_instructions, true),
            ("unlock_permalink", &self.unlock_permalink, false),
            ("unmute", &self.unmute, false),
            ("up_to_xxx", &self.up_to_xxx, false),
            ("visual_impairment", &self.visual_impairment, false),
            ("volume", &self.volume, false),
            ("xxx_and_others", &self.xxx_and_others, false),
            ("xxx_hours", &self.xxx_hours, false),
            ("xxx_minutes", &self.xxx_minutes, false),
            ("xxx_or_more", &self.xxx_or_more, false),
            ("xxx_seconds", &self.xxx_seconds, false)
        ]
    }

    pub fn audio_player_widget_for_xxx(&self, title: &str) -> String {
        self.audio_player_widget_for_xxx.replace("{title}", title)
    }

    pub fn count_untranslated(&self) -> usize {
        self.all_strings()
            .iter()
            .filter(|(_key, value, _is_multiline)| value.is_untranslated())
            .count()
    }

    pub fn count_unreviewed(&self) -> usize {
        self.all_strings()
            .iter()
            .filter(|(_key, value, _is_multiline)| value.is_unreviewed())
            .count()
    }

    pub fn javascript_is_disabled_listen_at_xxx(&self, link: &str) -> String {
        self.javascript_is_disabled_listen_at_xxx.replace("{link}", link)
    }

    pub fn percent_reviewed(&self) -> f32 {
        let mut total = 0;
        let mut reviewed = 0;

        for (_key, value, _is_multiline) in self.all_strings() {
            total += 1;

            match value {
                Reviewed(_) => {
                    reviewed += 1;
                }
                Unreviewed(_) |
                Untranslated(_) => ()
            }
        }

        (reviewed as f32 / total as f32) * 100.0
    }

    pub fn percent_translated(&self) -> f32 {
        let mut total = 0;
        let mut translated = 0;

        for (_key, value, _is_multiline) in self.all_strings() {
            total += 1;

            match value {
                Reviewed(_) |
                Unreviewed(_) => {
                    translated += 1;
                }
                Untranslated(_) => ()
            }
        }

        (translated as f32 / total as f32) * 100.0
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
    use sanitize_filename::sanitize;

    const LOCALES: &[Translations] = &[
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
        TR
    ];

    for translations in LOCALES {
        assert!(&translations.audio_player_widget_for_xxx.contains("{title}"));
        assert!(&translations.javascript_is_disabled_listen_at_xxx.contains("{link}"));
        assert!(&translations.nothing_found_for_xxx.contains("{query}"));
        assert!(&translations.player_open_playing_xxx.contains("{title}"));
        assert!(&translations.player_open_with_xxx.contains("{title}"));
        assert!(&translations.showing_xxx_results_for_xxx.contains("{count}"));
        assert!(&translations.showing_xxx_results_for_xxx.contains("{query}"));
        assert!(&translations.unlock_manual_instructions.contains("{downloads_permalink}"));
        assert!(&translations.unlock_manual_instructions.contains("{index_suffix}"));
        assert!(&translations.unlock_manual_instructions.contains("{page_hash}"));
        assert!(&translations.unlock_manual_instructions.contains("{unlock_permalink}"));
        assert!(&translations.up_to_xxx.contains("{xxx}"));
        assert!(&translations.xxx_and_others.contains("{xxx}"));
        assert!(&translations.xxx_and_others.contains("{others_link}"));
        assert!(&translations.xxx_hours.contains("{xxx}"));
        assert!(&translations.xxx_minutes.contains("{xxx}"));
        assert!(&translations.xxx_or_more.contains("{xxx}"));
        assert!(&translations.xxx_seconds.contains("{xxx}"));

        let disallowed_char = |c: char| !c.is_ascii_alphanumeric() && c != '-';

        assert!(!&translations.downloads_permalink.contains(disallowed_char));
        assert!(!&translations.image_descriptions_permalink.contains(disallowed_char));
        assert!(!&translations.purchase_permalink.contains(disallowed_char));
        assert!(!&translations.subscribe_permalink.contains(disallowed_char));
        assert!(!&translations.unlock_permalink.contains(disallowed_char));

        // The translation for "Extras" must be file-system safe because we interpolate
        // it into the track extras directory names when be build zip archives for releases.
        assert!(*translations.extras == sanitize(*translations.extras));
    }
}
