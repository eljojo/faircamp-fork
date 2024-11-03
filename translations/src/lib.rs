// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// The reviewed!("Example") macro basically just serves as a marker for
/// a translation, to say it has been checked by at least one native speaker
/// or expert of a given language. Otherwise no transformation occurs.
macro_rules! reviewed {
    ($_value:literal) => {
        $_value
    };
}

macro_rules! unreviewed {
    ($_value:literal) => {
        $_value
    };
}

/// In debug builds all untranslated strings return "UNTRANSLATED",
#[cfg(all(debug_assertions, not(test)))]
macro_rules! untranslated {
    ($_key:ident) => {
        "UNTRANSLATED"
    };
}

// In release and test builds all untranslated strings return an english fallback translation
#[cfg(any(not(debug_assertions), test))]
macro_rules! untranslated {
    ($key:ident) => {
        crate::en::translations().$key
    };
}

pub mod de;
pub mod en;
pub mod es;
pub mod fr;
pub mod he;
pub mod it;
pub mod nb;
pub mod nl;
pub mod pl;
pub mod sv;
pub mod tr;
pub mod uk;

pub fn all_translations() -> Vec<(&'static str, Translations)> {
	vec![
		("de", de::translations()),
		("en", en::translations()),
		("es", es::translations()),
		("fr", fr::translations()),
		("he", he::translations()),
		("it", it::translations()),
		("nb", nb::translations()),
		("nl", nl::translations()),
		("pl", pl::translations()),
		("sv", sv::translations()),
		("tr", tr::translations()),
		("uk", uk::translations())
	]
}

/// A key-value mapping for every translatable string found in the interface.
/// Used at build time to interpolate text in the right language.
/// Translations whose fields are not public are instead accessed through
/// a method of the same name - these are translations that need to be called
/// as a function because they interpolate some parameter into the translation.
pub struct Translations {
    pub audio_format_alac: &'static str,
    pub audio_format_average: &'static str,
    pub audio_format_flac: &'static str,
    pub audio_format_mp3: &'static str,
    pub audio_format_opus_48: &'static str,
    pub audio_format_opus_96: &'static str,
    pub audio_format_opus_128: &'static str,
    pub audio_format_uncompressed: &'static str,
    pub audio_player_widget_for_release: &'static str,
    pub audio_player_widget_for_track: &'static str,
    pub auto_generated_cover: &'static str,
    pub available_formats: &'static str,
    pub buy: &'static str,
    pub copied: &'static str,
    pub copy: &'static str,
    pub copy_link: &'static str,
    pub confirm: &'static str,
    pub r#continue: &'static str,
    pub cover_image: &'static str,
    pub default_unlock_text: &'static str,
    pub dimmed: &'static str,
    pub download: &'static str,
    pub downloads: &'static str,
    /// Must be unique and only contain url-safe characters
    pub downloads_permalink: &'static str,
    pub embed: &'static str,
    pub embed_entire_release: &'static str,
    pub enter_code_here: &'static str,
    pub external_link: &'static str,
    pub extras: &'static str,
    pub failed: &'static str,
    pub feed: &'static str,
    pub fixed_price: &'static str,
    pub image_descriptions: &'static str,
    pub image_descriptions_guide: &'static str,
    /// Must be unique and only contain url-safe characters
    pub image_descriptions_permalink: &'static str,
    pub listen: &'static str,
    pub loading: &'static str,
    pub m3u_playlist: &'static str,
    pub made_or_arranged_payment: &'static str,
    pub missing_image_description_note: &'static str,
    pub more: &'static str,
    pub muted: &'static str,
    pub name_your_price: &'static str,
    pub next_track: &'static str,
    pub pause: &'static str,
    pub play: &'static str,
    pub previous_track: &'static str,
    pub purchase_downloads: &'static str,
    /// Must be unique and only contain url-safe characters
    pub purchase_permalink: &'static str,
    pub recommended_format: &'static str,
    pub rss_feed: &'static str,
    pub this_site_was_created_with_faircamp: &'static str,
    pub unlisted: &'static str,
    pub unlock: &'static str,
    pub unlock_downloads: &'static str,
    /// Must be unique and only contain url-safe characters
    pub unlock_permalink: &'static str,
    pub unlock_code_seems_incorrect: &'static str,
    pub unlock_manual_instructions: &'static str,
    pub up_to_xxx: &'static str,
    pub visual_impairment: &'static str,
    pub volume: &'static str,
    pub xxx_and_others: &'static str,
    pub xxx_or_more: &'static str
}

