use iso_currency::Currency;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadOption {
    Codes {
        codes: Vec<String>,
        unlock_text: Option<String>
    },
    Disabled,
    Free,
    Paid(Currency, Range<f32>)
}

impl DownloadOption {
    /// Scans a price string, returns a DownloadOption::Paid (or DownloadOption::Free if the price is exactly 0).
    /// Valid price strings look like this: "EUR 4+", "3 USD", "1-9 CAN", etc.
    pub fn new_from_price_string(string: &str) -> Result<DownloadOption, String> {
        let parse_price = |currency: Currency, amount: &str| {
            if let Some(amount_min_str) = amount.strip_suffix('+') {
                if let Ok(amount_min) = amount_min_str.parse::<f32>() {
                    return Ok(DownloadOption::Paid(currency, amount_min..f32::INFINITY));
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
                                if amount_parsed == 0.0 && max_amount_parsed == 0.0 {
                                    return Ok(DownloadOption::Free);
                                }

                                return Ok(DownloadOption::Paid(currency, amount_parsed..max_amount_parsed));
                            } else {
                                return Err(String::from("Minimum price can not be higher than maximum price"));
                            }
                        } else {
                            return Err(String::from("Malformed maximum price"));
                        }
                    }
                } else {
                    if amount_parsed == 0.0 {
                        return Ok(DownloadOption::Free);
                    }

                    return Ok(DownloadOption::Paid(currency, amount_parsed..amount_parsed));
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