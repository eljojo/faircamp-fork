use crate::util;


// TODO: Revisit option terminology? (in tandem with what the user sets in metadata) e.g. ...
// Disabled
// NoPrice
// AnyPrice
// MinimumPrice
// ExactPrice
#[derive(Clone, Debug)]
pub enum DownloadOption {
    Disabled,
    Free(String),
    NameYourPrice,
    PayExactly(String),
    PayMinimum(String)
}

impl DownloadOption {
    pub fn init_free() -> DownloadOption {
        let download_uuid = util::uuid();
        
        DownloadOption::Free(download_uuid)
    }
}