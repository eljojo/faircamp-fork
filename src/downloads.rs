// SPDX-FileCopyrightText: 2021-2024 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ops::Range;

use iso_currency::Currency;

use crate::Overrides;

#[derive(Clone, Debug)]
pub enum DownloadAccess {
    Code {
        download_codes: Vec<String>,
        unlock_info: Option<String>
    },
    Disabled,
    External {
        link: String
    },
    Free,
    Paycurtain {
        price: Price,
        payment_info: Option<String>,
    }
}

/// This is the "proto-version" of DownloadAccess, which for us stores just
/// the general setting that is supplied through "release_download_access"
/// and "track_download_access" in the manifests. When we store download
/// access settings on discrete instances of Release or Track, we combine
/// this information with payment, price and/or unlock info and form the
/// final DownloadAccess enum (see the assemble() method).
#[derive(Clone, Debug)]
pub enum DownloadAccessOption {
    Code,
    Disabled,
    External { link: String },
    Free,
    Paycurtain
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

impl DownloadAccessOption {
    /// Combines DownloadAccess with payment, price and/or unlock info
    /// in order to form the final DownloadAccess data.
    pub fn assemble(
        &self,
        overrides: &Overrides,
        price: &Price
    ) -> DownloadAccess {
        match &self {
            DownloadAccessOption::Code => DownloadAccess::Code {
                download_codes: overrides.download_codes.clone(),
                unlock_info: overrides.unlock_info.clone()
            },
            DownloadAccessOption::Disabled => DownloadAccess::Disabled,
            DownloadAccessOption::External { link } => DownloadAccess::External { link: link.clone() },
            DownloadAccessOption::Free => DownloadAccess::Free,
            DownloadAccessOption::Paycurtain => DownloadAccess::Paycurtain {
                payment_info: overrides.payment_info.clone(),
                price: price.clone()
            }
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
