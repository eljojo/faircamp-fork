use iso_currency::Currency;
use std::ops::Range;

use crate::util;

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadOption {
    Disabled,
    Free {
        download_page_uid: String
    },
    Paid {
        checkout_page_uid: String,
        currency: Currency,
        download_page_uid: String,
        range: Range<f32>
    }
}

impl DownloadOption {
    pub fn init_free() -> DownloadOption {
        DownloadOption::Free{
            download_page_uid: util::uid()
        }
    }
    
    pub fn init_paid(currency: Currency, range: Range<f32>) -> DownloadOption {
        DownloadOption::Paid {
            checkout_page_uid: util::uid(),
            currency,
            download_page_uid: util::uid(),
            range
        }
    }
}