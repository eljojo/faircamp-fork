// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ops::Range;

use iso_currency::Currency;

use crate::DownloadFormat;

#[derive(Clone, Debug)]
pub enum DownloadOption {
    Code,
    Disabled,
    External { link: String },
    Free,
    Paycurtain
}

#[derive(Clone, Debug)]
pub enum DownloadAccess {
    Code {
        download_codes: Vec<String>,
        unlock_info: Option<String>
    },
    Free,
    Paycurtain {
        price: Price,
        payment_info: Option<String>,
    }
}

#[derive(Clone, Debug)]
pub enum Downloads {
    Disabled,
    Enabled {
        download_access: DownloadAccess,
        downloads_config: DownloadsConfig
    },
    External {
        link: String
    }
}

#[derive(Clone, Debug)]
pub struct DownloadsConfig {
    pub archive_formats: Vec<DownloadFormat>,
    pub extra_downloads: ExtraDownloads,
    pub track_formats: Vec<DownloadFormat>
}

#[derive(Clone, Debug)]
pub struct ExtraDownloads {
    pub bundled: bool,
    pub separate: bool
}

#[derive(Clone, Debug)]
pub struct Price {
    pub currency: Currency,
    pub range: Range<f32>
}

impl DownloadsConfig {
    pub fn all_formats(&self) -> Vec<DownloadFormat> {
        let mut all_formats = self.archive_formats.clone();

        for format in &self.track_formats {
            if !all_formats.contains(format) {
                all_formats.push(*format);
            }
        }

        all_formats
    }

    /// (format, write_archive, write_tracks)
    pub fn all_formats_for_writing(&self) -> Vec<(DownloadFormat, bool, bool)> {
        let mut all_specs: Vec<(DownloadFormat, bool, bool)> = self.archive_formats
            .iter()
            .map(|format| (*format, true, false))
            .collect();

        for format in &self.track_formats {
            if let Some(spec) = all_specs.iter_mut().find(|spec| spec.0 == *format) {
                spec.2 = true;
            } else {
                all_specs.push((*format, false, true));
            }
        }

        all_specs
    }

    pub fn all_formats_sorted(&self) -> Vec<DownloadFormat> {
        let mut all_formats = self.all_formats();
        all_formats.sort_by_key(|format| format.download_rank());
        all_formats
    }

    pub fn default() -> DownloadsConfig {
        DownloadsConfig {
            archive_formats: Vec::new(),
            extra_downloads: ExtraDownloads::BUNDLED,
            track_formats: Vec::new()
        }
    }
}

impl ExtraDownloads {
    pub const BUNDLED: ExtraDownloads = ExtraDownloads { bundled: true, separate: false };
    pub const DISABLED: ExtraDownloads = ExtraDownloads { bundled: false, separate: false };
    pub const SEPARATE: ExtraDownloads = ExtraDownloads { bundled: false, separate: true };
}

impl Price {
    pub fn default() -> Price {
        Price {
            currency: Currency::USD,
            range: 0.0..f32::INFINITY
        }
    }

    /// Scans a price string, returns a DownloadOption::Paid (or DownloadOption::Free if the price is exactly 0).
    /// Valid price strings look like this: "EUR 4+", "3 USD", "1-9 CAN", etc.
    pub fn new_from_price_string(string: &str) -> Result<Price, String> {
        let parse_price = |currency: Currency, amount: &str| {
            if let Some(amount_min_str) = amount.strip_suffix('+') {
                if let Ok(amount_min) = amount_min_str.parse::<f32>() {
                    return Ok(Price {
                        currency,
                        range: amount_min..f32::INFINITY
                    });
                } else {
                    return Err(String::from("Malformed minimum price"));
                }
            }

            let mut split_by_dash = amount.split('-');

            if let Ok(amount_parsed) = split_by_dash.next().unwrap().parse::<f32>() {
                if let Some(max_amount) = split_by_dash.next() {
                    if split_by_dash.next().is_none() {
                        if let Ok(max_amount_parsed) = max_amount.parse::<f32>() {
                            if amount_parsed <= max_amount_parsed {
                                return Ok(Price {
                                    currency,
                                    range: amount_parsed..max_amount_parsed
                                });
                            } else {
                                return Err(String::from("Minimum price can not be higher than maximum price"));
                            }
                        } else {
                            return Err(String::from("Malformed maximum price"));
                        }
                    }
                } else {
                    return Ok(Price {
                        currency,
                        range: amount_parsed..amount_parsed
                    });
                }
            }

            Err(String::from("Malformed price"))
        };

        let mut split_by_whitespace = string.split_ascii_whitespace();

        if let Some(first_token) = split_by_whitespace.next() {
            if let Some(second_token) = split_by_whitespace.next() {
                if split_by_whitespace.next().is_none() {
                    if let Some(currency) = Currency::from_code(first_token) {
                        return parse_price(currency, second_token);
                    } else if let Some(currency) = Currency::from_code(second_token) {
                        return parse_price(currency, first_token);
                    } else {
                        return Err(String::from("Currency code not recognized"));
                    }
                }
            }
        }

        Err(String::from("Price format must consist of two tokens"))
    }
}
