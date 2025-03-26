// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Translations, Unreviewed};

pub const HE: Translations = Translations {
    buy: Unreviewed("לִקְנוֹת"),
    copy: Unreviewed("לְהַעְתִיק"),
    copy_link: Unreviewed("העתק קישור"),
    download: Unreviewed("הורד"),
    downloads: Unreviewed("הורדות"),
    downloads_permalink: Unreviewed("hordot"),
    embed: Unreviewed("לְשַׁבֵּץ"),
    feed: Unreviewed("לְהַאֲכִיל"),
    listen: Unreviewed("לְהַקְשִׁיב"),
    more: Unreviewed("יוֹתֵר"),
    pause: Unreviewed("הַפסָקָה"),
    play: Unreviewed("לְשַׂחֵק"),
    unlisted: Unreviewed("לא רשום"),
    unlock: Unreviewed("לִפְתוֹחַ"),
    ..Translations::UNTRANSLATED
};
