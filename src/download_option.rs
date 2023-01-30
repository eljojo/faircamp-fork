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