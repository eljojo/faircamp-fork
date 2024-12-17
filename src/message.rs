// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

macro_rules! color {
    (blue) => ("\x1b[34m");
    (cyan) => ("\x1b[36m");
    (magenta) => ("\x1b[35m");
    (red) => ("\x1b[31m");
    (reset) => ("\x1b[0m");
    (yellow) => ("\x1b[33m");
}

macro_rules! error {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(red), "[ERROR] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(blue), "[INFO] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info_cache {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(magenta), "[CACHE] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info_decoding {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(magenta), "[DECODING] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info_resizing {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(blue), "[RESIZING] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info_stats {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(cyan), "[STATS] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info_transcoding {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(blue), "[TRANSCODING] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! info_zipping {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(blue), "[ZIPPING] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! warn {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(yellow), "[WARNING] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}

macro_rules! warn_discouraged {
    ($format_str:expr $(,$args:expr)*) => {
        println!(
            concat!(color!(yellow), "[DISCOURAGED] ", $format_str, color!(reset))
            $(,$args)*
        )
    };
}
