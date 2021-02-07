use uuid::Uuid;

#[derive(Debug)]
pub enum DownloadOption {
    Disabled,
    Free(String),
    NameYourPrice,
    PayExactly(String),
    PayMinimum(String)
}

impl DownloadOption {
    pub fn init_free() -> DownloadOption {
        let download_uuid = Uuid::new_v4().to_string();
        
        DownloadOption::Free(download_uuid)
    }
}