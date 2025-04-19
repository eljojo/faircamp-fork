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

    // Converts a number to a roman numeral, using standard "modern" style
    // roman numerals (i.e. with subtractive notation and in the range
    // 1-3999).
    fn to_roman(number: usize) -> String {
        let thousands = match number / 1000 {
            0 => "",
            1 => "M",
            2 => "MM",
            3 => "MMM",
            _ => panic!("Modern style roman numerals can only represent numbers up to 3999")
        };

        let hundreds = match (number % 1000) / 100 {
            0 => "",
            1 => "C",
            2 => "CC",
            3 => "CCC",
            4 => "CD",
            5 => "D",
            6 => "DC",
            7 => "DCC",
            8 => "DCCC",
            9 => "CM",
            _ => unreachable!()
        };

        let tens = match (number % 100) / 10 {
            0 => "",
            1 => "X",
            2 => "XX",
            3 => "XXX",
            4 => "XL",
            5 => "L",
            6 => "LX",
            7 => "LXX",
            8 => "LXXX",
            9 => "XC",
            _ => unreachable!()
        };

        let ones = match number % 10 {
            0 => "",
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            _ => unreachable!()
        };

        format!("{thousands}{hundreds}{tens}{ones}")
    }
}
