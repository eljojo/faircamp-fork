use iso_currency::Currency;
use std::ops::Range;

use crate::util;

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadOption {
    Disabled,
    Free {
        download_page_nanoid: String
    },
    Paid {
        checkout_page_nanoid: String,
        currency: Currency,
        download_page_nanoid: String,
        range: Range<f32>
    }
}

impl DownloadOption {
    pub fn init_free() -> DownloadOption {
        DownloadOption::Free{
            download_page_nanoid: util::nanoid()
        }
    }
    
    pub fn init_paid(currency: Currency, range: Range<f32>) -> DownloadOption {
        DownloadOption::Paid {
            checkout_page_nanoid: util::nanoid(),
            currency,
            download_page_nanoid: util::nanoid(),
            range
        }
    }
}