use iso_currency::Currency;
use std::ops::Range;

use crate::util;

#[derive(Clone, Debug, PartialEq)]
pub enum DownloadOption {
    Disabled,
    Free(String),
    Paid {
        currency: Currency,
        range: Range<f32>
    }
}

impl DownloadOption {
    pub fn init_free() -> DownloadOption {
        let download_uuid = util::uuid();
        
        DownloadOption::Free(download_uuid)
    }
    
    pub fn init_paid(currency: Currency, range: Range<f32>) -> DownloadOption {
        DownloadOption::Paid {
            currency,
            range
        }
    }
}