use iso_currency::Currency;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadOption {
    Codes(Vec<String>),
    Disabled,
    Free,
    Paid(Currency, Range<f32>)
}