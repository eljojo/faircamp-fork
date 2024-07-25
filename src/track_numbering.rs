// SPDX-FileCopyrightText: 2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

#[derive(Clone, Debug)]
pub enum TrackNumbering {
    Arabic,
    ArabicDotted,
    ArabicPadded,
    Disabled,
    Hexadecimal,
    HexadecimalPadded,
    Roman,
    RomanDotted
}

impl TrackNumbering {
    pub fn format(&self, number: usize) -> String {
        match self {
            TrackNumbering::Arabic => number.to_string(),
            TrackNumbering::ArabicDotted => format!("{number}."),
            TrackNumbering::ArabicPadded => format!("{number:02}"),
            TrackNumbering::Disabled => String::from(""),
            TrackNumbering::Hexadecimal => format!("0x{number:X}"),
            TrackNumbering::HexadecimalPadded => format!("0x{number:02X}"),
            TrackNumbering::Roman => Self::to_roman(number),
            TrackNumbering::RomanDotted => format!("{}.", Self::to_roman(number))
        }
    }

    pub fn from_manifest_key(key: &str) -> Option<TrackNumbering> {
        match key {
            "arabic" => Some(TrackNumbering::Arabic),
            "arabic-dotted" => Some(TrackNumbering::ArabicDotted),
            "arabic-padded" => Some(TrackNumbering::ArabicPadded),
            "disabled" => Some(TrackNumbering::Disabled),
            "hexadecimal" => Some(TrackNumbering::Hexadecimal),
            "hexadecimal-padded" => Some(TrackNumbering::HexadecimalPadded),
            "roman" => Some(TrackNumbering::Roman),
            "roman-dotted" => Some(TrackNumbering::RomanDotted),
            _ =>  None
        }
    }

    fn to_roman(number: usize) -> String {
        // TODO: Implement to at least ~256 (or more) using proper algorithm
        let roman = match number {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            10 => "X",
            11 => "XI",
            12 => "XII",
            13 => "XIII",
            14 => "XIV",
            15 => "XV",
            16 => "XVI",
            17 => "XVII",
            18 => "XVIII",
            19 => "XIX",
            20 => "XX",
            21 => "XXI",
            22 => "XXII",
            23 => "XXIII",
            24 => "XXIV",
            25 => "XXV",
            26 => "XXVI",
            27 => "XXVII",
            28 => "XXVIII",
            29 => "XXIX",
            30 => "XXX",
            31 => "XXXI",
            32 => "XXXII",
            33 => "XXXIII",
            34 => "XXXIV",
            35 => "XXXV",
            36 => "XXXVI",
            37 => "XXXVII",
            38 => "XXXVIII",
            39 => "XXXIX",
            40 => "XL",
            _ => unimplemented!()
        };

        roman.to_string()
    }
}