impl Translations {
	pub fn all_strings(&self) -> Vec<(&'static str, &str)> {
		vec![
			("audio_format_alac", &self.audio_format_alac),
			("audio_format_average", &self.audio_format_average),
			("audio_format_flac", &self.audio_format_flac),
			("audio_format_mp3", &self.audio_format_mp3),
			("audio_format_opus_48", &self.audio_format_opus_48),
			("audio_format_opus_96", &self.audio_format_opus_96),
			("audio_format_opus_128", &self.audio_format_opus_128),
			("audio_format_uncompressed", &self.audio_format_uncompressed),
			("audio_player_widget_for_release", &self.audio_player_widget_for_release),
			("audio_player_widget_for_track", &self.audio_player_widget_for_track),
			("auto_generated_cover", &self.auto_generated_cover),
			("available_formats", &self.available_formats),
			("buy", &self.buy),
			("copied", &self.copied),
			("copy", &self.copy),
			("copy_link", &self.copy_link),
			("confirm", &self.confirm),
			("continue", &self.r#continue),
			("cover_image", &self.cover_image),
			("default_unlock_text", &self.default_unlock_text),
			("dimmed", &self.dimmed),
			("download", &self.download),
			("downloads", &self.downloads),
			("downloads_permalink", &self.downloads_permalink),
			("embed", &self.embed),
			("embed_entire_release", &self.embed_entire_release),
			("enter_code_here", &self.enter_code_here),
			("external_link", &self.external_link),
			("extras", &self.extras),
			("failed", &self.failed),
			("feed", &self.feed),
			("fixed_price", &self.fixed_price),
			("image_descriptions", &self.image_descriptions),
			("image_descriptions_guide", &self.image_descriptions_guide),
			("image_descriptions_permalink", &self.image_descriptions_permalink),
			("listen", &self.listen),
			("loading", &self.loading),
			("m3u_playlist", &self.m3u_playlist),
			("made_or_arranged_payment", &self.made_or_arranged_payment),
			("missing_image_description_note", &self.missing_image_description_note),
			("more", &self.more),
			("muted", &self.muted),
			("name_your_price", &self.name_your_price),
			("next_track", &self.next_track),
			("pause", &self.pause),
			("play", &self.play),
			("previous_track", &self.previous_track),
			("purchase_downloads", &self.purchase_downloads),
			("purchase_permalink", &self.purchase_permalink),
			("recommended_format", &self.recommended_format),
			("rss_feed", &self.rss_feed),
			("this_site_was_created_with_faircamp", &self.this_site_was_created_with_faircamp),
			("unlisted", &self.unlisted),
			("unlock", &self.unlock),
			("unlock_downloads", &self.unlock_downloads),
			("unlock_permalink", &self.unlock_permalink),
			("unlock_code_seems_incorrect", &self.unlock_code_seems_incorrect),
			("unlock_manual_instructions", &self.unlock_manual_instructions),
			("up_to_xxx", &self.up_to_xxx),
			("visual_impairment", &self.visual_impairment),
			("volume", &self.volume),
			("xxx_and_others", &self.xxx_and_others),
			("xxx_or_more", &self.xxx_or_more)
		]
	}

    pub fn audio_player_widget_for_release(&self, title: &str) -> String {
        self.audio_player_widget_for_release.replace("{title}", title)
    }

    pub fn audio_player_widget_for_track(&self, title: &str) -> String {
        self.audio_player_widget_for_track.replace("{title}", title)
    }

    pub fn keys() -> Translations {
        Translations {
            audio_format_alac: "audio_format_alac",
            audio_format_average: "audio_format_average",
            audio_format_flac: "audio_format_flac",
            audio_format_mp3: "audio_format_mp3",
            audio_format_opus_48: "audio_format_opus_48",
            audio_format_opus_96: "audio_format_opus_96",
            audio_format_opus_128: "audio_format_opus_128",
            audio_format_uncompressed: "audio_format_uncompressed",
            audio_player_widget_for_release: "audio_player_widget_for_release",
            audio_player_widget_for_track: "audio_player_widget_for_track",
            auto_generated_cover: "audio_player_widget_for_track",
            available_formats: "available_formats",
            buy: "buy",
            copied: "copied",
            copy: "copy",
            copy_link: "copy_link",
            confirm: "confirm",
            r#continue: "continue",
            cover_image: "cover_image",
            default_unlock_text: "default_unlock_text",
            dimmed: "dimmed",
            download: "download",
            downloads: "downloads",
            downloads_permalink: "downloads_permalink",
            embed: "embed",
            embed_entire_release: "embed_entire_release",
            enter_code_here: "enter_code_here",
            external_link: "external_link",
            extras: "extras",
            failed: "failed",
            feed: "feed",
            fixed_price: "fixed_price",
            image_descriptions: "image_descriptions",
            image_descriptions_guide: "image_descriptions_guide",
            image_descriptions_permalink: "image_descriptions_permalink",
            listen: "listen",
            loading: "loading",
            m3u_playlist: "m3u_playlist",
            made_or_arranged_payment: "made_or_arranged_payment",
            missing_image_description_note: "missing_image_description_note",
            more: "more",
            muted: "muted",
            name_your_price: "name_your_price",
            next_track: "next_track",
            pause: "pause",
            play: "play",
            previous_track: "previous_track",
            purchase_downloads: "purchase_downloads",
            purchase_permalink: "purchase_permalink",
            recommended_format: "recommended_format",
            rss_feed: "rss_feed",
            this_site_was_created_with_faircamp: "this_site_was_created_with_faircamp",
            unlisted: "unlisted",
            unlock: "unlock",
            unlock_downloads: "unlock_downloads",
            unlock_permalink: "unlock_permalink",
            unlock_code_seems_incorrect: "unlock_code_seems_incorrect",
            unlock_manual_instructions: "unlock_manual_instructions",
            up_to_xxx: "up_to_xxx",
            visual_impairment: "visual_impairment",
            volume: "volume",
            xxx_and_others: "xxx_and_others",
            xxx_or_more: "xxx_or_more"
        }
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
        he::translations(),
        it::translations(),
        nb::translations(),
        nl::translations(),
        pl::translations(),
        sv::translations(),
        tr::translations()
    ];

    for translations in &locales {
        assert!(&translations.audio_player_widget_for_release.contains("{title}"));
        assert!(&translations.audio_player_widget_for_track.contains("{title}"));
        assert!(&translations.this_site_was_created_with_faircamp.contains("{faircamp_link}"));
        assert!(&translations.unlock_manual_instructions.contains("{downloads_permalink}"));
        assert!(&translations.unlock_manual_instructions.contains("{index_suffix}"));
        assert!(&translations.unlock_manual_instructions.contains("{page_hash}"));
        assert!(&translations.unlock_manual_instructions.contains("{unlock_permalink}"));
        assert!(&translations.up_to_xxx.contains("{xxx}"));
        assert!(&translations.xxx_and_others.contains("{xxx}"));
        assert!(&translations.xxx_and_others.contains("{others_link}"));
        assert!(&translations.xxx_or_more.contains("{xxx}"));

        let disallowed_char = |c: char| !c.is_ascii_alphanumeric() && c != '-' ;

        assert!(!&translations.downloads_permalink.contains(disallowed_char));
        assert!(!&translations.image_descriptions_permalink.contains(disallowed_char));
        assert!(!&translations.purchase_permalink.contains(disallowed_char));
        assert!(!&translations.unlock_permalink.contains(disallowed_char));
    }
}
