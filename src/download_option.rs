#[derive(Debug)]
pub enum DownloadOption {
    Disabled,
    Free,
    NameYourPrice,
    PayExactly(String),
    PayMinimum(String)
}